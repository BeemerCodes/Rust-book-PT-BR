<a id="advanced-types"></a>
## Tipos Avançados

O sistema de tipos do Rust possui alguns recursos que já mencionamos até aqui,
mas ainda não discutimos. Começaremos falando sobre *newtypes* em geral ao
examinarmos por que eles são úteis como tipos. Em seguida, passaremos para os
*type aliases* (sinônimos de tipo), um recurso semelhante aos *newtypes*, mas com
semântica ligeiramente diferente. Também discutiremos o tipo `!` e os tipos de
tamanho dinâmico.

<!-- Old headings. Do not remove or links may break. -->

<a id="using-the-newtype-pattern-for-type-safety-and-abstraction"></a>
<a id="type-safety-and-abstraction-with-the-newtype-pattern"></a>

### Segurança de Tipos e Abstração com o Padrão *Newtype*

Esta seção pressupõe que você leu a seção anterior [“Implementando *Traits*
Externas com o Padrão *Newtype*”][newtype]<!-- ignore -->. O padrão *newtype*
também é útil para tarefas além das que discutimos até agora, incluindo garantir
estaticamente que valores nunca sejam confundidos e indicar as unidades de um
valor. Você viu um exemplo de uso de *newtypes* para indicar unidades na
Listagem 20-16: lembre-se de que as *structs* `Millimeters` e `Meters` envolviam
valores `u32` em um *newtype*. Se escrevêssemos uma função com um parâmetro do
tipo `Millimeters`, não conseguiríamos compilar um programa que tentasse
acidentalmente chamar essa função com um valor do tipo `Meters` ou um `u32`
simples.

Também podemos usar o padrão *newtype* para abstrair alguns detalhes de
implementação de um tipo: o novo tipo pode expor uma API pública diferente da
API do tipo interno privado.

*Newtypes* também podem ocultar a implementação interna. Por exemplo, poderíamos
fornecer um tipo `People` para envolver um `HashMap<i32, String>` que armazena
o ID de uma pessoa associado ao seu nome. O código que usa `People` só
interagiria com a API pública que fornecemos, como um método para adicionar uma
string de nome à coleção `People`; esse código não precisaria saber que
atribuímos um ID `i32` aos nomes internamente. O padrão *newtype* é uma maneira
leve de alcançar o encapsulamento para ocultar detalhes de implementação, sobre
o qual discutimos na seção [“Encapsulamento que Oculta Detalhes de
Implementação”][encapsulation-that-hides-implementation-details]<!-- ignore -->
no Capítulo 18.

<!-- Old headings. Do not remove or links may break. -->

<a id="creating-type-synonyms-with-type-aliases"></a>
<a id="type-synonyms-and-type-aliases"></a>

### Sinônimos de Tipo e *Type Aliases*

O Rust oferece a capacidade de declarar um *type alias* (apelido de tipo) para
dar outro nome a um tipo existente. Para isso, usamos a palavra-chave `type`.
Por exemplo, podemos criar o apelido `Kilometers` para `i32` assim:

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-04-kilometers-alias/src/main.rs:here}}
```

Agora o apelido `Kilometers` é um *sinônimo* de `i32`; diferentemente dos tipos
`Millimeters` e `Meters` que criamos na Listagem 20-16, `Kilometers` não é um
novo tipo separado. Valores que têm o tipo `Kilometers` serão tratados da mesma
forma que valores do tipo `i32`:

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-04-kilometers-alias/src/main.rs:there}}
```

Como `Kilometers` e `i32` são o mesmo tipo, podemos somar valores de ambos os
tipos e passar valores `Kilometers` para funções que recebem parâmetros `i32`.
No entanto, usando esse método, não obtemos os benefícios de verificação de
tipos que temos com o padrão *newtype* discutido anteriormente. Em outras
palavras, se confundirmos valores `Kilometers` e `i32` em algum lugar, o
compilador não nos dará um erro.

O principal caso de uso para sinônimos de tipo é reduzir a repetição. Por
exemplo, podemos ter um tipo longo como este:

```rust,ignore
Box<dyn Fn() + Send + 'static>
```

Escrever esse tipo longo em assinaturas de função e como anotações de tipo por
todo o código pode ser cansativo e propenso a erros. Imagine ter um projeto cheio
de código como o da Listagem 20-25.

<Listing number="20-25" caption="Usando um tipo longo em muitos lugares">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-25/src/main.rs:here}}
```

</Listing>

Um *type alias* torna esse código mais gerenciável ao reduzir a repetição. Na
Listagem 20-26, introduzimos um apelido chamado `Thunk` para o tipo verboso e
podemos substituir todos os usos do tipo pelo apelido mais curto `Thunk`.

<Listing number="20-26" caption="Introduzindo um *type alias*, `Thunk`, para reduzir repetição">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-26/src/main.rs:here}}
```

</Listing>

Este código é muito mais fácil de ler e escrever! Escolher um nome significativo
para um *type alias* também pode ajudar a comunicar sua intenção (*thunk* é uma
palavra para código a ser avaliado em um momento posterior, portanto é um nome
apropriado para uma *closure* que é armazenada).

*Type aliases* também são comumente usados com o tipo `Result<T, E>` para
reduzir repetição. Considere o módulo `std::io` na biblioteca padrão. Operações
de E/S (I/O) frequentemente retornam um `Result<T, E>` para lidar com situações
em que as operações falham. Essa biblioteca tem uma *struct* `std::io::Error`
que representa todos os possíveis erros de E/S. Muitas das funções em `std::io`
retornarão `Result<T, E>` onde `E` é `std::io::Error`, como estas funções na
*trait* `Write`:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-05-write-trait/src/lib.rs}}
```

O `Result<..., Error>` se repete bastante. Por isso, `std::io` tem esta
declaração de *type alias*:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-06-result-alias/src/lib.rs:here}}
```

Como essa declaração está no módulo `std::io`, podemos usar o apelido totalmente
qualificado `std::io::Result<T>`; isto é, um `Result<T, E>` com o `E`
preenchido como `std::io::Error`. As assinaturas de função da *trait* `Write`
acabam ficando assim:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-06-result-alias/src/lib.rs:there}}
```

O *type alias* ajuda de duas maneiras: ele torna o código mais fácil de escrever
*e* nos dá uma interface consistente em todo o `std::io`. Por ser um apelido,
ele é apenas outro `Result<T, E>`, o que significa que podemos usar quaisquer
métodos que funcionam em `Result<T, E>` com ele, bem como sintaxe especial como
o operador `?`.

<a id="the-never-type-that-never-returns"></a>
### O Tipo *Never* Que Nunca Retorna

O Rust tem um tipo especial chamado `!` que é conhecido no jargão da teoria dos
tipos como o *tipo vazio* (*empty type*) porque não tem valores. Preferimos
chamá-lo de *tipo never* (*never type*) porque ele ocupa o lugar do tipo de
retorno quando uma função nunca retornará. Aqui está um exemplo:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-07-never-type/src/lib.rs:here}}
```

Este código é lido como “a função `bar` retorna *never* (nunca).” Funções que
nunca retornam são chamadas de *funções divergentes* (*diverging functions*).
Não podemos criar valores do tipo `!`, então `bar` nunca poderá retornar.

Mas qual é a utilidade de um tipo para o qual você nunca pode criar valores?
Lembre-se do código da Listagem 2-5, parte do jogo de adivinhação; reproduzimos
um trecho dele aqui na Listagem 20-27.

<Listing number="20-27" caption="Um `match` com um braço que termina em `continue`">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:ch19}}
```

</Listing>

Na época, pulamos alguns detalhes nesse código. Na seção [“A Construção de
Fluxo de Controle `match`”][the-match-control-flow-construct]<!-- ignore --> no
Capítulo 6, discutimos que todos os braços de um `match` devem retornar o mesmo
tipo. Então, por exemplo, o código a seguir não funciona:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-08-match-arms-different-types/src/main.rs:here}}
```

O tipo de `guess` neste código teria que ser um inteiro *e* uma string, e o
Rust exige que `guess` tenha apenas um tipo. Então, o que `continue` retorna?
Como tivemos permissão para retornar um `u32` de um braço e ter outro braço que
termina com `continue` na Listagem 20-27?

Como você já deve ter imaginado, `continue` tem um valor `!`. Ou seja, quando o
Rust calcula o tipo de `guess`, ele olha para ambos os braços do *match*, o
primeiro com um valor de `u32` e o segundo com um valor `!`. Como `!` nunca
pode ter um valor, o Rust decide que o tipo de `guess` é `u32`.

A maneira formal de descrever esse comportamento é que expressões do tipo `!`
podem sofrer coerção para qualquer outro tipo. Temos permissão para terminar
esse braço do `match` com `continue` porque `continue` não retorna um valor; em
vez disso, ele move o controle de volta para o topo do loop, então no caso
`Err`, nunca atribuímos um valor a `guess`.

O tipo *never* também é útil com a *macro* `panic!`. Lembre-se da função
`unwrap` que chamamos em valores `Option<T>` para produzir um valor ou entrar em
pânico com esta definição:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-09-unwrap-definition/src/lib.rs:here}}
```

Neste código, acontece a mesma coisa que no `match` da Listagem 20-27: o Rust
vê que `val` tem o tipo `T` e `panic!` tem o tipo `!`, então o resultado da
expressão `match` geral é `T`. Este código funciona porque `panic!` não produz
um valor; ele encerra o programa. No caso `None`, não retornaremos um valor de
`unwrap`, então este código é válido.

Uma última expressão que tem o tipo `!` é um loop:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-10-loop-returns-never/src/main.rs:here}}
```

Aqui, o loop nunca termina, então `!` é o valor da expressão. No entanto, isso
não seria verdade se incluíssemos um `break`, pois o loop terminaria quando
chegasse ao `break`.

<a id="dynamically-sized-types-and-the-sized-trait"></a>
### Tipos de Tamanho Dinâmico e a *Trait* `Sized`

O Rust precisa saber certos detalhes sobre seus tipos, como quanto espaço alocar
para um valor de um determinado tipo. Isso deixa um canto do seu sistema de
tipos um pouco confuso no começo: o conceito de *tipos de tamanho dinâmico*
(*dynamically sized types*). Às vezes chamados de *DSTs* ou *tipos sem tamanho*
(*unsized types*), esses tipos nos permitem escrever código usando valores cujo
tamanho só podemos conhecer em tempo de execução.

Vamos mergulhar nos detalhes de um tipo de tamanho dinâmico chamado `str`, que
temos usado ao longo do livro. Isso mesmo: não `&str`, mas `str` por si só, é um
DST. Em muitos casos, como ao armazenar texto digitado por um usuário, não
podemos saber qual é o comprimento da string até o tempo de execução. Isso
significa que não podemos criar uma variável do tipo `str`, nem podemos receber
um argumento do tipo `str`. Considere o seguinte código, que não funciona:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-11-cant-create-str/src/main.rs:here}}
```

O Rust precisa saber quanta memória alocar para qualquer valor de um determinado
tipo, e todos os valores de um tipo devem usar a mesma quantidade de memória.
Se o Rust nos permitisse escrever este código, esses dois valores `str`
precisariam ocupar a mesma quantidade de espaço. Mas eles têm comprimentos
diferentes: `s1` precisa de 12 bytes de armazenamento e `s2` precisa de 15. É
por isso que não é possível criar uma variável contendo um tipo de tamanho
dinâmico.

Então, o que fazemos? Neste caso, você já sabe a resposta: fazemos com que o
tipo de `s1` e `s2` seja um *slice* de string (`&str`) em vez de `str`.
Lembre-se da seção [“*Slices* de String”][string-slices]<!-- ignore --> no
Capítulo 4 que a estrutura de dados *slice* armazena apenas a posição inicial e
o comprimento do *slice*. Assim, embora `&T` seja um único valor que armazena o
endereço de memória onde `T` está localizado, um *slice* de string são *dois*
valores: o endereço do `str` e seu comprimento. Dessa forma, podemos saber o
tamanho de um valor *slice* de string em tempo de compilação: é o dobro do
comprimento de um `usize`. Isto é, sempre sabemos o tamanho de um *slice* de
string, não importa quão longa seja a string à qual ele se refere. Em geral,
essa é a maneira pela qual tipos de tamanho dinâmico são usados em Rust: eles
têm um pouco de metadados extras que armazenam o tamanho da informação
dinâmica. A regra de ouro dos tipos de tamanho dinâmico é que devemos sempre
colocar valores de tipos de tamanho dinâmico atrás de algum tipo de ponteiro.

Podemos combinar `str` com todos os tipos de ponteiros: por exemplo, `Box<str>`
ou `Rc<str>`. Na verdade, você já viu isso antes, mas com um tipo de tamanho
dinâmico diferente: *traits*. Toda *trait* é um tipo de tamanho dinâmico ao qual
podemos nos referir usando o nome da *trait*. Na seção [“Usando *Trait Objects*
para Abstrair Comportamentos Compartilhados”][using-trait-objects-to-abstract-over-shared-behavior]<!--
ignore --> no Capítulo 18, mencionamos que, para usar *traits* como *trait
objects*, precisamos colocá-las atrás de um ponteiro, como `&dyn Trait` ou
`Box<dyn Trait>` (`Rc<dyn Trait>` também funcionaria).

Para trabalhar com DSTs, o Rust fornece a *trait* `Sized` para determinar se o
tamanho de um tipo é conhecido em tempo de compilação ou não. Essa *trait* é
implementada automaticamente para tudo cujo tamanho seja conhecido em tempo de
compilação. Além disso, o Rust adiciona implicitamente uma restrição (*bound*)
em `Sized` a toda função genérica. Ou seja, uma definição de função genérica
como esta:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-12-generic-fn-definition/src/lib.rs}}
```

na verdade é tratada como se tivéssemos escrito isto:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-13-generic-implicit-sized-bound/src/lib.rs}}
```

Por padrão, funções genéricas funcionarão apenas com tipos que têm um tamanho
conhecido em tempo de compilação. No entanto, você pode usar a seguinte sintaxe
especial para relaxar essa restrição:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-14-generic-maybe-sized/src/lib.rs}}
```

Um *trait bound* em `?Sized` significa “`T` pode ou não ser `Sized`”, e essa
notação substitui o padrão de que tipos genéricos devem ter um tamanho conhecido
em tempo de compilação. A sintaxe `?Trait` com esse significado só está
disponível para `Sized`, não para quaisquer outras *traits*.

Observe também que mudamos o tipo do parâmetro `t` de `T` para `&T`. Como o tipo
pode não ser `Sized`, precisamos usá-lo atrás de algum tipo de ponteiro. Neste
caso, escolhemos uma referência.

A seguir, falaremos sobre funções e *closures*!

[encapsulation-that-hides-implementation-details]: ch18-01-what-is-oo.html#encapsulation-that-hides-implementation-details
[string-slices]: ch04-03-slices.html#string-slices
[the-match-control-flow-construct]: ch06-02-match.html#the-match-control-flow-construct
[using-trait-objects-to-abstract-over-shared-behavior]: ch18-02-trait-objects.html#using-trait-objects-to-abstract-over-shared-behavior
[newtype]: ch20-02-advanced-traits.html#implementing-external-traits-with-the-newtype-pattern
