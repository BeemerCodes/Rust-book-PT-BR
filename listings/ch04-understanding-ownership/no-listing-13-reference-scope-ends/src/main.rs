fn main() {
    // ANCHOR: here
    let mut s = String::from("hello");

    let r1 = &s; // sem problema
    let r2 = &s; // sem problema
    println!("{r1} and {r2}");
    // As variáveis r1 e r2 não serão usadas após este ponto.

    let r3 = &mut s; // sem problema
    println!("{r3}");
    // ANCHOR_END: here
}
