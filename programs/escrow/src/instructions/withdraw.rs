use crate::state::*;
use crate::errors::*;
use std::str::FromStr;
use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::{ get_feed_id_from_hex,
    PriceUpdateV2,};
use anchor_lang::solana_program::clock::Clock;

#[derive(Accounts)]
pub struct Withdraw<'info>{

    #[account(mut)]
    pub user:Signer<'info>,
    // escrow_account
    #[account(
        mut,
        seeds=[b"MICHEAL BURRY", user.key().as_ref()],
        bump,
        close=user
    )]
    pub escrow_account:Account<'info,EscrowState>,
    pub price_update: Account<'info, PriceUpdateV2>,
    pub system_program: Program<'info, System>,
}

pub fn withdraw_handler(ctx:Context<Withdraw>,escrow_amount:u64)->Result<()>{
    // Get accounts
    let escrow_state = &ctx.accounts.escrow_account;
    let price_update = &ctx.accounts.price_update;
     // get_price_no_older_than will fail if the price update is more than 30 seconds old
    let maximum_age: u64 = 30;

    // get_price_no_older_than will fail if the price update is for a different price feed.
    // This string is the id of the SOL/USD feed. See https://pyth.network/developers/price-feed-ids for all available IDs.

    let feed_id: [u8; 32] = get_feed_id_from_hex(SOL_USDC_FEED)?;
    let price = price_update.get_price_no_older_than(&Clock::get()?, maximum_age, &feed_id)?;

    let current_price = price.price
    .checked_mul(10_i64.pow(price.exponent.unsigned_abs()))
    .ok_or(EscrowErrorCode::PriceOverFlow)?;

    if current_price  < (escrow_state.unlock_price as u64).try_into().unwrap(){
       return Err(EscrowErrorCode::SolPriceAboveUnlockPrice.into())
    }
    let cpi_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        anchor_lang::system_program::Transfer{        
            from:ctx.accounts.escrow_account.to_account_info(),
            to:ctx.accounts.user.to_account_info(),
        },
    );
    anchor_lang::system_program::transfer(cpi_ctx,escrow_amount)?;

    Ok(())
}