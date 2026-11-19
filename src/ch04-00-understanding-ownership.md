<a id="understanding-ownership"></a>
# Entendendo Ownership

*Ownership* é o recurso mais singular do Rust e tem implicações profundas para o restante da linguagem. Ele permite que o Rust ofereça garantias de segurança de memória sem precisar de um coletor de lixo (*garbage collector*), por isso é importante entender como *ownership* funciona. Neste capítulo, falaremos sobre *ownership* bem como sobre vários recursos relacionados: empréstimo (*borrowing*), *slices* e como o Rust organiza dados na memória.
