use crate::models::{GaugeMetric, OsMetric};
use lit_observability::opentelemetry::KeyValue;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Serialize)]
pub struct IptablesRule {
    filter_chain: String,
    filter_policy: String,
    filter_target: String,
    filter_protocol: String,
    filter_src_port: String,
    filter_dst_port: String,
    filter_src_ip: String,
    filter_dst_ip: String,
    filter_iniface: String,
    filter_outiface: String,
    nat_chain: String,
    nat_policy: String,
    nat_target: String,
    nat_protocol: String,
    nat_src_port: String,
    nat_dst_port: String,
    nat_src_ip: String,
    nat_dst_ip: String,
    nat_iniface: String,
    nat_outiface: String,
    mangle_chain: String,
    mangle_policy: String,
    mangle_target: String,
    mangle_protocol: String,
    mangle_src_port: String,
    mangle_dst_port: String,
    mangle_src_ip: String,
    mangle_dst_ip: String,
    mangle_iniface: String,
    mangle_outiface: String,
    raw_chain: String,
    raw_policy: String,
    raw_target: String,
    raw_protocol: String,
    raw_src_port: String,
    raw_dst_port: String,
    raw_src_ip: String,
    raw_dst_ip: String,
    raw_iniface: String,
    raw_outiface: String,
}

impl OsMetric for IptablesRule {
    const NAME: &'static str = "iptables";
}

impl GaugeMetric for IptablesRule {
    fn gauge_value(&self) -> Option<f64> {
        Some(1.0)
    }

    fn gauge_labels(&self) -> Vec<KeyValue> {
        vec![
            KeyValue::new("filter_chain", self.filter_chain.clone()),
            KeyValue::new("filter_policy", self.filter_policy.clone()),
            KeyValue::new("filter_target", self.filter_target.clone()),
            KeyValue::new("filter_protocol", self.filter_protocol.clone()),
            KeyValue::new("nat_chain", self.nat_chain.clone()),
            KeyValue::new("nat_policy", self.nat_policy.clone()),
            KeyValue::new("nat_target", self.nat_target.clone()),
            KeyValue::new("nat_protocol", self.nat_protocol.clone()),
        ]
    }
}

impl TryFrom<&BTreeMap<String, String>> for IptablesRule {
    type Error = String;

    fn try_from(value: &BTreeMap<String, String>) -> Result<Self, Self::Error> {
        Ok(Self {
            filter_chain: value
                .get("filter_chain")
                .cloned()
                .ok_or_else(|| "Missing filter_chain".to_string())?,
            filter_policy: value
                .get("filter_policy")
                .cloned()
                .ok_or_else(|| "Missing filter_policy".to_string())?,
            filter_target: value
                .get("filter_target")
                .cloned()
                .ok_or_else(|| "Missing filter_target".to_string())?,
            filter_protocol: value
                .get("filter_protocol")
                .cloned()
                .ok_or_else(|| "Missing filter_protocol".to_string())?,
            filter_src_port: value
                .get("filter_src_port")
                .cloned()
                .ok_or_else(|| "Missing filter_src_port".to_string())?,
            filter_dst_port: value
                .get("filter_dst_port")
                .cloned()
                .ok_or_else(|| "Missing filter_dst_port".to_string())?,
            filter_src_ip: value
                .get("filter_src_ip")
                .cloned()
                .ok_or_else(|| "Missing filter_src_ip".to_string())?,
            filter_dst_ip: value
                .get("filter_dst_ip")
                .cloned()
                .ok_or_else(|| "Missing filter_dst_ip".to_string())?,
            filter_iniface: value
                .get("filter_iniface")
                .cloned()
                .ok_or_else(|| "Missing filter_iniface".to_string())?,
            filter_outiface: value
                .get("filter_outiface")
                .cloned()
                .ok_or_else(|| "Missing filter_outiface".to_string())?,
            nat_chain: value
                .get("nat_chain")
                .cloned()
                .ok_or_else(|| "Missing nat_chain".to_string())?,
            nat_policy: value
                .get("nat_policy")
                .cloned()
                .ok_or_else(|| "Missing nat_policy".to_string())?,
            nat_target: value
                .get("nat_target")
                .cloned()
                .ok_or_else(|| "Missing nat_target".to_string())?,
            nat_protocol: value
                .get("nat_protocol")
                .cloned()
                .ok_or_else(|| "Missing nat_protocol".to_string())?,
            nat_src_port: value
                .get("nat_src_port")
                .cloned()
                .ok_or_else(|| "Missing nat_src_port".to_string())?,
            nat_dst_port: value
                .get("nat_dst_port")
                .cloned()
                .ok_or_else(|| "Missing nat_dst_port".to_string())?,
            nat_src_ip: value
                .get("nat_src_ip")
                .cloned()
                .ok_or_else(|| "Missing nat_src_ip".to_string())?,
            nat_dst_ip: value
                .get("nat_dst_ip")
                .cloned()
                .ok_or_else(|| "Missing nat_dst_ip".to_string())?,
            nat_iniface: value
                .get("nat_iniface")
                .cloned()
                .ok_or_else(|| "Missing nat_iniface".to_string())?,
            nat_outiface: value
                .get("nat_outiface")
                .cloned()
                .ok_or_else(|| "Missing nat_outiface".to_string())?,
            mangle_chain: value
                .get("mangle_chain")
                .cloned()
                .ok_or_else(|| "Missing mangle_chain".to_string())?,
            mangle_policy: value
                .get("mangle_policy")
                .cloned()
                .ok_or_else(|| "Missing mangle_policy".to_string())?,
            mangle_target: value
                .get("mangle_target")
                .cloned()
                .ok_or_else(|| "Missing mangle_target".to_string())?,
            mangle_protocol: value
                .get("mangle_protocol")
                .cloned()
                .ok_or_else(|| "Missing mangle_protocol".to_string())?,
            mangle_src_port: value
                .get("mangle_src_port")
                .cloned()
                .ok_or_else(|| "Missing mangle_src_port".to_string())?,
            mangle_dst_port: value
                .get("mangle_dst_port")
                .cloned()
                .ok_or_else(|| "Missing mangle_dst_port".to_string())?,
            mangle_src_ip: value
                .get("mangle_src_ip")
                .cloned()
                .ok_or_else(|| "Missing mangle_src_ip".to_string())?,
            mangle_dst_ip: value
                .get("mangle_dst_ip")
                .cloned()
                .ok_or_else(|| "Missing mangle_dst_ip".to_string())?,
            mangle_iniface: value
                .get("mangle_iniface")
                .cloned()
                .ok_or_else(|| "Missing mangle_iniface".to_string())?,
            mangle_outiface: value
                .get("mangle_outiface")
                .cloned()
                .ok_or_else(|| "Missing mangle_outiface".to_string())?,
            raw_chain: value
                .get("raw_chain")
                .cloned()
                .ok_or_else(|| "Missing raw_chain".to_string())?,
            raw_policy: value
                .get("raw_policy")
                .cloned()
                .ok_or_else(|| "Missing raw_policy".to_string())?,
            raw_target: value
                .get("raw_target")
                .cloned()
                .ok_or_else(|| "Missing raw_target".to_string())?,
            raw_protocol: value
                .get("raw_protocol")
                .cloned()
                .ok_or_else(|| "Missing raw_protocol".to_string())?,
            raw_src_port: value
                .get("raw_src_port")
                .cloned()
                .ok_or_else(|| "Missing raw_src_port".to_string())?,
            raw_dst_port: value
                .get("raw_dst_port")
                .cloned()
                .ok_or_else(|| "Missing raw_dst_port".to_string())?,
            raw_src_ip: value
                .get("raw_src_ip")
                .cloned()
                .ok_or_else(|| "Missing raw_src_ip".to_string())?,
            raw_dst_ip: value
                .get("raw_dst_ip")
                .cloned()
                .ok_or_else(|| "Missing raw_dst_ip".to_string())?,
            raw_iniface: value
                .get("raw_iniface")
                .cloned()
                .ok_or_else(|| "Missing raw_iniface".to_string())?,
            raw_outiface: value
                .get("raw_outiface")
                .cloned()
                .ok_or_else(|| "Missing raw_outiface".to_string())?,
        })
    }
}
