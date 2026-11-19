fn main() {
    // ANCHOR: here
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note que s1 foi movido aqui e não pode mais ser usado
    // ANCHOR_END: here
}
