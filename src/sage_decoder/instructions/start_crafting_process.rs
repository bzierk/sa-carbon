
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x406c6d3e09808af6")]
pub struct StartCraftingProcess{
    pub input: KeyIndexInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct StartCraftingProcessInstructionAccounts {
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub crafting_instance: solana_pubkey::Pubkey,
    pub crafting_process: solana_pubkey::Pubkey,
    pub crafting_recipe: solana_pubkey::Pubkey,
    pub crafting_facility: solana_pubkey::Pubkey,
    pub game_accounts_and_profile: solana_pubkey::Pubkey,
    pub crafting_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for StartCraftingProcess {
    type ArrangedAccounts = StartCraftingProcessInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            starbase_and_starbase_player,
            crafting_instance,
            crafting_process,
            crafting_recipe,
            crafting_facility,
            game_accounts_and_profile,
            crafting_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(StartCraftingProcessInstructionAccounts {
            starbase_and_starbase_player: starbase_and_starbase_player.pubkey,
            crafting_instance: crafting_instance.pubkey,
            crafting_process: crafting_process.pubkey,
            crafting_recipe: crafting_recipe.pubkey,
            crafting_facility: crafting_facility.pubkey,
            game_accounts_and_profile: game_accounts_and_profile.pubkey,
            crafting_program: crafting_program.pubkey,
        })
    }
}