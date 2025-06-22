use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8f3e85508553a711")]
pub struct CreateCargoPod {
    pub input: StarbaseCreateCargoPodInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateCargoPodInstructionAccounts {
    pub funder: solana_pubkey::Pubkey,
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub cargo_pod: solana_pubkey::Pubkey,
    pub cargo_stats_definition: solana_pubkey::Pubkey,
    pub game_accounts_and_profile: solana_pubkey::Pubkey,
    pub cargo_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateCargoPod {
    type ArrangedAccounts = CreateCargoPodInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            funder,
            starbase_and_starbase_player,
            cargo_pod,
            cargo_stats_definition,
            game_accounts_and_profile,
            cargo_program,
            system_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(CreateCargoPodInstructionAccounts {
            funder: funder.pubkey,
            starbase_and_starbase_player: starbase_and_starbase_player.pubkey,
            cargo_pod: cargo_pod.pubkey,
            cargo_stats_definition: cargo_stats_definition.pubkey,
            game_accounts_and_profile: game_accounts_and_profile.pubkey,
            cargo_program: cargo_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
