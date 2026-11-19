fn main() {
    // ANCHOR: here
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 sai de escopo aqui, então podemos criar uma nova referência sem problemas.

    let r2 = &mut s;
    // ANCHOR_END: here
}
