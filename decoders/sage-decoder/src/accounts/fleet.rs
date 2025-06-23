use super::super::types::*;

use carbon_core::borsh::{self, BorshDeserialize};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
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
    pub fleet_state: FleetStance,
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, BorshDeserialize)]
pub enum FleetStance {
    StarbaseLoadingBay(StarbaseLoadingBay),
    Idle(Idle),
    MineAsteroid(MineAsteroid),
    MoveWarp(MoveWarp),
    MoveSubwarp(MoveSubwarp),
    Respawn(Respawn),
}

#[derive(BorshDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct StarbaseLoadingBay {
    pub starbase: solana_pubkey::Pubkey,
    pub last_update: i64,
}

#[derive(BorshDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Idle {
    pub sector: [i64; 2],
}

#[derive(BorshDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct MineAsteroid {
    pub asteroid: solana_pubkey::Pubkey,
    pub resource: solana_pubkey::Pubkey,
    pub start: i64,
    pub end: i64,
    pub amount_mined: u64,
    pub last_update: i64,
}

#[derive(BorshDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct MoveWarp {
    pub from_sector: [i64; 2],
    pub to_sector: [i64; 2],
    pub warp_start: i64,
    pub warp_finish: i64,
}

#[derive(BorshDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct MoveSubwarp {
    pub from_sector: [i64; 2],
    pub to_sector: [i64; 2],
    pub current_sector: [i64; 2],
    pub departure_time: i64,
    pub arrival_time: i64,
    pub fuel_expenditure: u64,
    pub last_update: i64,
}

#[derive(BorshDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Respawn {
    pub sector: [i64; 2],
    pub start: i64,
}

impl borsh::de::BorshDeserialize for Fleet
where
    u8: borsh::BorshDeserialize,
    solana_pubkey::Pubkey: borsh::BorshDeserialize,
    solana_pubkey::Pubkey: borsh::BorshDeserialize,
    solana_pubkey::Pubkey: borsh::BorshDeserialize,
    OptionalNonSystemPubkey: borsh::BorshDeserialize,
    solana_pubkey::Pubkey: borsh::BorshDeserialize,
    u8: borsh::BorshDeserialize,
    [u8; 32]: borsh::BorshDeserialize,
    ShipCounts: borsh::BorshDeserialize,
    i64: borsh::BorshDeserialize,
    i64: borsh::BorshDeserialize,
    ShipStats: borsh::BorshDeserialize,
    solana_pubkey::Pubkey: borsh::BorshDeserialize,
    solana_pubkey::Pubkey: borsh::BorshDeserialize,
    solana_pubkey::Pubkey: borsh::BorshDeserialize,
    u32: borsh::BorshDeserialize,
    u32: borsh::BorshDeserialize,
    u32: borsh::BorshDeserialize,
    u32: borsh::BorshDeserialize,
    i64: borsh::BorshDeserialize,
    i64: borsh::BorshDeserialize,
    i64: borsh::BorshDeserialize,
    u64: borsh::BorshDeserialize,
    u8: borsh::BorshDeserialize,
{
    fn deserialize_reader<R: borsh::maybestd::io::Read>(
        reader: &mut R,
    ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            version: borsh::BorshDeserialize::deserialize_reader(reader)?,
            game_id: borsh::BorshDeserialize::deserialize_reader(reader)?,
            owner_profile: borsh::BorshDeserialize::deserialize_reader(reader)?,
            fleet_ships: borsh::BorshDeserialize::deserialize_reader(reader)?,
            sub_profile: borsh::BorshDeserialize::deserialize_reader(reader)?,
            sub_profile_invalidator: borsh::BorshDeserialize::deserialize_reader(reader)?,
            faction: borsh::BorshDeserialize::deserialize_reader(reader)?,
            fleet_label: borsh::BorshDeserialize::deserialize_reader(reader)?,
            ship_counts: borsh::BorshDeserialize::deserialize_reader(reader)?,
            warp_cooldown_expires_at: borsh::BorshDeserialize::deserialize_reader(reader)?,
            scan_cooldown_expires_at: borsh::BorshDeserialize::deserialize_reader(reader)?,
            stats: borsh::BorshDeserialize::deserialize_reader(reader)?,
            cargo_hold: borsh::BorshDeserialize::deserialize_reader(reader)?,
            fuel_tank: borsh::BorshDeserialize::deserialize_reader(reader)?,
            ammo_bank: borsh::BorshDeserialize::deserialize_reader(reader)?,
            ap: borsh::BorshDeserialize::deserialize_reader(reader)?,
            sp: borsh::BorshDeserialize::deserialize_reader(reader)?,
            hp: borsh::BorshDeserialize::deserialize_reader(reader)?,
            pending_hp: borsh::BorshDeserialize::deserialize_reader(reader)?,
            ap_reload_expires_at: borsh::BorshDeserialize::deserialize_reader(reader)?,
            shield_break_delay_expires_at: borsh::BorshDeserialize::deserialize_reader(reader)?,
            last_combat_update: borsh::BorshDeserialize::deserialize_reader(reader)?,
            update_id: borsh::BorshDeserialize::deserialize_reader(reader)?,
            bump: borsh::BorshDeserialize::deserialize_reader(reader)?,
            fleet_state: borsh::BorshDeserialize::deserialize_reader(reader)?,
        })
    }
}

#[automatically_derived]
impl carbon_core::deserialize::CarbonDeserialize for Fleet {
    fn deserialize(data: &[u8]) -> Option<Self> {
        let discriminator: &[u8] = &[109u8, 207u8, 251u8, 48u8, 106u8, 2u8, 136u8, 163u8];
        if data.len() < discriminator.len() {
            return None;
        }
        let (disc, mut rest) = data.split_at(discriminator.len());
        if disc != discriminator {
            return None;
        }
        match carbon_core::borsh::BorshDeserialize::deserialize(&mut rest) {
            Ok(res) => {
                if !rest.is_empty() {
                    carbon_core::log::warn!(
                        "Not all bytes were read when deserializing {}: {} bytes remaining",
                        stringify!(Fleet),
                        rest.len(),
                    );
                }
                Some(res)
            }
            Err(_) => None,
        }
    }
}
