use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5466ea017e88ba93")]
pub struct ScanForSurveyDataUnits {
    pub input: ScanForSurveyDataUnitsInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ScanForSurveyDataUnitsInstructionAccounts {
    pub game_accounts_fleet_and_owner: solana_pubkey::Pubkey,
    pub survey_data_unit_tracker: solana_pubkey::Pubkey,
    pub survey_data_unit_tracker_signer: solana_pubkey::Pubkey,
    pub cargo_hold: solana_pubkey::Pubkey,
    pub sector: solana_pubkey::Pubkey,
    pub sdu_token_from: solana_pubkey::Pubkey,
    pub sdu_token_to: solana_pubkey::Pubkey,
    pub resource_token_from: solana_pubkey::Pubkey,
    pub resource_mint: solana_pubkey::Pubkey,
    pub sdu_cargo_type: solana_pubkey::Pubkey,
    pub resource_cargo_type: solana_pubkey::Pubkey,
    pub cargo_stats_definition: solana_pubkey::Pubkey,
    pub data_running_xp_accounts: solana_pubkey::Pubkey,
    pub council_rank_xp_accounts: solana_pubkey::Pubkey,
    pub progression_config: solana_pubkey::Pubkey,
    pub points_program: solana_pubkey::Pubkey,
    pub cargo_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub instructions_sysvar: solana_pubkey::Pubkey,
    pub recent_slothashes: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ScanForSurveyDataUnits {
    type ArrangedAccounts = ScanForSurveyDataUnitsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            game_accounts_fleet_and_owner,
            survey_data_unit_tracker,
            survey_data_unit_tracker_signer,
            cargo_hold,
            sector,
            sdu_token_from,
            sdu_token_to,
            resource_token_from,
            resource_mint,
            sdu_cargo_type,
            resource_cargo_type,
            cargo_stats_definition,
            data_running_xp_accounts,
            council_rank_xp_accounts,
            progression_config,
            points_program,
            cargo_program,
            token_program,
            instructions_sysvar,
            recent_slothashes,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(ScanForSurveyDataUnitsInstructionAccounts {
            game_accounts_fleet_and_owner: game_accounts_fleet_and_owner.pubkey,
            survey_data_unit_tracker: survey_data_unit_tracker.pubkey,
            survey_data_unit_tracker_signer: survey_data_unit_tracker_signer.pubkey,
            cargo_hold: cargo_hold.pubkey,
            sector: sector.pubkey,
            sdu_token_from: sdu_token_from.pubkey,
            sdu_token_to: sdu_token_to.pubkey,
            resource_token_from: resource_token_from.pubkey,
            resource_mint: resource_mint.pubkey,
            sdu_cargo_type: sdu_cargo_type.pubkey,
            resource_cargo_type: resource_cargo_type.pubkey,
            cargo_stats_definition: cargo_stats_definition.pubkey,
            data_running_xp_accounts: data_running_xp_accounts.pubkey,
            council_rank_xp_accounts: council_rank_xp_accounts.pubkey,
            progression_config: progression_config.pubkey,
            points_program: points_program.pubkey,
            cargo_program: cargo_program.pubkey,
            token_program: token_program.pubkey,
            instructions_sysvar: instructions_sysvar.pubkey,
            recent_slothashes: recent_slothashes.pubkey,
        })
    }
}
