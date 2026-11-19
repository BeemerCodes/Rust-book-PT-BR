fn main() {
    // ANCHOR: here
    {
        let s = String::from("hello"); // s é válida a partir deste ponto

        // faz coisas com s
    }                                  // este escopo terminou e s não é
                                       // mais válida
    // ANCHOR_END: here
}
