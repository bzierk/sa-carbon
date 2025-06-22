
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xeda699487ab3dc4e")]
pub struct AddShipToFleet{
    pub input: AddShipToFleetInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct AddShipToFleetInstructionAccounts {
    pub game_accounts_fleet_and_owner: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub fleet_ships: solana_pubkey::Pubkey,
    pub ship: solana_pubkey::Pubkey,
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AddShipToFleet {
    type ArrangedAccounts = AddShipToFleetInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            game_accounts_fleet_and_owner,
            funder,
            fleet_ships,
            ship,
            starbase_and_starbase_player,
            system_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(AddShipToFleetInstructionAccounts {
            game_accounts_fleet_and_owner: game_accounts_fleet_and_owner.pubkey,
            funder: funder.pubkey,
            fleet_ships: fleet_ships.pubkey,
            ship: ship.pubkey,
            starbase_and_starbase_player: starbase_and_starbase_player.pubkey,
            system_program: system_program.pubkey,
        })
    }
}