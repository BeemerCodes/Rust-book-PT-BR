<a id="pattern-syntax"></a>

## Sintaxe de Padrões

Nesta seção, reunimos toda a sintaxe que é válida em padrões e discutimos por
que e quando você pode querer usar cada uma delas.

<a id="matching-literals"></a>

### Correspondendo Literais

Como você viu no Capítulo 6, é possível fazer correspondência de padrões
diretamente com literais. O código a seguir traz alguns exemplos:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-01-literals/src/main.rs:here}}
```

Esse código imprime `one` porque o valor em `x` é `1`. Essa sintaxe é útil
quando você deseja que seu código execute uma ação caso receba um valor
concreto específico.

<a id="matching-named-variables"></a>

### Correspondendo Variáveis Nomeadas

Variáveis nomeadas são padrões irrefutáveis que correspondem a qualquer valor,
e já as usamos muitas vezes neste livro. No entanto, há uma complicação quando
você usa variáveis nomeadas em expressões `match`, `if let` ou `while let`. Como
cada um desses tipos de expressão inicia um novo escopo, variáveis declaradas
como parte de um padrão dentro dessas expressões farão sombreamento (*shadowing*)
daquelas com o mesmo nome fora das construções, como acontece com todas as
variáveis. Na Listagem 19-11, declaramos uma variável chamada `x` com o valor
`Some(5)` e uma variável `y` com o valor `10`. Em seguida, criamos uma
expressão `match` sobre o valor `x`. Observe os padrões nos braços do `match`
e o `println!` no final, e tente descobrir o que o código imprimirá antes de
executá-lo ou continuar lendo.

<Listing number="19-11" file-name="src/main.rs" caption="Uma expressão `match` com um braço que introduz uma nova variável que sombreia uma variável `y` existente">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-11/src/main.rs:here}}
```

</Listing>

Vamos acompanhar o que acontece quando a expressão `match` é executada. O
padrão no primeiro braço do `match` não corresponde ao valor definido de `x`,
então o código continua.

O padrão no segundo braço do `match` introduz uma nova variável chamada `y` que
corresponderá a qualquer valor dentro de um valor `Some`. Como estamos em um
novo escopo dentro da expressão `match`, trata-se de uma nova variável `y`, não
do `y` que declaramos no início com o valor `10`. Esse novo vínculo `y`
corresponderá a qualquer valor dentro de um `Some`, que é o que temos em `x`.
Portanto, esse novo `y` se vincula ao valor interno do `Some` em `x`. Esse
valor é `5`, então a expressão desse braço é executada e imprime
`Matched, y = 5`.

Se `x` tivesse sido um valor `None` em vez de `Some(5)`, os padrões nos dois
primeiros braços não teriam correspondido, então o valor teria correspondido ao
sublinhado (`_`). Não introduzimos a variável `x` no padrão do braço do
sublinhado, portanto o `x` na expressão ainda é o `x` externo que não foi
sombreado. Nesse caso hipotético, o `match` imprimiria `Default case, x = None`.

Quando a expressão `match` termina, seu escopo se encerra, assim como o escopo
do `y` interno. O último `println!` produz `at the end: x = Some(5), y = 10`.

Para criar uma expressão `match` que compare os valores do `x` e do `y`
externos, em vez de introduzir uma nova variável que sombreia a variável `y`
existente, precisaríamos usar uma condicional de guarda de `match` (*match
guard*). Falaremos sobre guardas de `match` mais adiante na seção
[“Adicionando Condicionais com Guardas de
Match”](#adding-conditionals-with-match-guards)<!-- ignore -->.

<!-- Old headings. Do not remove or links may break. -->
<a id="multiple-patterns"></a>
<a id="matching-multiple-patterns"></a>

### Correspondendo Múltiplos Padrões

Em expressões `match`, você pode corresponder múltiplos padrões usando a
sintaxe `|`, que é o operador *ou* (*or*) de padrões. Por exemplo, no código a
seguir, comparamos o valor de `x` com os braços do `match`, sendo que o primeiro
possui uma opção *ou*, o que significa que, se o valor de `x` corresponder a
qualquer um dos valores naquele braço, o código daquele braço será executado:


```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-02-multiple-patterns/src/main.rs:here}}
```

Esse código imprime `one or two`.

<a id="matching-ranges-of-values-with-"></a>

### Correspondendo Intervalos de Valores com `..=`

A sintaxe `..=` nos permite corresponder a um intervalo inclusivo de valores. No
código a seguir, quando um padrão corresponde a qualquer um dos valores dentro
do intervalo fornecido, aquele braço será executado:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-03-ranges/src/main.rs:here}}
```

Se `x` for `1`, `2`, `3`, `4` ou `5`, o primeiro braço corresponderá. Essa
sintaxe é mais conveniente para múltiplos valores de correspondência do que usar
o operador `|` para expressar a mesma ideia; se fôssemos usar `|`, teríamos que
especificar `1 | 2 | 3 | 4 | 5`. Especificar um intervalo é muito mais curto,
especialmente se quisermos corresponder, digamos, a qualquer número entre 1 e
1.000!

O compilador verifica em tempo de compilação que o intervalo não está vazio e,
como os únicos tipos para os quais o Rust consegue saber se um intervalo está
vazio ou não são `char` e valores numéricos, intervalos só são permitidos com
valores numéricos ou `char`.

Aqui está um exemplo usando intervalos de valores `char`:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-04-ranges-of-char/src/main.rs:here}}
```

O Rust consegue saber que `'c'` está dentro do intervalo do primeiro padrão e
imprime `early ASCII letter`.

<a id="destructuring-to-break-apart-values"></a>

### Desestruturando para Quebrar Valores em Partes

Também podemos usar padrões para desestruturar structs, enums e tuplas a fim de
usar diferentes partes desses valores. Vamos percorrer cada tipo de valor.

<!-- Old headings. Do not remove or links may break. -->

<a id="destructuring-structs"></a>
<a id="structs"></a>

#### Structs

A Listagem 19-12 mostra uma struct `Point` com dois campos, `x` e `y`, que
podemos separar usando um padrão com uma instrução `let`.

<Listing number="19-12" file-name="src/main.rs" caption="Desestruturando os campos de uma struct em variáveis separadas">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-12/src/main.rs}}
```

</Listing>

Esse código cria as variáveis `a` e `b` que correspondem aos valores dos campos
`x` e `y` da struct `p`. Esse exemplo mostra que os nomes das variáveis no
padrão não precisam ser iguais aos nomes dos campos da struct. Contudo, é comum
fazer os nomes das variáveis coincidirem com os nomes dos campos para facilitar
lembrar quais variáveis vieram de quais campos. Por causa desse uso comum, e
porque escrever `let Point { x: x, y: y } = p;` contém muita duplicação, Rust
possui uma abreviação (*shorthand*) para padrões que correspondem a campos de
struct: você só precisa listar o nome do campo da struct, e as variáveis
criadas a partir do padrão terão os mesmos nomes. A Listagem 19-13 se comporta
da mesma forma que o código na Listagem 19-12, mas as variáveis criadas no
padrão `let` são `x` e `y` em vez de `a` e `b`.

<Listing number="19-13" file-name="src/main.rs" caption="Desestruturando campos de struct usando a abreviação de campos de struct">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-13/src/main.rs}}
```

</Listing>

Esse código cria as variáveis `x` e `y` que correspondem aos campos `x` e `y` da
variável `p`. O resultado é que as variáveis `x` e `y` contêm os valores da
struct `p`.

Também podemos desestruturar com valores literais como parte do padrão da
struct em vez de criar variáveis para todos os campos. Fazer isso nos permite
testar alguns dos campos quanto a valores específicos enquanto criamos
variáveis para desestruturar os outros campos.

Na Listagem 19-14, temos uma expressão `match` que separa valores `Point` em
três casos: pontos que ficam diretamente sobre o eixo `x` (o que é verdade
quando `y = 0`), sobre o eixo `y` (`x = 0`) ou sobre nenhum dos eixos.

<Listing number="19-14" file-name="src/main.rs" caption="Desestruturando e correspondendo valores literais em um único padrão">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-14/src/main.rs:here}}
```

</Listing>

O primeiro braço corresponderá a qualquer ponto que esteja sobre o eixo `x` ao
especificar que o campo `y` corresponde se seu valor corresponder ao literal
`0`. O padrão ainda cria uma variável `x` que podemos usar no código desse
braço.

De modo semelhante, o segundo braço corresponde a qualquer ponto sobre o eixo
`y` ao especificar que o campo `x` corresponde se seu valor for `0` e cria uma
variável `y` para o valor do campo `y`. O terceiro braço não especifica nenhum
literal, portanto corresponde a qualquer outro `Point` e cria variáveis para
ambos os campos `x` e `y`.

Neste exemplo, o valor `p` corresponde ao segundo braço por conta de `x` conter
um `0`, então esse código imprimirá `On the y axis at 7`.

Lembre-se de que uma expressão `match` para de verificar os braços assim que
encontra o primeiro padrão correspondente, portanto, mesmo que `Point { x: 0,
y: 0 }` esteja tanto no eixo `x` quanto no eixo `y`, esse código só imprimiria
`On the x axis at 0`.

<!-- Old headings. Do not remove or links may break. -->

<a id="destructuring-enums"></a>
<a id="enums"></a>

#### Enums

Já desestruturamos enums neste livro (por exemplo, na Listagem 6-5 no Capítulo
6), mas ainda não discutimos explicitamente que o padrão para desestruturar um
enum corresponde à maneira como os dados armazenados dentro do enum são
definidos. Como exemplo, na Listagem 19-15, usamos o enum `Message` da Listagem
6-2 e escrevemos um `match` com padrões que desestruturarão cada valor interno.

<Listing number="19-15" file-name="src/main.rs" caption="Desestruturando variantes de enum que guardam diferentes tipos de valores">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-15/src/main.rs}}
```

</Listing>

Esse código imprimirá `Change color to red 0, green 160, and blue 255`.
Experimente alterar o valor de `msg` para ver o código dos outros braços ser
executado.

Para variantes de enum sem nenhum dado, como `Message::Quit`, não podemos
desestruturar o valor mais adiante. Só podemos fazer correspondência com o valor
literal `Message::Quit`, e não há variáveis nesse padrão.

Para variantes de enum semelhantes a structs, como `Message::Move`, podemos
usar um padrão semelhante ao padrão que especificamos para corresponder structs.
Após o nome da variante, colocamos chaves e listamos os campos com variáveis
para separar as partes a serem usadas no código desse braço. Aqui usamos a
forma abreviada, como fizemos na Listagem 19-13.

Para variantes de enum semelhantes a tuplas, como `Message::Write` que guarda
uma tupla com um elemento e `Message::ChangeColor` que guarda uma tupla com
três elementos, o padrão é semelhante ao padrão que especificamos para
corresponder tuplas. O número de variáveis no padrão deve corresponder ao número
de elementos na variante que estamos correspondendo.

<!-- Old headings. Do not remove or links may break. -->

<a id="destructuring-nested-structs-and-enums"></a>
<a id="nested-structs-and-enums"></a>

#### Structs e Enums Aninhados

Até aqui, todos os nossos exemplos têm feito correspondência de structs ou enums
com um nível de profundidade, mas a correspondência também funciona em itens
aninhados! Por exemplo, podemos refatorar o código da Listagem 19-15 para
suportar cores RGB e HSV na mensagem `ChangeColor`, conforme mostrado na
Listagem 19-16.

<Listing number="19-16" caption="Correspondendo em enums aninhados">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-16/src/main.rs}}
```

</Listing>

O padrão do primeiro braço na expressão `match` corresponde a uma variante de
enum `Message::ChangeColor` que contém uma variante `Color::Rgb`; então, o
padrão se vincula aos três valores internos `i32`. O padrão do segundo braço
também corresponde a uma variante de enum `Message::ChangeColor`, mas o enum
interno corresponde a `Color::Hsv`. Podemos especificar essas condições
complexas em uma única expressão `match`, mesmo que dois enums estejam
envolvidos.

<!-- Old headings. Do not remove or links may break. -->

<a id="destructuring-structs-and-tuples"></a>
<a id="structs-and-tuples"></a>

#### Structs e Tuplas

Podemos misturar, combinar e aninhar padrões de desestruturação de maneiras
ainda mais complexas. O exemplo a seguir mostra uma desestruturação elaborada na
qual aninhamos structs e tuplas dentro de uma tupla e desestruturamos todos os
valores primitivos para fora:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-05-destructuring-structs-and-tuples/src/main.rs:here}}
```

Esse código nos permite quebrar tipos complexos em suas partes componentes para
que possamos usar separadamente os valores nos quais estamos interessados.

Desestruturar com padrões é uma maneira conveniente de usar partes de valores,
como o valor de cada campo em uma struct, separadamente umas das outras.

<a id="ignoring-values-in-a-pattern"></a>

### Ignorando Valores em um Padrão

Você já viu que às vezes é útil ignorar valores em um padrão, como no último
braço de um `match`, para obter um caso abrangente que na verdade não faz nada,
mas contempla todos os valores possíveis restantes. Existem algumas formas de
ignorar valores inteiros ou partes de valores em um padrão: usar o padrão `_`
(que você já viu), usar o padrão `_` dentro de outro padrão, usar um nome que
comece com um sublinhado ou usar `..` para ignorar as partes restantes de um
valor. Vamos explorar como e por que usar cada um desses padrões.

<!-- Old headings. Do not remove or links may break. -->

<a id="ignoring-an-entire-value-with-_"></a>
<a id="an-entire-value-with-_"></a>

#### Um Valor Inteiro com `_`

Usamos o sublinhado como um padrão curinga que corresponderá a qualquer valor,
mas não se vinculará ao valor. Isso é especialmente útil como o último braço em
uma expressão `match`, mas também podemos usá-lo em qualquer padrão, incluindo
parâmetros de função, conforme mostrado na Listagem 19-17.

<Listing number="19-17" file-name="src/main.rs" caption="Usando `_` em uma assinatura de função">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-17/src/main.rs}}
```

</Listing>

Esse código ignorará completamente o valor `3` passado como primeiro argumento e
imprimirá `This code only uses the y parameter: 4`.

Na maioria dos casos em que você não precisa mais de um determinado parâmetro de
função, você alteraria a assinatura para que ela não incluísse o parâmetro não
utilizado. Ignorar um parâmetro de função pode ser especialmente útil nos casos
em que, por exemplo, você está implementando um trait que exige uma certa
assinatura de tipos, mas o corpo da função na sua implementação não precisa de
um dos parâmetros. Você evita assim receber um aviso do compilador sobre
parâmetros de função não utilizados, como receberia se usasse um nome.

<!-- Old headings. Do not remove or links may break. -->

<a id="ignoring-parts-of-a-value-with-a-nested-_"></a>
<a id="parts-of-a-value-with-a-nested-_"></a>

#### Partes de um Valor com um `_` Aninhado

Também podemos usar `_` dentro de outro padrão para ignorar apenas parte de um
valor, por exemplo, quando queremos testar apenas parte de um valor, mas não
temos uso para as outras partes no código correspondente que queremos executar.
A Listagem 19-18 mostra um código responsável por gerenciar o valor de uma
configuração. Os requisitos de negócio são que a pessoa usuária não deve ter
permissão para sobrescrever uma personalização existente de uma configuração,
mas pode limpar a configuração e atribuir um valor a ela se ela estiver
atualmente indefinida.

<Listing number="19-18" caption="Usando um sublinhado dentro de padrões que correspondem a variantes `Some` quando não precisamos usar o valor dentro de `Some`">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-18/src/main.rs:here}}
```

</Listing>

Esse código imprimirá `Can't overwrite an existing customized value` e depois
`setting is Some(5)`. No primeiro braço do `match`, não precisamos corresponder
nem usar os valores dentro de nenhuma das variantes `Some`, mas precisamos
testar o caso em que `setting_value` e `new_setting_value` são a variante
`Some`. Nesse caso, imprimimos o motivo de não alterar `setting_value`, e ele
não é alterado.

Em todos os outros casos (se `setting_value` ou `new_setting_value` for
`None`), expressos pelo padrão `_` no segundo braço, queremos permitir que
`new_setting_value` se torne `setting_value`.

Também podemos usar sublinhados em vários lugares dentro de um único padrão para
ignorar valores específicos. A Listagem 19-19 mostra um exemplo de como ignorar
o segundo e o quarto valores em uma tupla de cinco itens.

<Listing number="19-19" caption="Ignorando múltiplas partes de uma tupla">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-19/src/main.rs:here}}
```

</Listing>

Esse código imprimirá `Some numbers: 2, 8, 32`, e os valores `4` e `16` serão
ignorados.

<!-- Old headings. Do not remove or links may break. -->

<a id="ignoring-an-unused-variable-by-starting-its-name-with-_"></a>
<a id="an-unused-variable-by-starting-its-name-with-_"></a>

#### Uma Variável Não Utilizada Começando Seu Nome com `_`

Se você criar uma variável, mas não a usar em nenhum lugar, o Rust normalmente
emitirá um aviso porque uma variável não utilizada pode ser um bug. No entanto,
às vezes é útil poder criar uma variável que você ainda não vai usar, como
quando está prototipando ou apenas começando um projeto. Nessa situação, você
pode dizer ao Rust para não avisá-lo sobre a variável não utilizada começando o
nome da variável com um sublinhado. Na Listagem 19-20, criamos duas variáveis
não utilizadas, mas quando compilamos esse código, devemos receber um aviso
apenas sobre uma delas.

<Listing number="19-20" file-name="src/main.rs" caption="Começando o nome de uma variável com um sublinhado para evitar receber avisos de variável não utilizada">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-20/src/main.rs}}
```

</Listing>

Aqui, recebemos um aviso por não usar a variável `y`, mas não recebemos um
aviso por não usar `_x`.

Observe que há uma diferença sutil entre usar apenas `_` e usar um nome que
começa com um sublinhado. A sintaxe `_x` ainda vincula o valor à variável, ao
passo que `_` não vincula de jeito nenhum. Para mostrar um caso em que essa
distinção importa, a Listagem 19-21 nos dará um erro.

<Listing number="19-21" caption="Uma variável não utilizada começando com um sublinhado ainda vincula o valor, o que pode tomar *ownership* do valor.">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-21/src/main.rs:here}}
```

</Listing>

Receberemos um erro porque o valor `s` ainda será movido para `_s`, o que nos
impede de usar `s` novamente. Contudo, usar apenas o sublinhado por si só nunca
vincula ao valor. A Listagem 19-22 compilará sem nenhum erro porque `s` não é
movido para `_`.

<Listing number="19-22" caption="Usar apenas um sublinhado não vincula o valor.">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-22/src/main.rs:here}}
```

</Listing>

Esse código funciona perfeitamente porque nunca vinculamos `s` a nada; ele não
é movido.

<a id="ignoring-remaining-parts-of-a-value-with-"></a>
<a id="remaining-parts-of-a-value-with-"></a>

#### Partes Restantes de um Valor com `..`

Com valores que têm muitas partes, podemos usar a sintaxe `..` para usar partes
específicas e ignorar o resto, evitando a necessidade de listar sublinhados
para cada valor ignorado. O padrão `..` ignora quaisquer partes de um valor que
não tenhamos correspondido explicitamente no restante do padrão. Na Listagem
19-23, temos uma struct `Point` que armazena uma coordenada no espaço
tridimensional. Na expressão `match`, queremos operar apenas na coordenada `x` e
ignorar os valores nos campos `y` e `z`.

<Listing number="19-23" caption="Ignorando todos os campos de um `Point` exceto `x` usando `..`">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-23/src/main.rs:here}}
```

</Listing>

Listamos o valor `x` e, em seguida, apenas incluímos o padrão `..`. Isso é mais
rápido do que ter que listar `y: _` e `z: _`, particularmente quando estamos
trabalhando com structs que têm muitos campos em situações nas quais apenas um
ou dois campos são relevantes.

A sintaxe `..` se expandirá para quantos valores forem necessários. A Listagem
19-24 mostra como usar `..` com uma tupla.

<Listing number="19-24" file-name="src/main.rs" caption="Correspondendo apenas o primeiro e o último valor em uma tupla e ignorando todos os outros valores">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-24/src/main.rs}}
```

</Listing>

Nesse código, o primeiro e o último valores correspondem a `first` e `last`. O
`..` corresponderá e ignorará tudo o que estiver no meio.

Contudo, o uso de `..` não deve ser ambíguo. Se não estiver claro quais valores
devem ser correspondidos e quais devem ser ignorados, o Rust nos dará um erro. A
Listagem 19-25 mostra um exemplo de uso ambíguo de `..`, portanto ele não
compilará.

<Listing number="19-25" file-name="src/main.rs" caption="Uma tentativa de usar `..` de forma ambígua">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-25/src/main.rs}}
```

</Listing>

Quando compilamos esse exemplo, obtemos este erro:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-25/output.txt}}
```

É impossível para o Rust determinar quantos valores na tupla devem ser
ignorados antes de corresponder um valor com `second` e, depois, quantos
outros valores ignorar em seguida. Esse código poderia significar que queremos
ignorar `2`, vincular `second` a `4` e então ignorar `8`, `16` e `32`; ou que
queremos ignorar `2` e `4`, vincular `second` a `8` e então ignorar `16` e `32`;
e assim por diante. O nome de variável `second` não significa nada de especial
para o Rust, então recebemos um erro do compilador porque usar `..` em dois
lugares assim é ambíguo.

<!-- Old headings. Do not remove or links may break. -->

<a id="extra-conditionals-with-match-guards"></a>
<a id="adding-conditionals-with-match-guards"></a>

### Adicionando Condicionais com Guardas de Match

Uma *guarda de match* (*match guard*) é uma condição `if` adicional,
especificada após o padrão em um braço de `match`, que também precisa
corresponder para que aquele braço seja escolhido. Guardas de match são úteis
para expressar ideias mais complexas do que um padrão sozinho permite. Note,
porém, que elas estão disponíveis apenas em expressões `match`, e não em
expressões `if let` ou `while let`.

A condição pode usar variáveis criadas no padrão. A Listagem 19-26 mostra um
`match` em que o primeiro braço tem o padrão `Some(x)` e também tem uma guarda
de match `if x % 2 == 0` (que será `true` se o número for par).

<Listing number="19-26" caption="Adicionando uma guarda de match a um padrão">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-26/src/main.rs:here}}
```

</Listing>

Este exemplo imprimirá `The number 4 is even`. Quando `num` é comparado ao
padrão no primeiro braço, ele corresponde porque `Some(4)` corresponde a
`Some(x)`. Em seguida, a guarda de match verifica se o resto da divisão de `x`
por 2 é igual a 0 e, como é, o primeiro braço é selecionado.

Se `num` tivesse sido `Some(5)` em vez disso, a guarda de match no primeiro
braço teria sido `false` porque o resto de 5 dividido por 2 é 1, que não é
igual a 0. O Rust então passaria para o segundo braço, que corresponderia porque
o segundo braço não tem uma guarda de match e, portanto, corresponde a qualquer
variante `Some`.

Não há maneira de expressar a condição `if x % 2 == 0` dentro de um padrão,
então a guarda de match nos dá a capacidade de expressar essa lógica. A
desvantagem dessa expressividade adicional é que o compilador não tenta
verificar a exaustividade quando expressões de guarda de match estão envolvidas.

Ao discutir a Listagem 19-11, mencionamos que poderíamos usar guardas de match
para resolver nosso problema de sombreamento de padrão. Lembre-se de que
criamos uma nova variável dentro do padrão na expressão `match` em vez de usar
a variável de fora do `match`. Essa nova variável significava que não podíamos
testar em relação ao valor da variável externa. A Listagem 19-27 mostra como
podemos usar uma guarda de match para corrigir esse problema.

<Listing number="19-27" file-name="src/main.rs" caption="Usando uma guarda de match para testar igualdade com uma variável externa">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-27/src/main.rs}}
```

</Listing>

Esse código agora imprimirá `Default case, x = Some(5)`. O padrão no segundo
braço do `match` não introduz uma nova variável `y` que sombrearia o `y`
externo, o que significa que podemos usar o `y` externo na guarda de match. Em
vez de especificar o padrão como `Some(y)`, o que teria sombreado o `y` externo,
especificamos `Some(n)`. Isso cria uma nova variável `n` que não sombreia nada
porque não há nenhuma variável `n` fora do `match`.

A guarda de match `if n == y` não é um padrão e, portanto, não introduz novas
variáveis. Esse `y` *é* o `y` externo em vez de um novo `y` sombreando-o, e
podemos verificar se há um valor igual ao `y` externo comparando `n` com `y`.

Você também pode usar o operador *ou* `|` em uma guarda de match para
especificar múltiplos padrões; a condição da guarda de match se aplicará a
todos os padrões. A Listagem 19-28 mostra a precedência ao combinar um padrão
que usa `|` com uma guarda de match. A parte importante desse exemplo é que a
guarda de match `if y` se aplica a `4`, `5` *e* `6`, mesmo que possa parecer
que `if y` se aplica apenas a `6`.

<Listing number="19-28" caption="Combinando múltiplos padrões com uma guarda de match">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-28/src/main.rs:here}}
```

</Listing>

A condição de correspondência afirma que o braço só corresponde se o valor de
`x` for igual a `4`, `5` ou `6` *e* se `y` for `true`. Quando esse código é
executado, o padrão do primeiro braço corresponde porque `x` é `4`, mas a guarda
de match `if y` é `false`, então o primeiro braço não é escolhido. O código
avança para o segundo braço, que corresponde, e este programa imprime `no`. O
motivo é que a condição `if` se aplica a todo o padrão `4 | 5 | 6`, e não apenas
ao último valor `6`. Em outras palavras, a precedência de uma guarda de match em
relação a um padrão se comporta assim:

```text
(4 | 5 | 6) if y => ...
```

em vez disto:

```text
4 | 5 | (6 if y) => ...
```

Depois de executar o código, o comportamento de precedência fica evidente: se a
guarda de match fosse aplicada apenas ao valor final na lista de valores
especificada usando o operador `|`, o braço teria correspondido e o programa
teria impresso `yes`.

<!-- Old headings. Do not remove or links may break. -->

<a id="-bindings"></a>
<a id="using--bindings"></a>

### Usando Vínculos com `@`

O operador *arroba* (*at*) `@` nos permite criar uma variável que guarda um
valor ao mesmo tempo em que testamos esse valor para uma correspondência de
padrão. Na Listagem 19-29, queremos testar se o campo `id` de `Message::Hello`
está dentro do intervalo `3..=7`. Também queremos vincular o valor à variável
`id` para podermos usá-lo no código associado ao braço.

<Listing number="19-29" caption="Usando `@` para vincular a um valor em um padrão enquanto também o testamos">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-29/src/main.rs:here}}
```

</Listing>

Este exemplo imprimirá `Found an id in range: 5`. Ao especificar `id @` antes
do intervalo `3..=7`, estamos capturando qualquer valor que correspondeu ao
intervalo em uma variável chamada `id` enquanto também testamos se o valor
correspondeu ao padrão de intervalo.

No segundo braço, em que temos apenas um intervalo especificado no padrão, o
código associado ao braço não tem uma variável que contenha o valor real do
campo `id`. O valor do campo `id` poderia ter sido 10, 11 ou 12, mas o código
que acompanha esse padrão não sabe qual é. O código do padrão não consegue usar
o valor do campo `id` porque não salvamos o valor de `id` em uma variável.

No último braço, em que especificamos uma variável sem um intervalo, temos sim
o valor disponível para usar no código do braço em uma variável chamada `id`. O
motivo é que usamos a sintaxe abreviada de campo de struct. Mas não aplicamos
nenhum teste ao valor no campo `id` nesse braço, como fizemos nos dois
primeiros braços: qualquer valor corresponderia a esse padrão.

Usar `@` nos permite testar um valor e salvá-lo em uma variável dentro de um
único padrão.

<a id="summary"></a>

## Resumo

Os padrões do Rust são muito úteis para distinguir entre diferentes tipos de
dados. Quando usados em expressões `match`, o Rust garante que seus padrões
cubram todos os valores possíveis, ou seu programa não compilará. Padrões em
instruções `let` e parâmetros de função tornam essas construções mais úteis,
permitindo a desestruturação de valores em partes menores e a atribuição dessas
partes a variáveis. Podemos criar padrões simples ou complexos para atender às
nossas necessidades.

A seguir, para o penúltimo capítulo do livro, veremos alguns aspectos avançados
de vários recursos do Rust.
