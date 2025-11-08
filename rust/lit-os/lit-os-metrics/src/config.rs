use lit_core::config::{LitConfig, LitConfigBuilder};
use lit_logging::config::LitLoggingConfig;
use lit_os_core::config::LitOsGuestConfig;

use crate::error::Result;

pub trait LitOsMetricsRunnerConfig {
    fn try_new() -> Result<LitConfig>;
    fn must_new() -> LitConfig;
    fn from_builder(builder: LitConfigBuilder) -> Result<LitConfig>;
}

impl LitOsMetricsRunnerConfig for LitConfig {
    fn try_new() -> Result<Self> {
        <LitConfig as LitOsMetricsRunnerConfig>::from_builder(LitConfigBuilder::default())
    }

    fn must_new() -> Self {
        <LitConfig as LitOsMetricsRunnerConfig>::try_new().expect("failed to load config")
    }

    fn from_builder(mut builder: LitConfigBuilder) -> Result<LitConfig> {
        // Set defaults
        builder = <LitConfig as LitOsGuestConfig>::apply_defaults(builder)?;
        builder = <LitConfig as LitLoggingConfig>::apply_defaults(builder)?;
        builder.build()
    }
}
