use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6b7add180cf946c1")]
pub struct ChangeRental {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ChangeRentalInstructionAccounts {
    pub sub_profile_invalidator: solana_pubkey::Pubkey,
    pub new_sub_profile: solana_pubkey::Pubkey,
    pub fleet: solana_pubkey::Pubkey,
    pub game_id: solana_pubkey::Pubkey,
    pub sub_profile_faction: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ChangeRental {
    type ArrangedAccounts = ChangeRentalInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            sub_profile_invalidator,
            new_sub_profile,
            fleet,
            game_id,
            sub_profile_faction,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(ChangeRentalInstructionAccounts {
            sub_profile_invalidator: sub_profile_invalidator.pubkey,
            new_sub_profile: new_sub_profile.pubkey,
            fleet: fleet.pubkey,
            game_id: game_id.pubkey,
            sub_profile_faction: sub_profile_faction.pubkey,
        })
    }
}
