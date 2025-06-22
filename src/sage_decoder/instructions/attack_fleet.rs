
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x59ad057b6d0decd9")]
pub struct AttackFleet{
    pub input: AttackFleetInput,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct AttackFleetInstructionAccounts {
    pub game_and_fleet_and_owner: solana_pubkey::Pubkey,
    pub defending_fleet: solana_pubkey::Pubkey,
    pub attacking_cargo_pod: solana_pubkey::Pubkey,
    pub defending_cargo_pod: solana_pubkey::Pubkey,
    pub cargo_type: solana_pubkey::Pubkey,
    pub cargo_stats_definition: solana_pubkey::Pubkey,
    pub attacker_combat_xp: solana_pubkey::Pubkey,
    pub attacker_council_rank_xp: solana_pubkey::Pubkey,
    pub defender_combat_xp: solana_pubkey::Pubkey,
    pub defender_council_rank_xp: solana_pubkey::Pubkey,
    pub combat_xp_category: solana_pubkey::Pubkey,
    pub council_rank_xp_category: solana_pubkey::Pubkey,
    pub combat_xp_modifier: solana_pubkey::Pubkey,
    pub council_rank_xp_modifier: solana_pubkey::Pubkey,
    pub progression_config: solana_pubkey::Pubkey,
    pub combat_config: solana_pubkey::Pubkey,
    pub attacking_fleet_ammo_token: solana_pubkey::Pubkey,
    pub defending_fleet_ammo_token: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AttackFleet {
    type ArrangedAccounts = AttackFleetInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            game_and_fleet_and_owner,
            defending_fleet,
            attacking_cargo_pod,
            defending_cargo_pod,
            cargo_type,
            cargo_stats_definition,
            attacker_combat_xp,
            attacker_council_rank_xp,
            defender_combat_xp,
            defender_council_rank_xp,
            combat_xp_category,
            council_rank_xp_category,
            combat_xp_modifier,
            council_rank_xp_modifier,
            progression_config,
            combat_config,
            attacking_fleet_ammo_token,
            defending_fleet_ammo_token,
            token_mint,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(AttackFleetInstructionAccounts {
            game_and_fleet_and_owner: game_and_fleet_and_owner.pubkey,
            defending_fleet: defending_fleet.pubkey,
            attacking_cargo_pod: attacking_cargo_pod.pubkey,
            defending_cargo_pod: defending_cargo_pod.pubkey,
            cargo_type: cargo_type.pubkey,
            cargo_stats_definition: cargo_stats_definition.pubkey,
            attacker_combat_xp: attacker_combat_xp.pubkey,
            attacker_council_rank_xp: attacker_council_rank_xp.pubkey,
            defender_combat_xp: defender_combat_xp.pubkey,
            defender_council_rank_xp: defender_council_rank_xp.pubkey,
            combat_xp_category: combat_xp_category.pubkey,
            council_rank_xp_category: council_rank_xp_category.pubkey,
            combat_xp_modifier: combat_xp_modifier.pubkey,
            council_rank_xp_modifier: council_rank_xp_modifier.pubkey,
            progression_config: progression_config.pubkey,
            combat_config: combat_config.pubkey,
            attacking_fleet_ammo_token: attacking_fleet_ammo_token.pubkey,
            defending_fleet_ammo_token: defending_fleet_ammo_token.pubkey,
            token_mint: token_mint.pubkey,
        })
    }
}