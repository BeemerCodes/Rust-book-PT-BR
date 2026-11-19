fn main() {
    // ANCHOR: here
    let mut s = String::from("hello");

    let r1 = &s; // sem problema
    let r2 = &s; // sem problema
    let r3 = &mut s; // GRANDE PROBLEMA

    println!("{r1}, {r2}, and {r3}");
    // ANCHOR_END: here
}
