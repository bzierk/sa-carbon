use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc068c5281e279b30")]
pub struct StartSubwarp {
    pub input: StartSubwarpInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct StartSubwarpInstructionAccounts {
    pub game_accounts_fleet_and_owner: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for StartSubwarp {
    type ArrangedAccounts = StartSubwarpInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [game_accounts_fleet_and_owner, _remaining @ ..] = accounts else {
            return None;
        };

        Some(StartSubwarpInstructionAccounts {
            game_accounts_fleet_and_owner: game_accounts_fleet_and_owner.pubkey,
        })
    }
}
