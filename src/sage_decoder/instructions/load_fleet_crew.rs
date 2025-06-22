
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x2709d4daa76b89d0")]
pub struct LoadFleetCrew{
    pub input: FleetCrewInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct LoadFleetCrewInstructionAccounts {
    pub fleet_and_owner: solana_pubkey::Pubkey,
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub game_id: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LoadFleetCrew {
    type ArrangedAccounts = LoadFleetCrewInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            fleet_and_owner,
            starbase_and_starbase_player,
            game_id,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(LoadFleetCrewInstructionAccounts {
            fleet_and_owner: fleet_and_owner.pubkey,
            starbase_and_starbase_player: starbase_and_starbase_player.pubkey,
            game_id: game_id.pubkey,
        })
    }
}