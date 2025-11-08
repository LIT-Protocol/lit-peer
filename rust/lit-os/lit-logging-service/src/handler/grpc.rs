use std::sync::Arc;

use opentelemetry_proto::tonic::collector::logs::v1::logs_service_server::LogsService;
use opentelemetry_proto::tonic::collector::logs::v1::{
    ExportLogsServiceRequest, ExportLogsServiceResponse,
};
use opentelemetry_proto::tonic::collector::metrics::v1::metrics_service_server::MetricsService;
use opentelemetry_proto::tonic::collector::metrics::v1::{
    ExportMetricsServiceRequest, ExportMetricsServiceResponse,
};
use opentelemetry_proto::tonic::collector::trace::v1::trace_service_server::TraceService;
use opentelemetry_proto::tonic::collector::trace::v1::{
    ExportTraceServiceRequest, ExportTraceServiceResponse,
};
use tonic::{Request, Response, Status};

use crate::service::otel::{OTELService, OTELServiceValue};

#[derive(Clone)]
pub(crate) struct OTELGrpcService {
    pub(crate) otel_svc: Arc<OTELService>,
}

#[tonic::async_trait]
impl TraceService for OTELGrpcService {
    async fn export(
        &self, request: Request<ExportTraceServiceRequest>,
    ) -> Result<Response<ExportTraceServiceResponse>, Status> {
        // Send to the OTELService for processing.
        self.otel_svc.send(OTELServiceValue::Trace(request.into_inner())).map_err(|e| {
            Status::internal(format!("failed to send trace request to OTELService: {:?}", e))
        })?;

        let reply = ExportTraceServiceResponse { partial_success: None };

        Ok(Response::new(reply))
    }
}

#[tonic::async_trait]
impl MetricsService for OTELGrpcService {
    async fn export(
        &self, request: Request<ExportMetricsServiceRequest>,
    ) -> Result<Response<ExportMetricsServiceResponse>, Status> {
        // Send to the OTELService for processing.
        self.otel_svc.send(OTELServiceValue::Metric(request.into_inner())).map_err(|e| {
            Status::internal(format!("failed to send metric request to OTELService: {:?}", e))
        })?;

        let reply = ExportMetricsServiceResponse { partial_success: None };

        Ok(Response::new(reply))
    }
}

#[tonic::async_trait]
impl LogsService for OTELGrpcService {
    async fn export(
        &self, request: Request<ExportLogsServiceRequest>,
    ) -> Result<Response<ExportLogsServiceResponse>, Status> {
        // Send to the OTELService for processing.
        self.otel_svc.send(OTELServiceValue::Log(request.into_inner())).map_err(|e| {
            Status::internal(format!("failed to send log request to OTELService: {:?}", e))
        })?;

        let reply = ExportLogsServiceResponse { partial_success: None };

        Ok(Response::new(reply))
    }
}
