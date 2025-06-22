use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x60cb819e4a16e5f8")]
pub struct UpdateGameState {
    pub input: UpdateGameStateInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateGameStateInstructionAccounts {
    pub game_and_profile: solana_pubkey::Pubkey,
    pub game_state: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateGameState {
    type ArrangedAccounts = UpdateGameStateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [game_and_profile, game_state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateGameStateInstructionAccounts {
            game_and_profile: game_and_profile.pubkey,
            game_state: game_state.pubkey,
        })
    }
}
