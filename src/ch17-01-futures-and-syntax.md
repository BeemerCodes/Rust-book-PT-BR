<a id="futures-and-the-async-syntax"></a>
## Futures e a Sintaxe Async

Os elementos-chave da programação assíncrona em Rust são as *futures* e as
palavras-chave `async` e `await` do Rust.

Uma *future* é um valor que pode não estar pronto agora, mas ficará pronto em
algum momento no futuro. (Esse mesmo conceito aparece em muitas linguagens, às
vezes sob outros nomes como *task* ou *promise*.) O Rust fornece a *trait*
`Future` como um bloco de construção para que diferentes operações assíncronas
possam ser implementadas com diferentes estruturas de dados, mas com uma
interface em comum. Em Rust, *futures* são tipos que implementam a *trait*
`Future`. Cada *future* guarda suas próprias informações sobre o progresso que
já foi feito e o que significa estar “pronta”.

Você pode aplicar a palavra-chave `async` a blocos e funções para especificar
que eles podem ser interrompidos e retomados. Dentro de um bloco `async` ou de
uma função `async`, você pode usar a palavra-chave `await` para *aguardar uma
future* (isto é, esperar que ela fique pronta). Qualquer ponto em que você
aguarda uma *future* dentro de um bloco ou função `async` é um possível lugar
para aquele bloco ou função pausar e retomar. O processo de verificar uma
*future* para saber se seu valor já está disponível é chamado de *polling*.

Algumas outras linguagens, como C# e JavaScript, também usam as palavras-chave
`async` e `await` para programação assíncrona. Se você estiver familiarizado com
essas linguagens, poderá notar algumas diferenças significativas na forma como o
Rust lida com a sintaxe. Há um bom motivo para isso, como veremos!

Ao escrever código Rust assíncrono, usamos as palavras-chave `async` e `await`
na maior parte do tempo. O Rust as compila em código equivalente usando a
*trait* `Future`, assim como compila laços `for` em código equivalente usando a
*trait* `Iterator`. Como o Rust fornece a *trait* `Future`, porém, você também
pode implementá-la para seus próprios tipos de dados quando precisar. Muitas
das funções que veremos ao longo deste capítulo retornam tipos com suas
próprias implementações de `Future`. Voltaremos à definição da *trait* no final
do capítulo e nos aprofundaremos mais em como ela funciona, mas esse nível de
detalhe já basta para continuarmos avançando.

Tudo isso pode parecer um pouco abstrato, então vamos escrever nosso primeiro
programa assíncrono: um pequeno *web scraper*. Passaremos duas URLs pela linha
de comando, buscaremos ambas de forma concorrente e retornaremos o resultado da
que terminar primeiro. Este exemplo terá bastante sintaxe nova, mas não se
preocupe — explicaremos tudo o que você precisa saber ao longo do caminho.

<a id="our-first-async-program"></a>
## Nosso Primeiro Programa Assíncrono

Para manter o foco deste capítulo em aprender programação assíncrona em vez de
ficar fazendo malabarismo com partes do ecossistema, criamos o *crate* `trpl`
(`trpl` é abreviação de “The Rust Programming Language”). Ele reexporta todos os
tipos, *traits* e funções de que você precisará, principalmente dos *crates*
[`futures`][futures-crate]<!-- ignore --> e [`tokio`][tokio]<!-- ignore -->. O
*crate* `futures` é um lar oficial para experimentação de código assíncrono em
Rust, e foi lá que a *trait* `Future` foi originalmente projetada. Tokio é o
*runtime* assíncrono mais amplamente usado em Rust hoje, especialmente para
aplicações web. Existem outros ótimos *runtimes* por aí, e eles podem ser mais
adequados aos seus objetivos. Usamos o *crate* `tokio` por baixo dos panos no
`trpl` porque ele é bem testado e amplamente utilizado.

Em alguns casos, o `trpl` também renomeia ou encapsula as APIs originais para
manter você focado nos detalhes relevantes para este capítulo. Se quiser entender
o que o *crate* faz, encorajamos você a conferir [seu código-fonte][crate-source].
Você poderá ver de qual *crate* vem cada reexportação, e deixamos comentários
extensos explicando o que o *crate* faz.

Crie um novo projeto binário chamado `hello-async` e adicione o *crate* `trpl`
como dependência:

```console
$ cargo new hello-async
$ cd hello-async
$ cargo add trpl
```

Agora podemos usar os vários recursos fornecidos pelo `trpl` para escrever nosso
primeiro programa assíncrono. Vamos construir uma pequena ferramenta de linha de
comando que busca duas páginas web, extrai o elemento `<title>` de cada uma e
imprime o título da página que concluir todo esse processo primeiro.

<a id="defining-the-page_title-function"></a>
### Definindo a Função `page_title`

Vamos começar escrevendo uma função que recebe uma URL de página como parâmetro,
faz uma requisição para ela e retorna o texto do elemento `<title>` (veja a
Listagem 17-1).

<Listing number="17-1" file-name="src/main.rs" caption="Definindo uma função assíncrona para obter o elemento de título de uma página HTML">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-01/src/main.rs:all}}
```

</Listing>

Primeiro, definimos uma função chamada `page_title` e a marcamos com a
palavra-chave `async`. Depois, usamos a função `trpl::get` para buscar qualquer
URL passada e adicionamos a palavra-chave `await` para aguardar a resposta. Para
obter o texto de `response`, chamamos seu método `text` e novamente o
aguardamos com a palavra-chave `await`. Ambas as etapas são assíncronas. Para a
função `get`, temos que esperar o servidor enviar a primeira parte da resposta,
que incluirá cabeçalhos HTTP, cookies e assim por diante, e pode ser entregue
separadamente do corpo da resposta. Especialmente se o corpo for muito grande,
pode levar algum tempo para ele chegar por completo. Como precisamos esperar
pela *totalidade* da resposta chegar, o método `text` também é assíncrono.

Temos que aguardar explicitamente ambas as *futures*, porque *futures* em Rust
são *preguiçosas* (*lazy*): elas não fazem nada até que você peça com a
palavra-chave `await`. (Na verdade, o Rust mostrará um aviso do compilador se
você não usar uma *future*.) Isso pode lembrar a discussão sobre iteradores na
seção [“Processando uma Série de Itens com Iteradores”][iterators-lazy]<!-- ignore -->
no Capítulo 13. Iteradores não fazem nada a menos que você chame seu método
`next` — seja diretamente ou usando laços `for` ou métodos como `map` que usam
`next` por baixo dos panos. Da mesma forma, *futures* não fazem nada a menos que
você peça explicitamente. Essa característica *lazy* permite que o Rust evite
executar código assíncrono até que ele seja realmente necessário.

> Note: Isso é diferente do comportamento que vimos ao usar `thread::spawn` na
> seção [“Criando uma Nova Thread com spawn”][thread-spawn]<!-- ignore --> no
> Capítulo 16, em que a *closure* que passamos para outra *thread* começava a
> executar imediatamente. Também é diferente de como muitas outras linguagens
> abordam `async`. Mas isso é importante para que o Rust possa oferecer suas
> garantias de desempenho, exatamente como acontece com iteradores.

Assim que temos `response_text`, podemos fazer o *parsing* dele para uma
instância do tipo `Html` usando `Html::parse`. Em vez de uma string bruta, agora
temos um tipo de dados que podemos usar para trabalhar com o HTML como uma
estrutura de dados mais rica. Em particular, podemos usar o método `select_first`
para encontrar a primeira ocorrência de um determinado seletor CSS. Ao passar a
string `"title"`, obteremos o primeiro elemento `<title>` do documento, se houver
algum. Como pode não haver nenhum elemento correspondente, `select_first`
retorna um `Option<ElementRef>`. Por fim, usamos o método `Option::map`, que nos
permite trabalhar com o item dentro do `Option` se ele estiver presente e não
fazer nada caso não esteja. (Também poderíamos usar uma expressão `match` aqui,
mas `map` é mais idiomático.) No corpo da função que fornecemos ao `map`,
chamamos `inner_html` em `title` para obter seu conteúdo, que é uma `String`.
No fim das contas, temos um `Option<String>`.

Note que a palavra-chave `await` do Rust vai *depois* da expressão que você está
aguardando, não antes dela. Ou seja, ela é uma palavra-chave *pós-fixada*
(*postfix*). Isso pode ser diferente do que você está acostumado se já usou
`async` em outras linguagens, mas em Rust isso torna cadeias de métodos muito
mais agradáveis de usar. Como resultado, poderíamos alterar o corpo de
`page_title` para encadear as chamadas das funções `trpl::get` e `text` com
`await` entre elas, como mostrado na Listagem 17-2.

<Listing number="17-2" file-name="src/main.rs" caption="Encadeando chamadas com a palavra-chave `await`">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-02/src/main.rs:chaining}}
```

</Listing>

Com isso, escrevemos nossa primeira função assíncrona com sucesso! Antes de
adicionarmos algum código em `main` para chamá-la, vamos falar um pouco mais
sobre o que escrevemos e o que isso significa.

Quando o Rust vê um *bloco* marcado com a palavra-chave `async`, ele o compila
em um tipo de dados único e anônimo que implementa a *trait* `Future`. Quando o
Rust vê uma *função* marcada com `async`, ele a compila em uma função não
assíncrona cujo corpo é um bloco `async`. O tipo de retorno de uma função
assíncrona é o tipo do dado anônimo que o compilador cria para aquele bloco
`async`.

Portanto, escrever `async fn` é equivalente a escrever uma função que retorna
uma *future* do tipo de retorno. Para o compilador, uma definição de função como
a `async fn page_title` na Listagem 17-1 é aproximadamente equivalente a uma
função não assíncrona definida assim:

```rust
# extern crate trpl; // necessário para o mdbook test
use std::future::Future;
use trpl::Html;

fn page_title(url: &str) -> impl Future<Output = Option<String>> {
    async move {
        let text = trpl::get(url).await.text().await;
        Html::parse(&text)
            .select_first("title")
            .map(|title| title.inner_html())
    }
}
```

Vamos passar por cada parte da versão transformada:

- Ela usa a sintaxe `impl Trait` que discutimos lá no Capítulo 10, na seção
  [“Traits como Parâmetros”][impl-trait]<!-- ignore -->.
- O valor retornado implementa a *trait* `Future` com um tipo associado
  `Output`. Note que o tipo `Output` é `Option<String>`, que é o mesmo tipo de
  retorno original da versão `async fn` de `page_title`.
- Todo o código chamado no corpo da função original é envolvido em um bloco
  `async move`. Lembre-se de que blocos são expressões. Todo esse bloco é a
  expressão retornada pela função.
- Esse bloco `async` produz um valor com o tipo `Option<String>`, conforme
  acabamos de descrever. Esse valor corresponde ao tipo `Output` no tipo de
  retorno. Isso funciona exatamente como outros blocos que você já viu.
- O novo corpo da função é um bloco `async move` por causa da maneira como ele
  usa o parâmetro `url`. (Falaremos muito mais sobre `async` versus
  `async move` mais adiante no capítulo.)

Agora podemos chamar `page_title` em `main`.

<!-- Old headings. Do not remove or links may break. -->

<a id ="determining-a-single-pages-title"></a>
<a id="executing-an-async-function-with-a-runtime"></a>
### Executando uma Função Assíncrona com um Runtime

Para começar, vamos obter o título de uma única página, como mostrado na Listagem
17-3. Infelizmente, este código ainda não compila.

<Listing number="17-3" file-name="src/main.rs" caption="Chamando a função `page_title` a partir de `main` com um argumento fornecido pelo usuário">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-03/src/main.rs:main}}
```

</Listing>

Seguimos o mesmo padrão que usamos para obter argumentos de linha de comando na
seção [“Aceitando Argumentos de Linha de Comando”][cli-args]<!-- ignore --> no
Capítulo 12. Em seguida, passamos o argumento URL para `page_title` e aguardamos
o resultado. Como o valor produzido pela *future* é um `Option<String>`, usamos
uma expressão `match` para imprimir mensagens diferentes dependendo de a página
ter ou não um `<title>`.

O único lugar em que podemos usar a palavra-chave `await` é em funções ou blocos
`async`, e o Rust não nos deixa marcar a função especial `main` como `async`.

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-03
cargo build
copy just the compiler error
-->

```text
error[E0752]: `main` function is not allowed to be `async`
 --> src/main.rs:6:1
  |
6 | async fn main() {
  | ^^^^^^^^^^^^^^^ `main` function is not allowed to be `async`
```

O motivo pelo qual `main` não pode ser marcada como `async` é que código
assíncrono precisa de um *runtime*: um *crate* Rust que gerencia os detalhes da
execução de código assíncrono. A função `main` de um programa pode *inicializar*
um *runtime*, mas ela *em si* não é um *runtime*. (Veremos mais sobre o porquê
disso daqui a pouco.) Todo programa Rust que executa código assíncrono tem pelo
menos um lugar onde configura um *runtime* que executa as *futures*.

A maioria das linguagens que suportam `async` já vem com um *runtime* embutido,
mas o Rust não. Em vez disso, existem muitos *runtimes* assíncronos diferentes
disponíveis, cada um fazendo compromissos distintos adequados ao caso de uso
que almeja. Por exemplo, um servidor web de alta vazão com muitos núcleos de CPU
e uma grande quantidade de RAM tem necessidades muito diferentes de um
microcontrolador com um único núcleo, pouca RAM e nenhuma capacidade de alocação
na *heap*. Os *crates* que fornecem esses *runtimes* também costumam fornecer
versões assíncronas de funcionalidades comuns, como E/S de arquivos ou de rede.

Aqui, e ao longo do restante deste capítulo, usaremos a função `block_on` do
*crate* `trpl`, que recebe uma *future* como argumento e bloqueia a *thread*
atual até que essa *future* execute até a conclusão. Por baixo dos panos, chamar
`block_on` configura um *runtime* usando o *crate* `tokio`, que é usado para
executar a *future* passada (o comportamento da `block_on` do *crate* `trpl` é
semelhante ao das funções `block_on` de outros *crates* de *runtime*). Assim que
a *future* é concluída, `block_on` retorna qualquer valor que a *future* tenha
produzido.

Poderíamos passar a *future* retornada por `page_title` diretamente para
`block_on` e, assim que ela terminasse, poderíamos fazer o `match` no
`Option<String>` resultante, como tentamos fazer na Listagem 17-3. No entanto,
para a maioria dos exemplos do capítulo (e para a maioria dos códigos assíncronos
no mundo real), faremos mais do que apenas uma chamada de função assíncrona,
então em vez disso passaremos um bloco `async` e aguardaremos explicitamente o
resultado da chamada a `page_title`, como na Listagem 17-4.

<Listing number="17-4" caption="Aguardando um bloco `async` com `trpl::block_on`" file-name="src/main.rs">

<!-- should_panic,noplayground because mdbook test does not pass args -->

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch17-async-await/listing-17-04/src/main.rs:run}}
```

</Listing>

Quando executamos este código, obtemos o comportamento que esperávamos
inicialmente:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-04
cargo build # skip all the build noise
cargo run -- "https://www.rust-lang.org"
# copy the output here
-->

```console
$ cargo run -- "https://www.rust-lang.org"
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/async_await 'https://www.rust-lang.org'`
The title for https://www.rust-lang.org was
            Rust Programming Language
```

Ufa — finalmente temos algum código assíncrono funcionando! Mas, antes de
adicionarmos o código para colocar dois sites em disputa um contra o outro,
vamos voltar brevemente nossa atenção para como as *futures* funcionam.

Cada *ponto de await* — isto é, todo lugar onde o código usa a palavra-chave
`await` — representa um ponto onde o controle é devolvido ao *runtime*. Para que
isso funcione, o Rust precisa manter o controle do estado envolvido no bloco
`async`, de modo que o *runtime* possa iniciar algum outro trabalho e depois
voltar quando estiver pronto para tentar avançar o primeiro novamente. Isso é
uma máquina de estados invisível, como se você tivesse escrito um `enum` como
este para salvar o estado atual em cada ponto de `await`:

```rust
{{#rustdoc_include ../listings/ch17-async-await/no-listing-state-machine/src/lib.rs:enum}}
```

Escrever manualmente o código para fazer a transição entre cada estado seria
tedioso e propenso a erros, no entanto, especialmente quando você precisar
adicionar mais funcionalidades e mais estados ao código mais tarde. Felizmente,
o compilador Rust cria e gerencia automaticamente as estruturas de dados da
máquina de estados para código assíncrono. As regras normais de *borrowing* e
*ownership* em torno de estruturas de dados continuam todas valendo e, por sorte,
o compilador também cuida de verificá-las para nós e fornece mensagens de erro
úteis. Trabalharemos em algumas delas mais adiante no capítulo.

Em última instância, algo precisa executar essa máquina de estados, e esse algo
é um *runtime*. (É por isso que você pode se deparar com menções a *executores*
ao pesquisar sobre *runtimes*: um *executor* é a parte de um *runtime*
responsável por executar o código assíncrono.)

Agora você pode ver por que o compilador nos impediu de transformar a própria
`main` em uma função assíncrona lá na Listagem 17-3. Se `main` fosse uma função
assíncrona, alguma outra coisa precisaria gerenciar a máquina de estados para
qualquer *future* que `main` retornasse, mas `main` é o ponto de partida do
programa! Em vez disso, chamamos a função `trpl::block_on` em `main` para
configurar um *runtime* e executar a *future* retornada pelo bloco `async` até
que ela termine.

> Note: Alguns *runtimes* fornecem macros para que você *possa* escrever uma
> função `main` assíncrona. Essas macros reescrevem `async fn main() { ... }`
> para ser uma `fn main` normal, que faz a mesma coisa que fizemos manualmente
> na Listagem 17-4: chamar uma função que executa uma *future* até a conclusão
> da mesma forma que `trpl::block_on` faz.

Agora vamos juntar essas peças e ver como podemos escrever código concorrente.

<!-- Old headings. Do not remove or links may break. -->

<a id="racing-our-two-urls-against-each-other"></a>
<a id="racing-two-urls-against-each-other-concurrently"></a>
### Colocando Duas URLs em Disputa de Forma Concorrente

Na Listagem 17-5, chamamos `page_title` com duas URLs diferentes passadas pela
linha de comando e as colocamos em disputa selecionando qual *future* terminar
primeiro.

<Listing number="17-5" caption="Chamando `page_title` para duas URLs para ver qual retorna primeiro" file-name="src/main.rs">

<!-- should_panic,noplayground because mdbook does not pass args -->

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch17-async-await/listing-17-05/src/main.rs:all}}
```

</Listing>

Começamos chamando `page_title` para cada uma das URLs fornecidas pelo usuário.
Salvamos as *futures* resultantes como `title_fut_1` e `title_fut_2`. Lembre-se,
elas ainda não fazem nada, porque *futures* são *lazy* e ainda não as
aguardamos. Em seguida, passamos as *futures* para `trpl::select`, que retorna
um valor indicando qual das *futures* passadas para ela terminou primeiro.

> Note: Por baixo dos panos, `trpl::select` é construída sobre uma função
> `select` mais geral definida no *crate* `futures`. A função `select` do
> *crate* `futures` consegue fazer muitas coisas que a função `trpl::select` não
> consegue, mas também tem alguma complexidade adicional que podemos pular por
> enquanto.

Qualquer uma das *futures* pode legitimamente “vencer”, então não faz sentido
retornar um `Result`. Em vez disso, `trpl::select` retorna um tipo que ainda não
vimos, `trpl::Either`. O tipo `Either` é um tanto semelhante a um `Result` no
sentido de que possui dois casos. Ao contrário de `Result`, porém, não há noção
de sucesso ou falha embutida em `Either`. Em vez disso, ele usa `Left` e
`Right` para indicar “um ou outro”:

```rust
enum Either<A, B> {
    Left(A),
    Right(B),
}
```

A função `select` retorna `Left` com a saída daquela *future* se o primeiro
argumento vencer, e `Right` com a saída do segundo argumento *future* se *esse*
vencer. Isso corresponde à ordem em que os argumentos aparecem na chamada da
função: o primeiro argumento está à esquerda do segundo argumento.

Também atualizamos `page_title` para retornar a mesma URL passada a ela. Dessa
forma, se a página que retornar primeiro não tiver um `<title>` que possamos
resolver, ainda poderemos imprimir uma mensagem significativa. Com essa
informação disponível, finalizamos atualizando nossa saída de `println!` para
indicar tanto qual URL terminou primeiro quanto qual é, se houver, o `<title>`
da página web naquela URL.

Agora você construiu um pequeno *web scraper* funcional! Escolha algumas URLs e
execute a ferramenta de linha de comando. Você poderá descobrir que alguns
sites são consistentemente mais rápidos que outros, enquanto em outros casos o
site mais rápido varia de execução para execução. Mais importante ainda, você
aprendeu o básico sobre como trabalhar com *futures*, então agora podemos nos
aprofundar no que podemos fazer com `async`.

[impl-trait]: ch10-02-traits.html#traits-as-parameters
[iterators-lazy]: ch13-02-iterators.html
[thread-spawn]: ch16-01-threads.html#creating-a-new-thread-with-spawn
[cli-args]: ch12-01-accepting-command-line-arguments.html

<!-- TODO: map source link version to version of Rust? -->

[crate-source]: https://github.com/rust-lang/book/tree/main/packages/trpl
[futures-crate]: https://crates.io/crates/futures
[tokio]: https://tokio.rs
