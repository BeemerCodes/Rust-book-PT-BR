// ANCHOR: here
#[derive(Debug)] // para podermos inspecionar o estado em breve
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
// ANCHOR_END: here

fn main() {}
