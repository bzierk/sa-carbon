use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc9d056ca06b55c53")]
pub struct ReloadFleetAbilityPower {
    pub input: ReloadInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ReloadFleetAbilityPowerInstructionAccounts {
    pub game_and_fleet_and_owner: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ReloadFleetAbilityPower {
    type ArrangedAccounts = ReloadFleetAbilityPowerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [game_and_fleet_and_owner, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ReloadFleetAbilityPowerInstructionAccounts {
            game_and_fleet_and_owner: game_and_fleet_and_owner.pubkey,
        })
    }
}
