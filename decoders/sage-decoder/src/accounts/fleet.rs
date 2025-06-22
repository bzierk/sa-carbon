use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize)]
#[carbon(discriminator = "0x6dcffb306a0288a3")]
pub struct Fleet {
    pub version: u8,
    pub game_id: solana_pubkey::Pubkey,
    pub owner_profile: solana_pubkey::Pubkey,
    pub fleet_ships: solana_pubkey::Pubkey,
    pub sub_profile: OptionalNonSystemPubkey,
    pub sub_profile_invalidator: solana_pubkey::Pubkey,
    pub faction: u8,
    pub fleet_label: [u8; 32],
    pub ship_counts: ShipCounts,
    pub warp_cooldown_expires_at: i64,
    pub scan_cooldown_expires_at: i64,
    pub stats: ShipStats,
    pub cargo_hold: solana_pubkey::Pubkey,
    pub fuel_tank: solana_pubkey::Pubkey,
    pub ammo_bank: solana_pubkey::Pubkey,
    pub ap: u32,
    pub sp: u32,
    pub hp: u32,
    pub pending_hp: u32,
    pub ap_reload_expires_at: i64,
    pub shield_break_delay_expires_at: i64,
    pub last_combat_update: i64,
    pub update_id: u64,
    pub bump: u8,
}
