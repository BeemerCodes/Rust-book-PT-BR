// ANCHOR: here
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
// ANCHOR_END: here

fn main() {
    let my_string = String::from("hello world");

    // first_word funciona em slices de `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word funciona em slices de strings literais
    let word = first_word(&my_string_literal[..]);

    // Como strings literais *já são* slices de strings,
    // isso também funciona, sem a sintaxe de slice!
    let word = first_word(my_string_literal);
}
