use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x12f535779b39014e")]
pub struct ForceDisbandFleet {
    pub input: ForcedDisbandFleetInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ForceDisbandFleetInstructionAccounts {
    pub funder: solana_pubkey::Pubkey,
    pub disbanded_fleet: solana_pubkey::Pubkey,
    pub fleet: solana_pubkey::Pubkey,
    pub fleet_ships: solana_pubkey::Pubkey,
    pub cargo_hold: solana_pubkey::Pubkey,
    pub fuel_tank: solana_pubkey::Pubkey,
    pub ammo_bank: solana_pubkey::Pubkey,
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub ship: solana_pubkey::Pubkey,
    pub game_accounts: solana_pubkey::Pubkey,
    pub cargo_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ForceDisbandFleet {
    type ArrangedAccounts = ForceDisbandFleetInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            funder,
            disbanded_fleet,
            fleet,
            fleet_ships,
            cargo_hold,
            fuel_tank,
            ammo_bank,
            starbase_and_starbase_player,
            ship,
            game_accounts,
            cargo_program,
            system_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(ForceDisbandFleetInstructionAccounts {
            funder: funder.pubkey,
            disbanded_fleet: disbanded_fleet.pubkey,
            fleet: fleet.pubkey,
            fleet_ships: fleet_ships.pubkey,
            cargo_hold: cargo_hold.pubkey,
            fuel_tank: fuel_tank.pubkey,
            ammo_bank: ammo_bank.pubkey,
            starbase_and_starbase_player: starbase_and_starbase_player.pubkey,
            ship: ship.pubkey,
            game_accounts: game_accounts.pubkey,
            cargo_program: cargo_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
