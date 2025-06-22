
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize)] 
 

#[carbon(discriminator = "0xe09c815f0f1d84d0")] 
pub struct ProgressionConfig {
        pub version: u8,
        pub game_id: solana_pubkey::Pubkey,
        pub daily_lp_limit: u64,
        pub daily_council_rank_xp_limit: u64,
        pub daily_pilot_xp_limit: u64,
        pub daily_data_running_xp_limit: u64,
        pub daily_mining_xp_limit: u64,
        pub daily_crafting_xp_limit: u64,
        pub num_items: u16, 
}