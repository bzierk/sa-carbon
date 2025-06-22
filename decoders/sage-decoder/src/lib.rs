use solana_pubkey::Pubkey;

pub struct SageDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey =
    solana_pubkey::Pubkey::from_str_const("SAgeTraQfBMdvGVDJYoEvjnbq5szW7RJPi6obDTDQUF");
