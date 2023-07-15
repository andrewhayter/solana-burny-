```rust
use anchor_lang::prelude::*;
use crate::Vote;

// Main entry point for the program
#[program]
pub mod community_governance {
    use super::*;

    // Function to cast a vote
    #[access_control(ctx.accounts.token_owner.did_sign())]
    pub fn cast_vote(ctx: Context<CastVote>, vote: Vote) -> ProgramResult {
        let vote_account = &mut ctx.accounts.vote_account;
        vote_account.vote = vote;
        Ok(())
    }
}

// Accounts data and metadata for the cast_vote function
#[derive(Accounts)]
pub struct CastVote<'info> {
    // The account of the voter
    #[account(mut, signer)]
    pub token_owner: AccountInfo<'info>,
    // The account to store the vote
    #[account(mut)]
    pub vote_account: ProgramAccount<'info, Vote>,
}

// Error messages
#[error]
pub enum ErrorCode {
    #[msg("The given account did not sign the transaction.")]
    NotAuthorized,
}
```
This is the main.rs file for the "Community Governance" phase. It includes the main entry point for the program and the function to cast a vote. The function uses the Anchor framework's access control to ensure that the vote is cast by the owner of the token. The function takes in a context that includes the accounts data and metadata, and the vote. The vote is then stored in the vote account. The file also includes error messages for unauthorized transactions.