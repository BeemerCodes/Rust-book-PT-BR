fn main() {
    let s1 = gives_ownership();        // gives_ownership move seu valor de
                                       // retorno para s1

    let s2 = String::from("hello");    // s2 entra em escopo

    let s3 = takes_and_gives_back(s2); // s2 é movida para
                                       // takes_and_gives_back, que também
                                       // move seu valor de retorno para s3
} // Aqui, s3 sai de escopo e é descartada. s2 foi movida, então nada
  // acontece. s1 sai de escopo e é descartada.

fn gives_ownership() -> String {       // gives_ownership vai mover seu
                                       // valor de retorno para a função
                                       // que a chamar

    let some_string = String::from("yours"); // some_string entra em escopo

    some_string                        // some_string é retornada e
                                       // movida para fora, para a função
                                       // chamadora
}

// Esta função recebe uma String e retorna uma String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string entra em
    // escopo

    a_string  // a_string é retornada e movida para fora, para a função chamadora
}
