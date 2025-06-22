use super::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RegisterShipInput {
    #[serde(with = "serde_big_array::BigArray")]
    pub name: [u8; 64],
    pub size_class: SizeClass,
    pub stats: ShipStatsUnpacked,
    pub key_index: u16,
    pub is_active: bool,
}
