
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize)] 
 

#[carbon(discriminator = "0x0a374bea7e0e2f92")] 
pub struct SagePlayerProfile {
        pub version: u8,
        pub player_profile: solana_pubkey::Pubkey,
        pub game_id: solana_pubkey::Pubkey,
        pub bump: u8, 
}