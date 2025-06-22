

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct RegisterStarInput {
    #[serde(with = "serde_big_array::BigArray")]
    pub name: [u8; 64],
    pub size: u64,
    pub sub_coordinates: [i64; 2],
    pub star_type: u8,
    pub key_index: u16,
}
