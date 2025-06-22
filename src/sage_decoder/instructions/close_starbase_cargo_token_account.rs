
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x61a4629eb4c3fb50")]
pub struct CloseStarbaseCargoTokenAccount{
    pub input: KeyIndexInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseStarbaseCargoTokenAccountInstructionAccounts {
    pub funds_to: solana_pubkey::Pubkey,
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub game_accounts_and_profile: solana_pubkey::Pubkey,
    pub cargo_pod: solana_pubkey::Pubkey,
    pub cargo_type: solana_pubkey::Pubkey,
    pub cargo_stats_definition: solana_pubkey::Pubkey,
    pub token: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub cargo_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseStarbaseCargoTokenAccount {
    type ArrangedAccounts = CloseStarbaseCargoTokenAccountInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            funds_to,
            starbase_and_starbase_player,
            game_accounts_and_profile,
            cargo_pod,
            cargo_type,
            cargo_stats_definition,
            token,
            token_mint,
            cargo_program,
            token_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(CloseStarbaseCargoTokenAccountInstructionAccounts {
            funds_to: funds_to.pubkey,
            starbase_and_starbase_player: starbase_and_starbase_player.pubkey,
            game_accounts_and_profile: game_accounts_and_profile.pubkey,
            cargo_pod: cargo_pod.pubkey,
            cargo_type: cargo_type.pubkey,
            cargo_stats_definition: cargo_stats_definition.pubkey,
            token: token.pubkey,
            token_mint: token_mint.pubkey,
            cargo_program: cargo_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}