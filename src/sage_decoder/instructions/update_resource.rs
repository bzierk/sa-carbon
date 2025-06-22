
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xf0d09c56e6d80164")]
pub struct UpdateResource{
    pub input: UpdateResourceInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateResourceInstructionAccounts {
    pub game_and_profile: solana_pubkey::Pubkey,
    pub mine_item: solana_pubkey::Pubkey,
    pub resource: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateResource {
    type ArrangedAccounts = UpdateResourceInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            game_and_profile,
            mine_item,
            resource,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(UpdateResourceInstructionAccounts {
            game_and_profile: game_and_profile.pubkey,
            mine_item: mine_item.pubkey,
            resource: resource.pubkey,
        })
    }
}