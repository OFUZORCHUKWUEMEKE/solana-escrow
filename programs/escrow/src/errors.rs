use anchor_lang::prelude::*;

#[error_code]
#[derive(Eq,PartialEq)]
pub enum EscrowErrorCode{
    #[msg("Not a valid switchboard account")]
    InvalidSwitchboardAccount,
    #[msg("Switchboard feed has not been updated in 5 minutes")]
    StaleFeed,
    #[msg("Switchboard feed exceeded provided confidence interval")]
    ConfidenceIntervalExceeded,
    #[msg("current SOL price is not above Escrow unlock price.")]
    SolPriceAboveUnlockPrice,
    #[msg("Price Overflow")]
    PriceOverFlow
}