use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8f5cc0f9156cad51")]
pub struct CompleteStarbaseUpgrade {
    pub input: KeyIndexInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CompleteStarbaseUpgradeInstructionAccounts {
    pub funder: solana_pubkey::Pubkey,
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub crafting_facility: solana_pubkey::Pubkey,
    pub upgrade_facility: solana_pubkey::Pubkey,
    pub upgrade_recipe: solana_pubkey::Pubkey,
    pub new_recipe_category: solana_pubkey::Pubkey,
    pub crafting_domain: solana_pubkey::Pubkey,
    pub game_accounts_and_profile: solana_pubkey::Pubkey,
    pub crafting_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CompleteStarbaseUpgrade {
    type ArrangedAccounts = CompleteStarbaseUpgradeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            funder,
            starbase_and_starbase_player,
            crafting_facility,
            upgrade_facility,
            upgrade_recipe,
            new_recipe_category,
            crafting_domain,
            game_accounts_and_profile,
            crafting_program,
            system_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(CompleteStarbaseUpgradeInstructionAccounts {
            funder: funder.pubkey,
            starbase_and_starbase_player: starbase_and_starbase_player.pubkey,
            crafting_facility: crafting_facility.pubkey,
            upgrade_facility: upgrade_facility.pubkey,
            upgrade_recipe: upgrade_recipe.pubkey,
            new_recipe_category: new_recipe_category.pubkey,
            crafting_domain: crafting_domain.pubkey,
            game_accounts_and_profile: game_accounts_and_profile.pubkey,
            crafting_program: crafting_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
