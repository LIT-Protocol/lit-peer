use std::fs;
use std::path::{Path, PathBuf};

use log::{info, warn};
use posix_acl::{ACL_EXECUTE, ACL_READ, ACL_WRITE};

use lit_core::config::{CFG_DIR_GUEST_INIT, CFG_EXT, CFG_NAME};
use lit_core::utils::toml::SimpleToml;
use lit_os_core::config::LitOsGuestConfig;
use lit_os_core::error::{Result, config_err, generic_err, io_err, validation_err};
use lit_os_core::guest::cloud_init::context::CloudInitContext;
use lit_os_core::guest::cloud_init::meta_data::CloudInitMetaData;
use lit_os_core::guest::cloud_init::network_config::CloudInitNetworkConfig;
use lit_os_core::guest::cloud_init::user_data::CloudInitUserData;
use lit_os_core::guest::cloud_init::{
    ALLOWED_CLOUD_INIT_FILES, CLOUD_INIT_FILE_INIT_PW, CLOUD_INIT_FILE_META_DATA,
    CLOUD_INIT_FILE_NETWORK_CONFIG, CLOUD_INIT_FILE_NO_RESIZE, CLOUD_INIT_FILE_RPC_CONFIG_OVERLAY,
    CLOUD_INIT_FILE_SAFE_BOOT, CLOUD_INIT_FILE_USER_DATA,
};
use lit_os_core::guest::types::GuestType;
use lit_os_core::utils::mount::mount;

use crate::init::context::{CTX_KEY_CLOUD_INIT_CTX, CTX_KEY_PASSPHRASE_INIT, InitContext};
use crate::init::stage::setup::common::verify_allowed_in_mount;
use crate::init::stage::sync::{AclType, SyncItemAcl};
use lit_blockchain::resolver::rpc::config::{
    RPC_RESOLVER_CFG_LOCAL, RPC_RESOLVER_CFG_SYSTEM, RpcConfig,
};

pub(crate) fn mount_cloud_init(ctx: &mut InitContext) -> Result<PathBuf> {
    let dev_path = ctx.cfg().litos_cloud_init_dev();
    if !dev_path.exists() {
        return Err(generic_err(format!("expected cloud-init dev to exist: {dev_path:?}"), None));
    }

    let fs_type = "iso9660";

    let mnt_path = ctx.cfg().litos_cloud_init_mnt();
    if !mnt_path.exists() {
        fs::create_dir_all(&mnt_path)
            .map_err(|e| io_err(e, Some(format!("failed to make dir: {:?}", &mnt_path))))?;
    }

    info!("Mounting {:?} on {:?} (type: {}, read-only: {})", &dev_path, &mnt_path, &fs_type, true);
    mount(fs_type, &dev_path, &mnt_path, Some("ro"))?;

    Ok(mnt_path)
}

pub(crate) fn load_cloud_init_context(ctx: &mut InitContext) -> Result<CloudInitContext> {
    // Mount cloud-init
    let mnt = mount_cloud_init(ctx)?;

    // Verify allowed files
    verify_allowed_in_mount(mnt.as_path(), ALLOWED_CLOUD_INIT_FILES.as_slice(), "cloud-init")?;

    // Install user config (required below)
    install_init_lit_config(ctx, &mnt)?;
    install_rpc_config(ctx, &mnt)?;

    // Verify cloud-init specific files.
    let env = ctx.build_env().env()?;
    let guest_type = ctx.build_env().guest_type()?;
    let instance_id = ctx.cfg().litos_guest_instance_id().map_err(|e| config_err(e, None))?;

    // Verify meta-data
    let mut meta_data_path = mnt.clone();
    meta_data_path.push(CLOUD_INIT_FILE_META_DATA);

    if !meta_data_path.exists() {
        return Err(io_err(format!("cloud-init file: {meta_data_path:?} is missing"), None));
    }

    let meta_data = CloudInitMetaData::try_from(meta_data_path.as_path())?;
    if ctx.is_release() {
        if GuestType::Custom.eq(&guest_type) {
            meta_data.verify(
                Some(&guest_type),
                Some(&env),
                Some(instance_id.as_str()),
                ctx.build_env().build_kind.as_deref(),
            )?;
        } else {
            meta_data.verify(Some(&guest_type), Some(&env), Some(instance_id.as_str()), None)?;
        }
    }

    // Verify user-data
    let mut user_data_path = mnt.clone();
    user_data_path.push(CLOUD_INIT_FILE_USER_DATA);

    if !user_data_path.exists() {
        return Err(io_err(format!("cloud-init file: {user_data_path:?} is missing"), None));
    }

    let user_data = CloudInitUserData::try_from(user_data_path.as_path())?;
    if ctx.is_release() {
        if GuestType::Custom.eq(&guest_type) {
            user_data.verify(
                Some(&guest_type),
                Some(&env),
                Some(instance_id.as_str()),
                ctx.build_env().build_kind.as_deref(),
            )?;
        } else {
            user_data.verify(Some(&guest_type), Some(&env), Some(instance_id.as_str()), None)?;
        }
    }

    // This should already be the case (due to validation above).
    if !meta_data.instance_id().eq(user_data.fqdn()) {
        return Err(validation_err(
            "cloud-init validation failed - meta-data instance_id does not match user-data fqdn",
            None,
        ));
    }

    // Verify network-config
    let mut network_config_path = mnt.clone();
    network_config_path.push(CLOUD_INIT_FILE_NETWORK_CONFIG);

    if !network_config_path.exists() {
        return Err(io_err(format!("cloud-init file: {network_config_path:?} is missing"), None));
    }

    let network_config = CloudInitNetworkConfig::try_from(network_config_path.as_path())?;
    network_config.verify()?;

    // Load init password (if present)
    let mut init_pw_path = mnt.clone();
    init_pw_path.push(CLOUD_INIT_FILE_INIT_PW);

    if init_pw_path.exists() {
        ctx.insert(
            CTX_KEY_PASSPHRASE_INIT,
            Box::new(fs::read(init_pw_path.as_path()).map_err(|e| {
                io_err(
                    e,
                    Some(format!("failed to read cloud-init file: {CLOUD_INIT_FILE_INIT_PW}")),
                )
            })?),
        );
    }

    // Is safe boot enabled?
    let mut safe_boot_path = mnt.clone();
    safe_boot_path.push(CLOUD_INIT_FILE_SAFE_BOOT);

    if safe_boot_path.exists() {
        if ctx.is_release() {
            return Err(validation_err(
                "cloud-init validation failed - safe boot file found in release.",
                None,
            ));
        } else {
            warn!("Safe boot has been enabled, some steps will be skipped");

            ctx.set_safe_boot(true);
        }
    }

    // Is no resize set?
    let mut no_resize_path = mnt.clone();
    no_resize_path.push(CLOUD_INIT_FILE_NO_RESIZE);

    if no_resize_path.exists() {
        ctx.set_no_resize(true);
    }

    info!("Cloud-init context verification: OK");

    // Load context
    let cloud_init_ctx = CloudInitContext::new(mnt.clone(), meta_data, user_data, network_config);

    ctx.insert(CTX_KEY_CLOUD_INIT_CTX, Box::new(cloud_init_ctx.clone()));

    // Generate network config
    generate_network_config(ctx, &cloud_init_ctx)?;

    Ok(cloud_init_ctx)
}

pub(crate) fn install_init_lit_config(ctx: &mut InitContext, cloud_init_mnt: &Path) -> Result<()> {
    let mut mnt_cfg_path = cloud_init_mnt.to_path_buf();
    mnt_cfg_path.push(format!("{CFG_NAME}.{CFG_EXT}"));

    let mut sys_cfg_path = PathBuf::from(CFG_DIR_GUEST_INIT);
    sys_cfg_path.push(format!("{CFG_NAME}.{CFG_EXT}"));

    if !mnt_cfg_path.exists() {
        return Err(config_err(format!("cloud-init iso missing: ./{CFG_NAME}.{CFG_EXT}"), None));
    }

    // Load the user config
    let mut init_cfg_map = SimpleToml::try_from(mnt_cfg_path.as_path()).map_err(|e| {
        config_err(e, Some(format!("error loading user cfg: {:?}", mnt_cfg_path.as_path())))
    })?;

    // Verify user provided portion
    ctx.cfg().verify_litos_guest_user_cfg(&init_cfg_map)?;

    // Write our changes
    if let Some(release_id) = ctx.cmdline_env().release_id.as_ref() {
        // set guest.release.id (lives in LitOsGuestConfig trait)
        init_cfg_map.insert("guest.release".into(), "id".into(), release_id.clone());
    }
    if let Some(subnet_id) = ctx.cmdline_env().subnet_id.as_ref() {
        // set subnet.id (lives in LitConfig)
        init_cfg_map.insert("subnet".into(), "id".into(), subnet_id.clone());
    }

    // Install the config
    let sys_cfg_parent = sys_cfg_path
        .parent()
        .ok_or_else(|| io_err(format!("failed to get parent dir of: {sys_cfg_path:?}"), None))?;

    fs::create_dir_all(sys_cfg_parent)
        .map_err(|e| io_err(e, Some(format!("failed to create: {sys_cfg_parent:?}"))))?;

    // Copy the config to the system (avoid it being modified after it's verified, if that's even possible?)
    info!("Installing {:?}", &sys_cfg_path);

    init_cfg_map.write_file(sys_cfg_path.as_path()).map_err(|e| io_err(e, None))?;

    // Reload and verify
    ctx.reload_cfg(true)?;
    ctx.cfg().verify_litos_guest_instance_id()?;

    ctx.push_synced_acl(
        sys_cfg_path,
        None,
        vec![SyncItemAcl::new(AclType::Group, "lit-config", ACL_READ | ACL_EXECUTE)],
    );

    Ok(())
}

pub(crate) fn install_rpc_config(ctx: &mut InitContext, cloud_init_mnt: &Path) -> Result<()> {
    let mut rpc_cfg_path = cloud_init_mnt.to_path_buf();
    rpc_cfg_path.push(CLOUD_INIT_FILE_RPC_CONFIG_OVERLAY);

    if !rpc_cfg_path.exists() {
        // No override file.
        return Ok(());
    }

    info!("Found override for rpc config, merging...");

    // Load override config
    let override_cfg = RpcConfig::try_from(rpc_cfg_path.as_path())?;
    override_cfg.verify()?;

    // Load system config
    let mut sys_cfg = RpcConfig::try_from(PathBuf::from(RPC_RESOLVER_CFG_SYSTEM).as_path())?;

    // Merge
    sys_cfg.merge(override_cfg)?;
    sys_cfg.verify()?;

    // Write to temp & schedule for sync
    let tmp_path = PathBuf::from("/tmp/rpc-config.yaml.merged");
    sys_cfg.write_file(tmp_path.as_path())?;

    // The RPC config should live in a /var directory so it can be found.
    let guest_type = ctx.build_env().guest_type()?;
    let (base_path, group_name) = match guest_type {
        GuestType::Prov => (PathBuf::from("/var/lit/os/prov/api"), "lit-prov"),
        GuestType::Node => (PathBuf::from("/var/lit/node"), "lit-node"),
        other => {
            warn!("Skipping rpc-config merge for unsupported guest type: {}", other);
            return Ok(());
        }
    };

    let rpc_dest_path =
        base_path.join(PathBuf::from(RPC_RESOLVER_CFG_LOCAL).file_name().ok_or_else(|| {
            config_err("Could not get filename from RPC_RESOLVER_CFG_LOCAL", None)
        })?);

    ctx.push_synced_acl(
        tmp_path,
        Some(rpc_dest_path),
        vec![SyncItemAcl::new(AclType::Group, group_name, ACL_READ | ACL_WRITE)],
    );

    info!("Finished merging rpc config");

    Ok(())
}

pub(crate) fn generate_network_config(
    ctx: &mut InitContext, cloud_init_ctx: &CloudInitContext,
) -> Result<()> {
    let path = PathBuf::from("/tmp/network-interfaces.local");
    cloud_init_ctx.network_config().to_network_interfaces(path.as_path(), false)?;

    ctx.push_purged(PathBuf::from("/var/local/etc/network/interfaces.d"));
    ctx.push_synced(path, Some(PathBuf::from("/var/local/etc/network/interfaces.d/local")));

    Ok(())
}
