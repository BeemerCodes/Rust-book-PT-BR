extern crate trpl; // necessário para o mdbook test

use std::{thread, time::Duration};

fn main() {
    trpl::block_on(async {
        // Chamaremos `slow` aqui mais tarde
    });
}

// ANCHOR: slow
fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}
// ANCHOR_END: slow
