
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x0a6dda73332b55cb")]
pub struct RepairDockedFleet{
    pub input: RepairFleetInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RepairDockedFleetInstructionAccounts {
    pub game_accounts_fleet_and_owner: solana_pubkey::Pubkey,
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub cargo_pod_from: solana_pubkey::Pubkey,
    pub cargo_type: solana_pubkey::Pubkey,
    pub cargo_stats_definition: solana_pubkey::Pubkey,
    pub token_from: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub fee_token_from: solana_pubkey::Pubkey,
    pub fee_token_to: solana_pubkey::Pubkey,
    pub fee_mint: solana_pubkey::Pubkey,
    pub cargo_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RepairDockedFleet {
    type ArrangedAccounts = RepairDockedFleetInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            game_accounts_fleet_and_owner,
            starbase_and_starbase_player,
            cargo_pod_from,
            cargo_type,
            cargo_stats_definition,
            token_from,
            token_mint,
            fee_token_from,
            fee_token_to,
            fee_mint,
            cargo_program,
            token_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(RepairDockedFleetInstructionAccounts {
            game_accounts_fleet_and_owner: game_accounts_fleet_and_owner.pubkey,
            starbase_and_starbase_player: starbase_and_starbase_player.pubkey,
            cargo_pod_from: cargo_pod_from.pubkey,
            cargo_type: cargo_type.pubkey,
            cargo_stats_definition: cargo_stats_definition.pubkey,
            token_from: token_from.pubkey,
            token_mint: token_mint.pubkey,
            fee_token_from: fee_token_from.pubkey,
            fee_token_to: fee_token_to.pubkey,
            fee_mint: fee_mint.pubkey,
            cargo_program: cargo_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}