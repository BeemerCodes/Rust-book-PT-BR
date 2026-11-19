// ANCHOR: here
fn first_word(s: &str) -> &str {
    // ANCHOR_END: here
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// ANCHOR: usage
fn main() {
    let my_string = String::from("hello world");

    // `first_word` funciona em slices de `String`s, sejam parciais ou inteiras.
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` também funciona em referências para `String`s, que são equivalentes
    // a slices inteiras de `String`s.
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` funciona em slices de literais de string, sejam parciais ou
    // inteiros.
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Como literais de string já *são* string slices,
    // isso também funciona, sem a sintaxe de slice!
    let word = first_word(my_string_literal);
}
// ANCHOR_END: usage
