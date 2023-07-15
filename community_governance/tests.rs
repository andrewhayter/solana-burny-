```rust
use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Transfer};
use community_governance::Vote;
use solana_program::instruction::Instruction;
use solana_program_test::BanksClient;
use solana_sdk::signer::Signer;

// Test the castVote function
#[tokio::test]
async fn test_cast_vote() {
    // Create a new test environment
    let mut test = ProgramTest::new(
        "community_governance", 
        id(),
        processor!(Vote::process),
    );

    // Add the SPL Token program
    test.add_program(
        "spl_token", 
        token::id(), 
        processor!(token::processor::Processor::process),
    );

    // Start the test
    let (mut banks_client, payer, recent_blockhash) = test.start().await;

    // Create a new vote account
    let vote_account = Keypair::new();
    let vote_account_balance = 10;

    // Create a new token account for the payer
    let payer_token_account = Keypair::new();
    let payer_token_account_balance = 100;

    // Create a new token mint
    let mint = Keypair::new();
    let mint_authority = Keypair::new();

    // Initialize the token mint
    let mut transaction = Transaction::new_with_payer(
        &[token::initialize_mint(
            &token::id(),
            &mint.pubkey(),
            &mint_authority.pubkey(),
            None,
            2,
        )
        .unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &mint, &mint_authority], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Initialize the payer's token account
    let mut transaction = Transaction::new_with_payer(
        &[token::initialize_account(
            &token::id(),
            &payer_token_account.pubkey(),
            &mint.pubkey(),
            &payer.pubkey(),
        )
        .unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &payer_token_account], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Mint tokens to the payer's token account
    let mut transaction = Transaction::new_with_payer(
        &[token::mint_to(
            &token::id(),
            &mint.pubkey(),
            &payer_token_account.pubkey(),
            &mint_authority.pubkey(),
            &[],
            payer_token_account_balance,
        )
        .unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &mint_authority], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Cast a vote
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            id(),
            &Vote::CastVote {
                amount: 1,
            },
            vec![
                AccountMeta::new(payer_token_account.pubkey(), false),
                AccountMeta::new(vote_account.pubkey(), false),
                AccountMeta::new(payer.pubkey(), true),
            ],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Check the vote account's balance
    let vote_account = banks_client
        .get_account(vote_account.pubkey())
        .await
        .unwrap()
        .unwrap();
    let vote_account: TokenAccount = TokenAccount::unpack_from_slice(vote_account.data.as_slice()).unwrap();
    assert_eq!(vote_account.amount, 1);
}
```