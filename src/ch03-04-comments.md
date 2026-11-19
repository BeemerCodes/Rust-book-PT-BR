<a id="comments"></a>
## Comentários

Todos os programadores se esforçam para tornar seu código fácil de entender, mas às vezes uma explicação extra é bem-vinda. Nesses casos, os programadores deixam _comentários_ no código-fonte que o compilador ignora, mas que podem ser úteis para as pessoas que leem o código.

Aqui está um comentário simples:

```rust
// hello, world
```

Em Rust, o estilo idiomático de comentário começa com duas barras, e o comentário continua até o final da linha. Para comentários que se estendem por mais de uma linha, você precisará incluir `//` em cada linha, assim:

```rust
// Então, estamos fazendo algo complicado aqui, longo o suficiente para precisarmos
// de várias linhas de comentários para explicar! Ufa! Tomara que este comentário
// explique o que está acontecendo.
```

Comentários também podem ser colocados no final de linhas que contêm código:

<span class="filename">Arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-24-comments-end-of-line/src/main.rs}}
```

Mas você os verá com mais frequência usados neste formato, com o comentário em uma linha separada acima do código que ele está anotando:

<span class="filename">Arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-25-comments-above-line/src/main.rs}}
```

O Rust também tem outro tipo de comentário, os comentários de documentação, que discutiremos na seção ["Publicando um Crate no Crates.io"][publishing]<!-- ignore --> do Capítulo 14.

[publishing]: ch14-02-publishing-to-crates-io.html
