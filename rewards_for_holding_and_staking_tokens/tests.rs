```rust
use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Transfer};
use anchor_test::assert_ok;
use rewards_for_holding_and_staking_tokens::lib::*;

// Define the test context
#[test]
fn test_rewards_for_holding_and_staking_tokens() {
    let mut context = ProgramTest::new(
        "rewards_for_holding_and_staking_tokens", 
        id(),
        processor!(Processor::process),
    );

    // Add the SPL Token program
    context.add_program(
        "spl_token", 
        spl_token::id(),
        processor!(spl_token::processor::Processor::process),
    );

    // Start the test
    let (mut banks_client, payer, recent_blockhash) = context.start().await;

    // Create a new token mint
    let mint = Keypair::new();
    let mint_authority = Keypair::new();
    let token = Keypair::new();

    // Create the mint instruction
    let create_mint_instruction = create_mint(
        &spl_token::id(),
        &mint,
        &mint_authority.pubkey(),
        None,
        2,
        token::id(),
    );

    // Send the create mint instruction
    let mut transaction = Transaction::new_with_payer(
        &[create_mint_instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &mint, &mint_authority], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Create a new token account for the user
    let user_token_account = Keypair::new();

    // Create the create token account instruction
    let create_token_account_instruction = create_token_account(
        &spl_token::id(),
        &user_token_account,
        &token.pubkey(),
        &payer.pubkey(),
        &payer.pubkey(),
        token::id(),
    );

    // Send the create token account instruction
    let mut transaction = Transaction::new_with_payer(
        &[create_token_account_instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &user_token_account], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Mint some tokens to the user's token account
    let mint_tokens_instruction = mint_to(
        &spl_token::id(),
        &token.pubkey(),
        &user_token_account.pubkey(),
        &mint_authority.pubkey(),
        &[],
        1000,
        2,
        token::id(),
    );

    // Send the mint tokens instruction
    let mut transaction = Transaction::new_with_payer(
        &[mint_tokens_instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &mint_authority], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Stake some tokens
    let stake_tokens_instruction = stake_tokens(
        &id(),
        &user_token_account.pubkey(),
        &payer.pubkey(),
        500,
    );

    // Send the stake tokens instruction
    let mut transaction = Transaction::new_with_payer(
        &[stake_tokens_instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Check the user's token balance
    let user_token_account = banks_client
        .get_account(user_token_account.pubkey())
        .await
        .unwrap()
        .unwrap();
    let user_token_account: TokenAccount = TokenAccount::unpack(&user_token_account.data).unwrap();
    assert_eq!(user_token_account.amount, 500);

    // Check the user's staked tokens
    let user_staked_tokens = banks_client
        .get_account(user_staked_tokens.pubkey())
        .await
        .unwrap()
        .unwrap();
    let user_staked_tokens: StakedTokens = StakedTokens::unpack(&user_staked_tokens.data).unwrap();
    assert_eq!(user_staked_tokens.amount, 500);
}
```