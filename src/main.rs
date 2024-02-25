use dotenv::from_filename;
use std::env;

enum Result<String, VarError> {
    Ok(String),
    Err(VarError),
}

fn main() {
    from_filename(".env").ok();

    let result_descriptor = env::var("WALLET_DESCRIPTOR");

    let descriptor = match result_descriptor {
        Ok(descriptor) => descriptor,
        Err(e) => panic!("Error: {}", e),
    };

    println!("Descriptor: {}", descriptor);
}
