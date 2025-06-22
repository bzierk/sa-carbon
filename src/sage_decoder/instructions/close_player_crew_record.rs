
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x42e8136cb4c012e9")]
pub struct ClosePlayerCrewRecord{
    pub input: KeyIndexInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ClosePlayerCrewRecordInstructionAccounts {
    pub funds_to: solana_pubkey::Pubkey,
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub crew_record: solana_pubkey::Pubkey,
    pub game_and_profile_and_faction: solana_pubkey::Pubkey,
    pub crew_config: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClosePlayerCrewRecord {
    type ArrangedAccounts = ClosePlayerCrewRecordInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            funds_to,
            starbase_and_starbase_player,
            crew_record,
            game_and_profile_and_faction,
            crew_config,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(ClosePlayerCrewRecordInstructionAccounts {
            funds_to: funds_to.pubkey,
            starbase_and_starbase_player: starbase_and_starbase_player.pubkey,
            crew_record: crew_record.pubkey,
            game_and_profile_and_faction: game_and_profile_and_faction.pubkey,
            crew_config: crew_config.pubkey,
        })
    }
}