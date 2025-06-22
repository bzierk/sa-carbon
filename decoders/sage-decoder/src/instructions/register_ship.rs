use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x06337f5c394cb0a5")]
pub struct RegisterShip {
    pub input: RegisterShipInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RegisterShipInstructionAccounts {
    pub game_and_profile: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub ship: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RegisterShip {
    type ArrangedAccounts = RegisterShipInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            game_and_profile,
            funder,
            ship,
            mint,
            system_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(RegisterShipInstructionAccounts {
            game_and_profile: game_and_profile.pubkey,
            funder: funder.pubkey,
            ship: ship.pubkey,
            mint: mint.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
