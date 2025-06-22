
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xdd907d9e1c19bad1")]
pub struct RegisterSageCrewConfig{
    pub input: KeyIndexInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RegisterSageCrewConfigInstructionAccounts {
    pub funder: solana_pubkey::Pubkey,
    pub sage_crew_config: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub game_and_profile: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RegisterSageCrewConfig {
    type ArrangedAccounts = RegisterSageCrewConfigInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            funder,
            sage_crew_config,
            config,
            game_and_profile,
            system_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(RegisterSageCrewConfigInstructionAccounts {
            funder: funder.pubkey,
            sage_crew_config: sage_crew_config.pubkey,
            config: config.pubkey,
            game_and_profile: game_and_profile.pubkey,
            system_program: system_program.pubkey,
        })
    }
}