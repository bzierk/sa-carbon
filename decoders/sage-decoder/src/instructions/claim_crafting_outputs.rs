use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd7473b1c9c5dbcff")]
pub struct ClaimCraftingOutputs {
    pub input: IngredientIndexInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ClaimCraftingOutputsInstructionAccounts {
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub crafting_instance: solana_pubkey::Pubkey,
    pub crafting_process: solana_pubkey::Pubkey,
    pub crafting_facility: solana_pubkey::Pubkey,
    pub crafting_recipe: solana_pubkey::Pubkey,
    pub craftable_item: solana_pubkey::Pubkey,
    pub cargo_pod_to: solana_pubkey::Pubkey,
    pub cargo_type: solana_pubkey::Pubkey,
    pub cargo_stats_definition: solana_pubkey::Pubkey,
    pub game_accounts: solana_pubkey::Pubkey,
    pub token_from: solana_pubkey::Pubkey,
    pub token_to: solana_pubkey::Pubkey,
    pub crafting_program: solana_pubkey::Pubkey,
    pub cargo_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimCraftingOutputs {
    type ArrangedAccounts = ClaimCraftingOutputsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            starbase_and_starbase_player,
            crafting_instance,
            crafting_process,
            crafting_facility,
            crafting_recipe,
            craftable_item,
            cargo_pod_to,
            cargo_type,
            cargo_stats_definition,
            game_accounts,
            token_from,
            token_to,
            crafting_program,
            cargo_program,
            token_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(ClaimCraftingOutputsInstructionAccounts {
            starbase_and_starbase_player: starbase_and_starbase_player.pubkey,
            crafting_instance: crafting_instance.pubkey,
            crafting_process: crafting_process.pubkey,
            crafting_facility: crafting_facility.pubkey,
            crafting_recipe: crafting_recipe.pubkey,
            craftable_item: craftable_item.pubkey,
            cargo_pod_to: cargo_pod_to.pubkey,
            cargo_type: cargo_type.pubkey,
            cargo_stats_definition: cargo_stats_definition.pubkey,
            game_accounts: game_accounts.pubkey,
            token_from: token_from.pubkey,
            token_to: token_to.pubkey,
            crafting_program: crafting_program.pubkey,
            cargo_program: cargo_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
