use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd571aa7bbb5a1c73")]
pub struct AddRental {
    pub owner_key_index: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct AddRentalInstructionAccounts {
    pub owner_profile: solana_pubkey::Pubkey,
    pub owner_key: solana_pubkey::Pubkey,
    pub invalidator: solana_pubkey::Pubkey,
    pub fleet: solana_pubkey::Pubkey,
    pub game_id: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AddRental {
    type ArrangedAccounts = AddRentalInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            owner_profile,
            owner_key,
            invalidator,
            fleet,
            game_id,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(AddRentalInstructionAccounts {
            owner_profile: owner_profile.pubkey,
            owner_key: owner_key.pubkey,
            invalidator: invalidator.pubkey,
            fleet: fleet.pubkey,
            game_id: game_id.pubkey,
        })
    }
}
