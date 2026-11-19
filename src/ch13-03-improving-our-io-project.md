<a id="improving-our-io-project"></a>
## Melhorando Nosso Projeto de I/O

Com esse novo conhecimento sobre iteradores, podemos melhorar o projeto de I/O do Capítulo 12 usando iteradores para deixar trechos do código mais claros e concisos. Vamos ver como iteradores podem melhorar nossa implementação da função `Config::build` e da função `search`.

<a id="removing-a-clone-using-an-iterator"></a>
### Removendo um `clone` Usando um Iterador

Na Listagem 12-6, adicionamos um código que recebia uma *slice* de valores `String` e criava uma instância da *struct* `Config` indexando a *slice* e clonando os valores, permitindo que a *struct* `Config` tivesse *ownership* desses valores. Na Listagem 13-17, reproduzimos a implementação da função `Config::build` como ela estava na Listagem 12-23.

<Listing number="13-17" file-name="src/main.rs" caption="Reprodução da função `Config::build` da Listagem 12-23">

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-23-reproduced/src/main.rs:ch13}}
```

</Listing>

Naquela hora, dissemos para não se preocupar com as chamadas ineficientes a `clone` porque iríamos removê-las no futuro. Pois bem, essa hora chegou!

Precisávamos de `clone` aqui porque temos uma *slice* com elementos `String` no parâmetro `args`, mas a função `build` não tem *ownership* de `args`. Para retornar a *ownership* de uma instância de `Config`, tivemos que clonar os valores dos campos `query` e `file_path` de `Config` para que a instância pudesse ser dona de seus valores.

Com nosso novo conhecimento sobre iteradores, podemos alterar a função `build` para receber a *ownership* de um iterador como argumento, em vez de emprestar uma *slice*. Usaremos os recursos do iterador no lugar do código que verifica o comprimento da *slice* e indexa posições específicas. Isso deixará mais claro o que a função `Config::build` está fazendo, já que o iterador cuidará de acessar os valores.

Assim que `Config::build` assumir a *ownership* do iterador e deixar de usar operações de indexação que fazem *borrowing*, poderemos mover os valores `String` do iterador diretamente para `Config`, em vez de chamar `clone` e fazer uma nova alocação.

<a id="using-the-returned-iterator-directly"></a>
#### Usando o Iterador Retornado Diretamente

Abra o arquivo _src/main.rs_ do seu projeto de I/O, que deve estar assim:

<span class="filename">Arquivo: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-24-reproduced/src/main.rs:ch13}}
```

Primeiro, vamos alterar o início da função `main` que tínhamos na Listagem 12-24 para o código da Listagem 13-18, que desta vez usa um iterador. Isso não vai compilar até atualizarmos `Config::build` também.

<Listing number="13-18" file-name="src/main.rs" caption="Passando o valor de retorno de `env::args` para `Config::build`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-18/src/main.rs:here}}
```

</Listing>

A função `env::args` retorna um iterador! Em vez de coletar os valores do iterador em um vetor e depois passar uma *slice* para `Config::build`, agora estamos passando a *ownership* do iterador retornado por `env::args` diretamente para `Config::build`.

Em seguida, precisamos atualizar a definição de `Config::build`. Vamos mudar a assinatura de `Config::build` para ficar como na Listagem 13-19. Isso ainda não vai compilar porque precisamos atualizar o corpo da função.

<Listing number="13-19" file-name="src/main.rs" caption="Atualizando a assinatura de `Config::build` para esperar um iterador">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-19/src/main.rs:here}}
```

</Listing>

A documentação da biblioteca padrão para a função `env::args` mostra que o tipo do iterador retornado por ela é `std::env::Args`, e esse tipo implementa o *trait* `Iterator` retornando valores `String`.

Atualizamos a assinatura da função `Config::build` para que o parâmetro `args` tenha um tipo genérico com a restrição de *trait* `impl Iterator<Item = String>` em vez de `&[String]`. Esse uso da sintaxe `impl Trait`, que discutimos na seção [“Traits como Parâmetros”][impl-trait]<!-- ignore --> do Capítulo 10, significa que `args` pode ser qualquer tipo que implemente o *trait* `Iterator` e retorne itens do tipo `String`.

Como estamos assumindo a *ownership* de `args` e vamos modificá-lo ao iterar sobre ele, adicionamos a palavra-chave `mut` na especificação do parâmetro `args` para torná-lo mutável.

<!-- Old headings. Do not remove or links may break. -->

<a id="using-iterator-trait-methods-instead-of-indexing"></a>

<a id="using-iterator-trait-methods"></a>
#### Usando Métodos do Trait `Iterator`

Agora vamos corrigir o corpo de `Config::build`. Como `args` implementa o *trait* `Iterator`, sabemos que podemos chamar o método `next` nele! A Listagem 13-20 atualiza o código da Listagem 12-23 para usar o método `next`.

<Listing number="13-20" file-name="src/main.rs" caption="Alterando o corpo de `Config::build` para usar métodos de iterador">

```rust,ignore,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-20/src/main.rs:here}}
```

</Listing>

Lembre-se de que o primeiro valor retornado por `env::args` é o nome do programa. Queremos ignorar isso e ir para o próximo valor, então primeiro chamamos `next` e não fazemos nada com o retorno. Depois, chamamos `next` para obter o valor que queremos colocar no campo `query` de `Config`. Se `next` retornar `Some`, usamos um `match` para extrair o valor. Se retornar `None`, significa que não foram passados argumentos suficientes, e retornamos imediatamente com um valor `Err`. Fazemos o mesmo para o valor de `file_path`.

<!-- Old headings. Do not remove or links may break. -->

<a id="making-code-clearer-with-iterator-adapters"></a>

<a id="clarifying-code-with-iterator-adapters"></a>
### Deixando o Código Mais Claro com Adaptadores de Iterador

Também podemos aproveitar iteradores na função `search` do nosso projeto de I/O, reproduzida aqui na Listagem 13-21 exatamente como estava na Listagem 12-19.

<Listing number="13-21" file-name="src/lib.rs" caption="A implementação da função `search` da Listagem 12-19">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:ch13}}
```

</Listing>

Podemos escrever esse código de forma mais concisa usando métodos adaptadores de iterador. Fazer isso também nos evita ter um vetor intermediário mutável `results`. O estilo de programação funcional prefere minimizar a quantidade de estado mutável para deixar o código mais claro. Remover o estado mutável pode até facilitar uma melhoria futura para executar a busca em paralelo, já que não precisaríamos gerenciar acesso concorrente ao vetor `results`. A Listagem 13-22 mostra essa mudança.

<Listing number="13-22" file-name="src/lib.rs" caption="Usando métodos adaptadores de iterador na implementação da função `search`">

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-22/src/lib.rs:here}}
```

</Listing>

Lembre-se de que o objetivo da função `search` é retornar todas as linhas em `contents` que contêm `query`. De forma semelhante ao exemplo de `filter` na Listagem 13-16, esse código usa o adaptador `filter` para manter apenas as linhas em que `line.contains(query)` retorna `true`. Em seguida, coletamos as linhas correspondentes em outro vetor com `collect`. Muito mais simples! Sinta-se à vontade para fazer a mesma mudança e usar métodos de iterador na função `search_case_insensitive` também.

Como melhoria adicional, você pode retornar um iterador da função `search` removendo a chamada a `collect` e alterando o tipo de retorno para `impl Iterator<Item = &'a str>`, fazendo com que a função se torne um adaptador de iterador. Note que você também precisará atualizar os testes! Faça uma busca em um arquivo grande usando sua ferramenta `minigrep` antes e depois dessa mudança para observar a diferença de comportamento. Antes da mudança, o programa não imprime nenhum resultado até coletar todos eles; depois da mudança, os resultados são impressos à medida que cada linha correspondente é encontrada, porque o laço `for` na função `run` consegue aproveitar a avaliação preguiçosa (*laziness*) do iterador.

<!-- Old headings. Do not remove or links may break. -->

<a id="choosing-between-loops-or-iterators"></a>

<a id="choosing-between-loops-and-iterators"></a>
### Escolhendo Entre Laços e Iteradores

A próxima pergunta lógica é qual estilo você deve escolher no seu próprio código e por quê: a implementação original da Listagem 13-21 ou a versão com iteradores da Listagem 13-22 (assumindo que estamos coletando todos os resultados antes de retorná-los, em vez de retornar o iterador). A maioria dos programadores Rust prefere usar o estilo com iteradores. É um pouco mais difícil pegar o jeito no começo, mas assim que você se familiariza com os vários adaptadores de iterador e o que eles fazem, os iteradores podem ser mais fáceis de entender. Em vez de mexer com vários detalhes de controle de laço e construção de novos vetores, o código foca no objetivo de alto nível do laço. Isso abstrai parte do código rotineiro, deixando mais fácil enxergar os conceitos específicos daquele trecho, como a condição de filtragem que cada elemento do iterador precisa satisfazer.

Mas será que as duas implementações são realmente equivalentes? A intuição pode sugerir que o laço de nível mais baixo seria mais rápido. Vamos falar sobre desempenho.

[impl-trait]: ch10-02-traits.html#traits-as-parameters
