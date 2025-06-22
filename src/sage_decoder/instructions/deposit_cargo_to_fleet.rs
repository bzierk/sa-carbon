
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x37eb697b00fd40ed")]
pub struct DepositCargoToFleet{
    pub input: DepositCargoToFleetInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DepositCargoToFleetInstructionAccounts {
    pub game_accounts_fleet_and_owner: solana_pubkey::Pubkey,
    pub funds_to: solana_pubkey::Pubkey,
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub cargo_pod_from: solana_pubkey::Pubkey,
    pub cargo_pod_to: solana_pubkey::Pubkey,
    pub cargo_type: solana_pubkey::Pubkey,
    pub cargo_stats_definition: solana_pubkey::Pubkey,
    pub token_from: solana_pubkey::Pubkey,
    pub token_to: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub cargo_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DepositCargoToFleet {
    type ArrangedAccounts = DepositCargoToFleetInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            game_accounts_fleet_and_owner,
            funds_to,
            starbase_and_starbase_player,
            cargo_pod_from,
            cargo_pod_to,
            cargo_type,
            cargo_stats_definition,
            token_from,
            token_to,
            token_mint,
            cargo_program,
            token_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(DepositCargoToFleetInstructionAccounts {
            game_accounts_fleet_and_owner: game_accounts_fleet_and_owner.pubkey,
            funds_to: funds_to.pubkey,
            starbase_and_starbase_player: starbase_and_starbase_player.pubkey,
            cargo_pod_from: cargo_pod_from.pubkey,
            cargo_pod_to: cargo_pod_to.pubkey,
            cargo_type: cargo_type.pubkey,
            cargo_stats_definition: cargo_stats_definition.pubkey,
            token_from: token_from.pubkey,
            token_to: token_to.pubkey,
            token_mint: token_mint.pubkey,
            cargo_program: cargo_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}