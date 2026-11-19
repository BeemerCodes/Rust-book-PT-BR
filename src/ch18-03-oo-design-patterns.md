<a id="implementing-an-object-oriented-design-pattern"></a>

## Implementando um Padrão de Projeto Orientado a Objetos

O *padrão de estado* (*state pattern*) é um padrão de projeto orientado a
objetos. O ponto central desse padrão é definirmos internamente um conjunto de
estados que um valor pode ter. Os estados são representados por um conjunto de
*objetos de estado*, e o comportamento do valor muda com base no seu estado.
Vamos trabalhar em um exemplo de uma struct de postagem de blog que tem um
campo para guardar seu estado, que será um objeto de estado do conjunto
“rascunho” (*draft*), “revisão” (*review*) ou “publicado” (*published*).

Os objetos de estado compartilham funcionalidades: em Rust, é claro, usamos
structs e traits em vez de objetos e herança. Cada objeto de estado é
responsável pelo seu próprio comportamento e por governar quando deve mudar para
outro estado. O valor que guarda um objeto de estado não sabe nada sobre o
comportamento diferente dos estados nem sobre quando fazer a transição entre
eles.

A vantagem de usar o padrão de estado é que, quando os requisitos de negócio do
programa mudam, não precisamos alterar o código do valor que guarda o estado nem
o código que usa esse valor. Precisaremos apenas atualizar o código dentro de
um dos objetos de estado para mudar suas regras ou talvez adicionar mais objetos
de estado.

Primeiro, vamos implementar o padrão de estado de uma maneira mais
tradicionalmente orientada a objetos. Em seguida, usaremos uma abordagem um
pouco mais natural em Rust. Vamos mergulhar na implementação incremental de um
fluxo de trabalho de postagem de blog usando o padrão de estado.

A funcionalidade final ficará assim:

1. Uma postagem de blog começa como um rascunho vazio.
1. Quando o rascunho fica pronto, uma revisão da postagem é solicitada.
1. Quando a postagem é aprovada, ela é publicada.
1. Apenas postagens de blog publicadas retornam conteúdo para exibição, para que
   postagens não aprovadas não possam ser publicadas acidentalmente.

Quaisquer outras alterações tentadas em uma postagem não devem ter efeito. Por
exemplo, se tentarmos aprovar uma postagem de rascunho antes de solicitarmos uma
revisão, a postagem deve permanecer como um rascunho não publicado.

<!-- Old headings. Do not remove or links may break. -->

<a id="a-traditional-object-oriented-attempt"></a>
<a id="attempting-traditional-object-oriented-style"></a>

### Tentando o Estilo Orientado a Objetos Tradicional

Existem infinitas maneiras de estruturar código para resolver o mesmo problema,
cada uma com diferentes *trade-offs*. A implementação desta seção segue um estilo
mais tradicionalmente orientado a objetos, o que é possível escrever em Rust,
mas não tira proveito de alguns dos pontos fortes do Rust. Mais adiante,
demonstraremos uma solução diferente que ainda usa o padrão de projeto orientado
a objetos, mas é estruturada de uma forma que pode parecer menos familiar para
pessoas programadoras com experiência em orientação a objetos. Compararemos as
duas soluções para vivenciar os *trade-offs* de projetar código Rust de maneira
diferente do código em outras linguagens.

A Listagem 18-11 mostra esse fluxo de trabalho em forma de código: este é um
exemplo de uso da API que implementaremos em um crate de biblioteca chamado
`blog`. Isso ainda não compilará porque ainda não implementamos o crate `blog`.

<Listing number="18-11" file-name="src/main.rs" caption="Código que demonstra o comportamento desejado que queremos que nosso crate `blog` tenha">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-oop/listing-18-11/src/main.rs:all}}
```

</Listing>

Queremos permitir que a pessoa usuária crie uma nova postagem de blog em
rascunho com `Post::new`. Queremos permitir que texto seja adicionado à postagem
do blog. Se tentarmos obter o conteúdo da postagem imediatamente, antes da
aprovação, não devemos receber nenhum texto porque a postagem ainda é um
rascunho. Adicionamos `assert_eq!` no código para fins de demonstração. Um
excelente teste unitário para isso seria afirmar que uma postagem de rascunho
retorna uma string vazia no método `content`, mas não vamos escrever testes para
este exemplo.

Em seguida, queremos permitir solicitar uma revisão da postagem e queremos que
`content` retorne uma string vazia enquanto aguarda a revisão. Quando a postagem
receber aprovação, ela deve ser publicada, o que significa que o texto da
postagem será retornado quando `content` for chamado.

Note que o único tipo com o qual estamos interagindo a partir do crate é o tipo
`Post`. Esse tipo usará o padrão de estado e armazenará um valor que será um de
três objetos de estado representando os vários estados em que uma postagem pode
estar — rascunho, revisão ou publicado. A mudança de um estado para outro será
gerenciada internamente dentro do tipo `Post`. Os estados mudam em resposta aos
métodos chamados pelos usuários da nossa biblioteca na instância de `Post`, mas
eles não precisam gerenciar as mudanças de estado diretamente. Além disso, os
usuários não podem cometer erros com os estados, como publicar uma postagem
antes de ela ser revisada.

<!-- Old headings. Do not remove or links may break. -->

<a id="defining-post-and-creating-a-new-instance-in-the-draft-state"></a>
<a id="defining-post-and-creating-a-new-instance"></a>

#### Definindo `Post` e Criando uma Nova Instância

Vamos começar a implementação da biblioteca! Sabemos que precisamos de uma
struct pública `Post` que guarde algum conteúdo, então começaremos com a
definição da struct e uma função pública associada `new` para criar uma
instância de `Post`, conforme mostrado na Listagem 18-12. Também criaremos um
trait privado `State` que definirá o comportamento que todos os objetos de
estado de um `Post` devem ter.

Então, `Post` guardará um objeto trait `Box<dyn State>` dentro de um `Option<T>`
em um campo privado chamado `state` para armazenar o objeto de estado. Você verá
por que o `Option<T>` é necessário daqui a pouco.

<Listing number="18-12" file-name="src/lib.rs" caption="Definição de uma struct `Post` e de uma função `new` que cria uma nova instância de `Post`, de um trait `State` e de uma struct `Draft`">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-12/src/lib.rs}}
```

</Listing>

O trait `State` define o comportamento compartilhado pelos diferentes estados da
postagem. Os objetos de estado são `Draft`, `PendingReview` e `Published`, e
todos eles implementarão o trait `State`. Por enquanto, o trait não tem nenhum
método, e começaremos definindo apenas o estado `Draft` porque esse é o estado
em que queremos que uma postagem comece.

Quando criamos um novo `Post`, definimos seu campo `state` como um valor `Some`
que guarda um `Box`. Esse `Box` aponta para uma nova instância da struct
`Draft`. Isso garante que, sempre que criarmos uma nova instância de `Post`, ela
começará como um rascunho. Como o campo `state` de `Post` é privado, não há
como criar um `Post` em qualquer outro estado! Na função `Post::new`, definimos
o campo `content` como uma nova `String` vazia.

<a id="storing-the-text-of-the-post-content"></a>

#### Armazenando o Texto do Conteúdo da Postagem

Vimos na Listagem 18-11 que queremos poder chamar um método chamado `add_text` e
passar a ele um `&str` que é então adicionado como conteúdo de texto da postagem
do blog. Implementamos isso como um método, em vez de expor o campo `content`
como `pub`, para que depois possamos implementar um método que controlará como
os dados do campo `content` são lidos. O método `add_text` é bem direto, então
vamos adicionar a implementação na Listagem 18-13 ao bloco `impl Post`.

<Listing number="18-13" file-name="src/lib.rs" caption="Implementando o método `add_text` para adicionar texto ao `content` de uma postagem">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-13/src/lib.rs:here}}
```

</Listing>

O método `add_text` recebe uma referência mutável a `self` porque estamos
alterando a instância de `Post` na qual estamos chamando `add_text`. Então
chamamos `push_str` na `String` em `content` e passamos o argumento `text` para
adicionar ao `content` salvo. Esse comportamento não depende do estado em que a
postagem está, portanto não faz parte do padrão de estado. O método `add_text`
não interage com o campo `state` de forma alguma, mas faz parte do comportamento
que queremos suportar.

<!-- Old headings. Do not remove or links may break. -->

<a id="ensuring-the-content-of-a-draft-post-is-empty"></a>

#### Garantindo que o Conteúdo de um Rascunho Seja Vazio

Mesmo depois de termos chamado `add_text` e adicionado algum conteúdo à nossa
postagem, ainda queremos que o método `content` retorne um *string slice* vazio
porque a postagem ainda está no estado de rascunho, conforme mostrado pelo
primeiro `assert_eq!` na Listagem 18-11. Por enquanto, vamos implementar o
método `content` com a coisa mais simples que atenderá a esse requisito: sempre
retornar um *string slice* vazio. Alteraremos isso mais tarde assim que
implementarmos a capacidade de mudar o estado de uma postagem para que ela possa
ser publicada. Até agora, as postagens só podem estar no estado de rascunho,
então o conteúdo da postagem deve estar sempre vazio. A Listagem 18-14 mostra
essa implementação provisória.

<Listing number="18-14" file-name="src/lib.rs" caption="Adicionando uma implementação provisória para o método `content` em `Post` que sempre retorna um *string slice* vazio">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-14/src/lib.rs:here}}
```

</Listing>

Com esse método `content` adicionado, tudo na Listagem 18-11 até o primeiro
`assert_eq!` funciona como esperado.

<!-- Old headings. Do not remove or links may break. -->

<a id="requesting-a-review-of-the-post-changes-its-state"></a>
<a id="requesting-a-review-changes-the-posts-state"></a>
<a id="requesting-a-review-which-changes-the-posts-state"></a>

#### Solicitando uma Revisão, o que Muda o Estado da Postagem

Em seguida, precisamos adicionar a funcionalidade para solicitar uma revisão de
uma postagem, o que deve mudar seu estado de `Draft` para `PendingReview`. A
Listagem 18-15 mostra esse código.

<Listing number="18-15" file-name="src/lib.rs" caption="Implementando métodos `request_review` em `Post` e no trait `State`">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-15/src/lib.rs:here}}
```

</Listing>

Damos a `Post` um método público chamado `request_review` que receberá uma
referência mutável a `self`. Então, chamamos um método interno `request_review`
no estado atual de `Post`, e esse segundo método `request_review` consome o
estado atual e retorna um novo estado.

Adicionamos o método `request_review` ao trait `State`; todos os tipos que
implementam o trait agora precisarão implementar o método `request_review`. Note
que, em vez de ter `self`, `&self` ou `&mut self` como o primeiro parâmetro do
método, temos `self: Box<Self>`. Essa sintaxe significa que o método só é
válido quando chamado em um `Box` que guarda o tipo. Essa sintaxe toma
*ownership* de `Box<Self>`, invalidando o estado antigo para que o valor de
estado do `Post` possa se transformar em um novo estado.

Para consumir o estado antigo, o método `request_review` precisa tomar
*ownership* do valor do estado. É aí que entra o `Option` no campo `state` de
`Post`: chamamos o método `take` para retirar o valor `Some` do campo `state` e
deixar um `None` em seu lugar, porque Rust não nos permite ter campos não
preenchidos em structs. Isso nos permite mover o valor de `state` para fora de
`Post` em vez de fazer *borrowing* dele. Em seguida, definiremos o valor `state`
da postagem como o resultado dessa operação.

Precisamos definir `state` como `None` temporariamente em vez de defini-lo
diretamente com um código como `self.state = self.state.request_review();` para
obter *ownership* do valor de `state`. Isso garante que `Post` não possa usar o
valor antigo de `state` depois de o termos transformado em um novo estado.

O método `request_review` em `Draft` retorna uma nova instância dentro de um
`Box` de uma nova struct `PendingReview`, que representa o estado quando uma
postagem está aguardando revisão. A struct `PendingReview` também implementa o
método `request_review`, mas não faz nenhuma transformação. Em vez disso, ela
retorna a si mesma porque, quando solicitamos uma revisão em uma postagem que já
está no estado `PendingReview`, ela deve permanecer no estado `PendingReview`.

Agora podemos começar a ver as vantagens do padrão de estado: o método
`request_review` em `Post` é o mesmo, não importa o seu valor de `state`. Cada
estado é responsável por suas próprias regras.

Deixaremos o método `content` em `Post` como está, retornando um *string slice*
vazio. Agora podemos ter um `Post` no estado `PendingReview`, bem como no estado
`Draft`, mas queremos o mesmo comportamento no estado `PendingReview`. A
Listagem 18-11 agora funciona até a segunda chamada `assert_eq!`!

<!-- Old headings. Do not remove or links may break. -->

<a id="adding-the-approve-method-that-changes-the-behavior-of-content"></a>
<a id="adding-approve-to-change-the-behavior-of-content"></a>
<a id="adding-approve-to-change-contents-behavior"></a>

#### Adicionando `approve` para Mudar o Comportamento de `content`

O método `approve` será semelhante ao método `request_review`: ele definirá
`state` para o valor que o estado atual diz que deveria ter quando esse estado
é aprovado, conforme mostrado na Listagem 18-16.

<Listing number="18-16" file-name="src/lib.rs" caption="Implementando o método `approve` em `Post` e no trait `State`">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-16/src/lib.rs:here}}
```

</Listing>

Adicionamos o método `approve` ao trait `State` e adicionamos uma nova struct
que implementa `State`, o estado `Published`.

De forma semelhante a como `request_review` em `PendingReview` funciona, se
chamarmos o método `approve` em um `Draft`, ele não terá efeito porque `approve`
retornará `self`. Quando chamamos `approve` em `PendingReview`, ele retorna uma
nova instância em `Box` da struct `Published`. A struct `Published` implementa o
trait `State` e, tanto para o método `request_review` quanto para o método
`approve`, retorna a si mesma porque a postagem deve permanecer no estado
`Published` nesses casos.

Agora precisamos atualizar o método `content` em `Post`. Queremos que o valor
retornado por `content` dependa do estado atual do `Post`, então faremos com que
o `Post` delegue para um método `content` definido em seu `state`, conforme
mostrado na Listagem 18-17.

<Listing number="18-17" file-name="src/lib.rs" caption="Atualizando o método `content` em `Post` para delegar a um método `content` em `State`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-oop/listing-18-17/src/lib.rs:here}}
```

</Listing>

Como o objetivo é manter todas essas regras dentro das structs que implementam
`State`, chamamos um método `content` no valor em `state` e passamos a
instância da postagem (isto é, `self`) como argumento. Em seguida, retornamos o
valor retornado pelo uso do método `content` no valor de `state`.

Chamamos o método `as_ref` no `Option` porque queremos uma referência ao valor
dentro do `Option`, em vez de *ownership* do valor. Como `state` é um
`Option<Box<dyn State>>`, quando chamamos `as_ref`, um `Option<&Box<dyn
State>>` é retornado. Se não chamássemos `as_ref`, receberíamos um erro porque
não podemos mover `state` para fora do `&self` emprestado do parâmetro da
função.

Em seguida, chamamos o método `unwrap`, que sabemos que nunca entrará em pânico
porque sabemos que os métodos em `Post` garantem que `state` sempre conterá um
valor `Some` quando esses métodos terminarem. Este é um dos casos sobre os
quais falamos na seção [“Casos em que Você Tem Mais Informações que o
Compilador”][more-info-than-rustc]<!-- ignore --> do Capítulo 9, quando sabemos
que um valor `None` nunca é possível, embora o compilador não consiga entender
isso.

Nesse ponto, quando chamamos `content` no `&Box<dyn State>`, a coerção deref
(*deref coercion*) entra em ação no `&` e no `Box`, de modo que o método
`content` acabará sendo chamado no tipo que implementa o trait `State`. Isso
significa que precisamos adicionar `content` à definição do trait `State`, e é
aí que colocaremos a lógica de qual conteúdo retornar dependendo de qual estado
temos, conforme mostrado na Listagem 18-18.

<Listing number="18-18" file-name="src/lib.rs" caption="Adicionando o método `content` ao trait `State`">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-18/src/lib.rs:here}}
```

</Listing>

Adicionamos uma implementação padrão para o método `content` que retorna um
*string slice* vazio. Isso significa que não precisamos implementar `content`
nas structs `Draft` e `PendingReview`. A struct `Published` sobrescreverá o
método `content` e retornará o valor em `post.content`. Embora conveniente, ter
o método `content` em `State` determinando o conteúdo do `Post` borra os
limites entre a responsabilidade de `State` e a responsabilidade de `Post`.

Note que precisamos de anotações de *lifetime* nesse método, conforme discutimos
no Capítulo 10. Estamos recebendo uma referência a um `post` como argumento e
retornando uma referência a parte desse `post`, portanto o *lifetime* da
referência retornada está relacionado ao *lifetime* do argumento `post`.

E pronto — toda a Listagem 18-11 agora funciona! Implementamos o padrão de
estado com as regras do fluxo de trabalho da postagem de blog. A lógica
relacionada às regras vive nos objetos de estado, em vez de ficar espalhada por
todo o `Post`.

<a id="why-not-an-enum"></a>

> ### Por que Não um Enum?
>
> Você pode ter se perguntado por que não usamos um enum com os diferentes
> estados possíveis da postagem como variantes. Essa certamente é uma solução
> possível; experimente implementá-la e compare os resultados finais para ver
> qual você prefere! Uma desvantagem de usar um enum é que todo lugar que
> verifica o valor do enum precisará de uma expressão `match` ou semelhante para
> lidar com cada variante possível. Isso poderia se tornar mais repetitivo do
> que essa solução com objetos trait.

<!-- Old headings. Do not remove or links may break. -->

<a id="trade-offs-of-the-state-pattern"></a>
<a id="evaluating-the-state-pattern"></a>

#### Avaliando o Padrão de Estado

Mostramos que Rust é capaz de implementar o padrão de estado orientado a
objetos para encapsular os diferentes tipos de comportamento que uma postagem
deve ter em cada estado. Os métodos em `Post` não sabem nada sobre os vários
comportamentos. Pela forma como organizamos o código, precisamos olhar em
apenas um lugar para saber as diferentes maneiras pelas quais uma postagem
publicada pode se comportar: a implementação do trait `State` na struct
`Published`.

Se criássemos uma implementação alternativa que não usasse o padrão de estado,
poderíamos usar expressões `match` nos métodos em `Post` ou até mesmo no código
em `main` que verifica o estado da postagem e muda o comportamento nesses
lugares. Isso significaria que teríamos que olhar em vários lugares para
entender todas as implicações de uma postagem estar no estado publicado.

Com o padrão de estado, os métodos de `Post` e os lugares onde usamos `Post` não
precisam de expressões `match` e, para adicionar um novo estado, precisaríamos
apenas adicionar uma nova struct e implementar os métodos do trait nessa única
struct em um único lugar.

A implementação usando o padrão de estado é fácil de estender para adicionar
mais funcionalidades. Para ver a simplicidade de manter código que usa o padrão
de estado, experimente algumas destas sugestões:

- Adicionar um método `reject` que mude o estado da postagem de `PendingReview`
  de volta para `Draft`.
- Exigir duas chamadas a `approve` antes que o estado possa ser alterado para
  `Published`.
- Permitir que os usuários adicionem conteúdo de texto apenas quando uma
  postagem estiver no estado `Draft`. Dica: faça o objeto de estado ser
  responsável pelo que pode mudar no conteúdo, mas não responsável por
  modificar o `Post`.

Uma desvantagem do padrão de estado é que, como os estados implementam as
transições entre estados, alguns dos estados ficam acoplados entre si. Se
adicionarmos outro estado entre `PendingReview` e `Published`, como `Scheduled`,
teríamos que alterar o código em `PendingReview` para fazer a transição para
`Scheduled`. Daria menos trabalho se `PendingReview` não precisasse mudar com a
adição de um novo estado, mas isso significaria mudar para outro padrão de
projeto.

Outra desvantagem é que duplicamos alguma lógica. Para eliminar parte da
duplicação, poderíamos tentar criar implementações padrão para os métodos
`request_review` e `approve` no trait `State` que retornassem `self`. No
entanto, isso não funcionaria: ao usar `State` como um objeto trait, o trait não
sabe exatamente qual será o `self` concreto, então o tipo de retorno não é
conhecido em tempo de compilação. (Essa é uma das regras de compatibilidade com
`dyn` mencionadas anteriormente.)

Outra duplicação inclui as implementações semelhantes dos métodos
`request_review` e `approve` em `Post`. Ambos os métodos usam `Option::take`
com o campo `state` de `Post` e, se `state` for `Some`, delegam para a
implementação do mesmo método no valor encapsulado e definem o novo valor do
campo `state` para o resultado. Se tivéssemos muitos métodos em `Post` que
seguissem esse padrão, poderíamos considerar definir uma macro para eliminar a
repetição (veja a seção [“Macros”][macros]<!-- ignore --> no Capítulo 20).

Ao implementar o padrão de estado exatamente como ele é definido para linguagens
orientadas a objetos, não estamos tirando o máximo proveito dos pontos fortes do
Rust quanto poderíamos. Vamos observar algumas mudanças que podemos fazer no
crate `blog` que podem transformar estados e transições inválidos em erros em
tempo de compilação.

<a id="encoding-states-and-behavior-as-types"></a>

### Codificando Estados e Comportamento como Tipos

Mostraremos como repensar o padrão de estado para obter um conjunto diferente de
*trade-offs*. Em vez de encapsular os estados e as transições completamente de
modo que o código externo não tenha conhecimento deles, codificaremos os estados
em tipos diferentes. Consequentemente, o sistema de verificação de tipos do Rust
impedirá tentativas de usar postagens em rascunho onde apenas postagens
publicadas são permitidas, emitindo um erro de compilação.

Vamos considerar a primeira parte de `main` na Listagem 18-11:

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch18-oop/listing-18-11/src/main.rs:here}}
```

</Listing>

Ainda permitimos a criação de novas postagens no estado de rascunho usando
`Post::new` e a capacidade de adicionar texto ao conteúdo da postagem. Mas, em
vez de ter um método `content` em uma postagem de rascunho que retorna uma
string vazia, faremos com que postagens de rascunho não tenham o método
`content` de jeito nenhum. Dessa forma, se tentarmos obter o conteúdo de uma
postagem de rascunho, receberemos um erro do compilador dizendo que o método não
existe. Como resultado, será impossível exibir acidentalmente o conteúdo de uma
postagem de rascunho em produção, porque esse código nem sequer compilará. A
Listagem 18-19 mostra a definição de uma struct `Post` e de uma struct
`DraftPost`, bem como métodos em cada uma.

<Listing number="18-19" file-name="src/lib.rs" caption="Um `Post` com um método `content` e um `DraftPost` sem um método `content`">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-19/src/lib.rs}}
```

</Listing>

Tanto a struct `Post` quanto a `DraftPost` têm um campo privado `content` que
armazena o texto da postagem do blog. As structs não têm mais o campo `state`
porque estamos movendo a codificação do estado para os tipos das structs. A
struct `Post` representará uma postagem publicada e possui um método `content`
que retorna o `content`.

Ainda temos uma função `Post::new`, mas, em vez de retornar uma instância de
`Post`, ela retorna uma instância de `DraftPost`. Como `content` é privado e
não existem funções que retornem `Post`, não é possível criar uma instância de
`Post` neste momento.

A struct `DraftPost` tem um método `add_text`, então podemos adicionar texto a
`content` como antes, mas note que `DraftPost` não tem um método `content`
definido! Portanto, agora o programa garante que todas as postagens comecem como
postagens em rascunho, e postagens em rascunho não têm seu conteúdo disponível
para exibição. Qualquer tentativa de contornar essas restrições resultará em um
erro de compilação.

<!-- Old headings. Do not remove or links may break. -->

<a id="implementing-transitions-as-transformations-into-different-types"></a>

Então, como obtemos uma postagem publicada? Queremos impor a regra de que uma
postagem em rascunho precisa ser revisada e aprovada antes de poder ser
publicada. Uma postagem no estado de revisão pendente ainda não deve exibir
nenhum conteúdo. Vamos implementar essas restrições adicionando outra struct,
`PendingReviewPost`, definindo o método `request_review` em `DraftPost` para
retornar um `PendingReviewPost` e definindo um método `approve` em
`PendingReviewPost` para retornar um `Post`, conforme mostrado na Listagem
18-20.

<Listing number="18-20" file-name="src/lib.rs" caption="Um `PendingReviewPost` criado ao chamar `request_review` em `DraftPost` e um método `approve` que transforma um `PendingReviewPost` em um `Post` publicado">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-20/src/lib.rs:here}}
```

</Listing>

Os métodos `request_review` e `approve` tomam *ownership* de `self`, consumindo
assim as instâncias de `DraftPost` e `PendingReviewPost` e transformando-as em
um `PendingReviewPost` e em um `Post` publicado, respectivamente. Dessa forma,
não teremos nenhuma instância de `DraftPost` remanescente depois de chamarmos
`request_review` nela, e assim por diante. A struct `PendingReviewPost` não tem
um método `content` definido, então tentar ler seu conteúdo resulta em um erro
de compilação, assim como acontece com `DraftPost`. Como a única maneira de
obter uma instância de `Post` publicada que tenha o método `content` definido é
chamar o método `approve` em um `PendingReviewPost`, e a única maneira de obter
um `PendingReviewPost` é chamar o método `request_review` em um `DraftPost`,
agora codificamos o fluxo de trabalho da postagem de blog no sistema de tipos.

Mas também precisamos fazer algumas pequenas alterações em `main`. Os métodos
`request_review` e `approve` retornam novas instâncias em vez de modificar a
struct na qual são chamados, então precisamos adicionar mais atribuições com
sombreamento `let post =` para salvar as instâncias retornadas. Também não
podemos ter as asserções de que os conteúdos das postagens em rascunho e em
revisão pendente sejam strings vazias, nem precisamos delas: não conseguimos
mais compilar código que tente usar o conteúdo de postagens nesses estados. O
código atualizado em `main` é mostrado na Listagem 18-21.

<Listing number="18-21" file-name="src/main.rs" caption="Modificações em `main` para usar a nova implementação do fluxo de trabalho da postagem de blog">

```rust,ignore
{{#rustdoc_include ../listings/ch18-oop/listing-18-21/src/main.rs}}
```

</Listing>

As alterações que precisamos fazer em `main` para reatribuir `post` significam
que essa implementação não segue mais exatamente o padrão de estado orientado a
objetos: as transformações entre os estados não estão mais encapsuladas
inteiramente dentro da implementação de `Post`. No entanto, nosso ganho é que
estados inválidos agora são impossíveis por causa do sistema de tipos e da
verificação de tipos que ocorre em tempo de compilação! Isso garante que certos
bugs, como a exibição do conteúdo de uma postagem não publicada, sejam
descobertos antes de chegarem à produção.

Experimente as tarefas sugeridas no início desta seção no crate `blog` como ele
está após a Listagem 18-21 para ver o que você acha do design dessa versão do
código. Note que algumas das tarefas já podem estar concluídas nesse design.

Vimos que, embora Rust seja capaz de implementar padrões de projeto orientados a
objetos, outros padrões, como codificar estados no sistema de tipos, também
estão disponíveis em Rust. Esses padrões têm *trade-offs* diferentes. Embora
você possa estar muito familiarizado com padrões orientados a objetos, repensar
o problema para aproveitar os recursos do Rust pode trazer benefícios, como
prevenir alguns bugs em tempo de compilação. Padrões orientados a objetos nem
sempre serão a melhor solução em Rust devido a certos recursos, como
*ownership*, que linguagens orientadas a objetos não possuem.

<a id="summary"></a>

## Resumo

Independentemente de você achar que Rust é uma linguagem orientada a objetos
depois de ler este capítulo, agora você sabe que pode usar objetos trait para
obter alguns recursos orientados a objetos em Rust. O despacho dinâmico pode dar
ao seu código alguma flexibilidade em troca de um pouco de desempenho em tempo de
execução. Você pode usar essa flexibilidade para implementar padrões orientados a
objetos que podem ajudar na manutenibilidade do seu código. Rust também possui
outros recursos, como *ownership*, que linguagens orientadas a objetos não têm.
Um padrão orientado a objetos nem sempre será a melhor maneira de aproveitar os
pontos fortes do Rust, mas é uma opção disponível.

A seguir, veremos padrões (*patterns*), que são outro recurso do Rust que
permite bastante flexibilidade. Nós os vimos brevemente ao longo do livro, mas
ainda não vimos toda a sua capacidade. Vamos lá!

[more-info-than-rustc]: ch09-03-to-panic-or-not-to-panic.html#cases-in-which-you-have-more-information-than-the-compiler
[macros]: ch20-05-macros.html#macros
