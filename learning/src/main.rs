extern crate dotenv;

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let atslega = env::var("ATSLEGA").expect("nu jabut");
    println!("Atslega: {}", atslega);

}