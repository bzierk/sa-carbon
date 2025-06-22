
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x57317594f1f7b012")]
pub struct DepositCargoToGame{
    pub input: CargoToGameInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DepositCargoToGameInstructionAccounts {
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub cargo_pod: solana_pubkey::Pubkey,
    pub cargo_type: solana_pubkey::Pubkey,
    pub cargo_stats_definition: solana_pubkey::Pubkey,
    pub game_accounts_and_profile: solana_pubkey::Pubkey,
    pub token_from: solana_pubkey::Pubkey,
    pub token_to: solana_pubkey::Pubkey,
    pub cargo_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DepositCargoToGame {
    type ArrangedAccounts = DepositCargoToGameInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            starbase_and_starbase_player,
            cargo_pod,
            cargo_type,
            cargo_stats_definition,
            game_accounts_and_profile,
            token_from,
            token_to,
            cargo_program,
            token_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(DepositCargoToGameInstructionAccounts {
            starbase_and_starbase_player: starbase_and_starbase_player.pubkey,
            cargo_pod: cargo_pod.pubkey,
            cargo_type: cargo_type.pubkey,
            cargo_stats_definition: cargo_stats_definition.pubkey,
            game_accounts_and_profile: game_accounts_and_profile.pubkey,
            token_from: token_from.pubkey,
            token_to: token_to.pubkey,
            cargo_program: cargo_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}