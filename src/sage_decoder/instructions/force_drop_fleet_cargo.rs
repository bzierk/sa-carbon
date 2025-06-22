

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xaa78bf2effc350dd")]
pub struct ForceDropFleetCargo{
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ForceDropFleetCargoInstructionAccounts {
    pub fleet: solana_pubkey::Pubkey,
    pub fleet_ships: solana_pubkey::Pubkey,
    pub cargo_pod: solana_pubkey::Pubkey,
    pub cargo_type: solana_pubkey::Pubkey,
    pub cargo_stats_definition: solana_pubkey::Pubkey,
    pub game_id: solana_pubkey::Pubkey,
    pub token_from: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub cargo_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ForceDropFleetCargo {
    type ArrangedAccounts = ForceDropFleetCargoInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            fleet,
            fleet_ships,
            cargo_pod,
            cargo_type,
            cargo_stats_definition,
            game_id,
            token_from,
            token_mint,
            cargo_program,
            token_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(ForceDropFleetCargoInstructionAccounts {
            fleet: fleet.pubkey,
            fleet_ships: fleet_ships.pubkey,
            cargo_pod: cargo_pod.pubkey,
            cargo_type: cargo_type.pubkey,
            cargo_stats_definition: cargo_stats_definition.pubkey,
            game_id: game_id.pubkey,
            token_from: token_from.pubkey,
            token_mint: token_mint.pubkey,
            cargo_program: cargo_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}