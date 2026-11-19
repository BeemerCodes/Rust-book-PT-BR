fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

// ANCHOR: here
fn calculate_length(s: &String) -> usize { // s é uma referência para uma String
    s.len()
} // Aqui, s sai de escopo. Mas como s não tem ownership daquilo a que
  // se refere, a String não é descartada.
// ANCHOR_END: here
