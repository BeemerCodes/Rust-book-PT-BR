<a id="functions"></a>
## Funções

Funções são onipresentes no código Rust. Você já viu uma das funções mais importantes da linguagem: a função `main`, que é o ponto de entrada de muitos programas. Você também viu a palavra-chave `fn`, que permite declarar novas funções.

O código Rust usa _snake case_ como estilo convencional para nomes de funções e variáveis, no qual todas as letras são minúsculas e sublinhados separam as palavras. Aqui está um programa que contém uma definição de função de exemplo:

<span class="filename">Arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-16-functions/src/main.rs}}
```

Definimos uma função em Rust digitando `fn` seguido pelo nome da função e um par de parênteses. As chaves informam ao compilador onde o corpo da função começa e termina.

Podemos chamar qualquer função que tenhamos definido digitando seu nome seguido por um par de parênteses. Como `another_function` está definida no programa, ela pode ser chamada de dentro da função `main`. Note que definimos `another_function` _depois_ da função `main` no código-fonte; poderíamos tê-la definido antes também. O Rust não se importa com onde você define suas funções, apenas que elas estejam definidas em algum lugar em um escopo visível para quem as chama.

Vamos iniciar um novo projeto binário chamado _functions_ para explorar melhor as funções. Coloque o exemplo `another_function` em _src/main.rs_ e execute-o. Você deverá ver a seguinte saída:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-16-functions/output.txt}}
```

As linhas são executadas na ordem em que aparecem na função `main`. Primeiro, a mensagem "Hello, world!" é impressa, e então `another_function` é chamada e sua mensagem é impressa.

<a id="parameters"></a>
### Parâmetros

Podemos definir funções para receberem _parâmetros_, que são variáveis especiais que fazem parte da assinatura de uma função. Quando uma função tem parâmetros, você pode fornecer valores concretos para esses parâmetros. Tecnicamente, os valores concretos são chamados de _argumentos_, mas em conversas casuais, as pessoas tendem a usar as palavras _parâmetro_ e _argumento_ de forma intercambiável, tanto para as variáveis na definição de uma função quanto para os valores concretos passados ao chamá-la.

Nesta versão de `another_function`, adicionamos um parâmetro:

<span class="filename">Arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/src/main.rs}}
```

Tente executar este programa; você deverá obter a seguinte saída:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/output.txt}}
```

A declaração de `another_function` tem um parâmetro chamado `x`. O tipo de `x` é especificado como `i32`. Quando passamos `5` para `another_function`, a *macro* `println!` coloca `5` onde estava o par de chaves contendo `x` na string de formato.

Em assinaturas de funções, você _deve_ declarar o tipo de cada parâmetro. Essa é uma decisão deliberada no design do Rust: exigir anotações de tipo nas definições de funções significa que o compilador quase nunca precisa que você as use em outros lugares do código para descobrir qual tipo você quer dizer. O compilador também consegue fornecer mensagens de erro mais úteis se souber quais tipos a função espera.

Ao definir múltiplos parâmetros, separe as declarações de parâmetros com vírgulas, assim:

<span class="filename">Arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/src/main.rs}}
```

Este exemplo cria uma função chamada `print_labeled_measurement` com dois parâmetros. O primeiro parâmetro se chama `value` e é um `i32`. O segundo se chama `unit_label` e é do tipo `char`. A função então imprime um texto contendo tanto `value` quanto `unit_label`.

Vamos tentar executar este código. Substitua o programa atual no arquivo _src/main.rs_ do seu projeto _functions_ pelo exemplo anterior e execute-o usando `cargo run`:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/output.txt}}
```

Como chamamos a função com `5` como valor para `value` e `'h'` como valor para `unit_label`, a saída do programa contém esses valores.

<a id="statements-and-expressions"></a>
### Instruções e Expressões

Corpos de funções são compostos por uma série de instruções (*statements*) terminando opcionalmente em uma expressão (*expression*). Até agora, as funções que vimos não incluíam uma expressão final, mas você já viu uma expressão como parte de uma instrução. Como o Rust é uma linguagem baseada em expressões, essa é uma distinção importante de entender. Outras linguagens não têm as mesmas distinções, então vamos ver o que são instruções e expressões e como suas diferenças afetam os corpos das funções.

- _Instruções_ (*statements*) são comandos que executam alguma ação e não retornam um valor.
- _Expressões_ (*expressions*) são avaliadas para um valor resultante.

Vamos ver alguns exemplos.

Na verdade, já usamos instruções e expressões. Criar uma variável e atribuir um valor a ela com a palavra-chave `let` é uma instrução. Na Listagem 3-1, `let y = 6;` é uma instrução.

<Listing number="3-1" file-name="src/main.rs" caption="Uma declaração da função `main` contendo uma instrução">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-01/src/main.rs}}
```

</Listing>

Definições de funções também são instruções; todo o exemplo anterior é uma instrução por si só. (Como veremos em breve, chamar uma função não é uma instrução, porém.)

Instruções não retornam valores. Portanto, você não pode atribuir uma instrução `let` a outra variável, como o código a seguir tenta fazer; você receberá um erro:

<span class="filename">Arquivo: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/src/main.rs}}
```

Quando você executa este programa, o erro que você receberá será parecido com este:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/output.txt}}
```

A instrução `let y = 6` não retorna um valor, então não há nada para vincular a `x`. Isso é diferente do que acontece em outras linguagens, como C e Ruby, onde a atribuição retorna o valor atribuído. Nessas linguagens, você pode escrever `x = y = 6` e fazer com que tanto `x` quanto `y` tenham o valor `6`; esse não é o caso em Rust.

Expressões são avaliadas para um valor e compõem a maior parte do restante do código que você escreverá em Rust. Considere uma operação matemática, como `5 + 6`, que é uma expressão que resulta no valor `11`. Expressões podem fazer parte de instruções: na Listagem 3-1, o `6` na instrução `let y = 6;` é uma expressão que resulta no valor `6`. Chamar uma função é uma expressão. Chamar uma *macro* é uma expressão. Um novo bloco de escopo criado com chaves é uma expressão, por exemplo:

<span class="filename">Arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-20-blocks-are-expressions/src/main.rs}}
```

Esta expressão:

```rust,ignore
{
    let x = 3;
    x + 1
}
```

é um bloco que, neste caso, resulta em `4`. Esse valor é vinculado a `y` como parte da instrução `let`. Note a linha `x + 1` sem ponto e vírgula no final, ao contrário da maioria das linhas que você viu até agora. Expressões não incluem ponto e vírgula no final. Se você adicionar um ponto e vírgula ao final de uma expressão, transforma-a em uma instrução, e ela então não retornará um valor. Tenha isso em mente enquanto exploramos valores de retorno de funções e expressões a seguir.

<a id="functions-with-return-values"></a>
### Funções com Valores de Retorno

Funções podem retornar valores para o código que as chama. Não nomeamos os valores de retorno, mas devemos declarar seu tipo após uma seta (`->`). Em Rust, o valor de retorno da função é sinônimo do valor da expressão final no bloco do corpo de uma função. Você pode retornar antecipadamente de uma função usando a palavra-chave `return` e especificando um valor, mas a maioria das funções retorna a última expressão implicitamente. Aqui está um exemplo de uma função que retorna um valor:

<span class="filename">Arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/src/main.rs}}
```

Não há chamadas de função, *macros* ou sequer instruções `let` na função `five` — apenas o número `5` sozinho. Essa é uma função perfeitamente válida em Rust. Note que o tipo de retorno da função também é especificado, como `-> i32`. Tente executar este código; a saída deve ficar assim:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/output.txt}}
```

O `5` em `five` é o valor de retorno da função, razão pela qual o tipo de retorno é `i32`. Vamos examinar isso com mais detalhes. Há dois pontos importantes: primeiro, a linha `let x = five();` mostra que estamos usando o valor de retorno de uma função para inicializar uma variável. Como a função `five` retorna `5`, essa linha é equivalente a:

```rust
let x = 5;
```

Segundo, a função `five` não tem parâmetros e define o tipo do valor de retorno, mas o corpo da função é um `5` solitário sem ponto e vírgula porque é uma expressão cujo valor queremos retornar.

Vamos ver outro exemplo:

<span class="filename">Arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-22-function-parameter-and-return/src/main.rs}}
```

Executar este código imprimirá `The value of x is: 6`. Mas o que acontece se colocarmos um ponto e vírgula no final da linha que contém `x + 1`, mudando-a de uma expressão para uma instrução?

<span class="filename">Arquivo: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/src/main.rs}}
```

Compilar este código produzirá um erro, como segue:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/output.txt}}
```

A mensagem de erro principal, `mismatched types`, revela o problema central deste código. A definição da função `plus_one` diz que ela retornará um `i32`, mas instruções não resultam em um valor, o que é expresso por `()`, o tipo *unit*. Portanto, nada é retornado, o que contradiz a definição da função e resulta em um erro. Nesta saída, o Rust fornece uma mensagem para ajudar a corrigir esse problema: ele sugere remover o ponto e vírgula, o que resolveria o erro.
