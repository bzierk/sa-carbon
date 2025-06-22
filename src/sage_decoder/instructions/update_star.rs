
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x91920c560c3027a9")]
pub struct UpdateStar{
    pub input: UpdateStarInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateStarInstructionAccounts {
    pub game_and_profile: solana_pubkey::Pubkey,
    pub star: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateStar {
    type ArrangedAccounts = UpdateStarInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            game_and_profile,
            star,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(UpdateStarInstructionAccounts {
            game_and_profile: game_and_profile.pubkey,
            star: star.pubkey,
        })
    }
}