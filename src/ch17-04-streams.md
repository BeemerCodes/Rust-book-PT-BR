<!-- Old headings. Do not remove or links may break. -->

<a id="streams"></a>
<a id="streams-futures-in-sequence"></a>

## Streams: Futures em Sequência

Lembre-se de como usamos o receptor do nosso canal assíncrono anteriormente
neste capítulo, na seção [“Troca de Mensagens”][17-02-messages]<!-- ignore -->.
O método assíncrono `recv` produz uma sequência de itens ao longo do tempo.
Esse é um exemplo de um padrão muito mais geral conhecido como _stream_.
Muitos conceitos são naturalmente representados como streams: itens que se
tornam disponíveis em uma fila, pedaços de dados sendo lidos incrementalmente do
sistema de arquivos quando o conjunto completo de dados é grande demais para a
memória do computador, ou dados chegando pela rede ao longo do tempo. Como
streams são futures, podemos usá-los com qualquer outro tipo de future e
combiná-los de maneiras interessantes. Por exemplo, podemos agrupar eventos em
lotes para evitar disparar chamadas de rede em excesso, definir timeouts em
sequências de operações demoradas ou limitar a frequência de eventos de
interface de usuário para evitar trabalho desnecessário.

Vimos uma sequência de itens lá no Capítulo 13, quando analisamos o trait
`Iterator` na seção [“O Trait Iterator e o Método `next`”][iterator-trait]<!--
ignore -->, mas há duas diferenças entre iteradores e o receptor do canal
assíncrono. A primeira diferença é o tempo: iteradores são síncronos, enquanto
o receptor do canal é assíncrono. A segunda diferença é a API. Ao trabalhar
diretamente com `Iterator`, chamamos seu método síncrono `next`. Já com o
stream `trpl::Receiver` em particular, chamamos o método assíncrono `recv`.
Fora isso, essas APIs parecem muito semelhantes, e essa semelhança não é
coincidência. Um stream é como uma forma assíncrona de iteração. Porém,
enquanto o `trpl::Receiver` espera especificamente para receber mensagens, a API
de stream de uso geral é muito mais ampla: ela fornece o próximo item da mesma
forma que `Iterator` faz, só que de maneira assíncrona.

A semelhança entre iteradores e streams em Rust significa que podemos, na
verdade, criar um stream a partir de qualquer iterador. Assim como acontece com
um iterador, podemos trabalhar com um stream chamando seu método `next` e então
fazendo await da saída, como na Listagem 17-21, que ainda não compila.

<Listing number="17-21" caption="Criando um stream a partir de um iterador e imprimindo seus valores" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-21/src/main.rs:stream}}
```

</Listing>

Começamos com um array de números, o qual convertemos em um iterador e então
chamamos `map` para dobrar todos os valores. Em seguida, convertemos o iterador
em um stream usando a função `trpl::stream_from_iter`. Depois, iteramos pelos
itens do stream conforme eles chegam com o laço `while let`.

Infelizmente, quando tentamos executar o código, ele não compila e informa que
não há nenhum método `next` disponível:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-21
cargo build
copy only the error output
-->

```text
error[E0599]: no method named `next` found for struct `tokio_stream::iter::Iter` in the current scope
  --> src/main.rs:10:40
   |
10 |         while let Some(value) = stream.next().await {
   |                                        ^^^^
   |
   = help: items from traits can only be used if the trait is in scope
help: the following traits which provide `next` are implemented but not in scope; perhaps you want to import one of them
   |
1  + use crate::trpl::StreamExt;
   |
1  + use futures_util::stream::stream::StreamExt;
   |
1  + use std::iter::Iterator;
   |
1  + use std::str::pattern::Searcher;
   |
help: there is a method `try_next` with a similar name
   |
10 |         while let Some(value) = stream.try_next().await {
   |                                        ~~~~~~~~
```

Como essa saída explica, o motivo do erro de compilação é que precisamos do
trait certo no escopo para podermos usar o método `next`. Considerando nossa
discussão até aqui, você poderia esperar naturalmente que esse trait fosse
`Stream`, mas na verdade é `StreamExt`. Abreviação de _extension_ (extensão),
`Ext` é um padrão comum na comunidade Rust para estender um trait com outro.

O trait `Stream` define uma interface de baixo nível que, na prática, combina os
traits `Iterator` e `Future`. `StreamExt` fornece um conjunto de APIs de nível
mais alto sobre `Stream`, incluindo o método `next`, além de outros métodos
utilitários semelhantes aos fornecidos pelo trait `Iterator`. `Stream` e
`StreamExt` ainda não fazem parte da biblioteca padrão do Rust, mas a maioria
dos crates do ecossistema usa definições semelhantes.

A correção para o erro de compilação é adicionar uma instrução `use` para
`trpl::StreamExt`, como na Listagem 17-22.

<Listing number="17-22" caption="Usando com sucesso um iterador como base para um stream" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-22/src/main.rs:all}}
```

</Listing>

Com todas essas peças reunidas, este código funciona da maneira que queremos!
Além disso, agora que temos `StreamExt` no escopo, podemos usar todos os seus
métodos utilitários, exatamente como fazemos com iteradores.

[17-02-messages]: ch17-02-concurrency-with-async.html#message-passing
[iterator-trait]: ch13-02-iterators.html#the-iterator-trait-and-the-next-method
