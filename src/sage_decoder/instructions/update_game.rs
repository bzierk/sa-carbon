
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x9f3d848303ead1dc")]
pub struct UpdateGame{
    pub input: UpdateGameInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateGameInstructionAccounts {
    pub game_and_profile: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateGame {
    type ArrangedAccounts = UpdateGameInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            game_and_profile,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(UpdateGameInstructionAccounts {
            game_and_profile: game_and_profile.pubkey,
        })
    }
}