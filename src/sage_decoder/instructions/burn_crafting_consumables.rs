
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x0de1cb5b36e87eaa")]
pub struct BurnCraftingConsumables{
    pub input: IngredientIndexInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct BurnCraftingConsumablesInstructionAccounts {
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub crafting_instance: solana_pubkey::Pubkey,
    pub crafting_process: solana_pubkey::Pubkey,
    pub crafting_facility: solana_pubkey::Pubkey,
    pub crafting_recipe: solana_pubkey::Pubkey,
    pub game_accounts: solana_pubkey::Pubkey,
    pub token_from: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub crafting_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for BurnCraftingConsumables {
    type ArrangedAccounts = BurnCraftingConsumablesInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            starbase_and_starbase_player,
            crafting_instance,
            crafting_process,
            crafting_facility,
            crafting_recipe,
            game_accounts,
            token_from,
            token_mint,
            crafting_program,
            token_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(BurnCraftingConsumablesInstructionAccounts {
            starbase_and_starbase_player: starbase_and_starbase_player.pubkey,
            crafting_instance: crafting_instance.pubkey,
            crafting_process: crafting_process.pubkey,
            crafting_facility: crafting_facility.pubkey,
            crafting_recipe: crafting_recipe.pubkey,
            game_accounts: game_accounts.pubkey,
            token_from: token_from.pubkey,
            token_mint: token_mint.pubkey,
            crafting_program: crafting_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}