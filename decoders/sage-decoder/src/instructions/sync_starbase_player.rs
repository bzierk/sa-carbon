use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x785ea4d8a73b03cc")]
pub struct SyncStarbasePlayer {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SyncStarbasePlayerInstructionAccounts {
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub game_accounts: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SyncStarbasePlayer {
    type ArrangedAccounts = SyncStarbasePlayerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [starbase_and_starbase_player, game_accounts, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SyncStarbasePlayerInstructionAccounts {
            starbase_and_starbase_player: starbase_and_starbase_player.pubkey,
            game_accounts: game_accounts.pubkey,
        })
    }
}
