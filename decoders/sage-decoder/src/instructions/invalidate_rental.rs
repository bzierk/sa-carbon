use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb4b76a99ff08e8b2")]
pub struct InvalidateRental {
    pub remove_invalidator: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InvalidateRentalInstructionAccounts {
    pub sub_profile_invalidator: solana_pubkey::Pubkey,
    pub fleet: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InvalidateRental {
    type ArrangedAccounts = InvalidateRentalInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [sub_profile_invalidator, fleet, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InvalidateRentalInstructionAccounts {
            sub_profile_invalidator: sub_profile_invalidator.pubkey,
            fleet: fleet.pubkey,
        })
    }
}
