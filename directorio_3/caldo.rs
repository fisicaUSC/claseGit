/*
Compilar con:
    cargo build --release --target wasm32-unknown-unknown
*/

use wasm_minimal_protocol::*;

initiate_protocol!();

#[wasm_func]
pub fn ola() -> Vec<u8> {
    b"Ola dende Wasm!!!".to_vec()
}

fn main() {
    println!("a");
}
