<a id="data-types"></a>
## Tipos de Dados

Todo valor em Rust possui um determinado _tipo de dados_, que informa ao Rust que tipo de dado está sendo especificado para que ele saiba como trabalhar com esse dado. Veremos dois subconjuntos de tipos de dados: escalares e compostos.

Tenha em mente que o Rust é uma linguagem _estaticamente tipada_, o que significa que ele precisa conhecer os tipos de todas as variáveis em tempo de compilação. O compilador geralmente consegue inferir qual tipo queremos usar com base no valor e em como o utilizamos. Nos casos em que vários tipos são possíveis, como quando convertemos uma `String` para um tipo numérico usando `parse` na seção ["Comparando o Palpite com o Número Secreto"][comparing-the-guess-to-the-secret-number]<!-- ignore --> no Capítulo 2, precisamos adicionar uma anotação de tipo, assim:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

Se não adicionarmos a anotação de tipo `: u32` mostrada no código anterior, o Rust exibirá o seguinte erro, que significa que o compilador precisa de mais informações nossas para saber qual tipo queremos usar:

```console
{{#include ../listings/ch03-common-programming-concepts/output-only-01-no-type-annotations/output.txt}}
```

Você verá diferentes anotações de tipo para outros tipos de dados.

<a id="scalar-types"></a>
### Tipos Escalares

Um tipo _escalar_ representa um único valor. O Rust tem quatro tipos escalares primários: inteiros, números de ponto flutuante, booleanos e caracteres. Você provavelmente já conhece esses tipos de outras linguagens de programação. Vamos ver como eles funcionam no Rust.

<a id="integer-types"></a>
#### Tipos Inteiros

Um _inteiro_ é um número sem parte fracionária. Usamos um tipo inteiro no Capítulo 2, o tipo `u32`. Essa declaração de tipo indica que o valor associado deve ser um inteiro sem sinal (tipos inteiros com sinal começam com `i` em vez de `u`) que ocupa 32 bits de espaço. A Tabela 3-1 mostra os tipos inteiros embutidos no Rust. Podemos usar qualquer uma dessas variantes para declarar o tipo de um valor inteiro.

<span class="caption">Tabela 3-1: Tipos Inteiros em Rust</span>

| Tamanho | Com sinal | Sem sinal |
| ------- | --------- | --------- |
| 8 bits  | `i8`      | `u8`      |
| 16 bits | `i16`     | `u16`     |
| 32 bits | `i32`     | `u32`     |
| 64 bits | `i64`     | `u64`     |
| 128 bits| `i128`    | `u128`    |
| Dependente da arquitetura | `isize` | `usize` |

Cada variante pode ser com sinal ou sem sinal e tem um tamanho explícito. _Com sinal_ (*signed*) e _sem sinal_ (*unsigned*) referem-se à possibilidade de o número ser negativo — em outras palavras, se o número precisa ter um sinal junto dele (com sinal) ou se será sempre positivo e, portanto, pode ser representado sem sinal (sem sinal). É como escrever números no papel: quando o sinal importa, o número aparece com um sinal de mais ou de menos; porém, quando é seguro assumir que o número é positivo, ele aparece sem sinal. Números com sinal são armazenados usando a representação em [complemento de dois][twos-complement]<!-- ignore -->.

Cada variante com sinal pode armazenar números de −(2<sup>n − 1</sup>) até 2<sup>n − 1</sup> − 1 inclusive, onde _n_ é o número de bits que essa variante usa. Assim, um `i8` pode armazenar números de −(2<sup>7</sup>) até 2<sup>7</sup> − 1, o que equivale a −128 até 127. Variantes sem sinal podem armazenar números de 0 até 2<sup>n</sup> − 1, então um `u8` pode armazenar números de 0 até 2<sup>8</sup> − 1, o que equivale a 0 até 255.

Além disso, os tipos `isize` e `usize` dependem da arquitetura do computador em que seu programa está rodando: 64 bits se você estiver em uma arquitetura de 64 bits e 32 bits se estiver em uma arquitetura de 32 bits.

Você pode escrever literais inteiros em qualquer uma das formas mostradas na Tabela 3-2. Note que literais numéricos que podem ser de múltiplos tipos numéricos permitem um sufixo de tipo, como `57u8`, para designar o tipo. Literais numéricos também podem usar `_` como separador visual para facilitar a leitura do número, como `1_000`, que terá o mesmo valor que `1000`.

<span class="caption">Tabela 3-2: Literais Inteiros em Rust</span>

| Literais numéricos  | Exemplo       |
| ------------------- | ------------- |
| Decimal             | `98_222`      |
| Hexadecimal         | `0xff`        |
| Octal               | `0o77`        |
| Binário             | `0b1111_0000` |
| Byte (apenas `u8`)  | `b'A'`        |

Então, como saber qual tipo de inteiro usar? Se você estiver em dúvida, os padrões do Rust geralmente são um bom ponto de partida: tipos inteiros têm `i32` como padrão. A principal situação em que você usaria `isize` ou `usize` é ao indexar algum tipo de coleção.

> ##### Estouro de Inteiro (*Integer Overflow*)
>
> Digamos que você tenha uma variável do tipo `u8` que pode guardar valores entre 0 e 255. Se você tentar alterar a variável para um valor fora desse intervalo, como 256, ocorrerá um _estouro de inteiro_ (*integer overflow*), o que pode resultar em um de dois comportamentos. Quando você compila em modo debug, o Rust inclui verificações de estouro de inteiro que fazem seu programa entrar em _pânico_ (*panic*) em tempo de execução se esse comportamento ocorrer. O Rust usa o termo _panicking_ quando um programa encerra com erro; discutiremos pânicos em mais detalhes na seção ["Erros Irrecuperáveis com `panic!`"][unrecoverable-errors-with-panic]<!-- ignore --> no Capítulo 9.
>
> Quando você compila em modo release com a flag `--release`, o Rust _não_ inclui verificações de estouro de inteiro que causam pânico. Em vez disso, se ocorrer estouro, o Rust realiza o _wrapping em complemento de dois_. Em resumo, valores maiores que o valor máximo que o tipo pode conter "dão a volta" para o valor mínimo do tipo. No caso de um `u8`, o valor 256 vira 0, o valor 257 vira 1, e assim por diante. O programa não entrará em pânico, mas a variável terá um valor que provavelmente não é o que você esperava. Confiar no comportamento de *wrapping* do estouro de inteiro é considerado um erro.
>
> Para lidar explicitamente com a possibilidade de estouro, você pode usar estas famílias de métodos fornecidas pela biblioteca padrão para tipos numéricos primitivos:
>
> - Fazer *wrapping* em todos os modos de compilação com os métodos `wrapping_*`, como `wrapping_add`.
> - Retornar o valor `None` se houver estouro com os métodos `checked_*`.
> - Retornar o valor e um booleano indicando se houve estouro com os métodos `overflowing_*`.
> - Saturar nos valores mínimo ou máximo do tipo com os métodos `saturating_*`.

<a id="floating-point-types"></a>
#### Tipos de Ponto Flutuante

O Rust também tem dois tipos primitivos para _números de ponto flutuante_, que são números com casas decimais. Os tipos de ponto flutuante do Rust são `f32` e `f64`, que têm 32 bits e 64 bits de tamanho, respectivamente. O tipo padrão é `f64` porque, em CPUs modernas, ele tem praticamente a mesma velocidade que `f32`, mas é capaz de oferecer mais precisão. Todos os tipos de ponto flutuante são com sinal.

Aqui está um exemplo que mostra números de ponto flutuante em ação:

<span class="filename">Arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-06-floating-point/src/main.rs}}
```

Números de ponto flutuante são representados de acordo com o padrão IEEE-754.

<a id="numeric-operations"></a>
#### Operações Numéricas

O Rust suporta as operações matemáticas básicas que você esperaria para todos os tipos numéricos: adição, subtração, multiplicação, divisão e resto. A divisão inteira trunca em direção a zero para o inteiro mais próximo. O código a seguir mostra como você usaria cada operação numérica em uma instrução `let`:

<span class="filename">Arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-07-numeric-operations/src/main.rs}}
```

Cada expressão nessas instruções usa um operador matemático e é avaliada para um único valor, que é então vinculado a uma variável. O [Apêndice B][appendix_b]<!-- ignore --> contém uma lista de todos os operadores que o Rust fornece.

<a id="the-boolean-type"></a>
#### O Tipo Booleano

Como na maioria das outras linguagens de programação, um tipo booleano em Rust tem dois valores possíveis: `true` e `false`. Booleanos têm um byte de tamanho. O tipo booleano em Rust é especificado usando `bool`. Por exemplo:

<span class="filename">Arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-08-boolean/src/main.rs}}
```

A principal maneira de usar valores booleanos é por meio de condicionais, como uma expressão `if`. Veremos como as expressões `if` funcionam em Rust na seção ["Fluxo de Controle"][control-flow]<!-- ignore -->.

<a id="the-character-type"></a>
#### O Tipo Caractere

O tipo `char` do Rust é o tipo alfabético mais primitivo da linguagem. Aqui estão alguns exemplos de declaração de valores `char`:

<span class="filename">Arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-09-char/src/main.rs}}
```

Note que especificamos literais `char` com aspas simples, ao contrário de literais de string, que usam aspas duplas. O tipo `char` do Rust tem 4 bytes de tamanho e representa um valor escalar Unicode, o que significa que pode representar muito mais do que apenas ASCII. Letras acentuadas; caracteres chineses, japoneses e coreanos; emojis; e espaços de largura zero são todos valores `char` válidos em Rust. Os valores escalares Unicode vão de `U+0000` a `U+D7FF` e de `U+E000` a `U+10FFFF` inclusive. No entanto, um "caractere" não é exatamente um conceito em Unicode, então sua intuição humana sobre o que é um "caractere" pode não coincidir com o que é um `char` em Rust. Discutiremos esse tópico em detalhes em ["Armazenando Texto Codificado em UTF-8 com Strings"][strings]<!-- ignore --> no Capítulo 8.

<a id="compound-types"></a>
### Tipos Compostos

_Tipos compostos_ podem agrupar múltiplos valores em um único tipo. O Rust tem dois tipos compostos primitivos: tuplas e arrays.

<a id="the-tuple-type"></a>
#### O Tipo Tupla

Uma _tupla_ é uma maneira geral de agrupar vários valores com uma variedade de tipos em um único tipo composto. Tuplas têm tamanho fixo: uma vez declaradas, não podem crescer nem encolher.

Criamos uma tupla escrevendo uma lista de valores separados por vírgula dentro de parênteses. Cada posição na tupla tem um tipo, e os tipos dos diferentes valores na tupla não precisam ser iguais. Adicionamos anotações de tipo opcionais neste exemplo:

<span class="filename">Arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-10-tuples/src/main.rs}}
```

A variável `tup` se vincula à tupla inteira, pois uma tupla é considerada um único elemento composto. Para extrair os valores individuais de uma tupla, podemos usar *pattern matching* para desestruturar o valor da tupla, assim:

<span class="filename">Arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-11-destructuring-tuples/src/main.rs}}
```

Este programa primeiro cria uma tupla e a vincula à variável `tup`. Em seguida, usa um padrão com `let` para pegar `tup` e transformá-la em três variáveis separadas: `x`, `y` e `z`. Isso é chamado de _desestruturação_, porque quebra a tupla única em três partes. Por fim, o programa imprime o valor de `y`, que é `6.4`.

Também podemos acessar um elemento da tupla diretamente usando um ponto (`.`) seguido pelo índice do valor que queremos acessar. Por exemplo:

<span class="filename">Arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-12-tuple-indexing/src/main.rs}}
```

Este programa cria a tupla `x` e depois acessa cada elemento da tupla usando seus respectivos índices. Como na maioria das linguagens de programação, o primeiro índice em uma tupla é 0.

A tupla sem nenhum valor tem um nome especial: _unit_. Esse valor e seu tipo correspondente são ambos escritos como `()` e representam um valor vazio ou um tipo de retorno vazio. Expressões retornam implicitamente o valor *unit* se não retornarem nenhum outro valor.

<a id="the-array-type"></a>
#### O Tipo Array

Outra forma de ter uma coleção de múltiplos valores é com um _array_. Diferente de uma tupla, todo elemento de um array deve ter o mesmo tipo. Diferente dos arrays em algumas outras linguagens, os arrays em Rust têm tamanho fixo.

Escrevemos os valores em um array como uma lista separada por vírgulas dentro de colchetes:

<span class="filename">Arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-13-arrays/src/main.rs}}
```

Arrays são úteis quando você quer seus dados alocados na *stack*, assim como os outros tipos que vimos até agora, em vez da *heap* (discutiremos mais sobre *stack* e *heap* no [Capítulo 4][stack-and-heap]<!-- ignore -->), ou quando você quer garantir que sempre terá um número fixo de elementos. Um array não é tão flexível quanto o tipo vetor, porém. Um vetor é um tipo de coleção semelhante fornecido pela biblioteca padrão que _pode_ crescer ou encolher de tamanho porque seu conteúdo vive na *heap*. Se você não tiver certeza se deve usar um array ou um vetor, provavelmente deve usar um vetor. O [Capítulo 8][vectors]<!-- ignore --> discute vetores em mais detalhes.

No entanto, arrays são mais úteis quando você sabe que o número de elementos não precisará mudar. Por exemplo, se você estivesse usando os nomes dos meses em um programa, provavelmente usaria um array em vez de um vetor porque sabe que ele sempre conterá 12 elementos:

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

Você escreve o tipo de um array usando colchetes com o tipo de cada elemento, um ponto e vírgula e, em seguida, o número de elementos no array, assim:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Aqui, `i32` é o tipo de cada elemento. Após o ponto e vírgula, o número `5` indica que o array contém cinco elementos.

Você também pode inicializar um array para conter o mesmo valor em cada elemento especificando o valor inicial, seguido de um ponto e vírgula e o tamanho do array entre colchetes, como mostrado aqui:

```rust
let a = [3; 5];
```

O array chamado `a` conterá `5` elementos que serão todos definidos inicialmente com o valor `3`. Isso é o mesmo que escrever `let a = [3, 3, 3, 3, 3];`, mas de forma mais concisa.

<!-- Old headings. Do not remove or links may break. -->
<a id="accessing-array-elements"></a>

<a id="array-element-access"></a>
#### Acesso a Elementos do Array

Um array é um único bloco de memória de tamanho conhecido e fixo que pode ser alocado na *stack*. Você pode acessar elementos de um array usando indexação, assim:

<span class="filename">Arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-14-array-indexing/src/main.rs}}
```

Neste exemplo, a variável chamada `first` receberá o valor `1`, porque esse é o valor no índice `[0]` do array. A variável chamada `second` receberá o valor `2` do índice `[1]` do array.

<a id="invalid-array-element-access"></a>
#### Acesso Inválido a Elementos do Array

Vamos ver o que acontece se você tentar acessar um elemento de um array além do final do array. Suponha que você execute este código, semelhante ao jogo de adivinhação do Capítulo 2, para obter um índice de array do usuário:

<span class="filename">Arquivo: src/main.rs</span>

```rust,ignore,panics
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access/src/main.rs}}
```

Este código compila com sucesso. Se você executá-lo usando `cargo run` e digitar `0`, `1`, `2`, `3` ou `4`, o programa imprimirá o valor correspondente naquele índice do array. Se, em vez disso, você digitar um número além do final do array, como `10`, verá uma saída como esta:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access
cargo run
10
-->

```console
thread 'main' panicked at src/main.rs:19:19:
index out of bounds: the len is 5 but the index is 10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

O programa resultou em um erro em tempo de execução no momento de usar um valor inválido na operação de indexação. O programa encerrou com uma mensagem de erro e não executou a instrução `println!` final. Quando você tenta acessar um elemento usando indexação, o Rust verifica se o índice especificado é menor que o tamanho do array. Se o índice for maior ou igual ao tamanho, o Rust entrará em pânico. Essa verificação precisa acontecer em tempo de execução, especialmente neste caso, porque o compilador não tem como saber qual valor o usuário vai digitar quando executar o código depois.

Este é um exemplo dos princípios de segurança de memória do Rust em ação. Em muitas linguagens de baixo nível, esse tipo de verificação não é feita e, quando você fornece um índice incorreto, memória inválida pode ser acessada. O Rust protege você contra esse tipo de erro encerrando imediatamente em vez de permitir o acesso à memória e continuar. O Capítulo 9 discute mais sobre o tratamento de erros do Rust e como você pode escrever código legível e seguro que nem entra em pânico nem permite acesso inválido à memória.

[comparing-the-guess-to-the-secret-number]: ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[twos-complement]: https://en.wikipedia.org/wiki/Two%27s_complement
[control-flow]: ch03-05-control-flow.html#control-flow
[strings]: ch08-02-strings.html#storing-utf-8-encoded-text-with-strings
[stack-and-heap]: ch04-01-what-is-ownership.html#the-stack-and-the-heap
[vectors]: ch08-01-vectors.html
[unrecoverable-errors-with-panic]: ch09-01-unrecoverable-errors-with-panic.html
[appendix_b]: appendix-02-operators.md
