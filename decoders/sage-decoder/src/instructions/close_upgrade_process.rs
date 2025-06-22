use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd6848a7b88115952")]
pub struct CloseUpgradeProcess {
    pub input: KeyIndexInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseUpgradeProcessInstructionAccounts {
    pub funds_to: solana_pubkey::Pubkey,
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub resource_crafting_instance: solana_pubkey::Pubkey,
    pub resource_crafting_process: solana_pubkey::Pubkey,
    pub resource_recipe: solana_pubkey::Pubkey,
    pub resource_crafting_facility: solana_pubkey::Pubkey,
    pub game_accounts_and_profile: solana_pubkey::Pubkey,
    pub crafting_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseUpgradeProcess {
    type ArrangedAccounts = CloseUpgradeProcessInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            funds_to,
            starbase_and_starbase_player,
            resource_crafting_instance,
            resource_crafting_process,
            resource_recipe,
            resource_crafting_facility,
            game_accounts_and_profile,
            crafting_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(CloseUpgradeProcessInstructionAccounts {
            funds_to: funds_to.pubkey,
            starbase_and_starbase_player: starbase_and_starbase_player.pubkey,
            resource_crafting_instance: resource_crafting_instance.pubkey,
            resource_crafting_process: resource_crafting_process.pubkey,
            resource_recipe: resource_recipe.pubkey,
            resource_crafting_facility: resource_crafting_facility.pubkey,
            game_accounts_and_profile: game_accounts_and_profile.pubkey,
            crafting_program: crafting_program.pubkey,
        })
    }
}
