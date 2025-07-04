use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6442d2bb6ec7d36b")]
pub struct DeregisterStarbase {
    pub input: KeyIndexInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DeregisterStarbaseInstructionAccounts {
    pub game_and_profile: solana_pubkey::Pubkey,
    pub funds_to: solana_pubkey::Pubkey,
    pub starbase: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DeregisterStarbase {
    type ArrangedAccounts = DeregisterStarbaseInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [game_and_profile, funds_to, starbase, _remaining @ ..] = accounts else {
            return None;
        };

        Some(DeregisterStarbaseInstructionAccounts {
            game_and_profile: game_and_profile.pubkey,
            funds_to: funds_to.pubkey,
            starbase: starbase.pubkey,
        })
    }
}
