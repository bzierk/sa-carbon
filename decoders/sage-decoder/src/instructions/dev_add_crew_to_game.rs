use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x50f4294448a91acf")]
pub struct DevAddCrewToGame {
    pub input: DevAddCrewInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DevAddCrewToGameInstructionAccounts {
    pub game_and_profile: solana_pubkey::Pubkey,
    pub starbase_player: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DevAddCrewToGame {
    type ArrangedAccounts = DevAddCrewToGameInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [game_and_profile, starbase_player, _remaining @ ..] = accounts else {
            return None;
        };

        Some(DevAddCrewToGameInstructionAccounts {
            game_and_profile: game_and_profile.pubkey,
            starbase_player: starbase_player.pubkey,
        })
    }
}
