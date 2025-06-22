

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x40de5ef395413684")]
pub struct MintCrewToGame{
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct MintCrewToGameInstructionAccounts {
    pub sage_player_profile: solana_pubkey::Pubkey,
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub sage_crew_config: solana_pubkey::Pubkey,
    pub crew_config: solana_pubkey::Pubkey,
    pub instructions_sysvar: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MintCrewToGame {
    type ArrangedAccounts = MintCrewToGameInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            sage_player_profile,
            starbase_and_starbase_player,
            sage_crew_config,
            crew_config,
            instructions_sysvar,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(MintCrewToGameInstructionAccounts {
            sage_player_profile: sage_player_profile.pubkey,
            starbase_and_starbase_player: starbase_and_starbase_player.pubkey,
            sage_crew_config: sage_crew_config.pubkey,
            crew_config: crew_config.pubkey,
            instructions_sysvar: instructions_sysvar.pubkey,
        })
    }
}