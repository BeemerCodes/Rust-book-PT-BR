<a id="bringing-paths-into-scope-with-the-use-keyword"></a>
## Trazendo Caminhos para o Escopo com a Palavra-chave `use`

Ter que escrever os caminhos completos para chamar funções pode parecer
inconveniente e repetitivo. Na Listagem 7-7, independentemente de escolhermos o
caminho absoluto ou relativo para a função `add_to_waitlist`, toda vez que
queríamos chamar `add_to_waitlist` precisávamos especificar `front_of_house` e
`hosting` também. Felizmente, existe uma maneira de simplificar esse processo:
podemos criar um atalho para um caminho com a palavra-chave `use` uma única vez
e depois usar o nome mais curto em todo o restante do escopo.

Na Listagem 7-11, trazemos o módulo `crate::front_of_house::hosting` para o
escopo da função `eat_at_restaurant`, de modo que só precisamos especificar
`hosting::add_to_waitlist` para chamar a função `add_to_waitlist` em
`eat_at_restaurant`.

<Listing number="7-11" file-name="src/lib.rs" caption="Trazendo um módulo para o escopo com `use`">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-11/src/lib.rs}}
```

</Listing>

Adicionar `use` e um caminho em um escopo é semelhante a criar um link simbólico
no sistema de arquivos. Ao adicionar `use crate::front_of_house::hosting` na
raiz do *crate*, `hosting` passa a ser um nome válido naquele escopo, exatamente
como se o módulo `hosting` tivesse sido definido na raiz do *crate*. Caminhos
trazidos para o escopo com `use` também verificam a privacidade, como quaisquer
outros caminhos.

Note que `use` cria o atalho apenas para o escopo específico em que o `use`
ocorre. A Listagem 7-12 move a função `eat_at_restaurant` para um novo módulo
filho chamado `customer`, que passa a ser um escopo diferente da declaração
`use`, fazendo com que o corpo da função não compile.

<Listing number="7-12" file-name="src/lib.rs" caption="Uma declaração `use` se aplica apenas ao escopo em que está.">

```rust,noplayground,test_harness,does_not_compile,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-12/src/lib.rs}}
```

</Listing>

O erro do compilador mostra que o atalho não se aplica mais dentro do módulo
`customer`:

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-12/output.txt}}
```

Observe que também há um aviso de que o `use` não está mais sendo usado em seu
escopo! Para corrigir esse problema, mova o `use` para dentro do módulo
`customer` também, ou referencie o atalho no módulo pai com `super::hosting`
dentro do módulo filho `customer`.

<a id="creating-idiomatic-use-paths"></a>
### Criando Caminhos `use` Idiomáticos

Na Listagem 7-11, você pode ter se perguntado por que especificamos `use
crate::front_of_house::hosting` e depois chamamos `hosting::add_to_waitlist` em
`eat_at_restaurant`, em vez de especificar o caminho do `use` até a função
`add_to_waitlist` para alcançar o mesmo resultado, como na Listagem 7-13.

<Listing number="7-13" file-name="src/lib.rs" caption="Trazendo a função `add_to_waitlist` para o escopo com `use`, o que não é idiomático">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-13/src/lib.rs}}
```

</Listing>

Embora tanto a Listagem 7-11 quanto a Listagem 7-13 realizem a mesma tarefa, a
Listagem 7-11 é a forma idiomática de trazer uma função para o escopo com `use`.
Trazer o módulo pai da função para o escopo com `use` significa que precisamos
especificar o módulo pai ao chamar a função. Especificar o módulo pai ao chamar
a função deixa claro que ela não foi definida localmente, ao mesmo tempo em que
minimiza a repetição do caminho completo. O código na Listagem 7-13 não deixa
claro onde `add_to_waitlist` está definida.

Por outro lado, ao trazer *structs*, *enums* e outros itens com `use`, é
idiomático especificar o caminho completo. A Listagem 7-14 mostra a forma
idiomática de trazer a *struct* `HashMap` da biblioteca padrão para o escopo de
um *crate* binário.

<Listing number="7-14" file-name="src/main.rs" caption="Trazendo `HashMap` para o escopo de forma idiomática">

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-14/src/main.rs}}
```

</Listing>

Não há um motivo forte por trás desse idioma: é apenas a convenção que surgiu, e
as pessoas se acostumaram a ler e escrever código Rust dessa maneira.

A exceção a esse idioma é quando estamos trazendo dois itens com o mesmo nome
para o escopo com declarações `use`, pois o Rust não permite isso. A Listagem
7-15 mostra como trazer dois tipos `Result` para o escopo que têm o mesmo nome,
mas módulos pai diferentes, e como se referir a eles.

<Listing number="7-15" file-name="src/lib.rs" caption="Trazer dois tipos com o mesmo nome para o mesmo escopo exige usar seus módulos pai.">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-15/src/lib.rs:here}}
```

</Listing>

Como você pode ver, usar os módulos pai distingue os dois tipos `Result`. Se em
vez disso especificássemos `use std::fmt::Result` e `use std::io::Result`,
teríamos dois tipos `Result` no mesmo escopo, e o Rust não saberia a qual deles
estávamos nos referindo ao usar `Result`.

<a id="providing-new-names-with-the-as-keyword"></a>
### Fornecendo Novos Nomes com a Palavra-chave `as`

Existe outra solução para o problema de trazer dois tipos com o mesmo nome para
o mesmo escopo com `use`: após o caminho, podemos especificar `as` e um novo
nome local, ou _alias_, para o tipo. A Listagem 7-16 mostra outra forma de
escrever o código da Listagem 7-15 renomeando um dos dois tipos `Result` com
`as`.

<Listing number="7-16" file-name="src/lib.rs" caption="Renomeando um tipo ao trazê-lo para o escopo com a palavra-chave `as`">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-16/src/lib.rs:here}}
```

</Listing>

Na segunda declaração `use`, escolhemos o novo nome `IoResult` para o tipo
`std::io::Result`, que não entrará em conflito com o `Result` de `std::fmt` que
também trouxemos para o escopo. Tanto a Listagem 7-15 quanto a Listagem 7-16 são
consideradas idiomáticas, então a escolha é sua!

<a id="re-exporting-names-with-pub-use"></a>
### Reexportando Nomes com `pub use`

Quando trazemos um nome para o escopo com a palavra-chave `use`, esse nome é
privado para o escopo no qual o importamos. Para permitir que código fora desse
escopo se refira a esse nome como se ele tivesse sido definido ali, podemos
combinar `pub` e `use`. Essa técnica é chamada de _reexportação_
(*re-exporting*), porque estamos trazendo um item para o escopo e, ao mesmo
tempo, tornando-o disponível para que outros o tragam para seus próprios
escopos.

A Listagem 7-17 mostra o código da Listagem 7-11 com o `use` no módulo raiz
alterado para `pub use`.

<Listing number="7-17" file-name="src/lib.rs" caption="Tornando um nome disponível para qualquer código usar a partir de um novo escopo com `pub use`">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-17/src/lib.rs}}
```

</Listing>

Antes dessa mudança, código externo teria que chamar a função `add_to_waitlist`
usando o caminho `restaurant::front_of_house::hosting::add_to_waitlist()`, o que
também exigiria que o módulo `front_of_house` fosse marcado como `pub`. Agora
que esse `pub use` reexportou o módulo `hosting` a partir do módulo raiz, código
externo pode usar o caminho `restaurant::hosting::add_to_waitlist()`.

A reexportação é útil quando a estrutura interna do seu código é diferente da
forma como os programadores que chamam seu código pensam sobre o domínio. Por
exemplo, nessa metáfora do restaurante, as pessoas que administram o restaurante
pensam em “front of house” e “back of house”. Mas os clientes que visitam um
restaurante provavelmente não pensam nas partes do restaurante nesses termos.
Com `pub use`, podemos escrever nosso código com uma estrutura, mas expor uma
estrutura diferente. Fazer isso deixa nossa biblioteca bem organizada tanto para
quem trabalha nela quanto para quem a utiliza. Veremos outro exemplo de `pub
use` e como ele afeta a documentação do seu *crate* em [“Exportando uma API
Pública Conveniente”][ch14-pub-use]<!-- ignore --> no Capítulo 14.

<a id="using-external-packages"></a>
### Usando Packages Externos

No Capítulo 2, programamos um projeto de jogo de adivinhação que usava um
*package* externo chamado `rand` para gerar números aleatórios. Para usar `rand`
no nosso projeto, adicionamos esta linha ao _Cargo.toml_:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:

* ch01-01-installation.md
* ch02-00-guessing-game-tutorial.md
* ch14-03-cargo-workspaces.md
-->

<Listing file-name="Cargo.toml">

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:9:}}
```

</Listing>

Adicionar `rand` como dependência no _Cargo.toml_ diz ao Cargo para baixar o
*package* `rand` e quaisquer dependências dele do [crates.io](https://crates.io/)
e tornar `rand` disponível para o nosso projeto.

Então, para trazer as definições de `rand` para o escopo do nosso *package*,
adicionamos uma linha `use` começando com o nome do *crate*, `rand`, e listamos
os itens que queríamos trazer para o escopo. Lembre-se de que em [“Gerando um
Número Aleatório”][rand]<!-- ignore --> no Capítulo 2, trouxemos itens do módulo
`rand::prelude` para o escopo e chamamos a função `rand::rng`:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:ch07-04}}
```

Membros da comunidade Rust disponibilizaram muitos *packages* no
[crates.io](https://crates.io/), e adicionar qualquer um deles ao seu *package*
envolve esses mesmos passos: listá-los no arquivo _Cargo.toml_ do seu *package*
e usar `use` para trazer itens dos seus *crates* para o escopo.

Note que a biblioteca padrão `std` também é um *crate* externo ao nosso
*package*. Como a biblioteca padrão já vem junto com a linguagem Rust, não
precisamos alterar o _Cargo.toml_ para incluir `std`. Mas precisamos nos referir
a ela com `use` para trazer itens dela para o escopo do nosso *package*. Por
exemplo, com `HashMap` usaríamos esta linha:

```rust
use std::collections::HashMap;
```

Esse é um caminho absoluto começando com `std`, o nome do *crate* da biblioteca
padrão.

<!-- Old headings. Do not remove or links may break. -->

<a id="using-nested-paths-to-clean-up-large-use-lists"></a>

<a id="using-nested-paths-to-clean-up-use-lists"></a>
### Usando Caminhos Aninhados para Limpar Listas de `use`

Se estivermos usando vários itens definidos no mesmo *crate* ou no mesmo módulo,
listar cada item em sua própria linha pode ocupar muito espaço vertical nos
nossos arquivos. Por exemplo, estas duas declarações `use` que tínhamos no jogo
de adivinhação na Listagem 2-4 trazem itens de `std` para o escopo:

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-01-use-std-unnested/src/main.rs:here}}
```

</Listing>

Em vez disso, podemos usar caminhos aninhados para trazer os mesmos itens para o
escopo em uma única linha. Fazemos isso especificando a parte comum do caminho,
seguida por dois pontos duplos e, em seguida, chaves ao redor de uma lista das
partes dos caminhos que diferem, conforme mostrado na Listagem 7-18.

<Listing number="7-18" file-name="src/main.rs" caption="Especificando um caminho aninhado para trazer vários itens com o mesmo prefixo para o escopo">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-18/src/main.rs:here}}
```

</Listing>

Em programas maiores, trazer muitos itens para o escopo do mesmo *crate* ou
módulo usando caminhos aninhados pode reduzir bastante o número de declarações
`use` separadas necessárias!

Podemos usar um caminho aninhado em qualquer nível de um caminho, o que é útil
ao combinar duas declarações `use` que compartilham um subcaminho. Por exemplo,
a Listagem 7-19 mostra duas declarações `use`: uma que traz `std::io` para o
escopo e outra que traz `std::io::Write` para o escopo.

<Listing number="7-19" file-name="src/lib.rs" caption="Duas declarações `use` em que uma é um subcaminho da outra">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-19/src/lib.rs}}
```

</Listing>

A parte comum desses dois caminhos é `std::io`, e esse é o primeiro caminho
completo. Para mesclar esses dois caminhos em uma única declaração `use`,
podemos usar `self` no caminho aninhado, conforme mostrado na Listagem 7-20.

<Listing number="7-20" file-name="src/lib.rs" caption="Combinando os caminhos da Listagem 7-19 em uma única declaração `use`">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-20/src/lib.rs}}
```

</Listing>

Essa linha traz `std::io` e `std::io::Write` para o escopo.

<!-- Old headings. Do not remove or links may break. -->

<a id="the-glob-operator"></a>

<a id="importing-items-with-the-glob-operator"></a>
### Importando Itens com o Operador Glob

Se quisermos trazer _todos_ os itens públicos definidos em um caminho para o
escopo, podemos especificar esse caminho seguido pelo operador glob `*`:

```rust
use std::collections::*;
```

Essa declaração `use` traz todos os itens públicos definidos em
`std::collections` para o escopo atual. Tenha cuidado ao usar o operador glob! O
glob pode dificultar saber quais nomes estão em escopo e onde um nome usado no
seu programa foi definido. Além disso, se a dependência alterar suas definições,
o que você importou também muda, o que pode levar a erros de compilação ao
atualizar a dependência caso ela adicione uma definição com o mesmo nome de uma
definição sua no mesmo escopo, por exemplo.

O operador glob é frequentemente usado ao escrever testes para trazer tudo o que
está sendo testado para o módulo `tests`; falaremos sobre isso em [“Como
Escrever Testes”][writing-tests]<!-- ignore --> no Capítulo 11. O operador glob
também é usado às vezes como parte do padrão *prelude*: consulte [a documentação
da biblioteca padrão](../std/prelude/index.html#other-preludes)<!-- ignore -->
para mais informações sobre esse padrão.

[ch14-pub-use]: ch14-02-publishing-to-crates-io.html#exporting-a-convenient-public-api
[rand]: ch02-00-guessing-game-tutorial.html#generating-a-random-number
[writing-tests]: ch11-01-writing-tests.html#how-to-write-tests
