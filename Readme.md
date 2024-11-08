# Solana Escrow with Price Trigger

This is a Solana program that implements an escrow system with a price trigger. Users can deposit SOL into the escrow, and the funds will be locked until the following conditions are met:

1. The current SOL/USD price reaches a target price specified by the user when the escrow was created.
2. The duration (in seconds) specified by the user when the escrow was created has elapsed.

Once these conditions are met, the user who created the escrow can withdraw the deposited SOL.

## Features

- Create an escrow account with a target SOL/USD price and a duration
- Deposit SOL into the escrow account
- Withdraw the deposited SOL once the target price is reached and the duration has elapsed

## Installation and Usage

1. Deploy the Solana program to your desired Solana cluster.
2. Use the provided Anchor instructions to interact with the program:
   - `deposit_sol`: Create Escrow and Deposit SOL into the escrow account.
   - `withdraw_sol`: Withdraw the deposited SOL from an escrow account if the conditions are met.

Here's an example of how to use the `create_escrow` instruction:

```rust
let ctx = CpiContext::new(
    program_id,
    CreateEscrow {
        escrow_account: escrow_account.into(),
        owner: owner_account.into(),
        payer: payer_account.into(),
        system_program: system_program.into(),
    },
);

deposit(ctx, 50.0, 86400)?; // Create an escrow with a $50 target price and 1 day duration