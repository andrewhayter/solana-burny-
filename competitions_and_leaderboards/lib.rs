```rust
use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Transfer};

// Define the Competition data structure
#[derive(Accounts)]
pub struct Competition<'info> {
    #[account(mut)]
    pub competition_account: Account<'info, CompetitionAccount>,
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
    pub system_program: Program<'info, System>,
}

// Define the CompetitionAccount data structure
#[account]
pub struct CompetitionAccount {
    pub leaderboard: Vec<LeaderboardEntry>,
}

// Define the UserAccount data structure
#[account]
pub struct UserAccount {
    pub user_id: u64,
    pub score: u64,
}

// Define the LeaderboardEntry data structure
#[derive(Clone)]
pub struct LeaderboardEntry {
    pub user_id: u64,
    pub score: u64,
}

// Define the StartCompetition instruction
#[derive(Instruction)]
pub struct StartCompetition {
    pub leaderboard_size: u64,
}

// Define the UpdateScore instruction
#[derive(Instruction)]
pub struct UpdateScore {
    pub user_id: u64,
    pub score: u64,
}

// Define the DisplayLeaderboard instruction
#[derive(Instruction)]
pub struct DisplayLeaderboard {}

impl<'info> Competition<'info> {
    // Define the start_competition function
    pub fn start_competition(ctx: Context<Competition>, leaderboard_size: u64) -> ProgramResult {
        let leaderboard = vec![LeaderboardEntry { user_id: 0, score: 0 }; leaderboard_size as usize];
        ctx.accounts.competition_account.leaderboard = leaderboard;
        Ok(())
    }

    // Define the update_score function
    pub fn update_score(ctx: Context<Competition>, user_id: u64, score: u64) -> ProgramResult {
        for entry in &mut ctx.accounts.competition_account.leaderboard {
            if entry.user_id == user_id {
                entry.score = score;
                break;
            }
        }
        Ok(())
    }

    // Define the display_leaderboard function
    pub fn display_leaderboard(ctx: Context<Competition>) -> ProgramResult {
        for entry in &ctx.accounts.competition_account.leaderboard {
            msg!("User ID: {}, Score: {}", entry.user_id, entry.score);
        }
        Ok(())
    }
}
```