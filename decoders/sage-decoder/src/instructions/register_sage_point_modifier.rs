use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd4fba4b49e13ad16")]
pub struct RegisterSagePointModifier {
    pub input: RegisterSagePointsModifierInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RegisterSagePointModifierInstructionAccounts {
    pub game_and_profile: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub points_category: solana_pubkey::Pubkey,
    pub points_modifier: solana_pubkey::Pubkey,
    pub points_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RegisterSagePointModifier {
    type ArrangedAccounts = RegisterSagePointModifierInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            game_and_profile,
            funder,
            points_category,
            points_modifier,
            points_program,
            system_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(RegisterSagePointModifierInstructionAccounts {
            game_and_profile: game_and_profile.pubkey,
            funder: funder.pubkey,
            points_category: points_category.pubkey,
            points_modifier: points_modifier.pubkey,
            points_program: points_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
