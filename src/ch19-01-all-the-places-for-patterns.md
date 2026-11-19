<a id="all-the-places-patterns-can-be-used"></a>

## Todos os Lugares Onde Padrões Podem Ser Usados

Padrões aparecem em vários lugares em Rust, e você tem usado muitos deles sem
perceber! Esta seção discute todos os lugares onde padrões são válidos.

<a id="match-arms"></a>

### Braços de `match`

Como discutido no Capítulo 6, usamos padrões nos braços de expressões `match`.
Formalmente, expressões `match` são definidas pela palavra-chave `match`, um
valor a ser comparado e um ou mais braços de `match` que consistem em um padrão
e uma expressão a ser executada se o valor corresponder ao padrão daquele braço,
assim:

<!--
  Manually formatted rather than using Markdown intentionally: Markdown does not
  support italicizing code in the body of a block like this!
-->

<pre><code>match <em>VALOR</em> {
    <em>PADRÃO</em> => <em>EXPRESSÃO</em>,
    <em>PADRÃO</em> => <em>EXPRESSÃO</em>,
    <em>PADRÃO</em> => <em>EXPRESSÃO</em>,
}</code></pre>

Por exemplo, aqui está a expressão `match` da Listagem 6-5 que faz
correspondência em um valor `Option<i32>` na variável `x`:

```rust,ignore
match x {
    None => None,
    Some(i) => Some(i + 1),
}
```

Os padrões nessa expressão `match` são o `None` e o `Some(i)` à esquerda de
cada seta.

Um requisito das expressões `match` é que elas precisam ser exaustivas, no
sentido de que todas as possibilidades para o valor na expressão `match` devem
ser contempladas. Uma forma de garantir que você cobriu todas as possibilidades
é ter um padrão abrangente (*catch-all*) para o último braço: por exemplo, um
nome de variável que corresponde a qualquer valor nunca pode falhar e,
portanto, cobre todos os casos restantes.

O padrão particular `_` corresponderá a qualquer coisa, mas nunca se vincula a
uma variável, por isso é frequentemente usado no último braço de um `match`. O
padrão `_` pode ser útil quando você deseja ignorar qualquer valor não
especificado, por exemplo. Cobriremos o padrão `_` em mais detalhes em
[“Ignorando Valores em um Padrão”][ignoring-values-in-a-pattern]<!-- ignore -->
mais adiante neste capítulo.

<a id="let-statements"></a>

### Instruções `let`

Antes deste capítulo, havíamos discutido explicitamente apenas o uso de padrões
com `match` e `if let`, mas, na verdade, já usamos padrões em outros lugares
também, incluindo em instruções `let`. Por exemplo, considere esta atribuição
direta de variável com `let`:

```rust
let x = 5;
```

Toda vez que você usou uma instrução `let` como esta, estava usando padrões,
embora talvez não tenha percebido! Mais formalmente, uma instrução `let` tem
esta aparência:

<!--
  Manually formatted rather than using Markdown intentionally: Markdown does not
  support italicizing code in the body of a block like this!
-->

<pre>
<code>let <em>PADRÃO</em> = <em>EXPRESSÃO</em>;</code>
</pre>

Em instruções como `let x = 5;` com um nome de variável na posição de PADRÃO,
o nome da variável é apenas uma forma particularmente simples de padrão. Rust
compara a expressão com o padrão e atribui quaisquer nomes que encontrar.
Assim, no exemplo `let x = 5;`, `x` é um padrão que significa “vincule o que
corresponder aqui à variável `x`”. Como o nome `x` é o padrão inteiro, esse
padrão efetivamente significa “vincule tudo à variável `x`, qualquer que seja o
valor”.

Para ver o aspecto de *pattern matching* do `let` com mais clareza, considere a
Listagem 19-1, que usa um padrão com `let` para desestruturar uma tupla.


<Listing number="19-1" caption="Usando um padrão para desestruturar uma tupla e criar três variáveis de uma vez">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-01/src/main.rs:here}}
```

</Listing>

Aqui, comparamos uma tupla com um padrão. Rust compara o valor `(1, 2, 3)` com
o padrão `(x, y, z)` e vê que o valor corresponde ao padrão — isto é, vê que o
número de elementos é o mesmo em ambos —, então Rust vincula `1` a `x`, `2` a
`y` e `3` a `z`. Você pode pensar nesse padrão de tupla como o aninhamento de
três padrões de variáveis individuais dentro dele.

Se o número de elementos no padrão não corresponder ao número de elementos na
tupla, o tipo geral não corresponderá e receberemos um erro do compilador. Por
exemplo, a Listagem 19-2 mostra uma tentativa de desestruturar uma tupla com
três elementos em duas variáveis, o que não funcionará.

<Listing number="19-2" caption="Construindo incorretamente um padrão cujas variáveis não correspondem ao número de elementos da tupla">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-02/src/main.rs:here}}
```

</Listing>

Tentar compilar esse código resulta neste erro de tipo:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-02/output.txt}}
```

Para corrigir o erro, poderíamos ignorar um ou mais dos valores na tupla usando
`_` ou `..`, como você verá na seção [“Ignorando Valores em um
Padrão”][ignoring-values-in-a-pattern]<!-- ignore -->. Se o problema for termos
variáveis demais no padrão, a solução é fazer os tipos corresponderem removendo
variáveis para que o número de variáveis seja igual ao número de elementos na
tupla.

<a id="conditional-if-let-expressions"></a>

### Expressões Condicionais `if let`

No Capítulo 6, discutimos como usar expressões `if let` principalmente como uma
maneira mais curta de escrever o equivalente a um `match` que corresponde a
apenas um caso. Opcionalmente, `if let` pode ter um `else` correspondente
contendo o código a ser executado se o padrão no `if let` não corresponder.

A Listagem 19-3 mostra que também é possível misturar e combinar expressões
`if let`, `else if` e `else if let`. Fazer isso nos dá mais flexibilidade do que
uma expressão `match`, na qual só podemos expressar um único valor para comparar
com os padrões. Além disso, Rust não exige que as condições em uma série de
braços `if let`, `else if` e `else if let` estejam relacionadas entre si.

O código na Listagem 19-3 determina qual cor usar no seu plano de fundo com base
em uma série de verificações para diversas condições. Para este exemplo,
criamos variáveis com valores fixos no código que um programa real poderia
receber da entrada do usuário.

<Listing number="19-3" file-name="src/main.rs" caption="Misturando `if let`, `else if`, `else if let` e `else`">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-03/src/main.rs}}
```

</Listing>

Se a pessoa usuária especificar uma cor favorita, essa cor é usada como plano de
fundo. Se nenhuma cor favorita for especificada e hoje for terça-feira, a cor
de fundo será verde. Caso contrário, se a pessoa usuária especificar sua idade
como uma string e conseguirmos analisá-la com sucesso como um número, a cor será
roxa ou laranja dependendo do valor do número. Se nenhuma dessas condições se
aplicar, a cor de fundo será azul.

Essa estrutura condicional nos permite suportar requisitos complexos. Com os
valores fixos que temos aqui, este exemplo imprimirá `Using purple as the
background color`.

Você pode ver que `if let` também pode introduzir novas variáveis que fazem
sombreamento (*shadowing*) de variáveis existentes da mesma forma que os braços
de `match`: a linha `if let Ok(age) = age` introduz uma nova variável `age` que
contém o valor dentro da variante `Ok`, sombreando a variável `age` existente.
Isso significa que precisamos colocar a condição `if age > 30` dentro daquele
bloco: não podemos combinar essas duas condições em `if let Ok(age) = age &&
age > 30`. O novo `age` que queremos comparar com 30 não é válido até que o novo
escopo comece com a chave de abertura.

A desvantagem de usar expressões `if let` é que o compilador não verifica a
exaustividade, ao passo que com expressões `match` ele verifica. Se omitíssemos
o último bloco `else` e, portanto, deixássemos de tratar alguns casos, o
compilador não nos alertaria sobre o possível bug de lógica.

<a id="while-let-conditional-loops"></a>

### Laços Condicionais `while let`

Semelhante em construção ao `if let`, o laço condicional `while let` permite que
um laço `while` execute enquanto um padrão continuar correspondendo. Na Listagem
19-4, mostramos um laço `while let` que aguarda mensagens enviadas entre
*threads*, mas neste caso verificando um `Result` em vez de um `Option`.

<Listing number="19-4" caption="Usando um laço `while let` para imprimir valores enquanto `rx.recv()` retornar `Ok`">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-04/src/main.rs:here}}
```

</Listing>

Este exemplo imprime `1`, `2` e depois `3`. O método `recv` retira a primeira
mensagem do lado receptor do canal e retorna um `Ok(value)`. Quando vimos `recv`
pela primeira vez no Capítulo 16, fizemos *unwrap* do erro diretamente ou
interagimos com ele como um iterador usando um laço `for`. Como mostra a
Listagem 19-4, porém, também podemos usar `while let`, porque o método `recv`
retorna um `Ok` cada vez que uma mensagem chega, enquanto o remetente existir, e
então produz um `Err` assim que o lado remetente se desconecta.

<a id="for-loops"></a>

### Laços `for`

Em um laço `for`, o valor que segue diretamente a palavra-chave `for` é um
padrão. Por exemplo, em `for x in y`, o `x` é o padrão. A Listagem 19-5
demonstra como usar um padrão em um laço `for` para desestruturar, ou quebrar em
partes, uma tupla como parte do laço `for`.


<Listing number="19-5" caption="Usando um padrão em um laço `for` para desestruturar uma tupla">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-05/src/main.rs:here}}
```

</Listing>

O código na Listagem 19-5 imprimirá o seguinte:


```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-05/output.txt}}
```

Adaptamos um iterador usando o método `enumerate` para que ele produza um valor
e o índice para esse valor, colocados em uma tupla. O primeiro valor produzido é
a tupla `(0, 'a')`. Quando esse valor corresponde ao padrão `(index, value)`,
`index` será `0` e `value` será `'a'`, imprimindo a primeira linha da saída.


<a id="function-parameters"></a>

### Parâmetros de Função

Parâmetros de função também podem ser padrões. O código na Listagem 19-6, que
declara uma função chamada `foo` que recebe um parâmetro chamado `x` do tipo
`i32`, já deve parecer familiar a esta altura.

<Listing number="19-6" caption="Uma assinatura de função usando padrões nos parâmetros">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-06/src/main.rs:here}}
```

</Listing>

A parte `x` é um padrão! Assim como fizemos com `let`, poderíamos corresponder
uma tupla nos argumentos de uma função ao padrão. A Listagem 19-7 separa os
valores de uma tupla enquanto a passamos para uma função.

<Listing number="19-7" file-name="src/main.rs" caption="Uma função com parâmetros que desestruturam uma tupla">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-07/src/main.rs}}
```

</Listing>

Este código imprime `Current location: (3, 5)`. Os valores `&(3, 5)`
correspondem ao padrão `&(x, y)`, portanto `x` é o valor `3` e `y` é o valor
`5`.

Também podemos usar padrões em listas de parâmetros de *closures* da mesma forma
que em listas de parâmetros de funções, porque *closures* são semelhantes a
funções, conforme discutido no Capítulo 13.

Até aqui, você viu várias maneiras de usar padrões, mas padrões não funcionam
da mesma forma em todo lugar onde podemos usá-los. Em alguns lugares, os padrões
precisam ser irrefutáveis; em outras circunstâncias, eles podem ser refutáveis.
Discutiremos esses dois conceitos a seguir.

[ignoring-values-in-a-pattern]: ch19-03-pattern-syntax.html#ignoring-values-in-a-pattern
