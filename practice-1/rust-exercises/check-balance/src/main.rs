use dotenvy::dotenv;
use solana_sdk::native_token::lamports_to_sol;
use solana_sdk::signature::{Keypair, SeedDerivable, Signer};

fn main() {
    dotenv().ok();
    let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let value =
        serde_json::from_str::<serde_json::Value>(&secret_key).expect("Failed to parse secret key");
    let secret_key = value
        .as_array()
        .expect("Secret key is not an array")
        .iter()
        .map(|v| v.as_u64().expect("Failed to convert to u64") as u8)
        .collect::<Vec<u8>>();

    let keypair =
        Keypair::from_seed(secret_key.as_slice()).expect("Failed to create keypair from seed");
    let client = solana_client::rpc_client::RpcClient::new("https://api.devnet.solana.com");
    let balance = client.get_balance(&keypair.pubkey());
    println!(
        "pubkey: {}, balance: {:?} SOL",
        keypair.pubkey(),
        lamports_to_sol(balance.unwrap())
    );
}
