

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xfb2e0cd0b8949d49")]
pub struct InitGame{
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitGameInstructionAccounts {
    pub signer: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub game_id: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitGame {
    type ArrangedAccounts = InitGameInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            signer,
            profile,
            funder,
            game_id,
            system_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(InitGameInstructionAccounts {
            signer: signer.pubkey,
            profile: profile.pubkey,
            funder: funder.pubkey,
            game_id: game_id.pubkey,
            system_program: system_program.pubkey,
        })
    }
}