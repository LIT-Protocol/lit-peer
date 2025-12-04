// re export counter

#[allow(dead_code)]
pub mod grpc {
    //! Metrics for the gRPC service.

    use lit_observability::metrics::LitMetric;

    pub enum GrpcServiceMetrics {
        RequestSize,
        RequestLatency,
    }

    impl LitMetric for GrpcServiceMetrics {
        fn get_meter(&self) -> &str {
            "lit.grpc"
        }
        fn get_description(&self) -> &str {
            ""
        }
        fn get_unit(&self) -> &str {
            ""
        }
        fn get_namespace(&self) -> &str {
            "service"
        }
        fn get_name(&self) -> &str {
            match self {
                GrpcServiceMetrics::RequestSize => "request-size",
                GrpcServiceMetrics::RequestLatency => "request-latency",
            }
        }
    }
}

#[allow(dead_code)]
pub mod device {
    //! Metrics for the serial devices.

    use lit_observability::metrics::LitMetric;

    pub enum DeviceMetrics {
        WriteSize,
    }

    impl LitMetric for DeviceMetrics {
        fn get_meter(&self) -> &str {
            "lit.device"
        }

        fn get_description(&self) -> &str {
            ""
        }
        fn get_unit(&self) -> &str {
            ""
        }
        fn get_namespace(&self) -> &str {
            "serial"
        }
        fn get_name(&self) -> &str {
            match self {
                DeviceMetrics::WriteSize => "write-size",
            }
        }
    }
}

#[allow(dead_code)]
pub mod queue {
    //! Metrics for the queue.

    use lit_observability::metrics::LitMetric;

    pub enum QueueMetrics {
        OtelServiceQueueSize,
    }

    impl LitMetric for QueueMetrics {
        fn get_meter(&self) -> &str {
            "lit.queue"
        }

        fn get_description(&self) -> &str {
            ""
        }

        fn get_unit(&self) -> &str {
            ""
        }

        fn get_namespace(&self) -> &str {
            "queue"
        }

        fn get_name(&self) -> &str {
            match self {
                QueueMetrics::OtelServiceQueueSize => "otel-service-queue-size",
            }
        }
    }
}
