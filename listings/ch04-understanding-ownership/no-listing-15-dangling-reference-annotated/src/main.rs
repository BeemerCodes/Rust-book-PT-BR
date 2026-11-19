fn main() {
    let reference_to_nothing = dangle();
}

// ANCHOR: here
fn dangle() -> &String { // dangle retorna uma referência para uma String

    let s = String::from("hello"); // s é uma nova String

    &s // retornamos uma referência para a String, s
} // Aqui, s sai de escopo e é descartada, então sua memória é liberada.
  // Perigo!
  // ANCHOR_END: here
