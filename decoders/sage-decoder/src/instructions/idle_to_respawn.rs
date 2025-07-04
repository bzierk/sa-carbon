use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xebcfc8aea1745ba8")]
pub struct IdleToRespawn {
    pub input: IdleToRespawnInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct IdleToRespawnInstructionAccounts {
    pub game_accounts_fleet_and_owner: solana_pubkey::Pubkey,
    pub atlas_token_from: solana_pubkey::Pubkey,
    pub atlas_token_to: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for IdleToRespawn {
    type ArrangedAccounts = IdleToRespawnInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            game_accounts_fleet_and_owner,
            atlas_token_from,
            atlas_token_to,
            token_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(IdleToRespawnInstructionAccounts {
            game_accounts_fleet_and_owner: game_accounts_fleet_and_owner.pubkey,
            atlas_token_from: atlas_token_from.pubkey,
            atlas_token_to: atlas_token_to.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
