fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name e field_value são inválidos neste ponto, tente usá-los e
    // veja qual erro de compilação você recebe!
    // ANCHOR_END: here
}
