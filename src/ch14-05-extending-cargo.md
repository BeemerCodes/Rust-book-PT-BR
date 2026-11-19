<a id="extending-cargo-with-custom-commands"></a>
## Estendendo o Cargo com Comandos Personalizados

O Cargo foi projetado para que você possa estendê-lo com novos subcomandos sem precisar
modificá-lo. Se um binário no seu `$PATH` se chamar `cargo-algumacoisa`, você pode
executá-lo como se fosse um subcomando do Cargo executando `cargo algumacoisa`. Comandos
personalizados como esse também são listados quando você executa `cargo --list`. Poder
usar `cargo install` para instalar extensões e depois executá-las exatamente como as
ferramentas nativas do Cargo é um benefício super conveniente do design do Cargo!

<a id="summary"></a>
## Resumo

Compartilhar código com o Cargo e o [crates.io](https://crates.io/)<!-- ignore --> faz
parte do que torna o ecossistema Rust útil para tantas tarefas diferentes. A biblioteca
padrão do Rust é pequena e estável, mas os *crates* são fáceis de compartilhar, usar e
evoluir em um cronograma diferente do da linguagem. Não tenha receio de
compartilhar códigos que sejam úteis para você no [crates.io](https://crates.io/)<!-- ignore
-->; é bem provável que eles sejam úteis para outras pessoas também!
