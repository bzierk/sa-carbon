

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xc891776755be788a")]
pub struct RemoveConnection{
    pub sector1_index: u16,
    pub sector2_index: u16,
    pub key_index: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RemoveConnectionInstructionAccounts {
    pub game_and_profile: solana_pubkey::Pubkey,
    pub funds_to: solana_pubkey::Pubkey,
    pub sector1: solana_pubkey::Pubkey,
    pub sector2: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemoveConnection {
    type ArrangedAccounts = RemoveConnectionInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            game_and_profile,
            funds_to,
            sector1,
            sector2,
            system_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(RemoveConnectionInstructionAccounts {
            game_and_profile: game_and_profile.pubkey,
            funds_to: funds_to.pubkey,
            sector1: sector1.pubkey,
            sector2: sector2.pubkey,
            system_program: system_program.pubkey,
        })
    }
}