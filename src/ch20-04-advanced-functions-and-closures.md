<a id="advanced-functions-and-closures"></a>
## Funções e *Closures* Avançadas

Esta seção explora alguns recursos avançados relacionados a funções e
*closures*, incluindo ponteiros de função e retorno de *closures*.

<a id="function-pointers"></a>
### Ponteiros de Função

Já falamos sobre como passar *closures* para funções; você também pode passar
funções normais para funções! Essa técnica é útil quando você quer passar uma
função que já definiu em vez de definir uma nova *closure*. Funções sofrem
coerção para o tipo `fn` (com _f_ minúsculo), que não deve ser confundido com a
*trait* de *closure* `Fn`. O tipo `fn` é chamado de *ponteiro de função*
(*function pointer*). Passar funções usando ponteiros de função permite usar
funções como argumentos para outras funções.

A sintaxe para especificar que um parâmetro é um ponteiro de função é parecida
com a de *closures*, como mostra a Listagem 20-28, onde definimos uma função
`add_one` que soma 1 ao seu parâmetro. A função `do_twice` recebe dois
parâmetros: um ponteiro de função para qualquer função que receba um parâmetro
`i32` e retorne um `i32`, e um valor `i32`. A função `do_twice` chama a função
`f` duas vezes, passando o valor `arg`, e então soma os resultados das duas
chamadas. A função `main` chama `do_twice` com os argumentos `add_one` e `5`.

<Listing number="20-28" file-name="src/main.rs" caption="Usando o tipo `fn` para aceitar um ponteiro de função como argumento">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-28/src/main.rs}}
```

</Listing>

Este código imprime `The answer is: 12`. Especificamos que o parâmetro `f` em
`do_twice` é um `fn` que recebe um parâmetro do tipo `i32` e retorna um `i32`.
Podemos então chamar `f` no corpo de `do_twice`. Em `main`, podemos passar o
nome da função `add_one` como o primeiro argumento para `do_twice`.

Ao contrário de *closures*, `fn` é um tipo em vez de uma *trait*, então
especificamos `fn` como o tipo do parâmetro diretamente, em vez de declarar um
parâmetro de tipo genérico com uma das *traits* `Fn` como *trait bound*.

Ponteiros de função implementam todas as três *traits* de *closure* (`Fn`,
`FnMut` e `FnOnce`), o que significa que você sempre pode passar um ponteiro de
função como argumento para uma função que espera uma *closure*. O ideal é
escrever funções usando um tipo genérico e uma das *traits* de *closure* para
que suas funções aceitem tanto funções quanto *closures*.

Dito isso, um exemplo de situação em que você gostaria de aceitar apenas `fn` e
não *closures* é ao fazer interface com código externo que não possui *closures*:
funções em C podem aceitar funções como argumentos, mas C não tem *closures*.

Como exemplo de onde você pode usar tanto uma *closure* definida inline quanto
uma função nomeada, vamos observar o uso do método `map` fornecido pela *trait*
`Iterator` na biblioteca padrão. Para usar o método `map` para transformar um
vetor de números em um vetor de strings, poderíamos usar uma *closure*, como na
Listagem 20-29.

<Listing number="20-29" caption="Usando uma *closure* com o método `map` para converter números em strings">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-29/src/main.rs:here}}
```

</Listing>

Ou podemos passar uma função como argumento para `map` no lugar da *closure*. A
Listagem 20-30 mostra como isso ficaria.

<Listing number="20-30" caption="Usando a função `String::to_string` com o método `map` para converter números em strings">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-30/src/main.rs:here}}
```

</Listing>

Note que precisamos usar a sintaxe totalmente qualificada sobre a qual falamos
na seção [“*Traits* Avançadas”][advanced-traits]<!-- ignore --> porque existem
várias funções disponíveis chamadas `to_string`.

Aqui, estamos usando a função `to_string` definida na *trait* `ToString`, que a
biblioteca padrão implementa para qualquer tipo que implemente `Display`.

Lembre-se da seção [“Valores de *Enum*”][enum-values]<!-- ignore --> no Capítulo
6 que o nome de cada variante de *enum* que definimos também se torna uma
função inicializadora. Podemos usar essas funções inicializadoras como ponteiros
de função que implementam as *traits* de *closure*, o que significa que podemos
passar as funções inicializadoras como argumentos para métodos que recebem
*closures*, como visto na Listagem 20-31.

<Listing number="20-31" caption="Usando um inicializador de *enum* com o método `map` para criar uma instância de `Status` a partir de números">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-31/src/main.rs:here}}
```

</Listing>

Aqui, criamos instâncias de `Status::Value` usando cada valor `u32` no
intervalo sobre o qual `map` é chamado, utilizando a função inicializadora de
`Status::Value`. Algumas pessoas preferem esse estilo e outras preferem usar
*closures*. Ambos compilam para o mesmo código, então use o estilo que for mais
claro para você.

<a id="returning-closures"></a>
### Retornando *Closures*

*Closures* são representadas por *traits*, o que significa que você não pode
retornar *closures* diretamente. Na maioria dos casos em que você gostaria de
retornar uma *trait*, você pode usar o tipo concreto que implementa a *trait*
como valor de retorno da função. No entanto, geralmente não é possível fazer
isso com *closures* porque elas não têm um tipo concreto que possa ser retornado;
você não tem permissão para usar o ponteiro de função `fn` como tipo de retorno
se a *closure* capturar quaisquer valores do seu escopo, por exemplo.

Em vez disso, você normalmente usará a sintaxe `impl Trait` que aprendemos no
Capítulo 10. Você pode retornar qualquer tipo de função usando `Fn`, `FnOnce` e
`FnMut`. Por exemplo, o código na Listagem 20-32 compila perfeitamente.

<Listing number="20-32" caption="Retornando uma *closure* de uma função usando a sintaxe `impl Trait`">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-32/src/lib.rs}}
```

</Listing>

Contudo, como observamos na seção [“Inferência e Anotação de Tipos de
*Closure*”][closure-types]<!-- ignore --> no Capítulo 13, cada *closure* também
é seu próprio tipo distinto. Se você precisar trabalhar com múltiplas funções
que têm a mesma assinatura, mas implementações diferentes, precisará usar um
*trait object* para elas. Considere o que acontece se você escrever um código
como o mostrado na Listagem 20-33.

<Listing file-name="src/main.rs" number="20-33" caption="Criando um `Vec<T>` de *closures* definidas por funções que retornam tipos `impl Fn`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-33/src/main.rs}}
```

</Listing>

Aqui temos duas funções, `returns_closure` e `returns_initialized_closure`, que
ambas retornam `impl Fn(i32) -> i32`. Note que as *closures* que elas retornam
são diferentes, mesmo implementando o mesmo tipo. Se tentarmos compilar isso, o
Rust nos avisa que não vai funcionar:

```text
{{#include ../listings/ch20-advanced-features/listing-20-33/output.txt}}
```

A mensagem de erro nos diz que, sempre que retornamos um `impl Trait`, o Rust
cria um *tipo opaco* (*opaque type*) único — um tipo cujos detalhes internos do
que o Rust constrói não podemos ver, nem podemos adivinhar para escrever por
conta própria. Portanto, embora essas funções retornem *closures* que
implementam a mesma *trait*, `Fn(i32) -> i32`, os tipos opacos que o Rust gera
para cada uma são distintos. (Isso é semelhante ao modo como o Rust produz tipos
concretos diferentes para blocos `async` distintos, mesmo quando têm o mesmo
tipo de saída, como vimos em [“O Tipo `Pin` e a *Trait*
`Unpin`”][future-types]<!-- ignore --> no Capítulo 17.) Já vimos uma solução
para esse problema algumas vezes: podemos usar um *trait object*, como na
Listagem 20-34.

<Listing number="20-34" caption="Criando um `Vec<T>` de *closures* definidas por funções que retornam `Box<dyn Fn>` para que tenham o mesmo tipo">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-34/src/main.rs:here}}
```

</Listing>

Este código compila sem problemas. Para saber mais sobre *trait objects*,
consulte a seção [“Usando *Trait Objects* para Abstrair Comportamentos
Compartilhados”][trait-objects]<!-- ignore --> no Capítulo 18.

A seguir, vamos dar uma olhada em *macros*!

[advanced-traits]: ch20-02-advanced-traits.html#advanced-traits
[enum-values]: ch06-01-defining-an-enum.html#enum-values
[closure-types]: ch13-01-closures.html#closure-type-inference-and-annotation
[future-types]: ch17-03-more-futures.html
[trait-objects]: ch18-02-trait-objects.html
