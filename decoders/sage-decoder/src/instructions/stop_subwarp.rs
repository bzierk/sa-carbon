use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x963a030006cf772e")]
pub struct StopSubwarp {
    pub input: StopSubwarpInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct StopSubwarpInstructionAccounts {
    pub game_accounts_fleet_and_owner: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for StopSubwarp {
    type ArrangedAccounts = StopSubwarpInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [game_accounts_fleet_and_owner, _remaining @ ..] = accounts else {
            return None;
        };

        Some(StopSubwarpInstructionAccounts {
            game_accounts_fleet_and_owner: game_accounts_fleet_and_owner.pubkey,
        })
    }
}
