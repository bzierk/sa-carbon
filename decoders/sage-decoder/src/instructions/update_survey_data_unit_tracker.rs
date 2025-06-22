use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xce1bf719ddcfdb23")]
pub struct UpdateSurveyDataUnitTracker {
    pub input: UpdateSurveyDataUnitTrackerInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateSurveyDataUnitTrackerInstructionAccounts {
    pub game_and_profile: solana_pubkey::Pubkey,
    pub survey_data_unit_tracker: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateSurveyDataUnitTracker {
    type ArrangedAccounts = UpdateSurveyDataUnitTrackerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [game_and_profile, survey_data_unit_tracker, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateSurveyDataUnitTrackerInstructionAccounts {
            game_and_profile: game_and_profile.pubkey,
            survey_data_unit_tracker: survey_data_unit_tracker.pubkey,
        })
    }
}
