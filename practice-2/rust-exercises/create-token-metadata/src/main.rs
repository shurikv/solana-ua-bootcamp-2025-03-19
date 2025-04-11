use dotenvy::dotenv;
use mpl_token_metadata::accounts::Metadata;
use mpl_token_metadata::instructions::{
    CreateMetadataAccountV3, CreateMetadataAccountV3InstructionArgs,
};
use mpl_token_metadata::types::DataV2;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, SeedDerivable, Signer};
use solana_sdk::system_program;
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
    let secret_key = parse_secret_key(&secret_key);
    let keypair =
        Keypair::from_seed(secret_key.as_slice()).expect("Failed to create keypair from seed");
    let client = solana_client::rpc_client::RpcClient::new("https://api.devnet.solana.com");
    println!("keypair: {}", keypair.pubkey());

    let mint = std::env::var("MINT")
        .expect("MINT must be set")
        .parse::<Pubkey>()
        .expect("Invalid mint pubkey");

    let metadata_account = Metadata::find_pda(&mint).0;

    // Нові метадані
    let new_data = DataV2 {
        name: "Super Duper Token".to_string(),
        symbol: "SDT".to_string(),
        uri: "https://example.com/metadata.json".to_string(),
        seller_fee_basis_points: 500, // 5%
        creators: None,
        collection: None,
        uses: None,
    };

    let ix = CreateMetadataAccountV3 {
        metadata: metadata_account,
        mint,
        mint_authority: keypair.pubkey(),
        payer: keypair.pubkey(),
        update_authority: (keypair.pubkey(), true),
        system_program: system_program::id(),
        rent: None,
    }
    .instruction(CreateMetadataAccountV3InstructionArgs {
        data: new_data,
        is_mutable: true,
        collection_details: None,
    });

    let recent_blockhash = client
        .get_latest_blockhash()
        .expect("Failed to get blockhash");

    let transaction = Transaction::new_signed_with_payer(
        &[ix],
        Some(&keypair.pubkey()),
        &[&keypair],
        recent_blockhash,
    );

    let signature = client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");
    println!("Transaction signature: {:?}", signature);

    let account_data = client
        .get_account_data(&metadata_account)
        .expect("Failed to get account data");

    let metadata = Metadata::from_bytes(&account_data[..]).expect("Failed to deserialize metadata");

    println!("📛 Name: {}", metadata.name);
    println!("🔖 Symbol: {}", metadata.symbol);
    println!("🌐 URI: {}", metadata.uri);
    println!("✍️ Update authority: {}", metadata.update_authority);
    println!("👤 You're signing with: {}", keypair.pubkey());
}