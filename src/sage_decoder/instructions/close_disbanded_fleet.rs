
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xd696959cf57b25a5")]
pub struct CloseDisbandedFleet{
    pub input: CloseDisbandedFleetInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseDisbandedFleetInstructionAccounts {
    pub key: solana_pubkey::Pubkey,
    pub player_profile: solana_pubkey::Pubkey,
    pub funds_to: solana_pubkey::Pubkey,
    pub disbanded_fleet: solana_pubkey::Pubkey,
    pub fleet_ships: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseDisbandedFleet {
    type ArrangedAccounts = CloseDisbandedFleetInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            key,
            player_profile,
            funds_to,
            disbanded_fleet,
            fleet_ships,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(CloseDisbandedFleetInstructionAccounts {
            key: key.pubkey,
            player_profile: player_profile.pubkey,
            funds_to: funds_to.pubkey,
            disbanded_fleet: disbanded_fleet.pubkey,
            fleet_ships: fleet_ships.pubkey,
        })
    }
}