use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x69c48bee0d467de2")]
pub struct RespawnToLoadingBay {
    pub input: RespawnToLoadingBayInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RespawnToLoadingBayInstructionAccounts {
    pub game_fleet_and_owner: solana_pubkey::Pubkey,
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub cargo_hold: solana_pubkey::Pubkey,
    pub fuel_tank: solana_pubkey::Pubkey,
    pub ammo_bank: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RespawnToLoadingBay {
    type ArrangedAccounts = RespawnToLoadingBayInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            game_fleet_and_owner,
            starbase_and_starbase_player,
            cargo_hold,
            fuel_tank,
            ammo_bank,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(RespawnToLoadingBayInstructionAccounts {
            game_fleet_and_owner: game_fleet_and_owner.pubkey,
            starbase_and_starbase_player: starbase_and_starbase_player.pubkey,
            cargo_hold: cargo_hold.pubkey,
            fuel_tank: fuel_tank.pubkey,
            ammo_bank: ammo_bank.pubkey,
        })
    }
}
