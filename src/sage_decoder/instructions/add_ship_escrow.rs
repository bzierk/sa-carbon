
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xba13da96a7b5d459")]
pub struct AddShipEscrow{
    pub input: AddShipEscrowInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct AddShipEscrowInstructionAccounts {
    pub funder: solana_pubkey::Pubkey,
    pub sage_player_profile: solana_pubkey::Pubkey,
    pub origin_token_account: solana_pubkey::Pubkey,
    pub ship: solana_pubkey::Pubkey,
    pub ship_escrow_token_account: solana_pubkey::Pubkey,
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub game_accounts_and_profile: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AddShipEscrow {
    type ArrangedAccounts = AddShipEscrowInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            funder,
            sage_player_profile,
            origin_token_account,
            ship,
            ship_escrow_token_account,
            starbase_and_starbase_player,
            game_accounts_and_profile,
            token_program,
            system_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(AddShipEscrowInstructionAccounts {
            funder: funder.pubkey,
            sage_player_profile: sage_player_profile.pubkey,
            origin_token_account: origin_token_account.pubkey,
            ship: ship.pubkey,
            ship_escrow_token_account: ship_escrow_token_account.pubkey,
            starbase_and_starbase_player: starbase_and_starbase_player.pubkey,
            game_accounts_and_profile: game_accounts_and_profile.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}