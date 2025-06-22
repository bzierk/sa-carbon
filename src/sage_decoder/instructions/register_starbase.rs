
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x6921248aa5b53339")]
pub struct RegisterStarbase{
    pub input: RegisterStarbaseInputUnpacked,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RegisterStarbaseInstructionAccounts {
    pub funder: solana_pubkey::Pubkey,
    pub starbase: solana_pubkey::Pubkey,
    pub sector: solana_pubkey::Pubkey,
    pub game_state_and_profile: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RegisterStarbase {
    type ArrangedAccounts = RegisterStarbaseInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            funder,
            starbase,
            sector,
            game_state_and_profile,
            system_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(RegisterStarbaseInstructionAccounts {
            funder: funder.pubkey,
            starbase: starbase.pubkey,
            sector: sector.pubkey,
            game_state_and_profile: game_state_and_profile.pubkey,
            system_program: system_program.pubkey,
        })
    }
}