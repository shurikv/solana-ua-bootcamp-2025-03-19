use solana_sdk::signature::{Keypair, Signer};
fn main() {
    let keypair = Keypair::new();
    println!(
        "pubkey: {:?}; secret: {:?}, private: {:?}",
        keypair.pubkey(),
        keypair.secret(),
        keypair.to_bytes()
    );
}
