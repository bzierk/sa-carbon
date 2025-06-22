

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct RegisterMineItemInput {
    #[serde(with = "serde_big_array::BigArray")]
    pub name: [u8; 64],
    pub resource_hardness: u16,
    pub key_index: u16,
}
