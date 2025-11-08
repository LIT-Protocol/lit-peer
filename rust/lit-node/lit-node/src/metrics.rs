// re export counter
pub use lit_observability::metrics::counter;

pub mod tss {
    use lit_observability::metrics::LitMetric;

    pub enum TssMetrics {
        SignatureShare,
        SignatureShareFail,
    }

    impl LitMetric for TssMetrics {
        fn get_meter(&self) -> &str {
            "lit.tss"
        }
        fn get_description(&self) -> &str {
            ""
        }
        fn get_unit(&self) -> &str {
            ""
        }
        fn get_namespace(&self) -> &str {
            "signature"
        }
        fn get_name(&self) -> &str {
            match self {
                TssMetrics::SignatureShare => "share",
                TssMetrics::SignatureShareFail => "share.fail",
            }
        }
    }

    pub enum PresignMetrics {
        Generate,
        GenerateRealTime,
        Store,
    }

    impl LitMetric for PresignMetrics {
        fn get_meter(&self) -> &str {
            "lit.tss"
        }
        fn get_description(&self) -> &str {
            ""
        }
        fn get_unit(&self) -> &str {
            ""
        }
        fn get_namespace(&self) -> &str {
            "presign"
        }
        fn get_name(&self) -> &str {
            match self {
                PresignMetrics::Generate => "generate",
                PresignMetrics::GenerateRealTime => "generate.realtime",
                PresignMetrics::Store => "store",
            }
        }
    }
}

pub mod dkg {
    use lit_observability::metrics::LitMetric;

    pub enum DkgMetrics {
        DkgInit,
        DkgComplete,
    }

    impl LitMetric for DkgMetrics {
        fn get_meter(&self) -> &str {
            "lit.dkg"
        }
        fn get_description(&self) -> &str {
            ""
        }
        fn get_unit(&self) -> &str {
            ""
        }
        fn get_namespace(&self) -> &str {
            "dkg"
        }
        fn get_name(&self) -> &str {
            match self {
                DkgMetrics::DkgInit => "init",
                DkgMetrics::DkgComplete => "complete",
            }
        }
    }
}

pub mod complaint {
    use lit_observability::metrics::LitMetric;

    // Attributes
    pub const ATTRIBUTE_COMPLAINT_REASON: &str = "complaint_reason";
    pub const ATTRIBUTE_EVICTION_CAUSE: &str = "eviction_cause";
    pub const ATTRIBUTE_PEER_KEY: &str = "peer_key";

    pub enum ComplaintMetrics {
        ComplaintRemembered,
        VotedToKick,
        ComplaintCacheEvicted,
    }

    impl LitMetric for ComplaintMetrics {
        fn get_meter(&self) -> &str {
            "lit.complaint"
        }
        fn get_description(&self) -> &str {
            ""
        }
        fn get_unit(&self) -> &str {
            ""
        }
        fn get_namespace(&self) -> &str {
            "complaint"
        }
        fn get_name(&self) -> &str {
            match self {
                ComplaintMetrics::ComplaintRemembered => "remembered",
                ComplaintMetrics::VotedToKick => "voted_to_kick",
                ComplaintMetrics::ComplaintCacheEvicted => "cache.evicted",
            }
        }
    }
}
