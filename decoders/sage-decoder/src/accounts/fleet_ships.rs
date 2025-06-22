use carbon_core::{CarbonDeserialize, borsh};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize)]
#[carbon(discriminator = "0xfc5193f6de8db96e")]
pub struct FleetShips {
    pub version: u8,
    pub fleet: solana_pubkey::Pubkey,
    pub fleet_ships_info_count: u32,
    pub bump: u8,
}
