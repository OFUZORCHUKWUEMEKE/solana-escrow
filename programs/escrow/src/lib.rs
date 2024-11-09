use anchor_lang::prelude::*;
use instructions::*;

declare_id!("8abobQGPEbz2RfwhhoDbfCHZQXtvTMQ4JbrKfUmyjq57");

#[program]
pub mod escrow {
    use super::*;

    pub fn deposit_sol(ctx:Context<Deposit>,escrow_amt:u64,unlock_price:f64)->Result<()>{
       deposit_handler(ctx,escrow_amt,unlock_price)
    }

    pub fn withdraw_sol(ctx: Context<Withdraw>) -> Result<()> {
        withdraw_handler(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
