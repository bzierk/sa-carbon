use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb8ef76ed5ca9b1ae")]
pub struct SetNextShip {
    pub key_index: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetNextShipInstructionAccounts {
    pub game_and_profile: solana_pubkey::Pubkey,
    pub ship: solana_pubkey::Pubkey,
    pub next_ship: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetNextShip {
    type ArrangedAccounts = SetNextShipInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [game_and_profile, ship, next_ship, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetNextShipInstructionAccounts {
            game_and_profile: game_and_profile.pubkey,
            ship: ship.pubkey,
            next_ship: next_ship.pubkey,
        })
    }
}
