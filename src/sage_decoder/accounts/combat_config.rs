
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize)] 
 

#[carbon(discriminator = "0xf5d3483f2c8276c1")] 
pub struct CombatConfig {
        pub version: u8,
        pub game_id: solana_pubkey::Pubkey,
        pub global_ceasefire: u8,
        pub loot_exclusivity_time: u16,
        pub starbase_upgrade_progress_continuation: u32,
        pub crew_respawn_time: u16,
        pub raw_ships_respawn_time: u16, 
}