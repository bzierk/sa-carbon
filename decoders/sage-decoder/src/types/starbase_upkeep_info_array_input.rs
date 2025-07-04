use super::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct StarbaseUpkeepInfoArrayInput {
    pub level: u8,
    pub info: StarbaseUpkeepInfoUnpacked,
}
