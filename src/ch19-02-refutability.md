<a id="refutability-whether-a-pattern-might-fail-to-match"></a>

## Refutabilidade: Se um Padrão Pode Falhar ao Corresponder

Padrões existem em duas formas: refutáveis e irrefutáveis. Padrões que
corresponderão a qualquer valor possível passado são *irrefutáveis*. Um exemplo
seria `x` na instrução `let x = 5;`, porque `x` corresponde a qualquer coisa e,
portanto, não pode falhar na correspondência. Padrões que podem falhar na
correspondência para algum valor possível são *refutáveis*. Um exemplo seria
`Some(x)` na expressão `if let Some(x) = a_value`, porque se o valor na
variável `a_value` for `None` em vez de `Some`, o padrão `Some(x)` não
corresponderá.

Parâmetros de função, instruções `let` e laços `for` só podem aceitar padrões
irrefutáveis porque o programa não pode fazer nada significativo quando os
valores não correspondem. As expressões `if let` e `while let` e a instrução
`let...else` aceitam padrões refutáveis e irrefutáveis, mas o compilador alerta
contra padrões irrefutáveis porque, por definição, eles são destinados a lidar
com possíveis falhas: a funcionalidade de uma condicional está na sua capacidade
de agir de forma diferente dependendo do sucesso ou da falha.

Em geral, você não deveria ter que se preocupar com a distinção entre padrões
refutáveis e irrefutáveis; no entanto, você precisa estar familiarizado com o
conceito de refutabilidade para poder reagir quando o vir em uma mensagem de
erro. Nesses casos, você precisará alterar o padrão ou a construção com a qual
está usando o padrão, dependendo do comportamento pretendido do código.

Vamos ver um exemplo do que acontece quando tentamos usar um padrão refutável
onde o Rust exige um padrão irrefutável e vice-versa. A Listagem 19-8 mostra uma
instrução `let`, mas para o padrão especificamos `Some(x)`, um padrão
refutável. Como você deve imaginar, esse código não compilará.

<Listing number="19-8" caption="Tentando usar um padrão refutável com `let`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-08/src/main.rs:here}}
```

</Listing>

Se `some_option_value` fosse um valor `None`, ele falharia em corresponder ao
padrão `Some(x)`, o que significa que o padrão é refutável. Contudo, a instrução
`let` só pode aceitar um padrão irrefutável porque não há nada válido que o
código possa fazer com um valor `None`. Em tempo de compilação, o Rust reclamará
que tentamos usar um padrão refutável onde um padrão irrefutável é exigido:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-08/output.txt}}
```

Como não cobrimos (e não poderíamos cobrir!) todos os valores válidos com o
padrão `Some(x)`, o Rust corretamente produz um erro de compilação.

Se tivermos um padrão refutável onde um padrão irrefutável é necessário, podemos
corrigir alterando o código que usa o padrão: em vez de usar `let`, podemos usar
`let...else`. Então, se o padrão não corresponder, o código entre chaves
tratará o valor. A Listagem 19-9 mostra como corrigir o código da Listagem 19-8.

<Listing number="19-9" caption="Usando `let...else` e um bloco com padrões refutáveis em vez de `let`">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-09/src/main.rs:here}}
```

</Listing>

Demos uma saída ao código! Esse código é perfeitamente válido, embora signifique
que não podemos usar um padrão irrefutável sem receber um aviso. Se dermos ao
`let...else` um padrão que sempre corresponderá, como `x`, conforme mostrado na
Listagem 19-10, o compilador emitirá um aviso.

<Listing number="19-10" caption="Tentando usar um padrão irrefutável com `let...else`">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-10/src/main.rs:here}}
```

</Listing>

O Rust reclama que não faz sentido usar `let...else` com um padrão irrefutável
porque o `else` nunca será alcançado:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-10/output.txt}}
```

Por esse motivo, os braços de um `match` devem usar padrões refutáveis, exceto o
último braço, que deve corresponder a quaisquer valores restantes com um padrão
irrefutável. O Rust nos permite usar um padrão irrefutável em um `match` com
apenas um braço, mas essa sintaxe não é particularmente útil e poderia ser
substituída por uma instrução `let` mais simples.

Agora que você sabe onde usar padrões e a diferença entre padrões refutáveis e
irrefutáveis, vamos cobrir toda a sintaxe que podemos usar para criar padrões.
