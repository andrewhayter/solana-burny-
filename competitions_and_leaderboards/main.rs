```rust
use anchor_lang::prelude::*;
use crate::lib::*;

#[program]
pub mod competitions_and_leaderboards {
    use super::*;

    // Function to start a new competition
    #[access_control(ctx.accounts.token.owner == *ctx.accounts.owner.key)]
    pub fn start_competition(ctx: Context<StartCompetition>, competition_id: u64, reward: u64) -> ProgramResult {
        let competition = &mut ctx.accounts.competition;
        competition.competition_id = competition_id;
        competition.reward = reward;
        competition.is_active = true;
        Ok(())
    }

    // Function to end a competition
    #[access_control(ctx.accounts.token.owner == *ctx.accounts.owner.key)]
    pub fn end_competition(ctx: Context<EndCompetition>, competition_id: u64) -> ProgramResult {
        let competition = &mut ctx.accounts.competition;
        if competition.competition_id == competition_id {
            competition.is_active = false;
        }
        Ok(())
    }

    // Function to participate in a competition
    pub fn participate_in_competition(ctx: Context<ParticipateInCompetition>, competition_id: u64) -> ProgramResult {
        let competition = &mut ctx.accounts.competition;
        if competition.competition_id == competition_id && competition.is_active {
            competition.participants.push(*ctx.accounts.participant.key);
        }
        Ok(())
    }

    // Function to display the leaderboard
    pub fn display_leaderboard(ctx: Context<DisplayLeaderboard>, competition_id: u64) -> ProgramResult {
        let competition = &mut ctx.accounts.competition;
        if competition.competition_id == competition_id {
            msg!("Leaderboard for competition {}: {:?}", competition_id, competition.participants);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartCompetition<'info> {
    #[account(mut)]
    pub competition: ProgramAccount<'info, Competition>,
    pub owner: Signer<'info>,
    pub token: CpiAccount<'info, Token>,
}

#[derive(Accounts)]
pub struct EndCompetition<'info> {
    #[account(mut)]
    pub competition: ProgramAccount<'info, Competition>,
    pub owner: Signer<'info>,
    pub token: CpiAccount<'info, Token>,
}

#[derive(Accounts)]
pub struct ParticipateInCompetition<'info> {
    #[account(mut)]
    pub competition: ProgramAccount<'info, Competition>,
    pub participant: Signer<'info>,
    pub token: CpiAccount<'info, Token>,
}

#[derive(Accounts)]
pub struct DisplayLeaderboard<'info> {
    #[account(mut)]
    pub competition: ProgramAccount<'info, Competition>,
    pub token: CpiAccount<'info, Token>,
}
```