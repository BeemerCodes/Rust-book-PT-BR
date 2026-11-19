<a id="putting-it-all-together-futures-tasks-and-threads"></a>

## Juntando Tudo: Futures, Tarefas e Threads

Como vimos no [Capítulo 16][ch16]<!-- ignore -->, threads oferecem uma abordagem
para concorrência. Vimos outra abordagem neste capítulo: usar async com futures
e streams. Se você está se perguntando quando escolher um método em vez do
outro, a resposta é: depende! E, em muitos casos, a escolha não é threads _ou_
async, mas sim threads _e_ async.

Muitos sistemas operacionais fornecem modelos de concorrência baseados em
threads há décadas, e muitas linguagens de programação dão suporte a eles como
resultado. No entanto, esses modelos não deixam de ter seus compromissos. Em
muitos sistemas operacionais, eles consomem uma quantidade considerável de
memória para cada thread. Threads também só são uma opção quando seu sistema
operacional e hardware oferecem suporte a elas. Diferentemente de computadores
desktop e dispositivos móveis comuns, alguns sistemas embarcados nem sequer têm
um sistema operacional, portanto também não têm threads.

O modelo async oferece um conjunto diferente — e, em última instância,
complementar — de compromissos. No modelo async, operações concorrentes não
exigem suas próprias threads. Em vez disso, elas podem ser executadas em
tarefas, como quando usamos `trpl::spawn_task` para iniciar trabalho a partir de
uma função síncrona na seção sobre streams. Uma tarefa é semelhante a uma
thread, mas, em vez de ser gerenciada pelo sistema operacional, ela é gerenciada
por código em nível de biblioteca: o runtime.

Há uma razão pela qual as APIs para criar threads e criar tarefas são tão
semelhantes. Threads atuam como uma fronteira para conjuntos de operações
síncronas; a concorrência é possível _entre_ threads. Tarefas atuam como uma
fronteira para conjuntos de operações _assíncronas_; a concorrência é possível
tanto _entre_ quanto _dentro_ de tarefas, pois uma tarefa pode alternar entre
futures em seu corpo. Por fim, futures são a unidade de concorrência mais
granular do Rust, e cada future pode representar uma árvore de outros futures. O
runtime — especificamente, seu executor — gerencia tarefas, e tarefas gerenciam
futures. Nesse sentido, tarefas são semelhantes a threads leves gerenciadas pelo
runtime, com capacidades adicionais que vêm do fato de serem gerenciadas por um
runtime em vez de pelo sistema operacional.

Isso não significa que tarefas assíncronas sejam sempre melhores do que threads
(ou vice-versa). A concorrência com threads é, de certa forma, um modelo de
programação mais simples do que a concorrência com `async`. Isso pode ser uma
vantagem ou uma desvantagem. Threads são um tanto “disparar e esquecer”; elas
não têm um equivalente nativo a um future, portanto simplesmente executam até a
conclusão sem serem interrompidas, exceto pelo próprio sistema operacional.

E acontece que threads e tarefas frequentemente funcionam muito bem juntas,
porque tarefas podem (pelo menos em alguns runtimes) ser movidas entre threads.
Na verdade, por baixo dos panos, o runtime que temos usado — incluindo as
funções `spawn_blocking` e `spawn_task` — é multithread por padrão! Muitos
runtimes usam uma abordagem chamada _work stealing_ para mover tarefas de forma
transparente entre threads, com base em como as threads estão sendo utilizadas
no momento, para melhorar o desempenho geral do sistema. Essa abordagem na
verdade exige threads _e_ tarefas e, consequentemente, futures.

Ao pensar sobre qual método usar e quando, considere estas regras práticas:

- Se o trabalho for _muito paralelizável_ (isto é, limitado por CPU, ou
  *CPU-bound*), como processar um monte de dados em que cada parte pode ser
  processada separadamente, threads são uma escolha melhor.
- Se o trabalho for _muito concorrente_ (isto é, limitado por E/S, ou
  *I/O-bound*), como lidar com mensagens de várias fontes diferentes que podem
  chegar em intervalos ou taxas distintas, async é uma escolha melhor.

E se você precisar tanto de paralelismo quanto de concorrência, não precisa
escolher entre threads e async. Você pode usá-los juntos livremente, deixando
cada um desempenhar o papel em que é melhor. Por exemplo, a Listagem 17-25 mostra
um exemplo bastante comum desse tipo de combinação em código Rust do mundo real.

<Listing number="17-25" caption="Enviando mensagens com código bloqueante em uma thread e aguardando as mensagens em um bloco async" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-25/src/main.rs:all}}
```

</Listing>

Começamos criando um canal assíncrono e, em seguida, criando uma thread que
assume o *ownership* do lado transmissor do canal usando a palavra-chave `move`.
Dentro da thread, enviamos os números de 1 a 10, dormindo por um segundo entre
cada envio. Por fim, executamos um future criado com um bloco async passado para
`trpl::block_on`, exatamente como fizemos ao longo do capítulo. Nesse future,
fazemos await dessas mensagens, assim como nos outros exemplos de troca de
mensagens que vimos.

Para voltar ao cenário com o qual abrimos o capítulo, imagine executar um
conjunto de tarefas de codificação de vídeo usando uma thread dedicada (porque
codificação de vídeo é limitada por computação), mas notificar a interface de
usuário de que essas operações foram concluídas usando um canal assíncrono. Há
inúmeros exemplos desses tipos de combinações em casos de uso do mundo real.

<a id="summary"></a>

## Resumo

Esta não é a última vez que você verá concorrência neste livro. O projeto no
[Capítulo 21][ch21]<!-- ignore --> aplicará esses conceitos em uma situação mais
realista do que os exemplos mais simples discutidos aqui e comparará a resolução
de problemas com threads versus tarefas e futures mais diretamente.

Independentemente de qual dessas abordagens você escolher, o Rust oferece as
ferramentas de que você precisa para escrever código concorrente seguro e rápido
— seja para um servidor web de alta vazão ou para um sistema operacional
embarcado.

A seguir, falaremos sobre maneiras idiomáticas de modelar problemas e estruturar
soluções à medida que seus programas em Rust crescem. Além disso, discutiremos
como os idiomas do Rust se relacionam com aqueles com os quais você talvez já
esteja familiarizado na programação orientada a objetos.

[ch16]: ch16-00-concurrency.html
[combining-futures]: ch17-03-more-futures.html#building-our-own-async-abstractions
[streams]: ch17-04-streams.html#composing-streams
[ch21]: ch21-00-final-project-a-web-server.html
