use crate::state::*;
use anchor_lang::prelude::*;


pub fn deposit_handler(ctx:Context<Deposit>,escrow_amount:u64,unlock_price:u64)->Result<()>{
    msg!("Desposited funds in escrow....");

    let escrow_state = &mut ctx.accounts.escrow_account;
    escrow_state.unlock_price = unlock_price;
    escrow_state.escrow_amount = escrow_amount;

    let cpi_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer{
            from:ctx.accounts.user.to_account_info(),
            to:ctx.accounts.excrow_account.to_account_info()
        },
    );
    system_program::transfer(cpi_ctx,escrow_amount)?;
  
    msg!("Transfer complete. Escrow will unlock SOL at {}", &ctx.accounts.escrow_account.unlock_price);
    Ok(());
}

#[derive(Accounts)]
pub struct Deposit<'info>{
    // user account
    #[account(mut)]
    pub user:Signer<'info>,
    // account to store SOL in escrow
    #[account(
        init,
        seeds=[ESCROW_SEED,user.key().as_ref()],
        bump,
        payer=user,
        space=EscrowState::INIT_SPACE
    )]
    pub escrow_account:Account<'info,EscrowState>,

    pub system_program:Program<'info,System>
}
