use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe9a07b4deb33d62d")]
pub struct RetrieveLoot {
    pub input: RetrieveLootInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RetrieveLootInstructionAccounts {
    pub game_and_fleet_and_owner: solana_pubkey::Pubkey,
    pub sector: solana_pubkey::Pubkey,
    pub loot: solana_pubkey::Pubkey,
    pub cargo_hold: solana_pubkey::Pubkey,
    pub cargo_stats_definition: solana_pubkey::Pubkey,
    pub cargo_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RetrieveLoot {
    type ArrangedAccounts = RetrieveLootInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            game_and_fleet_and_owner,
            sector,
            loot,
            cargo_hold,
            cargo_stats_definition,
            cargo_program,
            token_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(RetrieveLootInstructionAccounts {
            game_and_fleet_and_owner: game_and_fleet_and_owner.pubkey,
            sector: sector.pubkey,
            loot: loot.pubkey,
            cargo_hold: cargo_hold.pubkey,
            cargo_stats_definition: cargo_stats_definition.pubkey,
            cargo_program: cargo_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
