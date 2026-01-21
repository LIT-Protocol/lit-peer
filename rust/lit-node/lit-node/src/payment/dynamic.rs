use super::batches::PendingPayment;
use crate::error::{EC, Error};
use crate::{config::chain::ChainDataConfigManager, error::unexpected_err_code};
use ethers::types::{Address, I256};
use lit_node_core::{DynamicPaymentItem, LitActionPriceComponent};
use serde::{Deserialize, Serialize};
// Notes:
// - Per Node Sync, Runtime length, code length & response length are not evaluated.  They have not been removed as they line up with contract enums, and may be evaluated in the future.

#[doc = "The different measurements that can be used to price the different components."]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum NodePriceMeasurement {
    PerSecond,
    PerMegabyte,
    PerCount,
}

#[doc = "A single price config for a given component."]
#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub struct LitActionPriceConfig {
    pub price_component: LitActionPriceComponent,
    pub price_measurement: NodePriceMeasurement,
    pub price: u64,
}

#[doc = "A list of lit action price configs."]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LitActionPriceConfigs {
    pub configs: Vec<LitActionPriceConfig>,
}

#[doc = "The dynamic payment struct, which is used to track the payment for a lit action as it is executed."]
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct DynamicPayment {
    pub configs: Vec<LitActionPriceConfig>, // list of used pricing items.
    pub items: Vec<DynamicPaymentItem>,
    pub price_multiplier: u64, // based on the pricing curve, each one of the prices will be multiplied by this value
    pub spending_limit: I256, // technically this is the users' source of funds divided by the number of nodes involved in the transaction
    pub running_total: I256,
    pub payer: Address,
    pub payment_enabled: bool,
}

#[doc = "Converts a u8 to a NodePriceMeasurement or returns an error if the value is invalid."]
impl TryFrom<u8> for NodePriceMeasurement {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(NodePriceMeasurement::PerSecond),
            1 => Ok(NodePriceMeasurement::PerMegabyte),
            2 => Ok(NodePriceMeasurement::PerCount),
            _ => Err(format!("Invalid node price measurement: {value}")),
        }
    }
}

impl Default for DynamicPayment {
    fn default() -> Self {
        Self {
            configs: vec![],
            items: vec![],
            price_multiplier: 1,
            spending_limit: I256::from(0),
            running_total: I256::from(0),
            payer: Address::zero(),
            payment_enabled: false,
        }
    }
}

impl DynamicPayment {
    #[doc = "Loads the dynamic payment configs from the chain data config manager."]
    pub fn load_from(
        payer: Address,
        chain_data_config_manager: &ChainDataConfigManager,
        price_multiplier: u64,
        spending_limit: I256,
        payment_enabled: bool,
    ) -> Result<Self, Error> {
        let configs = chain_data_config_manager.get_dynamic_lit_action_price_configs();

        trace!("Dynamic payment configs length: {}", configs.len());

        if configs.is_empty() {
            return Err(unexpected_err_code(
                "No lit action price configs found",
                EC::PaymentFailed,
                None,
            ));
        }

        Ok(Self {
            configs,
            items: vec![],
            price_multiplier,
            spending_limit,
            running_total: I256::from(0),
            payer,
            payment_enabled,
        })
    }

    #[doc = "Adds a single item to the dynamic payment struct, where the quantity is 1."]
    pub fn add(&mut self, component: LitActionPriceComponent, quantity: u64) -> Result<(), Error> {
        if !self.payment_enabled {
            return Ok(());
        }

        debug!("Adding item to dynamic payment: {:?}", component);
        trace!("Dynamic payment configs: {:?}", self.configs);
        let config = self.configs.iter().find(|c| c.price_component == component);
        if let Some(config) = config {
            let price = config.price * self.price_multiplier;
            if (self.running_total + price) > self.spending_limit {
                return Err(unexpected_err_code(
                    format!(
                        "Action aborted as next execution of '{:?}' would exceed wallet balance.",
                        component
                    ),
                    EC::PaymentFailed,
                    None,
                ));
            }

            trace!("Adding item to dynamic payment: {:?}", component);

            self.items.push(DynamicPaymentItem {
                component,
                quantity,
                price,
            });
            self.running_total += price;
            Ok(())
        } else {
            Err(unexpected_err_code(
                format!(
                    "Action aborted as pricing component '{:?}' was not found.",
                    component
                ),
                EC::PaymentFailed,
                None,
            ))
        }
    }

    #[doc = "Adds multiple items to the dynamic payment struct, where the quantity is 1. This is used for operations that require multiple items to be added at once."]
    pub fn add_multiple_single_count(
        &mut self,
        items: Vec<LitActionPriceComponent>,
    ) -> Result<(), Error> {
        for item in items {
            self.add(item, 1)?;
        }
        Ok(())
    }

    #[doc = "Converts the dynamic payment struct to a pending payment struct, which is used to track the payment for the payer."]
    pub fn to_pending_payment(&self) -> PendingPayment {
        trace!("to_pending_payment: {:?}", self.items);
        let price = self.items.iter().map(|item| item.price).sum::<u64>();

        PendingPayment {
            payer: self.payer,
            price: price.into(),
            spending_limit: self.spending_limit,
        }
    }
}
