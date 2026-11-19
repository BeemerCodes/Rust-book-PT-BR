<a id="control-flow"></a>
## Fluxo de Controle

A capacidade de executar algum código dependendo se uma condição é `true` e a capacidade de executar algum código repetidamente enquanto uma condição é `true` são blocos de construção básicos na maioria das linguagens de programação. Os construtos mais comuns que permitem controlar o fluxo de execução do código Rust são expressões `if` e *loops*.

<a id="if-expressions"></a>
### Expressões `if`

Uma expressão `if` permite ramificar seu código dependendo de condições. Você fornece uma condição e então afirma: "Se esta condição for atendida, execute este bloco de código. Se a condição não for atendida, não execute este bloco de código."

Crie um novo projeto chamado _branches_ no seu diretório _projects_ para explorar a expressão `if`. No arquivo _src/main.rs_, insira o seguinte:

<span class="filename">Arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/src/main.rs}}
```

Todas as expressões `if` começam com a palavra-chave `if`, seguida de uma condição. Neste caso, a condição verifica se a variável `number` tem um valor menor que 5 ou não. Colocamos o bloco de código a ser executado se a condição for `true` imediatamente após a condição, dentro de chaves. Blocos de código associados às condições em expressões `if` às vezes são chamados de _braços_ (*arms*), assim como os braços em expressões `match` que discutimos na seção ["Comparando o Palpite com o Número Secreto"][comparing-the-guess-to-the-secret-number]<!-- ignore --> do Capítulo 2.

Opcionalmente, também podemos incluir uma expressão `else`, o que optamos por fazer aqui, para dar ao programa um bloco de código alternativo a ser executado caso a condição seja avaliada como `false`. Se você não fornecer uma expressão `else` e a condição for `false`, o programa simplesmente pulará o bloco `if` e seguirá para o próximo trecho de código.

Tente executar este código; você deverá ver a seguinte saída:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/output.txt}}
```

Vamos tentar alterar o valor de `number` para um valor que torne a condição `false` para ver o que acontece:

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/src/main.rs:here}}
```

Execute o programa novamente e veja a saída:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/output.txt}}
```

Também vale notar que a condição neste código _precisa_ ser um `bool`. Se a condição não for um `bool`, receberemos um erro. Por exemplo, tente executar o seguinte código:

<span class="filename">Arquivo: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/src/main.rs}}
```

A condição do `if` resulta no valor `3` desta vez, e o Rust lança um erro:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/output.txt}}
```

O erro indica que o Rust esperava um `bool`, mas recebeu um inteiro. Diferente de linguagens como Ruby e JavaScript, o Rust não tenta converter automaticamente tipos não booleanos para booleano. Você precisa ser explícito e sempre fornecer um booleano como condição para o `if`. Se quisermos que o bloco de código do `if` execute apenas quando um número não for igual a `0`, por exemplo, podemos alterar a expressão `if` para o seguinte:

<span class="filename">Arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-29-if-not-equal-0/src/main.rs}}
```

Executar este código imprimirá `number was something other than zero`.

<a id="handling-multiple-conditions-with-else-if"></a>
#### Lidando com Múltiplas Condições com `else if`

Você pode usar múltiplas condições combinando `if` e `else` em uma expressão `else if`. Por exemplo:

<span class="filename">Arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/src/main.rs}}
```

Este programa tem quatro caminhos possíveis que pode seguir. Após executá-lo, você deverá ver a seguinte saída:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/output.txt}}
```

Quando este programa é executado, ele verifica cada expressão `if` em sequência e executa o primeiro corpo cuja condição for avaliada como `true`. Note que, embora 6 seja divisível por 2, não vemos a saída `number is divisible by 2`, nem vemos o texto `number is not divisible by 4, 3, or 2` do bloco `else`. Isso porque o Rust executa apenas o bloco da primeira condição `true` e, assim que encontra uma, nem sequer verifica o restante.

Usar expressões `else if` demais pode poluir seu código, então, se você tiver mais de uma, talvez queira refatorar seu código. O Capítulo 6 descreve um poderoso construto de ramificação do Rust chamado `match` para esses casos.

<a id="using-if-in-a-let-statement"></a>
#### Usando `if` em uma Instrução `let`

Como `if` é uma expressão, podemos usá-la no lado direito de uma instrução `let` para atribuir o resultado a uma variável, como na Listagem 3-2.

<Listing number="3-2" file-name="src/main.rs" caption="Atribuindo o resultado de uma expressão `if` a uma variável">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-02/src/main.rs}}
```

</Listing>

A variável `number` será vinculada a um valor com base no resultado da expressão `if`. Execute este código para ver o que acontece:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-02/output.txt}}
```

Lembre-se de que blocos de código são avaliados para a última expressão contida neles, e números por si só também são expressões. Neste caso, o valor de toda a expressão `if` depende de qual bloco de código é executado. Isso significa que os valores que têm o potencial de serem resultados de cada braço do `if` devem ser do mesmo tipo; na Listagem 3-2, os resultados tanto do braço `if` quanto do braço `else` eram inteiros `i32`. Se os tipos forem incompatíveis, como no exemplo a seguir, receberemos um erro:

<span class="filename">Arquivo: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/src/main.rs}}
```

Quando tentarmos compilar este código, receberemos um erro. Os braços `if` e `else` têm tipos de valor incompatíveis, e o Rust indica exatamente onde encontrar o problema no programa:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/output.txt}}
```

A expressão no bloco `if` resulta em um inteiro, e a expressão no bloco `else` resulta em uma string. Isso não funciona, porque variáveis devem ter um único tipo, e o Rust precisa saber definitivamente em tempo de compilação qual é o tipo da variável `number`. Saber o tipo de `number` permite que o compilador verifique se o tipo é válido em todos os lugares onde usamos `number`. O Rust não conseguiria fazer isso se o tipo de `number` fosse determinado apenas em tempo de execução; o compilador seria mais complexo e daria menos garantias sobre o código se tivesse que acompanhar múltiplos tipos hipotéticos para qualquer variável.

<a id="repetition-with-loops"></a>
### Repetição com Loops

Muitas vezes é útil executar um bloco de código mais de uma vez. Para essa tarefa, o Rust fornece vários _loops_, que percorrem o código dentro do corpo do *loop* até o final e então começam imediatamente de novo do início. Para experimentar com *loops*, vamos criar um novo projeto chamado _loops_.

O Rust tem três tipos de *loops*: `loop`, `while` e `for`. Vamos experimentar cada um.

<a id="repeating-code-with-loop"></a>
#### Repetindo Código com `loop`

A palavra-chave `loop` diz ao Rust para executar um bloco de código repetidamente para sempre ou até que você diga explicitamente para parar.

Como exemplo, altere o arquivo _src/main.rs_ no seu diretório _loops_ para ficar assim:

<span class="filename">Arquivo: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-loop/src/main.rs}}
```

Quando executarmos este programa, veremos `again!` sendo impresso repetidamente de forma contínua até pararmos o programa manualmente. A maioria dos terminais suporta o atalho de teclado <kbd>ctrl</kbd>-<kbd>C</kbd> para interromper um programa preso em um *loop* contínuo. Experimente:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-32-loop
cargo run
CTRL-C
-->

```console
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/loops`
again!
again!
again!
again!
^Cagain!
```

O símbolo `^C` representa o momento em que você pressionou <kbd>ctrl</kbd>-<kbd>C</kbd>.

Você pode ou não ver a palavra `again!` impressa após o `^C`, dependendo de onde o código estava no *loop* quando recebeu o sinal de interrupção.

Felizmente, o Rust também fornece uma maneira de sair de um *loop* usando código. Você pode colocar a palavra-chave `break` dentro do *loop* para dizer ao programa quando parar de executar o *loop*. Lembre-se de que fizemos isso no jogo de adivinhação na seção ["Saindo Após um Palpite Correto"][quitting-after-a-correct-guess]<!-- ignore --> do Capítulo 2 para encerrar o programa quando o usuário venceu o jogo adivinhando o número correto.

Também usamos `continue` no jogo de adivinhação, que em um *loop* diz ao programa para pular qualquer código restante nesta iteração do *loop* e ir para a próxima iteração.

<a id="returning-values-from-loops"></a>
#### Retornando Valores de Loops

Um dos usos de um `loop` é tentar novamente uma operação que você sabe que pode falhar, como verificar se uma *thread* concluiu seu trabalho. Você também pode precisar passar o resultado dessa operação para fora do *loop*, para o restante do seu código. Para fazer isso, você pode adicionar o valor que deseja retornar após a expressão `break` usada para parar o *loop*; esse valor será retornado para fora do *loop* para que você possa usá-lo, como mostrado aqui:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-33-return-value-from-loop/src/main.rs}}
```

Antes do *loop*, declaramos uma variável chamada `counter` e a inicializamos com `0`. Em seguida, declaramos uma variável chamada `result` para guardar o valor retornado pelo *loop*. Em cada iteração do *loop*, somamos `1` à variável `counter` e então verificamos se `counter` é igual a `10`. Quando for, usamos a palavra-chave `break` com o valor `counter * 2`. Após o *loop*, usamos um ponto e vírgula para encerrar a instrução que atribui o valor a `result`. Por fim, imprimimos o valor em `result`, que neste caso é `20`.

Você também pode usar `return` de dentro de um *loop*. Enquanto `break` sai apenas do *loop* atual, `return` sempre sai da função atual.

<!-- Old headings. Do not remove or links may break. -->
<a id="loop-labels-to-disambiguate-between-multiple-loops"></a>

<a id="disambiguating-with-loop-labels"></a>
#### Desambiguando com Rótulos de Loop

Se você tiver *loops* dentro de *loops*, `break` e `continue` aplicam-se ao *loop* mais interno naquele ponto. Você pode opcionalmente especificar um _rótulo de loop_ (*loop label*) em um *loop* e depois usá-lo com `break` ou `continue` para especificar que essas palavras-chave se aplicam ao *loop* rotulado em vez do *loop* mais interno. Rótulos de *loop* devem começar com uma aspa simples. Aqui está um exemplo com dois *loops* aninhados:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/src/main.rs}}
```

O *loop* externo tem o rótulo `'counting_up`, e ele contará de 0 até 2. O *loop* interno sem rótulo conta regressivamente de 10 até 9. O primeiro `break` que não especifica um rótulo sairá apenas do *loop* interno. A instrução `break 'counting_up;` sairá do *loop* externo. Este código imprime:

```console
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/output.txt}}
```

<!-- Old headings. Do not remove or links may break. -->
<a id="conditional-loops-with-while"></a>

<a id="streamlining-conditional-loops-with-while"></a>
#### Simplificando Loops Condicionais com `while`

Um programa frequentemente precisará avaliar uma condição dentro de um *loop*. Enquanto a condição for `true`, o *loop* é executado. Quando a condição deixa de ser `true`, o programa chama `break`, parando o *loop*. É possível implementar um comportamento assim usando uma combinação de `loop`, `if`, `else` e `break`; você pode tentar isso agora em um programa, se quiser. No entanto, esse padrão é tão comum que o Rust tem um construto embutido na linguagem para ele, chamado de *loop* `while`. Na Listagem 3-3, usamos `while` para executar o programa em *loop* três vezes, fazendo contagem regressiva a cada vez e, após o *loop*, imprimir uma mensagem e sair.

<Listing number="3-3" file-name="src/main.rs" caption="Usando um loop `while` para executar código enquanto uma condição for avaliada como `true`">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-03/src/main.rs}}
```

</Listing>

Esse construto elimina muito do aninhamento que seria necessário se você usasse `loop`, `if`, `else` e `break`, e fica mais claro. Enquanto uma condição for avaliada como `true`, o código é executado; caso contrário, ele sai do *loop*.

<a id="looping-through-a-collection-with-for"></a>
#### Percorrendo uma Coleção com `for`

Você pode optar por usar o construto `while` para percorrer os elementos de uma coleção, como um array. Por exemplo, o *loop* na Listagem 3-4 imprime cada elemento no array `a`.

<Listing number="3-4" file-name="src/main.rs" caption="Percorrendo cada elemento de uma coleção usando um loop `while`">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-04/src/main.rs}}
```

</Listing>

Aqui, o código percorre os elementos do array contando para cima. Ele começa no índice `0` e continua em *loop* até chegar ao índice final do array (ou seja, quando `index < 5` não é mais `true`). Executar este código imprimirá cada elemento do array:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-04/output.txt}}
```

Todos os cinco valores do array aparecem no terminal, conforme esperado. Embora `index` chegue ao valor `5` em algum momento, o *loop* para de executar antes de tentar buscar um sexto valor no array.

No entanto, essa abordagem é propensa a erros; poderíamos fazer o programa entrar em pânico se o valor do índice ou a condição de teste estiverem incorretos. Por exemplo, se você alterasse a definição do array `a` para ter quatro elementos, mas esquecesse de atualizar a condição para `while index < 4`, o código entraria em pânico. Também é mais lento, porque o compilador adiciona código em tempo de execução para realizar a verificação condicional de se o índice está dentro dos limites do array a cada iteração do *loop*.

Como alternativa mais concisa, você pode usar um *loop* `for` e executar algum código para cada item de uma coleção. Um *loop* `for` se parece com o código da Listagem 3-5.

<Listing number="3-5" file-name="src/main.rs" caption="Percorrendo cada elemento de uma coleção usando um loop `for`">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-05/src/main.rs}}
```

</Listing>

Quando executarmos este código, veremos a mesma saída da Listagem 3-4. Mais importante, agora aumentamos a segurança do código e eliminamos a chance de bugs que poderiam resultar de ir além do final do array ou não ir longe o suficiente e deixar alguns itens de fora. O código de máquina gerado por *loops* `for` também pode ser mais eficiente, porque o índice não precisa ser comparado ao tamanho do array a cada iteração.

Usando o *loop* `for`, você não precisaria se lembrar de alterar nenhum outro código se mudasse o número de valores no array, como precisaria com o método usado na Listagem 3-4.

A segurança e a concisão dos *loops* `for` os tornam o construto de *loop* mais comumente usado em Rust. Mesmo em situações nas quais você deseja executar algum código um determinado número de vezes, como no exemplo de contagem regressiva que usou um *loop* `while` na Listagem 3-3, a maioria dos Rustaceans usaria um *loop* `for`. A maneira de fazer isso seria usar um `Range`, fornecido pela biblioteca padrão, que gera todos os números em sequência começando de um número e terminando antes de outro número.

Veja como ficaria a contagem regressiva usando um *loop* `for` e outro método sobre o qual ainda não falamos, `rev`, para inverter o intervalo:

<span class="filename">Arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-34-for-range/src/main.rs}}
```

Este código é um pouco mais elegante, não é?

<a id="summary"></a>
## Resumo

Você conseguiu! Este foi um capítulo considerável: você aprendeu sobre variáveis, tipos de dados escalares e compostos, funções, comentários, expressões `if` e *loops*! Para praticar os conceitos discutidos neste capítulo, tente construir programas para fazer o seguinte:

- Converter temperaturas entre Fahrenheit e Celsius.
- Gerar o *n*-ésimo número de Fibonacci.
- Imprimir a letra da canção natalina "The Twelve Days of Christmas", aproveitando a repetição na música.

Quando estiver pronto para avançar, falaremos sobre um conceito no Rust que _não_ existe comumente em outras linguagens de programação: *ownership*.

[comparing-the-guess-to-the-secret-number]: ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[quitting-after-a-correct-guess]: ch02-00-guessing-game-tutorial.html#quitting-after-a-correct-guess
