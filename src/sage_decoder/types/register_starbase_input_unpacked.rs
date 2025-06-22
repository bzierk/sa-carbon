

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct RegisterStarbaseInputUnpacked {
    #[serde(with = "serde_big_array::BigArray")]
    pub name: [u8; 64],
    pub sub_coordinates: [i64; 2],
    pub starbase_level_index: u8,
    pub faction: u8,
    pub key_index: u16,
}
