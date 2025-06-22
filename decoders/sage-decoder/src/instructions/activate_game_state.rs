use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x86e32e1555787183")]
pub struct ActivateGameState {
    pub input: ManageGameInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ActivateGameStateInstructionAccounts {
    pub game_and_profile: solana_pubkey::Pubkey,
    pub game_state: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ActivateGameState {
    type ArrangedAccounts = ActivateGameStateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [game_and_profile, game_state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ActivateGameStateInstructionAccounts {
            game_and_profile: game_and_profile.pubkey,
            game_state: game_state.pubkey,
        })
    }
}
