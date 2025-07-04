use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6de7e779cb2c1628")]
pub struct DiscoverSector {
    pub input: DiscoverSectorInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DiscoverSectorInstructionAccounts {
    pub game_accounts_fleet_and_owner: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub sector: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DiscoverSector {
    type ArrangedAccounts = DiscoverSectorInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            game_accounts_fleet_and_owner,
            funder,
            sector,
            system_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(DiscoverSectorInstructionAccounts {
            game_accounts_fleet_and_owner: game_accounts_fleet_and_owner.pubkey,
            funder: funder.pubkey,
            sector: sector.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
