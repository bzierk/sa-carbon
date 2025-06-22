
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x5f4dfea2f8a81110")]
pub struct CopyGameState{
    pub input: ManageGameInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CopyGameStateInstructionAccounts {
    pub game_and_profile: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub old_game_state: solana_pubkey::Pubkey,
    pub new_game_state: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CopyGameState {
    type ArrangedAccounts = CopyGameStateInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            game_and_profile,
            funder,
            old_game_state,
            new_game_state,
            system_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(CopyGameStateInstructionAccounts {
            game_and_profile: game_and_profile.pubkey,
            funder: funder.pubkey,
            old_game_state: old_game_state.pubkey,
            new_game_state: new_game_state.pubkey,
            system_program: system_program.pubkey,
        })
    }
}