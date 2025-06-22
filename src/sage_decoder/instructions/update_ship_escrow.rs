
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xadcf65f7ace42769")]
pub struct UpdateShipEscrow{
    pub input: UpdateShipEscrowInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateShipEscrowInstructionAccounts {
    pub old_ship: solana_pubkey::Pubkey,
    pub next: solana_pubkey::Pubkey,
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub game_accounts: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateShipEscrow {
    type ArrangedAccounts = UpdateShipEscrowInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            old_ship,
            next,
            starbase_and_starbase_player,
            game_accounts,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(UpdateShipEscrowInstructionAccounts {
            old_ship: old_ship.pubkey,
            next: next.pubkey,
            starbase_and_starbase_player: starbase_and_starbase_player.pubkey,
            game_accounts: game_accounts.pubkey,
        })
    }
}