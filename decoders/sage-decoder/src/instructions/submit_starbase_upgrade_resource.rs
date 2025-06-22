use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x35b6e6e839c9a778")]
pub struct SubmitStarbaseUpgradeResource {
    pub input: SubmitStarbaseUpgradeResourceInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SubmitStarbaseUpgradeResourceInstructionAccounts {
    pub funds_to: solana_pubkey::Pubkey,
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub resource_crafting_instance: solana_pubkey::Pubkey,
    pub resource_crafting_process: solana_pubkey::Pubkey,
    pub resource_crafting_facility: solana_pubkey::Pubkey,
    pub upgrade_process_recipe: solana_pubkey::Pubkey,
    pub starbase_upgrade_recipe: solana_pubkey::Pubkey,
    pub resource_recipe: solana_pubkey::Pubkey,
    pub cargo_pod_to: solana_pubkey::Pubkey,
    pub cargo_type: solana_pubkey::Pubkey,
    pub cargo_stats_definition: solana_pubkey::Pubkey,
    pub token_from: solana_pubkey::Pubkey,
    pub token_to: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub game_accounts_and_profile: solana_pubkey::Pubkey,
    pub loyalty_points_accounts: solana_pubkey::Pubkey,
    pub progression_config: solana_pubkey::Pubkey,
    pub points_program: solana_pubkey::Pubkey,
    pub crafting_program: solana_pubkey::Pubkey,
    pub cargo_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SubmitStarbaseUpgradeResource {
    type ArrangedAccounts = SubmitStarbaseUpgradeResourceInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            funds_to,
            starbase_and_starbase_player,
            resource_crafting_instance,
            resource_crafting_process,
            resource_crafting_facility,
            upgrade_process_recipe,
            starbase_upgrade_recipe,
            resource_recipe,
            cargo_pod_to,
            cargo_type,
            cargo_stats_definition,
            token_from,
            token_to,
            token_mint,
            game_accounts_and_profile,
            loyalty_points_accounts,
            progression_config,
            points_program,
            crafting_program,
            cargo_program,
            token_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(SubmitStarbaseUpgradeResourceInstructionAccounts {
            funds_to: funds_to.pubkey,
            starbase_and_starbase_player: starbase_and_starbase_player.pubkey,
            resource_crafting_instance: resource_crafting_instance.pubkey,
            resource_crafting_process: resource_crafting_process.pubkey,
            resource_crafting_facility: resource_crafting_facility.pubkey,
            upgrade_process_recipe: upgrade_process_recipe.pubkey,
            starbase_upgrade_recipe: starbase_upgrade_recipe.pubkey,
            resource_recipe: resource_recipe.pubkey,
            cargo_pod_to: cargo_pod_to.pubkey,
            cargo_type: cargo_type.pubkey,
            cargo_stats_definition: cargo_stats_definition.pubkey,
            token_from: token_from.pubkey,
            token_to: token_to.pubkey,
            token_mint: token_mint.pubkey,
            game_accounts_and_profile: game_accounts_and_profile.pubkey,
            loyalty_points_accounts: loyalty_points_accounts.pubkey,
            progression_config: progression_config.pubkey,
            points_program: points_program.pubkey,
            crafting_program: crafting_program.pubkey,
            cargo_program: cargo_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
