use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use std::sync::atomic::{AtomicBool, AtomicU64};
use std::sync::Arc;

const NUM_THREADS: i16 = 10;
const PREFIX: &str = "anza";

fn main() {
    let start_time = std::time::Instant::now();
    let mut thread_handles: Vec<_> = Vec::new();
    let completed = Arc::new(AtomicBool::new(false));
    let iteration_count = Arc::new(AtomicU64::new(0));
    for _ in 0..NUM_THREADS {
        let completed = completed.clone();
        let iteration_count = iteration_count.clone();
        thread_handles.push(std::thread::spawn(move || loop {
            if completed.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }
            let attempts = iteration_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if attempts % 1_000_000 == 0 {
                println!(
                    "Attempts: {}, time: {}s",
                    attempts,
                    start_time.elapsed().as_secs()
                );
            }
            let generated_keypair = Keypair::new();
            if generated_keypair
                .pubkey()
                .to_string()
                .to_lowercase()
                .starts_with(PREFIX)
            {
                println!(
                    "pubkey: {:?}; private key: {:?}",
                    generated_keypair.pubkey(),
                    generated_keypair.secret()
                );
                println!(
                    "Elapsed time: {:?}; attempts: {}",
                    start_time.elapsed(),
                    attempts
                );
                completed.store(true, std::sync::atomic::Ordering::Relaxed);
                break;
            }
        }));
    }
    for thread_handle in thread_handles {
        thread_handle.join().unwrap();
    }
}
