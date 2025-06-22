
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xf9aef2bb8bdb2f76")]
pub struct DeregisterProgressionConfig{
    pub input: KeyIndexInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DeregisterProgressionConfigInstructionAccounts {
    pub funds_to: solana_pubkey::Pubkey,
    pub progression_config: solana_pubkey::Pubkey,
    pub game_and_profile: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DeregisterProgressionConfig {
    type ArrangedAccounts = DeregisterProgressionConfigInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            funds_to,
            progression_config,
            game_and_profile,
            system_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(DeregisterProgressionConfigInstructionAccounts {
            funds_to: funds_to.pubkey,
            progression_config: progression_config.pubkey,
            game_and_profile: game_and_profile.pubkey,
            system_program: system_program.pubkey,
        })
    }
}