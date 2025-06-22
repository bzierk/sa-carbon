use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa55a47de9b070e79")]
pub struct DisbandedFleetToEscrow {
    pub input: DisbandedFleetToEscrowInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DisbandedFleetToEscrowInstructionAccounts {
    pub game_accounts_and_profile: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub disbanded_fleet: solana_pubkey::Pubkey,
    pub fleet_ships: solana_pubkey::Pubkey,
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub ship: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DisbandedFleetToEscrow {
    type ArrangedAccounts = DisbandedFleetToEscrowInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            game_accounts_and_profile,
            funder,
            disbanded_fleet,
            fleet_ships,
            starbase_and_starbase_player,
            ship,
            system_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(DisbandedFleetToEscrowInstructionAccounts {
            game_accounts_and_profile: game_accounts_and_profile.pubkey,
            funder: funder.pubkey,
            disbanded_fleet: disbanded_fleet.pubkey,
            fleet_ships: fleet_ships.pubkey,
            starbase_and_starbase_player: starbase_and_starbase_player.pubkey,
            ship: ship.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
