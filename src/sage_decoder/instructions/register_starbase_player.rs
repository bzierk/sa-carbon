

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x3c129e13d09353e2")]
pub struct RegisterStarbasePlayer{
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RegisterStarbasePlayerInstructionAccounts {
    pub funder: solana_pubkey::Pubkey,
    pub game_accounts: solana_pubkey::Pubkey,
    pub sage_player_profile: solana_pubkey::Pubkey,
    pub profile_faction: solana_pubkey::Pubkey,
    pub starbase: solana_pubkey::Pubkey,
    pub starbase_player: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RegisterStarbasePlayer {
    type ArrangedAccounts = RegisterStarbasePlayerInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            funder,
            game_accounts,
            sage_player_profile,
            profile_faction,
            starbase,
            starbase_player,
            system_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(RegisterStarbasePlayerInstructionAccounts {
            funder: funder.pubkey,
            game_accounts: game_accounts.pubkey,
            sage_player_profile: sage_player_profile.pubkey,
            profile_faction: profile_faction.pubkey,
            starbase: starbase.pubkey,
            starbase_player: starbase_player.pubkey,
            system_program: system_program.pubkey,
        })
    }
}