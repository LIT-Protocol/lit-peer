# Lit Os Metrics

This crate relies on [osquery](https://www.osquery.io/downloads/official/) installed and running to execute system queries.

Installing `osquery` yields two binaries: `osqueryd` and `osqueryi`. 
The former is a daemon that runs in the background and listens for queries, while the latter is an interactive shell that can be used to run queries manually.

This tool can use either one of these binaries to execute queries.
The crate can just run and execute queries if `osqueryd` is running.

The default for `lit-os-metrics` is to export OpenTelemetry data to a GRPC service listening on `http://127.0.0.1:4317`. However, for testing purposes, you can run with `--plain` flag to see the output without OpenTelemetry formatting printed to `/dev/stdout`.

Finally, the crate expects the queries to be provided. At least one query must be provided, but any number of queries can be specified.

Each query can be specified by `--query=`. The following queries are supported.

1. running-process
2. established-outbound
3. cron-job
4. login-history
5. os-info
6. interface-address
7. docker-running-containers
8. debian-package

## Running Locally (plain)

Run `cargo run -- --plain --query=<QUERY>`.

## Running Locally (OTEL)

Pre-requisites:
- Install Docker

Instructions:
1. Run `docker compose up`
2. Run `lit-os-metrics`, eg. `cargo run -- --query=os-info`. You may / may not need to run as `root`.

You should see the following output in the Docker container:

```log
otel-collector-1  | 2024-08-19T20:24:21.199Z	info	MetricsExporter	{"kind": "exporter", "data_type": "metrics", "name": "debug", "resource metrics": 1, "metrics": 1, "data points": 1}
otel-collector-1  | 2024-08-19T20:24:21.200Z	info	ResourceMetrics #0
otel-collector-1  | Resource SchemaURL:
otel-collector-1  | Resource attributes:
otel-collector-1  |      -> service.name: Str(lit_os_metrics)
otel-collector-1  | ScopeMetrics #0
otel-collector-1  | ScopeMetrics SchemaURL:
otel-collector-1  | InstrumentationScope lit-os
otel-collector-1  | Metric #0
otel-collector-1  | Descriptor:
otel-collector-1  |      -> Name: os_info
otel-collector-1  |      -> Description:
otel-collector-1  |      -> Unit:
otel-collector-1  |      -> DataType: Sum
otel-collector-1  |      -> IsMonotonic: true
otel-collector-1  |      -> AggregationTemporality: Cumulative
otel-collector-1  | NumberDataPoints #0
otel-collector-1  | Data point attributes:
otel-collector-1  |      -> os_info: Str({"arch":"arm64","build":"23E224","codename":"","extra":"","major":"14","minor":"4","name":"macOS","patch":"1","platform":"darwin","platform_like":"darwin","query_time":1724099061,"version":"14.4.1"})
otel-collector-1  | StartTimestamp: 2024-08-19 20:24:21.175008 +0000 UTC
otel-collector-1  | Timestamp: 2024-08-19 20:24:21.175113 +0000 UTC
otel-collector-1  | Value: 1
otel-collector-1  | 	{"kind": "exporter", "data_type": "metrics", "name": "debug"}
```