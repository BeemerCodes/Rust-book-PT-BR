fn main() {
    // ANCHOR: here
    {                      // s não é válida aqui, pois ainda não foi declarada
        let s = "hello";   // s é válida a partir deste ponto

        // faz coisas com s
    }                      // este escopo terminou e s não é mais válida
    // ANCHOR_END: here
}
