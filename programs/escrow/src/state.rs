use anchor_lang::prelude::*;

pub const ESCROW_SEED:[&u8] = b"MICHEAL BURRY";
pub const SOL_USDC_FEED:&str ="0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d"

#[account]
#[derive(InitSpace)]
pub struct EscrowState{
    pub unlock_price:f64,
    pub escrow_amount:u64
}