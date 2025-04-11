mod macros;

use dotenvy::dotenv;
use solana_sdk::native_token::{lamports_to_sol, sol_to_lamports};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, SeedDerivable, Signer};
use solana_sdk::system_instruction;
use solana_sdk::transaction::Transaction;

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
    let recipient = std::env::var("RECIPIENT_WALLET").expect("RECIPIENT_WALLET must be set");
    let secret_key = parse_secret_key(&secret_key);
    let keypair =
        Keypair::from_seed(secret_key.as_slice()).expect("Failed to create keypair from seed");
    let client = solana_client::rpc_client::RpcClient::new("https://api.devnet.solana.com");
    let balance = client.get_balance(&keypair.pubkey());
    print_balance!(keypair.pubkey(), balance);

    let recipient_pubkey = recipient
        .parse::<Pubkey>()
        .expect("Invalid recipient pubkey");
    let transfer_amount = sol_to_lamports(0.05); // 0.1 SOL in lamports
    let recent_blockhash = client
        .get_latest_blockhash()
        .expect("Failed to get blockhash");
    let transaction = Transaction::new_signed_with_payer(
        &[system_instruction::transfer(
            &keypair.pubkey(),
            &recipient_pubkey,
            transfer_amount,
        )],
        Some(&keypair.pubkey()),
        &[&keypair],
        recent_blockhash,
    );
    let signature = client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");
    println!("Transaction signature: {:?}", signature);
    let balance = client.get_balance(&keypair.pubkey());
    print_balance!(keypair.pubkey(), balance);

    let balance = client.get_balance(&recipient_pubkey);
    print_balance!(recipient_pubkey, balance);

    println!("Transfer completed successfully.");
}
