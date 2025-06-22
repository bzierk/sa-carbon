use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb54d2da3671bd351")]
pub struct StopMiningAsteroid {
    pub input: StopMiningAsteroidInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct StopMiningAsteroidInstructionAccounts {
    pub game_accounts_fleet_and_owner: solana_pubkey::Pubkey,
    pub mine_item: solana_pubkey::Pubkey,
    pub resource: solana_pubkey::Pubkey,
    pub planet: solana_pubkey::Pubkey,
    pub fuel_tank: solana_pubkey::Pubkey,
    pub cargo_type: solana_pubkey::Pubkey,
    pub cargo_stats_definition: solana_pubkey::Pubkey,
    pub token_from: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub pilot_xp_accounts: solana_pubkey::Pubkey,
    pub mining_xp_accounts: solana_pubkey::Pubkey,
    pub council_rank_xp_accounts: solana_pubkey::Pubkey,
    pub progression_config: solana_pubkey::Pubkey,
    pub points_program: solana_pubkey::Pubkey,
    pub cargo_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for StopMiningAsteroid {
    type ArrangedAccounts = StopMiningAsteroidInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            game_accounts_fleet_and_owner,
            mine_item,
            resource,
            planet,
            fuel_tank,
            cargo_type,
            cargo_stats_definition,
            token_from,
            token_mint,
            pilot_xp_accounts,
            mining_xp_accounts,
            council_rank_xp_accounts,
            progression_config,
            points_program,
            cargo_program,
            token_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(StopMiningAsteroidInstructionAccounts {
            game_accounts_fleet_and_owner: game_accounts_fleet_and_owner.pubkey,
            mine_item: mine_item.pubkey,
            resource: resource.pubkey,
            planet: planet.pubkey,
            fuel_tank: fuel_tank.pubkey,
            cargo_type: cargo_type.pubkey,
            cargo_stats_definition: cargo_stats_definition.pubkey,
            token_from: token_from.pubkey,
            token_mint: token_mint.pubkey,
            pilot_xp_accounts: pilot_xp_accounts.pubkey,
            mining_xp_accounts: mining_xp_accounts.pubkey,
            council_rank_xp_accounts: council_rank_xp_accounts.pubkey,
            progression_config: progression_config.pubkey,
            points_program: points_program.pubkey,
            cargo_program: cargo_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
