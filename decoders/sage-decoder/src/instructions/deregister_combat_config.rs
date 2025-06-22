use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc979b1819383919a")]
pub struct DeregisterCombatConfig {
    pub input: KeyIndexInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DeregisterCombatConfigInstructionAccounts {
    pub funds_to: solana_pubkey::Pubkey,
    pub combat_config: solana_pubkey::Pubkey,
    pub game_and_profile: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DeregisterCombatConfig {
    type ArrangedAccounts = DeregisterCombatConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            funds_to,
            combat_config,
            game_and_profile,
            system_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(DeregisterCombatConfigInstructionAccounts {
            funds_to: funds_to.pubkey,
            combat_config: combat_config.pubkey,
            game_and_profile: game_and_profile.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
