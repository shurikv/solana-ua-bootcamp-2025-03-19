use dotenvy::dotenv;
use solana_sdk::signature::{Keypair, SeedDerivable, Signer};
use solana_sdk::system_instruction::create_account;
use solana_sdk::transaction::Transaction;
use spl_token::instruction::initialize_mint2;
use spl_token::solana_program::program_pack::Pack;
use spl_token::state::Mint;

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

    let mint_keypair = Keypair::new();

    let mint_space = Mint::LEN;
    let rent = client.get_minimum_balance_for_rent_exemption(mint_space).expect("Failed to get rent exemption");

    let create_account_instruction = create_account(
        &keypair.pubkey(),
        &mint_keypair.pubkey(),
        rent,
        mint_space as u64,
        &spl_token::id(),
    );

    let mint_result = initialize_mint2(
        &spl_token::id(),
        &mint_keypair.pubkey(),
        &keypair.pubkey(),
        None,
        9,
    );
    let mint_ix = mint_result.expect("Failed to create mint instruction");

    let recent_blockhash = client
        .get_latest_blockhash()
        .expect("Failed to get blockhash");
    let transaction = Transaction::new_signed_with_payer(
        &[create_account_instruction, mint_ix],
        Some(&keypair.pubkey()),
        &[&keypair, &mint_keypair],
        recent_blockhash,
    );
    let signature = client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");

    println!("Mint Address: {}", mint_keypair.pubkey());
    println!("Transaction signature: {:?}", signature);
}
