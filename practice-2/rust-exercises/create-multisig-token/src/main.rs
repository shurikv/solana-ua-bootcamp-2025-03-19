use dotenvy::dotenv;
use solana_sdk::signature::{Keypair, SeedDerivable, Signer};
use solana_sdk::system_instruction::create_account;
use solana_sdk::transaction::Transaction;
use spl_associated_token_account::{get_associated_token_address, instruction};
use spl_token::instruction::{initialize_mint2, initialize_multisig2, mint_to};
use spl_token::solana_program::program_pack::Pack;

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

    let second_signer = Keypair::new();
    println!(
        "second signer pubkey: {:?}; secret: {:?}, private: {:?}",
        second_signer.pubkey(),
        second_signer.secret(),
        second_signer.to_bytes()
    );
    let multisig = Keypair::new();

    println!(
        "multisig pubkey: {:?}; secret: {:?}, private: {:?}",
        multisig.pubkey(),
        multisig.secret(),
        multisig.to_bytes()
    );

    let multisig_rent = client
        .get_minimum_balance_for_rent_exemption(spl_token::state::Multisig::LEN)
        .expect("Failed to fetch rent exemption balance");

    let create_multisig_ix = create_account(
        &keypair.pubkey(),
        &multisig.pubkey(),
        multisig_rent,
        spl_token::state::Multisig::LEN as u64,
        &spl_token::id(),
    );

    let ms = initialize_multisig2(
        &spl_token::id(),
        &multisig.pubkey(),
        &[&keypair.pubkey(), &second_signer.pubkey()],
        2,
    )
    .expect("Failed to create multisig instruction");

    let mint_keypair = Keypair::new();

    let mint_rent = client
        .get_minimum_balance_for_rent_exemption(spl_token::state::Mint::LEN)
        .expect("Failed to fetch rent exemption balance");

    let create_mint_ix = create_account(
        &keypair.pubkey(),
        &mint_keypair.pubkey(),
        mint_rent,
        spl_token::state::Mint::LEN as u64,
        &spl_token::id(),
    );

    let mint_result = initialize_mint2(
        &spl_token::id(),
        &mint_keypair.pubkey(),
        &multisig.pubkey(),
        None,
        9,
    );
    let mint_ix = mint_result.expect("Failed to create mint instruction");

    let recent_blockhash = client
        .get_latest_blockhash()
        .expect("Failed to get blockhash");
    let transaction = Transaction::new_signed_with_payer(
        &[create_multisig_ix, ms, create_mint_ix, mint_ix],
        Some(&keypair.pubkey()),
        &[&keypair, &multisig, &mint_keypair],
        recent_blockhash,
    );
    let signature = client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");

    println!("Mint Address: {}", mint_keypair.pubkey());
    println!("Transaction signature 1: {:?}", signature);

    let recipient = keypair.pubkey();
    let ata = get_associated_token_address(&recipient, &mint_keypair.pubkey());

    let create_ata_ix = instruction::create_associated_token_account(
        &keypair.pubkey(),
        &recipient,
        &mint_keypair.pubkey(),
        &spl_token::id(),
    );

    let recent_blockhash = client
        .get_latest_blockhash()
        .expect("Failed to get blockhash");
    let transaction = Transaction::new_signed_with_payer(
        &[create_ata_ix],
        Some(&keypair.pubkey()),
        &[&keypair],
        recent_blockhash,
    );
    let signature = client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");

    println!("Transaction signature 2: {:?}", signature);

    let amount = 1_000_000_000;

    let mint_ix = mint_to(
        &spl_token::id(),
        &mint_keypair.pubkey(),
        &ata,
        &multisig.pubkey(),
        &[
            &keypair.pubkey(), // Хто підписує як частина мультипідпису
            &second_signer.pubkey(),
        ],
        amount,
    )
    .expect("Failed to create mint_to instruction");

    let recent_blockhash = client
        .get_latest_blockhash()
        .expect("Failed to get blockhash");
    let transaction = Transaction::new_signed_with_payer(
        &[mint_ix],
        Some(&keypair.pubkey()),
        &[&keypair, &second_signer],
        recent_blockhash,
    );
    let signature = client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");

    println!("Transaction signature 3: {:?}", signature);
}
