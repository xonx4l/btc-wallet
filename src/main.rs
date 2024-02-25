use bdk::{bitcoin::Network, database::MemoryDatabase, Wallet};
use std::env;

fn function_that_takes_strins(s: String) {
    println!("String: {}", s);
}

fn main() {
    dotenv::from_filename(".env").ok();
    dotenv::dotenv().ok();

    let result_descriptor = env::var("WALLET_DESCRIPTOR");

    let descriptor = match result_descriptor {
        Ok(descriptor) => descriptor,
        Err(e) => panic!("Error: {}", e),
    };

    println!("Descriptor: {}", descriptor);

    let str_desc = "WALLET_DESCRIPTOR";
    let string_desc = String::from(str_desc);

    let wallet = Wallet::new(
        &descriptor.clone(),
        None,
        Network::Testnet,
        MemoryDatabase::default(),
    )?;
}
