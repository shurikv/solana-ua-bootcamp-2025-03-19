use dotenvy::dotenv;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, SeedDerivable, Signer};
use solana_sdk::system_instruction::create_account;
use solana_sdk::transaction::Transaction;
use spl_token::instruction::initialize_account;
use spl_token::solana_program::program_pack::Pack;
use spl_token::state::Account;

fn parse_secret_key(secret_key: &str) -> Vec<u8> {
    serde_json::from_str::<serde_json::Value>(secret_key)
        .expect("Failed to parse secret key")
        .as_array()
        .expect("Secret key is not an array")
        .iter()
        .map(|v| v.as_u64().expect("Failed to convert to u64") as u8)
        .collect::<Vec<u8>>()
}
fn main() {
    dotenv().ok();
    let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let secret_key = parse_secret_key(&secret_key);
    let keypair =
        Keypair::from_seed(secret_key.as_slice()).expect("Failed to create keypair from seed");
    let client = solana_client::rpc_client::RpcClient::new("https://api.devnet.solana.com");

    let mint = std::env::var("MINT")
        .expect("MINT must be set")
        .parse::<Pubkey>()
        .expect("Invalid recipient pubkey");

    let token_account = Keypair::new();

    // Get token account size (in bytes)
    let token_account_space = Account::LEN;
    let token_account_rent = client
        .get_minimum_balance_for_rent_exemption(token_account_space)
        .expect("Failed to get rent exemption");

    // Instruction to create new account for token account (token 2022 program)
    let create_token_account_instruction = create_account(
        &keypair.pubkey(),          // payer
        &token_account.pubkey(),    // new account (token account)
        token_account_rent,         // lamports
        token_account_space as u64, // space
        &spl_token::id(),           // program id
    );

    // Instruction to initialize token account data
    let initialize_token_account_instruction = initialize_account(
        &spl_token::id(),
        &token_account.pubkey(), // account
        &mint,          // mint
        &keypair.pubkey(),       // owner
    )
    .expect("Failed to create initialize token account instruction");

    let recent_blockhash = client
        .get_latest_blockhash()
        .expect("Failed to get blockhash");

    // Create transaction and add instructions
    let transaction = Transaction::new_signed_with_payer(
        &[
            create_token_account_instruction,
            initialize_token_account_instruction,
        ],
        Some(&keypair.pubkey()),
        &[&keypair, &token_account],
        recent_blockhash,
    );

    // Send and confirm transaction
    let transaction_signature = client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");

    println!("Token Account Address: {}", token_account.pubkey());
    println!("Transaction Signature: {}", transaction_signature);
}
