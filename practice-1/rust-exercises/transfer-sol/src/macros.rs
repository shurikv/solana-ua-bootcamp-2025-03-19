#[macro_export]
macro_rules! print_balance {
    ( $keypair:expr, $balance:expr ) => {{
    println!(
        "pubkey: {}, balance: {:?} SOL",
        $keypair,
        lamports_to_sol($balance.unwrap())
    );
    }};
}