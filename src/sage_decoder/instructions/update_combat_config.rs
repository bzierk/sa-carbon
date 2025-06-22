
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x7ab24cd4b950aeeb")]
pub struct UpdateCombatConfig{
    pub input: UpdateCombatConfigInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateCombatConfigInstructionAccounts {
    pub combat_config: solana_pubkey::Pubkey,
    pub game_and_profile: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateCombatConfig {
    type ArrangedAccounts = UpdateCombatConfigInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            combat_config,
            game_and_profile,
            system_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(UpdateCombatConfigInstructionAccounts {
            combat_config: combat_config.pubkey,
            game_and_profile: game_and_profile.pubkey,
            system_program: system_program.pubkey,
        })
    }
}