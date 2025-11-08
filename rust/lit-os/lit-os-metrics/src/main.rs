use clap::Parser;
use config::LitOsMetricsRunnerConfig;
use error::{PKG_NAME, unexpected_err, validation_err};
use itertools::Itertools;
use lit_core::config::LitConfig;
use lit_observability::opentelemetry::{KeyValue, global};
use lit_observability::opentelemetry_sdk::{Resource, trace as sdktrace};
use lit_observability::opentelemetry_sdk::{logs::LoggerProvider, metrics::SdkMeterProvider};
use std::{fmt::Debug, path::PathBuf, sync::Arc};
use tracing_subscriber::util::SubscriberInitExt;

use error::Result;

mod args;
mod config;
mod error;

use args::*;
use lit_os_metrics::*;
use lit_os_metrics_internal::*;

fn main() -> Result<()> {
    let mut args = CmdArgs::parse();

    if args.query.is_empty() {
        return Err(validation_err("Must provide at least one query", None));
    }

    args.query = args.query.into_iter().unique().collect();

    let socket_path = PathBuf::from(&args.osquery_socket);
    if !socket_path.exists() {
        return Err(unexpected_err(
            format!("Socket path does not exist: {}", args.osquery_socket),
            None,
        ));
    }

    let os_query = OSQuery::new().set_socket(args.osquery_socket.as_str());

    if args.plain {
        handle_plain(args, os_query)?;
    } else {
        let cfg = Arc::new(<LitConfig as LitOsMetricsRunnerConfig>::must_new());

        // Init observability - this MUST occur after we have spawned the tasks to bring up the GRPC server.
        let observability_rt = tokio::runtime::Runtime::new().map_err(|e| {
            unexpected_err(e, Some("failed to create Observability Runtime: {:?}".into()))
        })?;
        let (metrics_provider, logger_provider) = observability_rt
            .block_on(async { init_observability(&cfg, &args.resource_attributes).await })?;

        handle_metrics(args, os_query)?;

        shutdown_observability(metrics_provider, logger_provider)?;
    }

    Ok(())
}

fn handle_plain(args: CmdArgs, os_query: OSQuery) -> Result<()> {
    for query in &args.query {
        match query {
            Query::RunningProcess => {
                print_query_results(execute_query::<RunningProcess>(&os_query, running_process())?)
            }
            Query::EstablishedOutbound => print_query_results(
                execute_query::<EstablishedOutbound>(&os_query, established_outbound())?,
            ),
            Query::CronJob => print_query_results(execute_query::<CronJob>(&os_query, crontab())?),
            Query::LoginHistory => {
                print_query_results(execute_query::<LoginHistory>(&os_query, login_history())?)
            }
            Query::OsInfo => print_query_results(execute_query::<OsInfo>(&os_query, os_info())?),
            Query::InterfaceAddress => print_query_results(execute_query::<InterfaceAddress>(
                &os_query,
                interface_addresses(),
            )?),
            Query::DockerRunningContainers => {
                print_query_results(execute_query::<DockerRunningContainers>(
                    &os_query,
                    docker_running_containers(),
                )?)
            }
            Query::DebianPackage => {
                print_query_results(execute_query::<DebianPackage>(&os_query, debian_packages())?)
            }
            Query::CpuInfo => print_query_results(execute_query::<CpuInfo>(&os_query, cpu_info())?),
            Query::DiskInfo => {
                print_query_results(execute_query::<DiskInfo>(&os_query, disk_info())?)
            }
            Query::MemoryInfo => {
                print_query_results(execute_query::<MemoryInfo>(&os_query, memory_info())?)
            }
            Query::LoadAverage => {
                print_query_results(execute_query::<LoadAverage>(&os_query, load_average())?)
            }
            Query::ListeningPorts => {
                print_query_results(execute_query::<ListeningPort>(&os_query, listening_ports())?)
            }
            Query::KernelInfo => {
                print_query_results(execute_query::<KernelInfo>(&os_query, kernel_info())?)
            }
            Query::Uptime => print_query_results(execute_query::<Uptime>(&os_query, uptime())?),
            Query::Iptables => {
                print_query_results(execute_query::<IptablesRule>(&os_query, iptables())?)
            }
            Query::SystemInfo => {
                print_query_results(execute_query::<SystemInfo>(&os_query, system_info())?)
            }
        }
    }
    Ok(())
}

fn print_query_results<T: Debug>(values: Vec<T>) {
    println!("{:#?}", values);
}

fn handle_metrics(args: CmdArgs, os_query: OSQuery) -> Result<()> {
    for query in &args.query {
        match query {
            Query::RunningProcess => {
                handle_running_process_telemetry(&os_query, running_process())?
            }
            Query::EstablishedOutbound => {
                add_value_metrics::<EstablishedOutbound>(&os_query, established_outbound())?
            }
            Query::CronJob => add_value_metrics::<CronJob>(&os_query, crontab())?,
            Query::LoginHistory => add_value_metrics::<LoginHistory>(&os_query, login_history())?,
            Query::OsInfo => add_value_metrics::<OsInfo>(&os_query, os_info())?,
            Query::InterfaceAddress => {
                add_value_metrics::<InterfaceAddress>(&os_query, interface_addresses())?
            }
            Query::DockerRunningContainers => {
                handle_docker_container_telemetry(&os_query, docker_running_containers())?
            }
            Query::DebianPackage => {
                add_value_metrics::<DebianPackage>(&os_query, debian_packages())?
            }
            Query::CpuInfo => add_value_metrics::<CpuInfo>(&os_query, cpu_info())?,
            Query::DiskInfo => add_value_metrics::<DiskInfo>(&os_query, disk_info())?,
            Query::MemoryInfo => add_value_metrics::<MemoryInfo>(&os_query, memory_info())?,
            Query::LoadAverage => add_value_metrics::<LoadAverage>(&os_query, load_average())?,
            Query::ListeningPorts => {
                add_value_metrics::<ListeningPort>(&os_query, listening_ports())?
            }
            Query::KernelInfo => add_value_metrics::<KernelInfo>(&os_query, kernel_info())?,
            Query::Uptime => add_value_metrics::<Uptime>(&os_query, uptime())?,
            Query::Iptables => add_value_metrics::<IptablesRule>(&os_query, iptables())?,
            Query::SystemInfo => add_value_metrics::<SystemInfo>(&os_query, system_info())?,
        }
    }

    Ok(())
}

/// NOTE: Run in a dedicated runtime, to avoid blocking a runtime that is needed to perform tasks.
async fn init_observability(
    cfg: &LitConfig, resource_attributes_override: &[(String, String)],
) -> Result<(SdkMeterProvider, LoggerProvider)> {
    lit_logging::set_panic_hook();

    let mut resource_key_values = vec![KeyValue::new(
        lit_observability::opentelemetry_semantic_conventions::resource::SERVICE_NAME,
        PKG_NAME,
    )];

    for (key, value) in resource_attributes_override {
        resource_key_values.push(KeyValue::new(key.clone(), value.clone()));
    }

    let otel_resource = Resource::new(resource_key_values);

    let (tracing_provider, metrics_provider, subscriber, logger_provider) =
        lit_observability::create_providers(
            cfg,
            otel_resource.clone(),
            sdktrace::Config::default().with_resource(otel_resource.clone()),
        )
        .await
        .map_err(|e| unexpected_err(e, Some("Failed to create observability providers".into())))?;

    // Set globals
    global::set_tracer_provider(tracing_provider);
    global::set_meter_provider(metrics_provider.clone());
    subscriber.init();

    Ok((metrics_provider, logger_provider))
}

fn shutdown_observability(
    metrics_provider: SdkMeterProvider, logger_provider: LoggerProvider,
) -> Result<()> {
    global::shutdown_tracer_provider();
    metrics_provider
        .shutdown()
        .map_err(|e| unexpected_err(e, Some("Failed to shutdown metrics provider".into())))?;
    logger_provider
        .shutdown()
        .map_err(|e| unexpected_err(e, Some("Failed to shutdown logger provider".into())))?;
    Ok(())
}
