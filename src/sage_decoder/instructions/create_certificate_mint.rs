

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xadbe2d74226d2259")]
pub struct CreateCertificateMint{
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateCertificateMintInstructionAccounts {
    pub funder: solana_pubkey::Pubkey,
    pub starbase: solana_pubkey::Pubkey,
    pub cargo_mint: solana_pubkey::Pubkey,
    pub certificate_mint: solana_pubkey::Pubkey,
    pub cargo_type: solana_pubkey::Pubkey,
    pub game_id: solana_pubkey::Pubkey,
    pub transfer_hook_extra_account_meta_list: solana_pubkey::Pubkey,
    pub transfer_hook_program: solana_pubkey::Pubkey,
    pub token2022_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateCertificateMint {
    type ArrangedAccounts = CreateCertificateMintInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            funder,
            starbase,
            cargo_mint,
            certificate_mint,
            cargo_type,
            game_id,
            transfer_hook_extra_account_meta_list,
            transfer_hook_program,
            token2022_program,
            system_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(CreateCertificateMintInstructionAccounts {
            funder: funder.pubkey,
            starbase: starbase.pubkey,
            cargo_mint: cargo_mint.pubkey,
            certificate_mint: certificate_mint.pubkey,
            cargo_type: cargo_type.pubkey,
            game_id: game_id.pubkey,
            transfer_hook_extra_account_meta_list: transfer_hook_extra_account_meta_list.pubkey,
            transfer_hook_program: transfer_hook_program.pubkey,
            token2022_program: token2022_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}