```rust
use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Mint, Burn};
use spl_token::state::Account as SplAccount;

// Define the Reward data structure
#[account]
pub struct Reward {
    pub owner: Pubkey,
    pub staked_tokens: u64,
    pub reward_tokens: u64,
    pub last_reward_claim: i64,
}

// Define the StakingPool data structure
#[account]
pub struct StakingPool {
    pub owner: Pubkey,
    pub staking_token_account: Pubkey,
    pub reward_token_account: Pubkey,
    pub total_staked_tokens: u64,
    pub total_reward_tokens: u64,
}

// Define the program state
#[account]
pub struct ProgramState {
    pub staking_pool: Pubkey,
}

// Define the error types
#[error]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
}

// Define the instructions
#[program]
pub mod rewards_for_holding_and_staking_tokens {
    use super::*;

    // Initialize the staking pool
    pub fn init_staking_pool(ctx: Context<InitStakingPool>, staking_token_account: Pubkey, reward_token_account: Pubkey) -> ProgramResult {
        let staking_pool = &mut ctx.accounts.staking_pool;
        staking_pool.owner = *ctx.accounts.owner.key;
        staking_pool.staking_token_account = staking_token_account;
        staking_pool.reward_token_account = reward_token_account;
        staking_pool.total_staked_tokens = 0;
        staking_pool.total_reward_tokens = 0;
        Ok(())
    }

    // Stake tokens
    pub fn stake_tokens(ctx: Context<StakeTokens>, amount: u64) -> ProgramResult {
        let reward = &mut ctx.accounts.reward;
        let staking_pool = &mut ctx.accounts.staking_pool;
        if reward.owner != *ctx.accounts.owner.key {
            return Err(ErrorCode::Unauthorized.into());
        }
        reward.staked_tokens += amount;
        staking_pool.total_staked_tokens += amount;
        Ok(())
    }

    // Claim rewards
    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> ProgramResult {
        let reward = &mut ctx.accounts.reward;
        let staking_pool = &mut ctx.accounts.staking_pool;
        if reward.owner != *ctx.accounts.owner.key {
            return Err(ErrorCode::Unauthorized.into());
        }
        reward.reward_tokens += calculate_reward(reward.last_reward_claim, reward.staked_tokens);
        staking_pool.total_reward_tokens -= reward.reward_tokens;
        reward.last_reward_claim = Clock::get()?.unix_timestamp;
        Ok(())
    }
}

// Define the contexts for the instructions
#[derive(Accounts)]
pub struct InitStakingPool<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 32 + 8 + 8)]
    pub staking_pool: ProgramAccount<'info, StakingPool>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct StakeTokens<'info> {
    #[account(mut)]
    pub reward: ProgramAccount<'info, Reward>,
    #[account(mut)]
    pub staking_pool: ProgramAccount<'info, StakingPool>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(mut)]
    pub reward: ProgramAccount<'info, Reward>,
    #[account(mut)]
    pub staking_pool: ProgramAccount<'info, StakingPool>,
    pub owner: Signer<'info>,
}

// Define helper functions
fn calculate_reward(last_reward_claim: i64, staked_tokens: u64) -> u64 {
    let current_time = Clock::get().unwrap().unix_timestamp;
    let time_difference = current_time - last_reward_claim;
    (time_difference as u64) * staked_tokens
}
```