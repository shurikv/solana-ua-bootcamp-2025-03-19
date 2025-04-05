use solana_sdk::signature::{Keypair, Signer};
fn main() {
    let keypair = Keypair::new();
    println!(
        "pubkey: {:?}; private: {:?}",
        keypair.pubkey(),
        keypair.secret()
    );
}
