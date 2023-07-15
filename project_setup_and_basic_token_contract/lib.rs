```rust
use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Transfer};

// Define the program's instruction handlers.
#[program]
pub mod basic_token_contract {
    use super::*;

    // Initialize the token contract.
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }

    // Mint tokens.
    pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> ProgramResult {
        let cpi_program = ctx.accounts.token_program.to_account_info().clone();
        let cpi_accounts = Transfer {
            from: ctx.accounts.source.to_account_info().clone(),
            to: ctx.accounts.destination.to_account_info().clone(),
            authority: ctx.accounts.owner.clone(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)
    }

    // Transfer tokens.
    pub fn transfer_token(ctx: Context<TransferToken>, amount: u64) -> ProgramResult {
        let cpi_program = ctx.accounts.token_program.to_account_info().clone();
        let cpi_accounts = Transfer {
            from: ctx.accounts.source.to_account_info().clone(),
            to: ctx.accounts.destination.to_account_info().clone(),
            authority: ctx.accounts.owner.clone(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)
    }

    // Burn tokens.
    pub fn burn_token(ctx: Context<BurnToken>, amount: u64) -> ProgramResult {
        let cpi_program = ctx.accounts.token_program.to_account_info().clone();
        let cpi_accounts = Transfer {
            from: ctx.accounts.source.to_account_info().clone(),
            to: ctx.accounts.destination.to_account_info().clone(),
            authority: ctx.accounts.owner.clone(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::burn(cpi_ctx, amount)
    }
}

// Define the accounts used in each instruction.
#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(mut)]
    pub source: AccountInfo<'info>,
    #[account(mut)]
    pub destination: AccountInfo<'info>,
    pub owner: AccountInfo<'info>,
    #[account("token_program.key == &token::ID")]
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct TransferToken<'info> {
    #[account(mut)]
    pub source: AccountInfo<'info>,
    #[account(mut)]
    pub destination: AccountInfo<'info>,
    pub owner: AccountInfo<'info>,
    #[account("token_program.key == &token::ID")]
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct BurnToken<'info> {
    #[account(mut)]
    pub source: AccountInfo<'info>,
    #[account(mut)]
    pub destination: AccountInfo<'info>,
    pub owner: AccountInfo<'info>,
    #[account("token_program.key == &token::ID")]
    pub token_program: AccountInfo<'info>,
}
```