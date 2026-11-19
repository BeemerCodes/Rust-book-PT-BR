<!-- Old headings. Do not remove or links may break. -->

<a id="digging-into-the-traits-for-async"></a>
<a id="a-closer-look-at-the-traits-for-async"></a>

## Um Olhar Mais Atento aos Traits para Async

Ao longo do capítulo, usamos os traits `Future`, `Stream` e `StreamExt` de várias
maneiras. Até agora, porém, evitamos entrar muito nos detalhes de como eles
funcionam ou de como se encaixam, o que é suficiente na maior parte do tempo
para o seu trabalho diário com Rust. Às vezes, no entanto, você encontrará
situações em que precisará entender um pouco mais sobre os detalhes desses
traits, juntamente com o tipo `Pin` e o trait `Unpin`. Nesta seção, vamos nos
aprofundar apenas o bastante para ajudar nesses cenários, deixando ainda o
mergulho _realmente_ profundo para outras documentações.

<!-- Old headings. Do not remove or links may break. -->

<a id="future"></a>
<a id="the-future-trait"></a>

### O Trait `Future`

Vamos começar examinando mais de perto como o trait `Future` funciona. É assim
que o Rust o define:

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

Essa definição de trait inclui vários tipos novos e também alguma sintaxe que
ainda não vimos, então vamos percorrer a definição parte por parte.

Primeiro, o tipo associado `Output` de `Future` diz para qual tipo o future se
resolve. Isso é análogo ao tipo associado `Item` do trait `Iterator`. Segundo,
`Future` possui o método `poll`, que recebe uma referência `Pin` especial para
seu parâmetro `self` e uma referência mutável para um tipo `Context`, e retorna
um `Poll<Self::Output>`. Falaremos mais sobre `Pin` e `Context` em um momento.
Por enquanto, vamos focar no que o método retorna, o tipo `Poll`:

```rust
pub enum Poll<T> {
    Ready(T),
    Pending,
}
```

Esse tipo `Poll` é semelhante a um `Option`. Ele tem uma variante que contém um
valor, `Ready(T)`, e uma que não contém, `Pending`. Contudo, `Poll` significa
algo bem diferente de `Option`! A variante `Pending` indica que o future ainda
tem trabalho a fazer, portanto o chamador precisará verificar novamente mais
tarde. A variante `Ready` indica que o `Future` concluiu seu trabalho e o valor
`T` está disponível.

> Note: É raro precisar chamar `poll` diretamente, mas, caso você precise,
> tenha em mente que, com a maioria dos futures, o chamador não deve chamar
> `poll` novamente depois que o future retornar `Ready`. Muitos futures entrarão
> em pânico se forem consultados novamente após ficarem prontos. Futures que são
> seguros para consultar novamente dirão isso explicitamente em sua
> documentação. Isso é semelhante ao comportamento de `Iterator::next`.

Quando você vê código que usa `await`, o Rust o compila por baixo dos panos
para um código que chama `poll`. Se você olhar novamente para a Listagem 17-4,
onde imprimimos o título da página de uma única URL assim que ela foi
resolvida, o Rust a compila para algo parecido (embora não exatamente igual)
com isto:

```rust,ignore
match page_title(url).poll() {
    Ready(page_title) => match page_title {
        Some(title) => println!("The title for {url} was {title}"),
        None => println!("{url} had no title"),
    }
    Pending => {
        // Mas o que vai aqui?
    }
}
```

O que devemos fazer quando o future ainda está em `Pending`? Precisamos de
alguma maneira de tentar novamente, de novo e de novo, até que o future esteja
finalmente pronto. Em outras palavras, precisamos de um laço:

```rust,ignore
let mut page_title_fut = page_title(url);
loop {
    match page_title_fut.poll() {
        Ready(value) => match page_title {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
        Pending => {
            // continua
        }
    }
}
```

Se o Rust compilasse exatamente para esse código, porém, todo `await` seria
bloqueante — exatamente o oposto do que queríamos! Em vez disso, o Rust garante
que o laço possa passar o controle para algo capaz de pausar o trabalho neste
future para trabalhar em outros futures e, depois, verificar este novamente mais
tarde. Como vimos, esse “algo” é um runtime assíncrono, e esse trabalho de
agendamento e coordenação é uma de suas principais funções.

Na seção [“Enviando Dados Entre Duas Tarefas Usando Troca de
Mensagens”][message-passing]<!-- ignore -->, descrevemos a espera por
`rx.recv`. A chamada a `recv` retorna um future, e fazer await no future faz o
poll dele. Notamos que um runtime pausará o future até que ele esteja pronto com
`Some(message)` ou `None` quando o canal for fechado. Com nosso entendimento
mais profundo do trait `Future`, e especificamente de `Future::poll`, podemos
ver como isso funciona. O runtime sabe que o future não está pronto quando ele
retorna `Poll::Pending`. Por outro lado, o runtime sabe que o future _está_
pronto e avança com ele quando `poll` retorna `Poll::Ready(Some(message))` ou
`Poll::Ready(None)`.

Os detalhes exatos de como um runtime faz isso estão além do escopo deste livro,
mas o essencial é enxergar a mecânica básica dos futures: um runtime faz _poll_
de cada future pelo qual é responsável, colocando o future de volta para dormir
quando ele ainda não está pronto.

<!-- Old headings. Do not remove or links may break. -->

<a id="pinning-and-the-pin-and-unpin-traits"></a>
<a id="the-pin-and-unpin-traits"></a>
<a id="the-pin-type-and-the-unpin-trait"></a>

### O Tipo `Pin` e o Trait `Unpin`

Lá na Listagem 17-13, usamos a macro `trpl::join!` para fazer await de três
futures. No entanto, é comum ter uma coleção, como um vetor, contendo uma
quantidade de futures que não será conhecida até o tempo de execução. Vamos
mudar a Listagem 17-13 para o código na Listagem 17-23, que coloca os três
futures em um vetor e chama a função `trpl::join_all` em vez disso, o que ainda
não compila.

<Listing number="17-23" caption="Aguardando futures em uma coleção"  file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-23/src/main.rs:here}}
```

</Listing>

Colocamos cada future dentro de um `Box` para transformá-los em _objetos trait_
(*trait objects*), assim como fizemos na seção “Retornando Erros de `run`” no
Capítulo 12. (Abordaremos objetos trait em detalhes no Capítulo 18.) Usar
objetos trait nos permite tratar cada um dos futures anônimos produzidos por
esses tipos como o mesmo tipo, pois todos eles implementam o trait `Future`.

Isso pode parecer surpreendente. Afinal, nenhum dos blocos async retorna nada,
então cada um produz um `Future<Output = ()>`. Lembre-se, porém, de que `Future`
é um trait e de que o compilador cria um enum exclusivo para cada bloco async,
mesmo quando eles têm tipos de saída idênticos. Assim como você não pode colocar
duas structs diferentes escritas à mão em um `Vec`, você não pode misturar enums
gerados pelo compilador.

Em seguida, passamos a coleção de futures para a função `trpl::join_all` e
fazemos await do resultado. Contudo, isso não compila; aqui está a parte
relevante das mensagens de erro.

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-23
cargo build
copy *only* the final `error` block from the errors
-->

```text
error[E0277]: `dyn Future<Output = ()>` cannot be unpinned
  --> src/main.rs:48:33
   |
48 |         trpl::join_all(futures).await;
   |                                 ^^^^^ the trait `Unpin` is not implemented for `dyn Future<Output = ()>`
   |
   = note: consider using the `pin!` macro
           consider using `Box::pin` if you need to access the pinned value outside of the current scope
   = note: required for `Box<dyn Future<Output = ()>>` to implement `Future`
note: required by a bound in `futures_util::future::join_all::JoinAll`
  --> file:///home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/futures-util-0.3.30/src/future/join_all.rs:29:8
   |
27 | pub struct JoinAll<F>
   |            ------- required by a bound in this struct
28 | where
29 |     F: Future,
   |        ^^^^^^ required by this bound in `JoinAll`
```

A nota nessa mensagem de erro nos diz que devemos usar a macro `pin!` para
_fixar_ (_pin_) os valores, o que significa colocá-los dentro do tipo `Pin`, que
garante que os valores não serão movidos na memória. A mensagem de erro diz que
o pin é necessário porque `dyn Future<Output = ()>` precisa implementar o trait
`Unpin`, e atualmente ele não implementa.

A função `trpl::join_all` retorna uma struct chamada `JoinAll`. Essa struct é
genérica sobre um tipo `F`, que é restrito a implementar o trait `Future`. Fazer
await diretamente em um future com `await` fixa o future com pin
implicitamente. É por isso que não precisamos usar `pin!` em todo lugar onde
queremos fazer await de futures.

Porém, aqui não estamos fazendo await diretamente de um future. Em vez disso,
construímos um novo future, `JoinAll`, passando uma coleção de futures para a
função `join_all`. A assinatura de `join_all` exige que os tipos dos itens na
coleção implementem todos o trait `Future`, e `Box<T>` só implementa `Future` se
o `T` que ele envolve for um future que implementa o trait `Unpin`.

É bastante coisa para absorver! Para entender isso de verdade, vamos mergulhar um
pouco mais fundo em como o trait `Future` realmente funciona, em particular em
torno do conceito de pin. Olhe novamente para a definição do trait `Future`:

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    // Método obrigatório
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

O parâmetro `cx` e seu tipo `Context` são a chave para como um runtime realmente
sabe quando verificar um determinado future mantendo-se preguiçoso (*lazy*).
Novamente, os detalhes de como isso funciona estão além do escopo deste
capítulo, e geralmente você só precisa pensar nisso ao escrever uma
implementação personalizada de `Future`. Em vez disso, vamos focar no tipo de
`self`, pois esta é a primeira vez que vemos um método em que `self` tem uma
anotação de tipo. Uma anotação de tipo para `self` funciona como anotações de
tipo para outros parâmetros de função, mas com duas diferenças principais:

- Ela diz ao Rust qual tipo `self` deve ter para que o método seja chamado.
- Não pode ser qualquer tipo. Ela é restrita ao tipo no qual o método é
  implementado, a uma referência ou smart pointer para esse tipo, ou a um `Pin`
  envolvendo uma referência para esse tipo.

Veremos mais sobre essa sintaxe no [Capítulo 18][ch-18]<!-- ignore -->. Por
enquanto, basta saber que, se quisermos fazer poll de um future para verificar
se ele está em `Pending` ou `Ready(Output)`, precisamos de uma referência mutável
envolvida em `Pin` para o tipo.

`Pin` é um invólucro (*wrapper*) para tipos semelhantes a ponteiros, como `&`,
`&mut`, `Box` e `Rc`. (Tecnicamente, `Pin` funciona com tipos que implementam os
traits `Deref` ou `DerefMut`, mas na prática isso equivale a trabalhar apenas
com referências e smart pointers.) `Pin` não é um ponteiro em si e não tem
nenhum comportamento próprio como `Rc` e `Arc` têm com contagem de referências;
ele é puramente uma ferramenta que o compilador pode usar para impor restrições
ao uso de ponteiros.

Lembrar que `await` é implementado em termos de chamadas a `poll` começa a
explicar a mensagem de erro que vimos antes, mas ela falava em termos de
`Unpin`, e não de `Pin`. Então, como exatamente `Pin` se relaciona com `Unpin`, e
por que `Future` precisa que `self` esteja em um tipo `Pin` para chamar `poll`?

Lembre-se do que vimos antes neste capítulo: uma série de pontos de await em um
future é compilada em uma máquina de estados, e o compilador garante que essa
máquina de estados siga todas as regras normais do Rust sobre segurança,
incluindo *borrowing* e *ownership*. Para fazer isso funcionar, o Rust analisa
quais dados são necessários entre um ponto de await e o próximo ponto de await
ou o fim do bloco async. Então ele cria uma variante correspondente na máquina de
estados compilada. Cada variante recebe o acesso de que precisa aos dados que
serão usados naquela seção do código-fonte, seja tomando *ownership* desses
dados ou obtendo uma referência mutável ou imutável para eles.

Até aqui, tudo bem: se errarmos alguma coisa sobre o *ownership* ou as
referências em um determinado bloco async, o *borrow checker* nos avisará.
Quando queremos mover de um lado para o outro o future que corresponde a esse
bloco — como movê-lo para dentro de um `Vec` para passar para `join_all` — as
coisas ficam mais complicadas.

Quando movemos um future — seja inserindo-o em uma estrutura de dados para usar
como iterador com `join_all` ou retornando-o de uma função — isso na verdade
significa mover a máquina de estados que o Rust cria para nós. E, ao contrário da
maioria dos outros tipos em Rust, os futures que o Rust cria para blocos async
podem acabar tendo referências a si mesmos nos campos de qualquer variante, como
mostra a ilustração simplificada na Figura 17-4.

<figure>

<img alt="Uma tabela de uma coluna e três linhas representando um future, fut1, que tem os valores de dados 0 e 1 nas duas primeiras linhas e uma seta apontando da terceira linha de volta para a segunda linha, representando uma referência interna dentro do future." src="img/trpl17-04.svg" class="center" />

<figcaption>Figura 17-4: Um tipo de dados autorreferencial</figcaption>

</figure>

Por padrão, porém, qualquer objeto que tenha uma referência a si mesmo não é
seguro para ser movido, porque referências sempre apontam para o endereço de
memória real daquilo a que se referem (veja a Figura 17-5). Se você mover a
própria estrutura de dados, essas referências internas continuarão apontando
para o local antigo. No entanto, esse local de memória agora é inválido. Por um
lado, seu valor não será atualizado quando você fizer alterações na estrutura de
dados. Por outro — e mais importante — o computador agora está livre para
reutilizar aquela memória para outros fins! Você poderia acabar lendo dados
completamente não relacionados mais tarde.

<figure>

<img alt="Duas tabelas, representando dois futures, fut1 e fut2, cada um com uma coluna e três linhas, mostrando o resultado de ter movido um future de fut1 para fut2. O primeiro, fut1, está acinzentado, com um ponto de interrogação em cada índice, representando memória desconhecida. O segundo, fut2, tem 0 e 1 na primeira e na segunda linhas e uma seta apontando da sua terceira linha de volta para a segunda linha de fut1, representando um ponteiro que está referenciando o local antigo na memória do future antes de ele ser movido." src="img/trpl17-05.svg" class="center" />

<figcaption>Figura 17-5: O resultado inseguro de mover um tipo de dados autorreferencial</figcaption>

</figure>

Teoricamente, o compilador Rust poderia tentar atualizar toda referência para um
objeto sempre que ele fosse movido, mas isso poderia adicionar muita sobrecarga
de desempenho, especialmente se toda uma teia de referências precisasse ser
atualizada. Se, em vez disso, pudéssemos garantir que a estrutura de dados em
questão _não se mova na memória_, não precisaríamos atualizar nenhuma
referência. É exatamente para isso que serve o *borrow checker* do Rust: em
código seguro, ele impede que você mova qualquer item que tenha uma referência
ativa para ele.

`Pin` se baseia nisso para nos dar a garantia exata de que precisamos. Quando
fazemos _pin_ de um valor envolvendo um ponteiro para esse valor em `Pin`, ele
não pode mais se mover. Assim, se você tem `Pin<Box<SomeType>>`, você na verdade
fixa o valor `SomeType`, e _não_ o ponteiro `Box`. A Figura 17-6 ilustra esse
processo.

<figure>

<img alt="Três caixas dispostas lado a lado. A primeira está rotulada como “Pin”, a segunda como “b1” e a terceira como “pinned”. Dentro de “pinned” há uma tabela rotulada como “fut”, com uma única coluna; ela representa um future com células para cada parte da estrutura de dados. Sua primeira célula tem o valor “0”, sua segunda célula tem uma seta saindo dela e apontando para a quarta e última célula, que contém o valor “1”, e a terceira célula tem linhas tracejadas e reticências para indicar que pode haver outras partes da estrutura de dados. No conjunto, a tabela “fut” representa um future que é autorreferencial. Uma seta sai da caixa rotulada como “Pin”, passa pela caixa rotulada como “b1” e termina dentro da caixa “pinned”, na tabela “fut”." src="img/trpl17-06.svg" class="center" />

<figcaption>Figura 17-6: Fixando com pin um `Box` que aponta para um tipo future autorreferencial</figcaption>

</figure>

Na verdade, o ponteiro `Box` ainda pode se mover livremente por aí. Lembre-se:
nós nos importamos em garantir que os dados referenciados em última instância
permaneçam no mesmo lugar. Se um ponteiro se move, _mas os dados para os quais
ele aponta_ estão no mesmo lugar, como na Figura 17-7, não há problema algum.
(Como exercício independente, dê uma olhada na documentação dos tipos, bem como
no módulo `std::pin`, e tente descobrir como você faria isso com um `Pin`
envolvendo um `Box`.) O ponto-chave é que o tipo autorreferencial em si não pode
se mover, porque ainda está fixado com pin.

<figure>

<img alt="Quatro caixas dispostas em três colunas aproximadas, idênticas ao diagrama anterior com uma mudança na segunda coluna. Agora há duas caixas na segunda coluna, rotuladas como “b1” e “b2”, “b1” está acinzentada e a seta de “Pin” passa por “b2” em vez de “b1”, indicando que o ponteiro se moveu de “b1” para “b2”, mas os dados em “pinned” não se moveram." src="img/trpl17-07.svg" class="center" />

<figcaption>Figura 17-7: Movendo um `Box` que aponta para um tipo future autorreferencial</figcaption>

</figure>

Entretanto, a maioria dos tipos é perfeitamente segura para ser movida por aí,
mesmo que esteja por trás de um ponteiro `Pin`. Só precisamos nos preocupar com
pinning quando os itens têm referências internas. Valores primitivos, como
números e booleanos, são seguros porque obviamente não têm nenhuma referência
interna. Nem a maioria dos tipos com os quais você trabalha normalmente em Rust.
Você pode mover um `Vec` por aí, por exemplo, sem se preocupar. Pelo que vimos
até agora, se você tivesse um `Pin<Vec<String>>`, teria de fazer tudo por meio
das APIs seguras, porém restritivas, fornecidas por `Pin`, mesmo que um
`Vec<String>` seja sempre seguro de mover se não houver outras referências para
ele. Precisamos de uma forma de dizer ao compilador que está tudo bem mover
itens em casos como esse — e é aí que entra `Unpin`.

`Unpin` é um trait marcador (*marker trait*), semelhante aos traits `Send` e
`Sync` que vimos no Capítulo 16, e portanto não tem funcionalidade própria.
Traits marcadores existem apenas para dizer ao compilador que é seguro usar o
tipo que implementa determinado trait em um contexto específico. `Unpin` informa
ao compilador que um determinado tipo _não_ precisa manter nenhuma garantia
sobre o valor em questão poder ou não ser movido com segurança.

<!--
  The inline `<code>` in the next block is to allow the inline `<em>` inside it,
  matching what NoStarch does style-wise, and emphasizing within the text here
  that it is something distinct from a normal type.
-->

Assim como acontece com `Send` e `Sync`, o compilador implementa `Unpin`
automaticamente para todos os tipos em que consegue provar que isso é seguro.
Um caso especial, novamente semelhante a `Send` e `Sync`, é quando `Unpin` _não_
é implementado para um tipo. A notação para isso é
<code>impl !Unpin for <em>SomeType</em></code>, em que
<code><em>SomeType</em></code> é o nome de um tipo que _precisa_ manter essas
garantias para ser seguro sempre que um ponteiro para esse tipo for usado em um
`Pin`.

Em outras palavras, há duas coisas para ter em mente sobre a relação entre `Pin`
e `Unpin`. Primeiro, `Unpin` é o caso “normal”, e `!Unpin` é o caso especial.
Segundo, o fato de um tipo implementar `Unpin` ou `!Unpin` _só_ importa quando
você está usando um ponteiro fixado com pin para esse tipo, como <code>Pin<&mut
<em>SomeType</em>></code>.

Para tornar isso concreto, pense em uma `String`: ela tem um comprimento e os
caracteres Unicode que a compõem. Podemos envolver uma `String` em `Pin`, como
visto na Figura 17-8. Contudo, `String` implementa `Unpin` automaticamente,
assim como a maioria dos outros tipos em Rust.

<figure>

<img alt="Uma caixa rotulada como “Pin” à esquerda com uma seta indo dela para uma caixa rotulada como “String” à direita. A caixa “String” contém os dados 5usize, representando o comprimento da string, e as letras “h”, “e”, “l”, “l” e “o” representando os caracteres da string “hello” armazenada nesta instância de String. Um retângulo pontilhado envolve a caixa “String” e seu rótulo, mas não a caixa “Pin”." src="img/trpl17-08.svg" class="center" />

<figcaption>Figura 17-8: Fixando uma `String` com pin; a linha pontilhada indica que `String` implementa o trait `Unpin` e, portanto, não está realmente fixada</figcaption>

</figure>

Como resultado, podemos fazer coisas que seriam ilegais se `String` implementasse
`!Unpin`, como substituir uma string por outra exatamente no mesmo local da
memória, como na Figura 17-9. Isso não viola o contrato de `Pin`, porque `String`
não tem referências internas que a tornem insegura para ser movida. É
precisamente por isso que ela implementa `Unpin` em vez de `!Unpin`.

<figure>

<img alt="Os mesmos dados da string “hello” do exemplo anterior, agora rotulada como “s1” e acinzentada. A caixa “Pin” do exemplo anterior agora aponta para uma instância diferente de String, rotulada como “s2”, válida, com comprimento 7usize e contendo os caracteres da string “goodbye”. s2 é envolvida por um retângulo pontilhado porque ela também implementa o trait Unpin." src="img/trpl17-09.svg" class="center" />

<figcaption>Figura 17-9: Substituindo a `String` por uma `String` totalmente diferente na memória</figcaption>

</figure>

Agora sabemos o suficiente para entender os erros relatados para aquela chamada a
`join_all` da Listagem 17-23. Originalmente, tentamos mover os futures
produzidos por blocos async para um `Vec<Box<dyn Future<Output = ()>>>`, mas,
como vimos, esses futures podem ter referências internas, portanto não
implementam `Unpin` automaticamente. Depois que os fixamos com pin, podemos
passar o tipo `Pin` resultante para o `Vec`, confiantes de que os dados
subjacentes nos futures _não_ serão movidos. A Listagem 17-24 mostra como
corrigir o código chamando a macro `pin!` no ponto em que cada um dos três
futures é definido e ajustando o tipo do objeto trait.

<Listing number="17-24" caption="Fixando os futures com pin para permitir movê-los para o vetor">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-24/src/main.rs:here}}
```

</Listing>

Agora esse exemplo compila e executa, e poderíamos adicionar ou remover futures
do vetor em tempo de execução e fazer join de todos eles.

`Pin` e `Unpin` são importantes principalmente para construir bibliotecas de nível
mais baixo, ou quando você está construindo um próprio runtime, em vez de no
código Rust do dia a dia. Quando você vir esses traits em mensagens de erro,
porém, agora terá uma ideia melhor de como corrigir seu código!

> Note: Essa combinação de `Pin` e `Unpin` torna possível implementar com
> segurança toda uma classe de tipos complexos em Rust que, de outra forma,
> seriam desafiadores por serem autorreferenciais. Tipos que exigem `Pin`
> aparecem mais comumente em Rust assíncrono hoje em dia, mas de vez em quando
> você pode vê-los em outros contextos também.
>
> Os detalhes de como `Pin` e `Unpin` funcionam, e as regras que eles precisam
> respeitar, são abordados extensivamente na documentação da API de `std::pin`,
> então, se você tiver interesse em aprender mais, esse é um ótimo lugar para
> começar.
>
> Se quiser entender em ainda mais detalhes como as coisas funcionam por baixo
> dos panos, veja os Capítulos [2][under-the-hood]<!-- ignore --> e
> [4][pinning]<!-- ignore --> de
> [_Asynchronous Programming in Rust_][async-book].

<a id="the-stream-trait"></a>

### O Trait `Stream`

Agora que você tem uma compreensão mais profunda dos traits `Future`, `Pin` e
`Unpin`, podemos voltar nossa atenção para o trait `Stream`. Como você aprendeu
anteriormente no capítulo, streams são semelhantes a iteradores assíncronos.
Diferentemente de `Iterator` e `Future`, porém, `Stream` não tem uma definição
na biblioteca padrão até o momento desta escrita, mas _existe_ uma definição
muito comum do crate `futures` usada em todo o ecossistema.

Vamos revisar as definições dos traits `Iterator` e `Future` antes de ver como
um trait `Stream` pode mesclá-los. De `Iterator`, temos a ideia de uma sequência:
seu método `next` fornece um `Option<Self::Item>`. De `Future`, temos a ideia de
prontidão ao longo do tempo: seu método `poll` fornece um `Poll<Self::Output>`.
Para representar uma sequência de itens que ficam prontos ao longo do tempo,
definimos um trait `Stream` que reúne esses recursos:

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

trait Stream {
    type Item;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<Option<Self::Item>>;
}
```

O trait `Stream` define um tipo associado chamado `Item` para o tipo dos itens
produzidos pelo stream. Isso é semelhante a `Iterator`, em que pode haver de
zero a muitos itens, e diferente de `Future`, em que há sempre um único
`Output`, mesmo que seja o tipo unit `()`.

`Stream` também define um método para obter esses itens. Nós o chamamos de
`poll_next`, para deixar claro que ele faz poll da mesma forma que
`Future::poll` e produz uma sequência de itens da mesma forma que
`Iterator::next`. Seu tipo de retorno combina `Poll` com `Option`. O tipo externo
é `Poll`, porque precisa ter a prontidão verificada, exatamente como um future.
O tipo interno é `Option`, porque precisa sinalizar se há mais mensagens,
exatamente como um iterador.

Algo muito semelhante a essa definição provavelmente acabará fazendo parte da
biblioteca padrão do Rust. Enquanto isso, ela faz parte do kit de ferramentas da
maioria dos runtimes, então você pode contar com ela, e tudo o que veremos a
seguir deve se aplicar de modo geral!

Nos exemplos que vimos na seção [“Streams: Futures em
Sequência”][streams]<!-- ignore -->, porém, não usamos `poll_next` _nem_
`Stream`, mas sim `next` e `StreamExt`. É claro que _poderíamos_ trabalhar
diretamente em termos da API `poll_next` escrevendo à mão nossas próprias
máquinas de estados de `Stream`, assim como _poderíamos_ trabalhar diretamente
com futures por meio de seu método `poll`. Usar `await` é muito mais agradável,
contudo, e o trait `StreamExt` fornece o método `next` para que possamos fazer
exatamente isso:

```rust
{{#rustdoc_include ../listings/ch17-async-await/no-listing-stream-ext/src/lib.rs:here}}
```

<!--
TODO: update this if/when tokio/etc. update their MSRV and switch to using async functions
in traits, since the lack thereof is the reason they do not yet have this.
-->

> Note: A definição real que usamos anteriormente no capítulo é ligeiramente
> diferente desta, pois oferece suporte a versões do Rust que ainda não
> suportavam o uso de funções async em traits. Como resultado, ela se parece com
> isto:
>
> ```rust,ignore
> fn next(&mut self) -> Next<'_, Self> where Self: Unpin;
> ```
>
> Esse tipo `Next` é uma `struct` que implementa `Future` e nos permite nomear
> o *lifetime* da referência a `self` com `Next<'_, Self>`, para que `await`
> possa funcionar com esse método.

O trait `StreamExt` também é o lar de todos os métodos interessantes disponíveis
para uso com streams. `StreamExt` é implementado automaticamente para todo tipo
que implementa `Stream`, mas esses traits são definidos separadamente para
permitir que a comunidade itere sobre APIs de conveniência sem afetar o trait
fundamental.

Na versão de `StreamExt` usada no crate `trpl`, o trait não apenas define o
método `next`, mas também fornece uma implementação padrão de `next` que lida
corretamente com os detalhes de chamar `Stream::poll_next`. Isso significa que,
mesmo quando você precisar escrever seu próprio tipo de dado de streaming, você
_só_ precisará implementar `Stream`, e então qualquer pessoa que usar seu tipo de
dado poderá usar `StreamExt` e seus métodos com ele automaticamente.

Isso é tudo o que vamos abordar sobre os detalhes de nível mais baixo desses
traits. Para encerrar, vamos considerar como futures (incluindo streams), tarefas
e threads se encaixam!

[message-passing]: ch17-02-concurrency-with-async.md#sending-data-between-two-tasks-using-message-passing
[ch-18]: ch18-00-oop.html
[async-book]: https://rust-lang.github.io/async-book/
[under-the-hood]: https://rust-lang.github.io/async-book/02_execution/01_chapter.html
[pinning]: https://rust-lang.github.io/async-book/04_pinning/01_chapter.html
[first-async]: ch17-01-futures-and-syntax.html#our-first-async-program
[any-number-futures]: ch17-03-more-futures.html#working-with-any-number-of-futures
[streams]: ch17-04-streams.html
