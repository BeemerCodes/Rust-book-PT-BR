<a id="methods"></a>
## Métodos

Métodos são parecidos com funções: declaramos eles com a palavra-chave `fn` e um
nome, eles podem ter parâmetros e um valor de retorno, e contêm código que é
executado quando o método é chamado de algum outro lugar. Diferente das funções,
porém, métodos são definidos dentro do contexto de uma struct (ou de um enum ou
de um _trait object_, que veremos no [Capítulo 6][enums]<!-- ignore --> e no
[Capítulo 18][trait-objects]<!-- ignore -->, respectivamente), e o primeiro
parâmetro deles é sempre `self`, que representa a instância da struct na qual o
método está sendo chamado.

<!-- Old headings. Do not remove or links may break. -->

<a id="defining-methods"></a>

<a id="method-syntax"></a>
### Sintaxe de Métodos

Vamos alterar a função `area`, que recebe uma instância de `Rectangle` como
parâmetro, e transformá-la em um método `area` definido na struct `Rectangle`,
como mostrado na Listagem 5-13.

<Listing number="5-13" file-name="src/main.rs" caption="Definindo um método `area` na struct `Rectangle`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-13/src/main.rs}}
```

</Listing>

Para definir a função dentro do contexto de `Rectangle`, abrimos um bloco `impl`
(de _implementation_, implementação) para `Rectangle`. Tudo o que estiver dentro
desse bloco `impl` ficará associado ao tipo `Rectangle`. Em seguida, movemos a
função `area` para dentro das chaves do `impl` e mudamos o primeiro (e, neste
caso, único) parâmetro para `self` na assinatura e em todo o corpo da função. Em
`main`, onde antes chamávamos a função `area` passando `rect1` como argumento,
podemos agora usar a _sintaxe de método_ para chamar o método `area` na nossa
instância de `Rectangle`. A sintaxe de método vem logo após uma instância:
adicionamos um ponto seguido do nome do método, parênteses e quaisquer
argumentos.

Na assinatura de `area`, usamos `&self` em vez de `rectangle: &Rectangle`. Na
verdade, `&self` é uma abreviação para `self: &Self`. Dentro de um bloco `impl`,
o tipo `Self` é um apelido (_alias_) para o tipo ao qual o bloco `impl` se
refere. Métodos precisam ter um parâmetro chamado `self` do tipo `Self` como
primeiro parâmetro, então o Rust permite abreviar isso escrevendo apenas `self`
na primeira posição. Note que ainda precisamos usar o `&` na frente da
abreviação `self` para indicar que esse método pega a instância de `Self`
emprestada, assim como fizemos em `rectangle: &Rectangle`. Métodos podem assumir
o ownership de `self`, pegar `self` emprestado de forma imutável (como fizemos
aqui) ou pegar `self` emprestado de forma mutável, exatamente como acontece com
qualquer outro parâmetro.

Escolhemos `&self` aqui pelo mesmo motivo que usamos `&Rectangle` na versão com
função: não queremos assumir o ownership, apenas ler os dados da struct sem
modificá-los. Se quiséssemos alterar a instância na qual chamamos o método como
parte do que o método faz, usaríamos `&mut self` como primeiro parâmetro. Ter um
método que assume o ownership da instância usando apenas `self` como primeiro
parâmetro é raro; essa técnica geralmente é usada quando o método transforma
`self` em outra coisa e você quer impedir que quem chamou o método continue
usando a instância original após a transformação.

O principal motivo para usar métodos em vez de funções — além de oferecer a
sintaxe de método e evitar repetir o tipo de `self` na assinatura de cada método
— é a organização. Colocamos todas as operações que podemos fazer com uma
instância de um tipo em um único bloco `impl`, em vez de obrigar futuros
usuários do nosso código a procurar os recursos de `Rectangle` espalhados por
vários lugares da biblioteca que fornecemos.

Note que podemos dar a um método o mesmo nome de um dos campos da struct. Por
exemplo, podemos definir em `Rectangle` um método que também se chama `width`:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-06-method-field-interaction/src/main.rs:here}}
```

</Listing>

Aqui, decidimos fazer o método `width` retornar `true` se o valor do campo
`width` da instância for maior que `0`, e `false` se o valor for `0`: podemos
usar um campo dentro de um método com o mesmo nome para qualquer finalidade. Em
`main`, quando colocamos parênteses depois de `rect1.width`, o Rust sabe que
estamos nos referindo ao método `width`. Quando não usamos parênteses, o Rust
sabe que estamos nos referindo ao campo `width`.

Frequentemente — mas nem sempre — quando damos a um método o mesmo nome de um
campo, queremos que ele apenas retorne o valor do campo e não faça mais nada.
Métodos assim são chamados de _getters_, e o Rust não os implementa
automaticamente para campos de structs como algumas outras linguagens fazem.
Getters são úteis porque permitem deixar o campo privado e o método público,
oferecendo acesso somente leitura a esse campo como parte da API pública do
tipo. Vamos discutir o que significa público e privado e como marcar um campo ou
método dessa forma no [Capítulo 7][public]<!-- ignore -->.

> <a id="wheres-the---operator"></a>
> ### Onde Está o Operador `->`?
>
> Em C e C++, dois operadores diferentes são usados para chamar métodos: você
> usa `.` se estiver chamando um método diretamente no objeto e `->` se estiver
> chamando o método em um ponteiro para o objeto e precisar desreferenciar o
> ponteiro primeiro. Em outras palavras, se `object` é um ponteiro,
> `object->something()` é equivalente a `(*object).something()`.
>
> O Rust não tem um equivalente ao operador `->`; em vez disso, o Rust possui um
> recurso chamado _referenciamento e desreferenciamento automático_. A chamada
> de métodos é um dos poucos lugares no Rust onde esse comportamento acontece.
>
> Funciona assim: quando você chama um método com `object.something()`, o Rust
> adiciona automaticamente `&`, `&mut` ou `*` para que `object` corresponda à
> assinatura do método. Em outras palavras, as duas linhas a seguir são
> equivalentes:
>
> <!-- CAN'T EXTRACT SEE BUG https://github.com/rust-lang/mdBook/issues/1127 -->
>
> ```rust
> # #[derive(Debug,Copy,Clone)]
> # struct Point {
> #     x: f64,
> #     y: f64,
> # }
> #
> # impl Point {
> #    fn distance(&self, other: &Point) -> f64 {
> #        let x_squared = f64::powi(other.x - self.x, 2);
> #        let y_squared = f64::powi(other.y - self.y, 2);
> #
> #        f64::sqrt(x_squared + y_squared)
> #    }
> # }
> # let p1 = Point { x: 0.0, y: 0.0 };
> # let p2 = Point { x: 5.0, y: 6.5 };
> p1.distance(&p2);
> (&p1).distance(&p2);
> ```
>
> A primeira forma fica muito mais limpa. Esse comportamento de referenciamento
> automático funciona porque métodos têm um receptor bem definido — o tipo de
> `self`. Com o receptor e o nome do método em mãos, o Rust consegue determinar
> com precisão se o método está lendo (`&self`), modificando (`&mut self`) ou
> consumindo (`self`). O fato de o Rust tornar o borrowing implícito para
> receptores de métodos é uma grande parte do que torna o ownership ergonômico na
> prática.

<a id="methods-with-more-parameters"></a>
### Métodos com Mais Parâmetros

Vamos praticar o uso de métodos implementando um segundo método na struct
`Rectangle`. Desta vez, queremos que uma instância de `Rectangle` receba outra
instância de `Rectangle` e retorne `true` se o segundo `Rectangle` couber
completamente dentro de `self` (o primeiro `Rectangle`); caso contrário, deve
retornar `false`. Ou seja, depois de definirmos o método `can_hold`, queremos
conseguir escrever o programa mostrado na Listagem 5-14.

<Listing number="5-14" file-name="src/main.rs" caption="Usando o método `can_hold`, que ainda não foi escrito">

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-14/src/main.rs}}
```

</Listing>

A saída esperada seria a seguinte, já que ambas as dimensões de `rect2` são
menores que as de `rect1`, mas `rect3` é mais largo que `rect1`:

```text
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```

Sabemos que queremos definir um método, então ele vai ficar dentro do bloco
`impl Rectangle`. O nome do método será `can_hold` e ele receberá um empréstimo
imutável de outro `Rectangle` como parâmetro. Podemos saber qual será o tipo do
parâmetro olhando para o código que chama o método: `rect1.can_hold(&rect2)`
passa `&rect2`, que é um empréstimo imutável para `rect2`, uma instância de
`Rectangle`. Isso faz sentido porque só precisamos ler `rect2` (em vez de
modificá-lo, o que exigiria um empréstimo mutável), e queremos que `main`
mantenha o ownership de `rect2` para poder usá-lo novamente depois de chamar o
método `can_hold`. O valor de retorno de `can_hold` será um booleano, e a
implementação vai verificar se a largura e a altura de `self` são maiores que a
largura e a altura do outro `Rectangle`, respectivamente. Vamos adicionar o novo
método `can_hold` ao bloco `impl` da Listagem 5-13, como mostrado na Listagem
5-15.

<Listing number="5-15" file-name="src/main.rs" caption="Implementando o método `can_hold` em `Rectangle` recebendo outra instância de `Rectangle` como parâmetro">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-15/src/main.rs:here}}
```

</Listing>

Ao executar esse código com a função `main` da Listagem 5-14, obteremos a saída
esperada. Métodos podem receber múltiplos parâmetros que adicionamos na
assinatura após o parâmetro `self`, e esses parâmetros funcionam exatamente como
parâmetros de funções.

<a id="associated-functions"></a>
### Funções Associadas

Todas as funções definidas dentro de um bloco `impl` são chamadas de _funções
associadas_ porque estão associadas ao tipo nomeado após o `impl`. Podemos
definir funções associadas que não têm `self` como primeiro parâmetro (e,
portanto, não são métodos) porque não precisam de uma instância do tipo para
funcionar. Já usamos uma função assim: a função `String::from`, definida no tipo
`String`.

Funções associadas que não são métodos são frequentemente usadas como
construtores que retornam uma nova instância da struct. Elas costumam se chamar
`new`, mas `new` não é um nome especial nem faz parte da sintaxe da linguagem.
Por exemplo, poderíamos oferecer uma função associada chamada `square` que
recebe um único parâmetro de dimensão e o usa tanto para largura quanto para
altura, facilitando a criação de um `Rectangle` quadrado sem precisar informar o
mesmo valor duas vezes:

<span class="filename">Arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-03-associated-functions/src/main.rs:here}}
```

As palavras-chave `Self` no tipo de retorno e no corpo da função são apelidos
para o tipo que aparece após a palavra-chave `impl`, que neste caso é
`Rectangle`.

Para chamar essa função associada, usamos a sintaxe `::` com o nome da struct;
`let sq = Rectangle::square(3);` é um exemplo. Essa função fica no _namespace_
da struct: a sintaxe `::` é usada tanto para funções associadas quanto para
namespaces criados por módulos. Vamos discutir módulos no [Capítulo
7][modules]<!-- ignore -->.

<a id="multiple-impl-blocks"></a>
### Múltiplos Blocos `impl`

Cada struct pode ter múltiplos blocos `impl`. Por exemplo, a Listagem 5-15 é
equivalente ao código mostrado na Listagem 5-16, que coloca cada método em seu
próprio bloco `impl`.

<Listing number="5-16" caption="Reescrevendo a Listagem 5-15 usando múltiplos blocos `impl`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-16/src/main.rs:here}}
```

</Listing>

Não há motivo para separar esses métodos em múltiplos blocos `impl` aqui, mas
essa é uma sintaxe válida. Veremos um caso em que múltiplos blocos `impl` são
úteis no Capítulo 10, quando discutirmos tipos genéricos e traits.

<a id="summary"></a>
## Resumo

Structs permitem criar tipos personalizados que fazem sentido para o domínio do
seu problema. Usando structs, você consegue manter dados relacionados conectados
entre si e nomear cada parte para deixar o código claro. Em blocos `impl`, você
pode definir funções associadas ao seu tipo, e métodos são um tipo de função
associada que permite especificar o comportamento que as instâncias das suas
structs possuem.

Mas structs não são a única forma de criar tipos personalizados: vamos agora
conhecer os enums do Rust para adicionar mais uma ferramenta à sua caixa de
ferramentas.

[enums]: ch06-00-enums.html
[trait-objects]: ch18-02-trait-objects.md
[public]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#exposing-paths-with-the-pub-keyword
[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
