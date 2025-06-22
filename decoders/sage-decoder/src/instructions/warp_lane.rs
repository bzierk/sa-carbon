use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe8bcc3316448e7f3")]
pub struct WarpLane {
    pub input: WarpLaneInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct WarpLaneInstructionAccounts {
    pub game_accounts_fleet_and_owner: solana_pubkey::Pubkey,
    pub from_starbase: solana_pubkey::Pubkey,
    pub to_starbase: solana_pubkey::Pubkey,
    pub from_sector: solana_pubkey::Pubkey,
    pub to_sector: solana_pubkey::Pubkey,
    pub fuel_tank: solana_pubkey::Pubkey,
    pub cargo_type: solana_pubkey::Pubkey,
    pub stats_definition: solana_pubkey::Pubkey,
    pub fuel_token_from: solana_pubkey::Pubkey,
    pub fuel_mint: solana_pubkey::Pubkey,
    pub fee_token_from: solana_pubkey::Pubkey,
    pub fee_token_to: solana_pubkey::Pubkey,
    pub fee_mint: solana_pubkey::Pubkey,
    pub cargo_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WarpLane {
    type ArrangedAccounts = WarpLaneInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            game_accounts_fleet_and_owner,
            from_starbase,
            to_starbase,
            from_sector,
            to_sector,
            fuel_tank,
            cargo_type,
            stats_definition,
            fuel_token_from,
            fuel_mint,
            fee_token_from,
            fee_token_to,
            fee_mint,
            cargo_program,
            token_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(WarpLaneInstructionAccounts {
            game_accounts_fleet_and_owner: game_accounts_fleet_and_owner.pubkey,
            from_starbase: from_starbase.pubkey,
            to_starbase: to_starbase.pubkey,
            from_sector: from_sector.pubkey,
            to_sector: to_sector.pubkey,
            fuel_tank: fuel_tank.pubkey,
            cargo_type: cargo_type.pubkey,
            stats_definition: stats_definition.pubkey,
            fuel_token_from: fuel_token_from.pubkey,
            fuel_mint: fuel_mint.pubkey,
            fee_token_from: fee_token_from.pubkey,
            fee_token_to: fee_token_to.pubkey,
            fee_mint: fee_mint.pubkey,
            cargo_program: cargo_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
