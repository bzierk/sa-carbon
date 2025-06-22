use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xff213d788877b8eb")]
pub struct DeregisterSurveyDataUnitTracker {
    pub input: DeregisterSurveyDataUnitTrackerInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DeregisterSurveyDataUnitTrackerInstructionAccounts {
    pub game_and_profile: solana_pubkey::Pubkey,
    pub funds_to: solana_pubkey::Pubkey,
    pub survey_data_unit_tracker: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DeregisterSurveyDataUnitTracker {
    type ArrangedAccounts = DeregisterSurveyDataUnitTrackerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            game_and_profile,
            funds_to,
            survey_data_unit_tracker,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(DeregisterSurveyDataUnitTrackerInstructionAccounts {
            game_and_profile: game_and_profile.pubkey,
            funds_to: funds_to.pubkey,
            survey_data_unit_tracker: survey_data_unit_tracker.pubkey,
        })
    }
}
