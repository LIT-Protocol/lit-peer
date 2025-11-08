use crate::cmd::os::guest::instance::cleanup::handle_cmd_os_guest_instances_cleanup;
use clap::{Args, Subcommand};
use lit_cli_core::cmd::CliGlobalOpts;
use lit_core::config::LitConfig;

use crate::cmd::os::guest::instance::create::{
    GuestInstanceCreate, handle_cmd_os_guest_instance_create,
};
use crate::cmd::os::guest::instance::delete::{
    GuestInstanceDelete, handle_cmd_os_guest_instance_delete,
};
use crate::cmd::os::guest::instance::describe::{
    GuestInstanceDescribe, handle_cmd_os_guest_instance_describe,
};
use crate::cmd::os::guest::instance::logs::{GuestInstanceLogs, handle_cmd_os_guest_instance_logs};
use crate::cmd::os::guest::instance::ls::handle_cmd_os_guest_instance_ls;
use crate::cmd::os::guest::instance::ps::handle_cmd_os_guest_instance_ps;
use crate::cmd::os::guest::instance::recreate::{
    GuestInstanceRecreate, handle_cmd_os_guest_instance_recreate,
};
use crate::cmd::os::guest::instance::repair::{
    GuestInstanceRepair, handle_cmd_os_guest_instance_repair,
};
use crate::cmd::os::guest::instance::resize::{
    GuestInstanceResize, handle_cmd_os_guest_instance_resize,
};
use crate::cmd::os::guest::instance::restart::{
    GuestInstanceRestart, handle_cmd_os_guest_instance_restart,
};
use crate::cmd::os::guest::instance::start::{
    GuestInstanceStart, handle_cmd_os_guest_instance_start,
};
use crate::cmd::os::guest::instance::status::{
    GuestInstanceStatus, handle_cmd_os_guest_instance_status,
};
use crate::cmd::os::guest::instance::stop::{GuestInstanceStop, handle_cmd_os_guest_instance_stop};

pub(crate) mod cleanup;
pub(crate) mod create;
pub(crate) mod delete;
pub(crate) mod describe;
pub(crate) mod logs;
pub(crate) mod ls;
pub(crate) mod ps;
pub(crate) mod recreate;
pub(crate) mod repair;
pub(crate) mod resize;
pub(crate) mod restart;
pub(crate) mod start;
pub(crate) mod status;
pub(crate) mod stop;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct GuestInstance {
    #[command(subcommand)]
    command: GuestInstanceCommands,
}

#[derive(Debug, Subcommand)]
#[command(arg_required_else_help = true)]
#[allow(clippy::large_enum_variant)]
pub(crate) enum GuestInstanceCommands {
    /// Create a guest instance
    #[command(arg_required_else_help = true)]
    Create(GuestInstanceCreate),
    /// Recreate a guest instance
    #[command(arg_required_else_help = true)]
    Recreate(GuestInstanceRecreate),
    /// Shutdown and delete a guest instance
    #[command(arg_required_else_help = true)]
    Delete(GuestInstanceDelete),
    /// Get the status of a guest instance
    #[command(arg_required_else_help = true)]
    Status(GuestInstanceStatus),
    /// Get the logs of a guest instance
    #[command(arg_required_else_help = true)]
    Logs(GuestInstanceLogs),
    /// Start a guest instance
    #[command(arg_required_else_help = true)]
    Start(GuestInstanceStart),
    /// Stop a guest instance
    #[command(arg_required_else_help = true)]
    Stop(GuestInstanceStop),
    /// Restart a guest instance
    #[command(arg_required_else_help = true)]
    Restart(GuestInstanceRestart),
    /// Show details for a guest instance
    #[command(arg_required_else_help = true)]
    Describe(GuestInstanceDescribe),
    /// Repair a guest instance
    #[command(arg_required_else_help = true)]
    Repair(GuestInstanceRepair),
    /// Resize the image of a guest instance
    #[command(arg_required_else_help = true)]
    Resize(GuestInstanceResize),
    /// List all guest instances
    Ls {},
    /// Show running guest instances
    Ps {},
    /// Clean up failed creations
    CleanUp {},
}

pub(crate) async fn handle_cmd_os_guest_instance(
    cfg: LitConfig, opts: CliGlobalOpts, args: GuestInstance,
) -> bool {
    match args.command {
        GuestInstanceCommands::Create(args) => {
            handle_cmd_os_guest_instance_create(cfg, opts, args).await
        }
        GuestInstanceCommands::Recreate(args) => {
            handle_cmd_os_guest_instance_recreate(cfg, opts, args).await
        }
        GuestInstanceCommands::Delete(args) => {
            handle_cmd_os_guest_instance_delete(&cfg, &opts, args)
        }
        GuestInstanceCommands::Describe(args) => {
            handle_cmd_os_guest_instance_describe(cfg, opts, args)
        }
        GuestInstanceCommands::Repair(args) => handle_cmd_os_guest_instance_repair(cfg, opts, args),
        GuestInstanceCommands::Resize(args) => handle_cmd_os_guest_instance_resize(cfg, opts, args),
        GuestInstanceCommands::Ls {} => handle_cmd_os_guest_instance_ls(&cfg, &opts),
        GuestInstanceCommands::Ps {} => handle_cmd_os_guest_instance_ps(&cfg, &opts),
        GuestInstanceCommands::Status(args) => handle_cmd_os_guest_instance_status(cfg, opts, args),
        GuestInstanceCommands::Logs(args) => handle_cmd_os_guest_instance_logs(&cfg, &opts, args),
        GuestInstanceCommands::Start(args) => handle_cmd_os_guest_instance_start(cfg, opts, args),
        GuestInstanceCommands::Stop(args) => handle_cmd_os_guest_instance_stop(cfg, opts, args),
        GuestInstanceCommands::Restart(args) => {
            handle_cmd_os_guest_instance_restart(cfg, opts, args).await
        }
        GuestInstanceCommands::CleanUp {} => handle_cmd_os_guest_instances_cleanup(&cfg, &opts),
    }
}
