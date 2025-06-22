
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize)] 
 

#[carbon(discriminator = "0x35067f17f70ce1f9")] 
pub struct DisbandedFleet {
        pub version: u8,
        pub game_id: solana_pubkey::Pubkey,
        pub owner_profile: solana_pubkey::Pubkey,
        pub starbase: solana_pubkey::Pubkey,
        pub fleet_label: [u8; 32],
        pub fleet_ships: solana_pubkey::Pubkey,
        pub bump: u8, 
}