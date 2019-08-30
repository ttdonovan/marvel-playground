use dotenv::dotenv;

use std::env;

fn main() {
    dotenv().ok();
    let public_key = env::var("MARVEL_API_PUBLIC_KEY").
        expect("MARVEL_API_PUBLIC_KEY must be set");
    let private_key = env::var("MARVEL_API_PRIVATE_KEY").
        expect("MARVEL_API_PRIVATE_KEY must be set");

    println!("Hello Marvel API!");
    println!("Marvel API Keys: (public) {} - (private) {}", public_key, private_key);
}