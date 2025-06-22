
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xdd4c5c7fe6232943")]
pub struct DrainMineItemBank{
    pub input: KeyIndexInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DrainMineItemBankInstructionAccounts {
    pub game_and_profile: solana_pubkey::Pubkey,
    pub funds_to: solana_pubkey::Pubkey,
    pub mine_item: solana_pubkey::Pubkey,
    pub token_from: solana_pubkey::Pubkey,
    pub token_to: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DrainMineItemBank {
    type ArrangedAccounts = DrainMineItemBankInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            game_and_profile,
            funds_to,
            mine_item,
            token_from,
            token_to,
            token_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(DrainMineItemBankInstructionAccounts {
            game_and_profile: game_and_profile.pubkey,
            funds_to: funds_to.pubkey,
            mine_item: mine_item.pubkey,
            token_from: token_from.pubkey,
            token_to: token_to.pubkey,
            token_program: token_program.pubkey,
        })
    }
}