#![cfg_attr(not(feature = "std"), no_std)]

pub const TOKEN_SYMBOL: &str = "SGS";
pub const SS58FORMAT: u16 = 4300;
pub const CHAIN_NAME: &str = "SG Solochain";
pub const CHAIN_ID: &str = "sg-solochain";
pub const TOKEN_DECIMALS: u8 = 12;
pub const UNIT: u128 = 10u128.pow(TOKEN_DECIMALS as u32);
pub const EXISTENTIAL_DEPOSIT: u128 = UNIT / 1_000_000;
/// This determines the average expected block time that we are targeting. Blocks will be
/// produced at a minimum duration defined by `SLOT_DURATION`. `SLOT_DURATION` is picked up by
/// `pallet_timestamp` which is in turn picked up by `pallet_aura` to implement `fn
/// slot_duration()`.
///
/// Change this to adjust the block time.
// NOTE: Currently it is not possible to change the slot duration after the chain has started.
// Attempting to do so will brick block production.
pub const MILLI_SECS_PER_BLOCK: u64 = 6000;
