<!-- Old headings. Do not remove or links may break. -->

<a id="the-match-control-flow-operator"></a>
<a id="the-match-control-flow-construct"></a>
## A Construção de Fluxo de Controle `match`

Rust tem uma construção de fluxo de controle extremamente poderosa chamada
`match` que permite comparar um valor com uma série de padrões e então executar
código com base em qual padrão corresponde (*matches*). Padrões podem ser
formados por valores literais, nomes de variáveis, curingas (*wildcards*) e
muitas outras coisas; o [Capítulo 19][ch19-00-patterns]<!-- ignore --> aborda
todos os diferentes tipos de padrões e o que eles fazem. O poder do `match` vem
da expressividade dos padrões e do fato de o compilador confirmar que todos os
casos possíveis foram tratados.

Pense em uma expressão `match` como uma máquina de separar moedas: as moedas
deslizam por uma trilha com furos de tamanhos variados ao longo dela, e cada
moeda cai pelo primeiro furo em que couber. Da mesma forma, os valores passam
por cada padrão em um `match`, e no primeiro padrão em que o valor “se encaixa”,
o valor cai no bloco de código associado para ser usado durante a execução.

Falando em moedas, vamos usá-las como exemplo com `match`! Podemos escrever uma
função que recebe uma moeda americana desconhecida e, de forma semelhante à
máquina contadora, determina qual moeda é e retorna seu valor em centavos, como
mostrado na Listagem 6-3.

<Listing number="6-3" caption="Um enum e uma expressão `match` que tem as variantes do enum como seus padrões">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-03/src/main.rs:here}}
```

</Listing>

Vamos destrinchar o `match` na função `value_in_cents`. Primeiro, colocamos a
palavra-chave `match` seguida de uma expressão, que neste caso é o valor `coin`.
Isso parece muito semelhante a uma expressão condicional usada com `if`, mas há
uma grande diferença: com `if`, a condição precisa avaliar para um valor
booleano, mas aqui ela pode ser de qualquer tipo. O tipo de `coin` neste exemplo
é o enum `Coin` que definimos na primeira linha.

Em seguida vêm os braços (*arms*) do `match`. Um braço tem duas partes: um
padrão e algum código. O primeiro braço aqui tem um padrão que é o valor
`Coin::Penny` e depois o operador `=>` que separa o padrão do código a ser
executado. O código neste caso é apenas o valor `1`. Cada braço é separado do
próximo por uma vírgula.

Quando a expressão `match` é executada, ela compara o valor resultante com o
padrão de cada braço, em ordem. Se um padrão corresponder ao valor, o código
associado a esse padrão é executado. Se esse padrão não corresponder ao valor, a
execução continua para o próximo braço, assim como na máquina de separar moedas.
Podemos ter quantos braços precisarmos: na Listagem 6-3, nosso `match` tem
quatro braços.

O código associado a cada braço é uma expressão, e o valor resultante da
expressão no braço correspondente é o valor retornado por toda a expressão
`match`.

Normalmente não usamos chaves se o código do braço do `match` for curto, como na
Listagem 6-3, onde cada braço apenas retorna um valor. Se você quiser executar
várias linhas de código em um braço do `match`, deve usar chaves, e a vírgula
após o braço passa a ser opcional. Por exemplo, o código a seguir imprime
“Lucky penny!” toda vez que a função é chamada com um `Coin::Penny`, mas ainda
retorna o último valor do bloco, `1`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-08-match-arm-multiple-lines/src/main.rs:here}}
```

<a id="patterns-that-bind-to-values"></a>
### Padrões Que Vinculam Valores

Outro recurso útil dos braços do `match` é que eles podem se vincular às partes
dos valores que correspondem ao padrão. É assim que podemos extrair valores de
variantes de enums.

Como exemplo, vamos alterar uma das nossas variantes de enum para guardar dados
dentro dela. De 1999 a 2008, os Estados Unidos cunharam moedas de 25 centavos
(*quarters*) com desenhos diferentes para cada um dos 50 estados em um dos
lados. Nenhuma outra moeda recebeu desenhos de estados, então apenas os
*quarters* têm esse valor extra. Podemos adicionar essa informação ao nosso
`enum` alterando a variante `Quarter` para incluir um valor `UsState` armazenado
dentro dela, o que fizemos na Listagem 6-4.

<Listing number="6-4" caption="Um enum `Coin` no qual a variante `Quarter` também guarda um valor `UsState`">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-04/src/main.rs:here}}
```

</Listing>

Vamos imaginar que um amigo está tentando colecionar todos os 50 *quarters* dos
estados. Enquanto separamos nosso troco por tipo de moeda, também vamos anunciar
o nome do estado associado a cada *quarter* para que, se for um que nosso amigo
não tem, ele possa adicioná-lo à sua coleção.

Na expressão `match` para esse código, adicionamos uma variável chamada `state`
ao padrão que corresponde aos valores da variante `Coin::Quarter`. Quando um
`Coin::Quarter` corresponder, a variável `state` será vinculada ao valor do
estado daquele *quarter*. Então, podemos usar `state` no código daquele braço,
assim:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-09-variable-in-pattern/src/main.rs:here}}
```

Se chamássemos `value_in_cents(Coin::Quarter(UsState::Alaska))`, `coin` seria
`Coin::Quarter(UsState::Alaska)`. Quando comparamos esse valor com cada um dos
braços do `match`, nenhum deles corresponde até chegarmos a
`Coin::Quarter(state)`. Nesse ponto, o vínculo de `state` será o valor
`UsState::Alaska`. Podemos então usar esse vínculo na expressão `println!`,
extraindo assim o valor interno do estado da variante `Quarter` do enum `Coin`.

<!-- Old headings. Do not remove or links may break. -->

<a id="matching-with-optiont"></a>
<a id="the-optiont-match-pattern"></a>
### O Padrão `match` com `Option<T>`

Na seção anterior, queríamos extrair o valor `T` interno do caso `Some` ao usar
`Option<T>`; também podemos lidar com `Option<T>` usando `match`, assim como
fizemos com o enum `Coin`! Em vez de comparar moedas, vamos comparar as
variantes de `Option<T>`, mas a forma como a expressão `match` funciona continua
a mesma.

Digamos que queremos escrever uma função que recebe um `Option<i32>` e, se
houver um valor dentro, soma 1 a esse valor. Se não houver um valor dentro, a
função deve retornar o valor `None` e não tentar realizar nenhuma operação.

Essa função é muito fácil de escrever, graças ao `match`, e ficará como na
Listagem 6-5.

<Listing number="6-5" caption="Uma função que usa uma expressão `match` em um `Option<i32>`">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:here}}
```

</Listing>

Vamos examinar a primeira execução de `plus_one` em mais detalhes. Quando
chamamos `plus_one(five)`, a variável `x` no corpo de `plus_one` terá o valor
`Some(5)`. Então comparamos isso com cada braço do `match`:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

O valor `Some(5)` não corresponde ao padrão `None`, então continuamos para o
próximo braço:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:second_arm}}
```

`Some(5)` corresponde a `Some(i)`? Sim! Temos a mesma variante. O `i` se vincula
ao valor contido em `Some`, então `i` assume o valor `5`. O código no braço do
`match` é então executado, somamos 1 ao valor de `i` e criamos um novo valor
`Some` com o nosso total `6` dentro.

Agora vamos considerar a segunda chamada de `plus_one` na Listagem 6-5, onde `x`
é `None`. Entramos no `match` e comparamos com o primeiro braço:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

Corresponde! Não há nenhum valor ao qual somar, então o programa para e retorna
o valor `None` do lado direito de `=>`. Como o primeiro braço correspondeu,
nenhum outro braço é comparado.

Combinar `match` e enums é útil em muitas situações. Você verá esse padrão com
frequência em código Rust: fazer `match` em um enum, vincular uma variável aos
dados internos e então executar código com base nisso. É um pouco complicado no
começo, mas depois que você se acostuma, vai desejar ter isso em todas as
linguagens. É consistentemente um dos recursos favoritos dos usuários.

<a id="matches-are-exhaustive"></a>
### Matches São Exaustivos

Há outro aspecto do `match` que precisamos discutir: os padrões dos braços devem
cobrir todas as possibilidades. Considere esta versão da nossa função
`plus_one`, que tem um bug e não compila:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/src/main.rs:here}}
```

Não tratamos o caso `None`, então esse código causaria um bug. Felizmente, é um
bug que Rust sabe detectar. Se tentarmos compilar esse código, receberemos este
erro:

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/output.txt}}
```

Rust sabe que não cobrimos todos os casos possíveis e até sabe qual padrão
esquecemos! Matches em Rust são *exaustivos*: precisamos esgotar até a última
possibilidade para que o código seja válido. Especialmente no caso de
`Option<T>`, quando Rust nos impede de esquecer de tratar explicitamente o caso
`None`, ele nos protege de assumir que temos um valor quando podemos ter null,
tornando impossível o erro de um bilhão de dólares discutido anteriormente.

<a id="catch-all-patterns-and-the-_-placeholder"></a>
### Padrões Coringa (Catch-All) e o Marcador `_`

Usando enums, também podemos tomar ações especiais para alguns valores
específicos, mas para todos os outros valores tomar uma ação padrão. Imagine
que estamos implementando um jogo onde, se você tirar um 3 no dado, seu jogador
não se move, mas em vez disso ganha um chapéu elegante novo. Se você tirar um 7,
seu jogador perde um chapéu elegante. Para todos os outros valores, seu jogador
avança aquele número de casas no tabuleiro. Aqui está um `match` que implementa
essa lógica, com o resultado da rolagem do dado fixo (*hardcoded*) em vez de um
valor aleatório, e toda a outra lógica representada por funções sem corpo porque
implementá-las de fato está fora do escopo deste exemplo:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-15-binding-catchall/src/main.rs:here}}
```

Para os dois primeiros braços, os padrões são os valores literais `3` e `7`.
Para o último braço, que cobre todos os outros valores possíveis, o padrão é a
variável que escolhemos chamar de `other`. O código que roda para o braço
`other` usa a variável passando-a para a função `move_player`.

Esse código compila, mesmo que não tenhamos listado todos os valores possíveis
que um `u8` pode ter, porque o último padrão corresponderá a todos os valores
não listados especificamente. Esse padrão *catch-all* (pega-tudo) atende ao
requisito de que `match` deve ser exaustivo. Note que precisamos colocar o braço
*catch-all* por último porque os padrões são avaliados em ordem. Se colocássemos
o braço *catch-all* antes, os outros braços nunca seriam executados, então Rust
nos avisará se adicionarmos braços depois de um *catch-all*!

Rust também tem um padrão que podemos usar quando queremos um *catch-all*, mas
não queremos *usar* o valor no padrão *catch-all*: `_` é um padrão especial que
corresponde a qualquer valor e não se vincula a esse valor. Isso diz ao Rust que
não vamos usar o valor, então Rust não nos avisará sobre uma variável não
utilizada.

Vamos mudar as regras do jogo: agora, se você tirar qualquer coisa além de 3 ou
7, precisa jogar o dado novamente. Não precisamos mais usar o valor do
*catch-all*, então podemos mudar nosso código para usar `_` em vez da variável
chamada `other`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-16-underscore-catchall/src/main.rs:here}}
```

Esse exemplo também atende ao requisito de exaustividade porque estamos
ignorando explicitamente todos os outros valores no último braço; não esquecemos
nada.

Por fim, vamos mudar as regras do jogo mais uma vez para que nada aconteça no
seu turno se você tirar qualquer coisa além de 3 ou 7. Podemos expressar isso
usando o valor unitário (o tipo tupla vazia que mencionamos na seção [“O Tipo
Tupla”][tuples]<!-- ignore -->) como o código associado ao braço `_`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-17-underscore-unit/src/main.rs:here}}
```

Aqui, estamos dizendo explicitamente ao Rust que não vamos usar nenhum outro
valor que não corresponda a um padrão em um braço anterior, e que não queremos
executar nenhum código nesse caso.

Há mais sobre padrões e correspondência que abordaremos no [Capítulo
19][ch19-00-patterns]<!-- ignore -->. Por enquanto, vamos passar para a sintaxe
`if let`, que pode ser útil em situações em que a expressão `match` é um pouco
verbosa.

[tuples]: ch03-02-data-types.html#the-tuple-type
[ch19-00-patterns]: ch19-00-patterns.html
