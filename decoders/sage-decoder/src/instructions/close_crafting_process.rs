use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xca15e19c0f046a5d")]
pub struct CloseCraftingProcess {
    pub input: KeyIndexInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseCraftingProcessInstructionAccounts {
    pub funds_to: solana_pubkey::Pubkey,
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub crafting_instance: solana_pubkey::Pubkey,
    pub crafting_process: solana_pubkey::Pubkey,
    pub crafting_recipe: solana_pubkey::Pubkey,
    pub crafting_facility: solana_pubkey::Pubkey,
    pub game_accounts_and_profile: solana_pubkey::Pubkey,
    pub crafting_xp_accounts: solana_pubkey::Pubkey,
    pub council_rank_xp_accounts: solana_pubkey::Pubkey,
    pub progression_config: solana_pubkey::Pubkey,
    pub points_program: solana_pubkey::Pubkey,
    pub crafting_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseCraftingProcess {
    type ArrangedAccounts = CloseCraftingProcessInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            funds_to,
            starbase_and_starbase_player,
            crafting_instance,
            crafting_process,
            crafting_recipe,
            crafting_facility,
            game_accounts_and_profile,
            crafting_xp_accounts,
            council_rank_xp_accounts,
            progression_config,
            points_program,
            crafting_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(CloseCraftingProcessInstructionAccounts {
            funds_to: funds_to.pubkey,
            starbase_and_starbase_player: starbase_and_starbase_player.pubkey,
            crafting_instance: crafting_instance.pubkey,
            crafting_process: crafting_process.pubkey,
            crafting_recipe: crafting_recipe.pubkey,
            crafting_facility: crafting_facility.pubkey,
            game_accounts_and_profile: game_accounts_and_profile.pubkey,
            crafting_xp_accounts: crafting_xp_accounts.pubkey,
            council_rank_xp_accounts: council_rank_xp_accounts.pubkey,
            progression_config: progression_config.pubkey,
            points_program: points_program.pubkey,
            crafting_program: crafting_program.pubkey,
        })
    }
}
