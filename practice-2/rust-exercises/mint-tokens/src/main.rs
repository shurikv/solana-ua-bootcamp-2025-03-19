use dotenvy::dotenv;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, SeedDerivable, Signer};
use solana_sdk::transaction::Transaction;
use spl_token::instruction::mint_to;

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
        .expect("Invalid mint pubkey");

    let token_account = std::env::var("TOKEN_ACCOUNT")
        .expect("TOKEN_ACCOUNT must be set")
        .parse::<Pubkey>()
        .expect("Invalid token account pubkey");

    let mint_result = mint_to(
        &spl_token::id(),
        &mint,
        &token_account,
        &keypair.pubkey(),
        &[&keypair.pubkey()],
        10 * 1000000000,
    ).expect("Failed to create mint instruction");

    let recent_blockhash = client
        .get_latest_blockhash()
        .expect("Failed to get blockhash");

    let transaction = Transaction::new_signed_with_payer(
        &[
            mint_result,
        ],
        Some(&keypair.pubkey()),
        &[&keypair],
        recent_blockhash,
    );

    let signature = client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");
    println!("Transaction signature: {:?}", signature);
}
