
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x4c7b5152ebe49ccb")]
pub struct CreateFleet{
    pub input: CreateFleetInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateFleetInstructionAccounts {
    pub game_accounts_and_profile: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub fleet: solana_pubkey::Pubkey,
    pub fleet_ships: solana_pubkey::Pubkey,
    pub cargo_hold: solana_pubkey::Pubkey,
    pub fuel_tank: solana_pubkey::Pubkey,
    pub ammo_bank: solana_pubkey::Pubkey,
    pub ship: solana_pubkey::Pubkey,
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub cargo_stats_definition: solana_pubkey::Pubkey,
    pub cargo_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateFleet {
    type ArrangedAccounts = CreateFleetInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            game_accounts_and_profile,
            funder,
            fleet,
            fleet_ships,
            cargo_hold,
            fuel_tank,
            ammo_bank,
            ship,
            starbase_and_starbase_player,
            cargo_stats_definition,
            cargo_program,
            system_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(CreateFleetInstructionAccounts {
            game_accounts_and_profile: game_accounts_and_profile.pubkey,
            funder: funder.pubkey,
            fleet: fleet.pubkey,
            fleet_ships: fleet_ships.pubkey,
            cargo_hold: cargo_hold.pubkey,
            fuel_tank: fuel_tank.pubkey,
            ammo_bank: ammo_bank.pubkey,
            ship: ship.pubkey,
            starbase_and_starbase_player: starbase_and_starbase_player.pubkey,
            cargo_stats_definition: cargo_stats_definition.pubkey,
            cargo_program: cargo_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}