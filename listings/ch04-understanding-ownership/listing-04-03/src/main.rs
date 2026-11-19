fn main() {
    let s = String::from("hello");  // s entra em escopo

    takes_ownership(s);             // o valor de s é movido para a função...
                                    // ... e, portanto, não é mais válido aqui

    let x = 5;                      // x entra em escopo

    makes_copy(x);                  // Como i32 implementa o trait Copy,
                                    // x NÃO é movido para a função,
                                    // então tudo bem usar x depois.

} // Aqui, x sai de escopo e depois s. Porém, como o valor de s foi movido,
  // nada de especial acontece.

fn takes_ownership(some_string: String) { // some_string entra em escopo
    println!("{some_string}");
} // Aqui, some_string sai de escopo e `drop` é chamada. A memória subjacente
  // é liberada.

fn makes_copy(some_integer: i32) { // some_integer entra em escopo
    println!("{some_integer}");
} // Aqui, some_integer sai de escopo. Nada de especial acontece.
