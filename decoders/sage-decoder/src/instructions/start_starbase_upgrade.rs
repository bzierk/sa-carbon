use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa5e153a79ed38fcd")]
pub struct StartStarbaseUpgrade {
    pub input: KeyIndexInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct StartStarbaseUpgradeInstructionAccounts {
    pub funder: solana_pubkey::Pubkey,
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub upgrade_facility: solana_pubkey::Pubkey,
    pub upgrade_recipe: solana_pubkey::Pubkey,
    pub game_accounts_and_profile: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for StartStarbaseUpgrade {
    type ArrangedAccounts = StartStarbaseUpgradeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            funder,
            starbase_and_starbase_player,
            upgrade_facility,
            upgrade_recipe,
            game_accounts_and_profile,
            system_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(StartStarbaseUpgradeInstructionAccounts {
            funder: funder.pubkey,
            starbase_and_starbase_player: starbase_and_starbase_player.pubkey,
            upgrade_facility: upgrade_facility.pubkey,
            upgrade_recipe: upgrade_recipe.pubkey,
            game_accounts_and_profile: game_accounts_and_profile.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
