fn main() {
    // ANCHOR: here
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() anexa um literal a uma String

    println!("{s}"); // isso imprimirá `hello, world!`
    // ANCHOR_END: here
}
