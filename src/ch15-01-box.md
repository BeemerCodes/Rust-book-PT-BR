<a id="using-boxt-to-point-to-data-on-the-heap"></a>
## Usando `Box<T>` para Apontar para Dados no *Heap*

O *smart pointer* mais simples é o *box*, cujo tipo é escrito como `Box<T>`.
*Boxes* permitem que você armazene dados no *heap* em vez de na *stack*. O que
fica na *stack* é o ponteiro para os dados no *heap*. Consulte o Capítulo 4 para
revisar a diferença entre *stack* e *heap*.

*Boxes* não têm custo de desempenho, além de armazenar seus dados no *heap* em
vez de na *stack*. Mas também não possuem muitas capacidades extras. Você os
usará com mais frequência nessas situações:

- Quando você tem um tipo cujo tamanho não pode ser conhecido em tempo de
  compilação, e você deseja usar um valor desse tipo em um contexto que requer
  um tamanho exato
- Quando você tem uma grande quantidade de dados e quer transferir a propriedade
  (*ownership*) sem garantir que os dados serão copiados
- Quando você deseja possuir um valor e se importa apenas que ele é de um tipo
  que implementa uma determinada *trait*, em vez de ser de um tipo específico

Demonstraremos a primeira situação em ["Habilitando Tipos Recursivos com
*Boxes*"](#enabling-recursive-types-with-boxes)<!-- ignore -->. No segundo caso,
transferir a propriedade de uma grande quantidade de dados pode demorar bastante
porque os dados são copiados na *stack*. Para melhorar o desempenho nessa
situação, podemos armazenar a grande quantidade de dados no *heap* dentro de um
*box*. Assim, apenas a pequena quantidade de dados de ponteiro é copiada na
*stack*, enquanto os dados referenciados permanecem em um único lugar no *heap*.
O terceiro caso é conhecido como um *objeto trait* (*trait object*), e
["Usando Objetos Trait para Abstrair Comportamentos
Compartilhados"][trait-objects]<!-- ignore --> no Capítulo 18 é dedicado a esse
tópico. Então, o que você aprender aqui você aplicará novamente naquela seção!

<!-- Old headings. Do not remove or links may break. -->

<a id="using-boxt-to-store-data-on-the-heap"></a>

<a id="storing-data-on-the-heap"></a>
### Armazenando Dados no *Heap*

Antes de discutir o caso de uso de armazenamento no *heap* para `Box<T>`,
abordaremos a sintaxe e como interagir com valores armazenados dentro de um
`Box<T>`.

A Listagem 15-1 mostra como usar um *box* para armazenar um valor `i32` no
*heap*.

<Listing number="15-1" file-name="src/main.rs" caption="Armazenando um valor `i32` no *heap* usando um *box*">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-01/src/main.rs}}
```

</Listing>

Definimos a variável `b` para ter o valor de um `Box` que aponta para o valor
`5`, o qual é alocado no *heap*. Este programa imprimirá `b = 5`; nesse caso,
podemos acessar os dados no *box* de maneira semelhante a como faríamos se esses
dados estivessem na *stack*. Assim como qualquer valor possuído, quando um *box*
sai de escopo, como `b` faz ao final de `main`, ele será desalocado. A
desalocação acontece tanto para o *box* (armazenado na *stack*) quanto para os
dados para os quais ele aponta (armazenados no *heap*).

Colocar um único valor no *heap* não é muito útil, então você não usará *boxes*
sozinhos dessa forma com muita frequência. Ter valores como um único `i32` na
*stack*, onde eles são armazenados por padrão, é mais apropriado na maioria das
situações. Vamos olhar um caso em que *boxes* nos permitem definir tipos que não
poderíamos definir sem eles.

<a id="enabling-recursive-types-with-boxes"></a>
### Habilitando Tipos Recursivos com *Boxes*

Um valor de um *tipo recursivo* pode ter outro valor do mesmo tipo como parte de
si mesmo. Tipos recursivos representam um problema porque o Rust precisa saber
em tempo de compilação quanto espaço um tipo ocupa. No entanto, o aninhamento de
valores de tipos recursivos poderia teoricamente continuar infinitamente, então
o Rust não consegue saber quanto espaço o valor precisa. Como os *boxes* têm um
tamanho conhecido, podemos habilitar tipos recursivos inserindo um *box* na
definição do tipo recursivo.

Como exemplo de tipo recursivo, vamos explorar a *cons list*. Este é um tipo de
dado comumente encontrado em linguagens de programação funcional. O tipo de *cons
list* que definiremos é simples, exceto pela recursão; portanto, os conceitos no
exemplo com que trabalharemos serão úteis sempre que você se deparar com
situações mais complexas envolvendo tipos recursivos.

<!-- Old headings. Do not remove or links may break. -->

<a id="more-information-about-the-cons-list"></a>

<a id="understanding-the-cons-list"></a>
#### Entendendo a *Cons List*

Uma *cons list* é uma estrutura de dados que vem da linguagem de programação Lisp
e seus dialetos, é composta de pares aninhados e é a versão Lisp de uma lista
encadeada. Seu nome vem da função `cons` (abreviação de *construct function*) em
Lisp que constrói um novo par a partir de seus dois argumentos. Ao chamar `cons`
em um par composto por um valor e outro par, podemos construir *cons lists* feitas
de pares recursivos.

Por exemplo, aqui está uma representação em pseudocódigo de uma *cons list*
contendo a lista `1, 2, 3` com cada par entre parênteses:

```text
(1, (2, (3, Nil)))
```

Cada item em uma *cons list* contém dois elementos: o valor do item atual e o
próximo item. O último item da lista contém apenas um valor chamado `Nil` sem
um próximo item. Uma *cons list* é produzida chamando recursivamente a função
`cons`. O nome canônico para denotar o caso base da recursão é `Nil`. Observe
que isso não é o mesmo que o conceito de "null" ou "nil" discutido no Capítulo
6, que é um valor inválido ou ausente.

A *cons list* não é uma estrutura de dados usada com frequência em Rust. Na
maioria das vezes quando você tem uma lista de itens em Rust, `Vec<T>` é uma
escolha melhor. Outros tipos de dados recursivos mais complexos *são* úteis em
várias situações, mas ao começar com a *cons list* neste capítulo, podemos
explorar como os *boxes* nos permitem definir um tipo de dado recursivo sem muita
distração.

A Listagem 15-2 contém uma definição de *enum* para uma *cons list*. Observe que
este código ainda não compilará, porque o tipo `List` não tem um tamanho
conhecido, o que demonstraremos.

<Listing number="15-2" file-name="src/main.rs" caption="A primeira tentativa de definir um *enum* para representar uma estrutura de dados de *cons list* de valores `i32`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-02/src/main.rs:here}}
```

</Listing>

> Nota: Estamos implementando uma *cons list* que armazena apenas valores `i32`
> para os fins deste exemplo. Poderíamos tê-la implementado usando genéricos,
> como discutimos no Capítulo 10, para definir um tipo de *cons list* que pudesse
> armazenar valores de qualquer tipo.

Usar o tipo `List` para armazenar a lista `1, 2, 3` seria como o código na
Listagem 15-3.

<Listing number="15-3" file-name="src/main.rs" caption="Usando o *enum* `List` para armazenar a lista `1, 2, 3`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-03/src/main.rs:here}}
```

</Listing>

O primeiro valor `Cons` armazena `1` e outro valor `List`. Esse valor `List` é
outro valor `Cons` que armazena `2` e outro valor `List`. Esse valor `List` é
mais um valor `Cons` que armazena `3` e um valor `List`, que é finalmente `Nil`,
a variante não recursiva que sinaliza o fim da lista.

Se tentarmos compilar o código da Listagem 15-3, obteremos o erro mostrado na
Listagem 15-4.

<Listing number="15-4" caption="O erro que obtemos ao tentar definir um *enum* recursivo">

```console
{{#include ../listings/ch15-smart-pointers/listing-15-03/output.txt}}
```

</Listing>

O erro mostra que esse tipo "tem tamanho infinito." A razão é que definimos
`List` com uma variante que é recursiva: ela armazena diretamente outro valor de
si mesma. Como resultado, o Rust não consegue descobrir quanto espaço precisa
para armazenar um valor `List`. Vamos entender por que obtemos esse erro. Primeiro,
vamos ver como o Rust decide quanto espaço precisa para armazenar um valor de
um tipo não recursivo.

<a id="computing-the-size-of-a-non-recursive-type"></a>
#### Calculando o Tamanho de um Tipo Não Recursivo

Lembre-se do *enum* `Message` que definimos na Listagem 6-2 quando discutimos
definições de *enum* no Capítulo 6:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-02/src/main.rs:here}}
```

Para determinar quanto espaço alocar para um valor `Message`, o Rust percorre
cada uma das variantes para ver qual variante precisa de mais espaço. O Rust vê
que `Message::Quit` não precisa de nenhum espaço, `Message::Move` precisa de
espaço suficiente para armazenar dois valores `i32`, e assim por diante. Como
apenas uma variante será usada, o máximo de espaço que um valor `Message`
precisará é o espaço que levaria para armazenar a maior de suas variantes.

Compare isso com o que acontece quando o Rust tenta determinar quanto espaço um
tipo recursivo como o *enum* `List` na Listagem 15-2 precisa. O compilador começa
olhando para a variante `Cons`, que contém um valor do tipo `i32` e um valor do
tipo `List`. Portanto, `Cons` precisa de uma quantidade de espaço igual ao
tamanho de um `i32` mais o tamanho de um `List`. Para descobrir quanta memória o
tipo `List` precisa, o compilador olha para as variantes, começando pela variante
`Cons`. A variante `Cons` armazena um valor do tipo `i32` e um valor do tipo
`List`, e esse processo continua infinitamente, como mostrado na Figura 15-1.

<img alt="Uma Cons list infinita: um retângulo rotulado como 'Cons' dividido em dois retângulos menores. O primeiro retângulo menor contém o rótulo 'i32', e o segundo retângulo menor contém o rótulo 'Cons' e uma versão menor do retângulo externo 'Cons'. Os retângulos 'Cons' continuam a conter versões cada vez menores de si mesmos até que o menor retângulo de tamanho confortável contenha um símbolo de infinito, indicando que essa repetição continua para sempre." src="img/trpl15-01.svg" class="center" style="width: 50%;" />

<span class="caption">Figura 15-1: Uma `List` infinita consistindo de infinitas
variantes `Cons`</span>

<!-- Old headings. Do not remove or links may break. -->

<a id="using-boxt-to-get-a-recursive-type-with-a-known-size"></a>

<a id="getting-a-recursive-type-with-a-known-size"></a>
#### Obtendo um Tipo Recursivo com Tamanho Conhecido

Como o Rust não consegue descobrir quanto espaço alocar para tipos definidos
recursivamente, o compilador gera um erro com esta sugestão útil:

<!-- manual-regeneration
after doing automatic regeneration, look at listings/ch15-smart-pointers/listing-15-03/output.txt and copy the relevant line
-->

```text
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
  |
2 |     Cons(i32, Box<List>),
  |               ++++    +
```

Nessa sugestão, *indireção* significa que, em vez de armazenar um valor
diretamente, devemos alterar a estrutura de dados para armazenar o valor
indiretamente, guardando um ponteiro para o valor.

Como um `Box<T>` é um ponteiro, o Rust sempre sabe quanto espaço um `Box<T>`
precisa: o tamanho de um ponteiro não muda com base na quantidade de dados para
os quais ele aponta. Isso significa que podemos colocar um `Box<T>` dentro da
variante `Cons` em vez de outro valor `List` diretamente. O `Box<T>` apontará
para o próximo valor `List` que ficará no *heap* em vez de dentro da variante
`Cons`. Conceitualmente, ainda temos uma lista, criada com listas que contêm
outras listas, mas essa implementação agora é mais parecida com colocar os itens
lado a lado em vez de dentro uns dos outros.

Podemos mudar a definição do *enum* `List` na Listagem 15-2 e o uso de `List`
na Listagem 15-3 para o código da Listagem 15-5, que compilará.

<Listing number="15-5" file-name="src/main.rs" caption="A definição de `List` que usa `Box<T>` para ter um tamanho conhecido">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-05/src/main.rs}}
```

</Listing>

A variante `Cons` precisa do tamanho de um `i32` mais o espaço para armazenar
os dados de ponteiro do *box*. A variante `Nil` não armazena nenhum valor,
portanto precisa de menos espaço na *stack* do que a variante `Cons`. Agora
sabemos que qualquer valor `List` ocupará o tamanho de um `i32` mais o tamanho
dos dados de ponteiro de um *box*. Ao usar um *box*, quebramos a cadeia
infinita e recursiva, para que o compilador consiga descobrir o tamanho que
precisa para armazenar um valor `List`. A Figura 15-2 mostra como a variante
`Cons` aparece agora.

<img alt="Um retângulo rotulado como 'Cons' dividido em dois retângulos menores. O primeiro retângulo menor contém o rótulo 'i32', e o segundo retângulo menor contém o rótulo 'Box' com um retângulo interno que contém o rótulo 'usize', representando o tamanho finito do ponteiro do box." src="img/trpl15-02.svg" class="center" />

<span class="caption">Figura 15-2: Uma `List` que não tem tamanho infinito
porque `Cons` armazena um `Box`</span>

*Boxes* fornecem apenas indireção e alocação no *heap*; eles não têm nenhuma
outra capacidade especial, como as que veremos com os outros tipos de *smart
pointers*. Também não têm o custo de desempenho que essas capacidades especiais
acarretam, portanto podem ser úteis em casos como a *cons list*, em que a
indireção é o único recurso necessário. Veremos mais casos de uso para *boxes*
no Capítulo 18.

O tipo `Box<T>` é um *smart pointer* porque implementa a *trait* `Deref`, o que
permite que valores `Box<T>` sejam tratados como referências. Quando um valor
`Box<T>` sai de escopo, os dados do *heap* para os quais o *box* aponta também
são limpos devido à implementação da *trait* `Drop`. Essas duas *traits* serão
ainda mais importantes para a funcionalidade fornecida pelos outros tipos de
*smart pointers* que discutiremos no restante deste capítulo. Vamos explorar
essas duas *traits* com mais detalhes.

[trait-objects]: ch18-02-trait-objects.html#using-trait-objects-to-abstract-over-shared-behavior
