use carbon_core::borsh::{self, BorshDeserialize};

#[derive(Debug, serde::Serialize, serde::Deserialize)]

// #[carbon(discriminator = "0xc0ea905648130563")]
pub struct StarbasePlayer {
    pub version: u8,
    pub player_profile: solana_pubkey::Pubkey,
    pub game_id: solana_pubkey::Pubkey,
    pub starbase: solana_pubkey::Pubkey,
    pub sage_player_profile: solana_pubkey::Pubkey,
    pub bump: u8,
    pub ship_escrow_count: u32,
    pub old_total_crew: u32,
    pub new_total_crew: u32,
    pub busy_crew: u64,
    pub update_id: u64,
    pub updated_ship_escrow_count: u32,
    pub ship_escrows: Vec<WrappedShipEscrow>,
}

#[derive(BorshDeserialize, Debug, serde::Serialize, serde::Deserialize)]
pub struct WrappedShipEscrow {
    pub ship: solana_pubkey::Pubkey,
    pub amount: u64,
    pub update_id: u64,
}

impl borsh::de::BorshDeserialize for StarbasePlayer
where
    u8: borsh::BorshDeserialize,
    solana_pubkey::Pubkey: borsh::BorshDeserialize,
    solana_pubkey::Pubkey: borsh::BorshDeserialize,
    solana_pubkey::Pubkey: borsh::BorshDeserialize,
    solana_pubkey::Pubkey: borsh::BorshDeserialize,
    u8: borsh::BorshDeserialize,
    u32: borsh::BorshDeserialize,
    u32: borsh::BorshDeserialize,
    u32: borsh::BorshDeserialize,
    u64: borsh::BorshDeserialize,
    u64: borsh::BorshDeserialize,
    u32: borsh::BorshDeserialize,
{
    fn deserialize_reader<R: borsh::maybestd::io::Read>(
        reader: &mut R,
    ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            version: borsh::BorshDeserialize::deserialize_reader(reader)?,
            player_profile: borsh::BorshDeserialize::deserialize_reader(reader)?,
            game_id: borsh::BorshDeserialize::deserialize_reader(reader)?,
            starbase: borsh::BorshDeserialize::deserialize_reader(reader)?,
            sage_player_profile: borsh::BorshDeserialize::deserialize_reader(reader)?,
            bump: borsh::BorshDeserialize::deserialize_reader(reader)?,
            ship_escrow_count: borsh::BorshDeserialize::deserialize_reader(reader)?,
            old_total_crew: borsh::BorshDeserialize::deserialize_reader(reader)?,
            new_total_crew: borsh::BorshDeserialize::deserialize_reader(reader)?,
            busy_crew: borsh::BorshDeserialize::deserialize_reader(reader)?,
            update_id: borsh::BorshDeserialize::deserialize_reader(reader)?,
            updated_ship_escrow_count: borsh::BorshDeserialize::deserialize_reader(reader)?,
            ship_escrows: Vec::new(),
        })
    }
}
#[automatically_derived]
impl carbon_core::deserialize::CarbonDeserialize for StarbasePlayer {
    fn deserialize(data: &[u8]) -> Option<Self> {
        let discriminator: &[u8] = &[192u8, 234u8, 144u8, 86u8, 72u8, 19u8, 5u8, 99u8];
        if data.len() < discriminator.len() {
            return None;
        }
        let (disc, mut rest) = data.split_at(discriminator.len());
        if disc != discriminator {
            return None;
        }
        let list_length = &rest[130..134];
        let list_length = u32::from_le_bytes(list_length.try_into().unwrap());
        println!("List length: {:?}", list_length);
        let starbase_player: StarbasePlayer =
            match carbon_core::borsh::BorshDeserialize::deserialize(&mut rest) {
                Ok(res) => res,
                Err(_) => return None,
            };

        let mut ship_escrows = Vec::new();
        for _ in 0..list_length {
            match WrappedShipEscrow::deserialize(&mut rest) {
                Ok(ship_escrow) => ship_escrows.push(ship_escrow),
                Err(_) => return None,
            }
        }

        let mut final_starbase_player = starbase_player;
        final_starbase_player.ship_escrows = ship_escrows;

        if !rest.is_empty() {
            carbon_core::log::warn!(
                "Not all bytes were read when deserializing {}: {} bytes remaining",
                stringify!(StarbasePlayer),
                rest.len(),
            );
        }

        Some(final_starbase_player)
    }
}
