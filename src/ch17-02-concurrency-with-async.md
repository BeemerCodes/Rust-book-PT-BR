<!-- Old headings. Do not remove or links may break. -->

<a id="concurrency-with-async"></a>
<a id="applying-concurrency-with-async"></a>
## Aplicando Concorrência com Async

Nesta seção, aplicaremos `async` a alguns dos mesmos desafios de concorrência
que enfrentamos com *threads* no Capítulo 16. Como já conversamos sobre muitas
das ideias principais lá, nesta seção vamos focar no que é diferente entre
*threads* e *futures*.

Em muitos casos, as APIs para trabalhar com concorrência usando `async` são
muito semelhantes às usadas com *threads*. Em outros casos, elas acabam sendo
bem diferentes. Mesmo quando as APIs *parecem* semelhantes entre *threads* e
`async`, elas frequentemente têm comportamentos diferentes — e quase sempre têm
características de desempenho diferentes.

<!-- Old headings. Do not remove or links may break. -->

<a id="counting"></a>
<a id="creating-a-new-task-with-spawn_task"></a>
### Criando uma Nova Tarefa com `spawn_task`

A primeira operação que abordamos na seção [“Criando uma Nova Thread com
`spawn`”][thread-spawn]<!-- ignore --> no Capítulo 16 foi contar em duas
*threads* separadas. Vamos fazer o mesmo usando `async`. O *crate* `trpl`
fornece uma função `spawn_task` que é muito parecida com a API `thread::spawn`,
e uma função `sleep` que é uma versão assíncrona da API `thread::sleep`. Podemos
usá-las juntas para implementar o exemplo da contagem, como mostrado na
Listagem 17-6.

<Listing number="17-6" caption="Criando uma nova tarefa para imprimir uma coisa enquanto a tarefa principal imprime outra" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-06/src/main.rs:all}}
```

</Listing>

Como ponto de partida, configuramos nossa função `main` com `trpl::block_on`
para que nossa função de nível superior possa ser assíncrona.

> Note: Deste ponto em diante no capítulo, todos os exemplos incluirão exatamente
> esse mesmo código de envolvimento com `trpl::block_on` em `main`, então
> frequentemente vamos omiti-lo, assim como fazemos com `main`. Lembre-se de
> incluí-lo no seu código!

Em seguida, escrevemos dois laços dentro desse bloco, cada um contendo uma
chamada a `trpl::sleep`, que espera por meio segundo (500 milissegundos) antes
de enviar a próxima mensagem. Colocamos um laço no corpo de um
`trpl::spawn_task` e o outro em um laço `for` no nível superior. Também
adicionamos um `await` após as chamadas a `sleep`.

Este código se comporta de maneira semelhante à implementação baseada em
*threads* — incluindo o fato de que você pode ver as mensagens aparecerem em uma
ordem diferente no seu próprio terminal ao executá-lo:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
hi number 1 from the second task!
hi number 1 from the first task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
```

Esta versão para assim que o laço `for` no corpo do bloco `async` principal
termina, porque a tarefa criada por `spawn_task` é encerrada quando a função
`main` termina. Se você quiser que ela execute até a conclusão completa da
tarefa, precisará usar um *join handle* para esperar a primeira tarefa terminar.
Com *threads*, usamos o método `join` para “bloquear” até que a *thread*
terminasse de executar. Na Listagem 17-7, podemos usar `await` para fazer a
mesma coisa, porque o próprio *handle* da tarefa é uma *future*. Seu tipo
`Output` é um `Result`, então também fazemos `unwrap` nele depois de aguardá-lo.

<Listing number="17-7" caption="Usando `await` com um *join handle* para executar uma tarefa até a conclusão" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-07/src/main.rs:handle}}
```

</Listing>

Esta versão atualizada executa até que *ambos* os laços terminem:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
hi number 1 from the second task!
hi number 1 from the first task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
hi number 6 from the first task!
hi number 7 from the first task!
hi number 8 from the first task!
hi number 9 from the first task!
```

Até aqui, parece que `async` e *threads* nos dão resultados semelhantes, apenas
com sintaxe diferente: usar `await` em vez de chamar `join` no *join handle*, e
aguardar as chamadas a `sleep`.

A maior diferença é que não precisamos criar outra *thread* do sistema
operacional para fazer isso. Na verdade, nem sequer precisamos criar uma tarefa
com `spawn_task` aqui. Como blocos `async` compilam para *futures* anônimas,
podemos colocar cada laço em um bloco `async` e fazer com que o *runtime*
execute ambos até a conclusão usando a função `trpl::join`.

Na seção [“Esperando Todas as Threads Terminarem”][join-handles]<!-- ignore -->
no Capítulo 16, mostramos como usar o método `join` no tipo `JoinHandle`
retornado quando você chama `std::thread::spawn`. A função `trpl::join` é
semelhante, mas para *futures*. Quando você passa duas *futures* para ela, ela
produz uma única nova *future* cuja saída é uma tupla contendo a saída de cada
*future* passada, assim que *ambas* forem concluídas. Assim, na Listagem 17-8,
usamos `trpl::join` para esperar que tanto `fut1` quanto `fut2` terminem. Nós
*não* aguardamos `fut1` e `fut2` diretamente, mas sim a nova *future* produzida
por `trpl::join`. Ignoramos a saída, porque ela é apenas uma tupla contendo dois
valores unitários (`()`).

<Listing number="17-8" caption="Usando `trpl::join` para aguardar duas *futures* anônimas" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-08/src/main.rs:join}}
```

</Listing>

Quando executamos isto, vemos ambas as *futures* executarem até a conclusão:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
hi number 1 from the first task!
hi number 1 from the second task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
hi number 6 from the first task!
hi number 7 from the first task!
hi number 8 from the first task!
hi number 9 from the first task!
```

Agora, você verá exatamente a mesma ordem todas as vezes, o que é muito
diferente do que vimos com *threads* e com `trpl::spawn_task` na Listagem 17-7.
Isso acontece porque a função `trpl::join` é *justa* (*fair*), o que significa
que ela verifica cada *future* com a mesma frequência, alternando entre elas, e
nunca deixa uma disparar na frente se a outra estiver pronta. Com *threads*, o
sistema operacional decide qual *thread* verificar e por quanto tempo deixá-la
rodar. Com Rust assíncrono, o *runtime* decide qual tarefa verificar. (Na
prática, os detalhes ficam complicados porque um *runtime* assíncrono pode usar
*threads* do sistema operacional por baixo dos panos como parte de como gerencia
a concorrência, portanto garantir justiça pode dar mais trabalho para um
*runtime* — mas ainda é possível!) Os *runtimes* não são obrigados a garantir
justiça para uma determinada operação, e muitas vezes oferecem APIs diferentes
para permitir que você escolha se quer justiça ou não.

Experimente algumas destas variações ao aguardar as *futures* e veja o que elas
fazem:

- Remova o bloco `async` ao redor de um dos laços ou de ambos.
- Aguarde cada bloco `async` imediatamente após defini-lo.
- Envolva apenas o primeiro laço em um bloco `async` e aguarde a *future*
  resultante depois do corpo do segundo laço.

Para um desafio extra, veja se você consegue descobrir qual será a saída em cada
caso *antes* de executar o código!

<!-- Old headings. Do not remove or links may break. -->

<a id="message-passing"></a>
<a id="counting-up-on-two-tasks-using-message-passing"></a>
<a id="sending-data-between-two-tasks-using-message-passing"></a>
### Enviando Dados Entre Duas Tarefas Usando Troca de Mensagens

Compartilhar dados entre *futures* também será familiar: usaremos troca de
mensagens (*message passing*) novamente, mas desta vez com versões assíncronas
dos tipos e funções. Seguiremos um caminho um pouco diferente do que seguimos na
seção [“Transferindo Dados Entre Threads com Troca de
Mensagens”][message-passing-threads]<!-- ignore --> no Capítulo 16 para ilustrar
algumas das diferenças fundamentais entre concorrência baseada em *threads* e
concorrência baseada em *futures*. Na Listagem 17-9, começaremos com apenas um
único bloco `async` — *sem* criar uma tarefa separada como fizemos ao criar uma
*thread* separada.

<Listing number="17-9" caption="Criando um canal assíncrono e atribuindo as duas metades a `tx` e `rx`" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-09/src/main.rs:channel}}
```

</Listing>

Aqui, usamos `trpl::channel`, uma versão assíncrona da API de canal com
múltiplos produtores e único consumidor (*multiple-producer, single-consumer*)
que usamos com *threads* lá no Capítulo 16. A versão assíncrona da API é apenas
um pouco diferente da versão baseada em *threads*: ela usa um receptor `rx`
mutável em vez de imutável, e seu método `recv` produz uma *future* que
precisamos aguardar em vez de produzir o valor diretamente. Agora podemos enviar
mensagens do transmissor para o receptor. Note que não precisamos criar uma
*thread* separada nem sequer uma tarefa; precisamos apenas aguardar a chamada a
`rx.recv`.

O método síncrono `Receiver::recv` em `std::mpsc::channel` bloqueia até receber
uma mensagem. O método `trpl::Receiver::recv` não bloqueia, porque é assíncrono.
Em vez de bloquear, ele devolve o controle ao *runtime* até que uma mensagem seja
recebida ou que o lado de envio do canal seja fechado. Em contraste, nós não
aguardamos a chamada a `send`, porque ela não bloqueia. Ela não precisa
bloquear, porque o canal para o qual estamos enviando não tem limite de
capacidade (*unbounded*).

> Note: Como todo esse código assíncrono é executado em um bloco `async` dentro
> de uma chamada a `trpl::block_on`, tudo dentro dele pode evitar bloqueios. No
> entanto, o código *fora* dele ficará bloqueado esperando a função `block_on`
> retornar. Esse é justamente o propósito da função `trpl::block_on`: ela
> permite que você *escolha* onde bloquear em algum conjunto de código
> assíncrono e, assim, onde fazer a transição entre código síncrono e
> assíncrono.

Repare em duas coisas sobre este exemplo. Primeiro, a mensagem chegará
imediatamente. Segundo, embora usemos uma *future* aqui, ainda não há
concorrência. Tudo na listagem acontece em sequência, exatamente como
aconteceria se não houvesse *futures* envolvidas.

Vamos resolver a primeira parte enviando uma série de mensagens e dormindo
entre elas, como mostrado na Listagem 17-10.

<!-- We cannot test this one because it never stops! -->

<Listing number="17-10" caption="Enviando e recebendo várias mensagens pelo canal assíncrono e dormindo com um `await` entre cada mensagem" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-10/src/main.rs:many-messages}}
```

</Listing>

Além de enviar as mensagens, precisamos recebê-las. Neste caso, como sabemos
quantas mensagens estão chegando, poderíamos fazer isso manualmente chamando
`rx.recv().await` quatro vezes. No mundo real, porém, geralmente estaremos
esperando por um número *desconhecido* de mensagens, então precisamos continuar
esperando até determinarmos que não há mais mensagens.

Na Listagem 16-10, usamos um laço `for` para processar todos os itens recebidos
de um canal síncrono. O Rust ainda não tem uma forma de usar um laço `for` com
uma série de itens *produzida assincronamente*, no entanto, então precisamos
usar um laço que ainda não vimos: o laço condicional `while let`. Essa é a
versão em laço da construção `if let` que vimos lá na seção [“Fluxo de Controle
Conciso com `if let` e `let...else`”][if-let]<!-- ignore --> no Capítulo 6. O
laço continuará executando enquanto o padrão especificado continuar casando com
o valor.

A chamada a `rx.recv` produz uma *future*, que nós aguardamos. O *runtime*
pausará a *future* até que ela esteja pronta. Assim que uma mensagem chegar, a
*future* resolverá para `Some(message)` quantas vezes uma mensagem chegar.
Quando o canal fechar, independentemente de *alguma* mensagem ter chegado ou
não, a *future* resolverá para `None` para indicar que não há mais valores e
que, portanto, devemos parar de fazer *polling* — isto é, parar de aguardar.

O laço `while let` junta tudo isso. Se o resultado da chamada `rx.recv().await`
for `Some(message)`, obtemos acesso à mensagem e podemos usá-la no corpo do
laço, exatamente como faríamos com `if let`. Se o resultado for `None`, o laço
termina. Toda vez que o laço termina uma iteração, ele atinge o ponto de
`await` novamente, então o *runtime* o pausa de novo até outra mensagem chegar.

O código agora envia e recebe todas as mensagens com sucesso. Infelizmente,
ainda há alguns problemas. Por um lado, as mensagens não chegam em intervalos de
meio segundo. Elas chegam todas de uma vez, 2 segundos (2.000 milissegundos)
depois que iniciamos o programa. Por outro lado, este programa também nunca
termina! Em vez disso, ele fica esperando para sempre por novas mensagens. Você
precisará encerrá-lo usando <kbd>ctrl</kbd>-<kbd>C</kbd>.

<a id="code-within-one-async-block-executes-linearly"></a>
#### O Código Dentro de um Bloco Async Executa Linearmente

Vamos começar examinando por que as mensagens chegam todas de uma vez após o
atraso completo, em vez de chegarem com pausas entre cada uma. Dentro de um
mesmo bloco `async`, a ordem em que as palavras-chave `await` aparecem no código
também é a ordem em que elas são executadas quando o programa roda.

Há apenas um bloco `async` na Listagem 17-10, então tudo dentro dele executa
linearmente. Ainda não há concorrência. Todas as chamadas a `tx.send`
acontecem, intercaladas com todas as chamadas a `trpl::sleep` e seus pontos de
`await` associados. Só depois disso é que o laço `while let` consegue passar
por qualquer um dos pontos de `await` nas chamadas a `recv`.

Para obter o comportamento que queremos, em que a pausa de espera acontece
entre cada mensagem, precisamos colocar as operações de `tx` e `rx` em seus
próprios blocos `async`, como mostrado na Listagem 17-11. Então o *runtime*
poderá executar cada um deles separadamente usando `trpl::join`, exatamente como
na Listagem 17-8. Mais uma vez, aguardamos o resultado de chamar `trpl::join`, e
não as *futures* individuais. Se aguardássemos as *futures* individuais em
sequência, acabaríamos voltando para um fluxo sequencial — exatamente o que
estamos tentando *não* fazer.

<!-- We cannot test this one because it never stops! -->

<Listing number="17-11" caption="Separando `send` e `recv` em seus próprios blocos `async` e aguardando as *futures* desses blocos" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-11/src/main.rs:futures}}
```

</Listing>

Com o código atualizado na Listagem 17-11, as mensagens são impressas em
intervalos de 500 milissegundos, em vez de todas de uma só vez após 2 segundos.

<a id="moving-ownership-into-an-async-block"></a>
#### Movendo Ownership para Dentro de um Bloco Async

O programa ainda nunca termina, porém, por causa da forma como o laço
`while let` interage com `trpl::join`:

- A *future* retornada por `trpl::join` só é concluída quando *ambas* as
  *futures* passadas para ela forem concluídas.
- A *future* `tx_fut` é concluída assim que termina de dormir após enviar a
  última mensagem em `vals`.
- A *future* `rx_fut` não será concluída até que o laço `while let` termine.
- O laço `while let` não terminará até que aguardar `rx.recv` produza `None`.
- Aguardar `rx.recv` só retornará `None` quando a outra extremidade do canal for
  fechada.
- O canal só fechará se chamarmos `rx.close` ou quando o lado transmissor, `tx`,
  sofrer *drop*.
- Nós não chamamos `rx.close` em lugar nenhum, e `tx` não sofrerá *drop* até que
  o bloco `async` mais externo passado para `trpl::block_on` termine.
- Esse bloco não pode terminar porque está bloqueado esperando `trpl::join`
  concluir, o que nos leva de volta ao topo desta lista.

No momento, o bloco `async` onde enviamos as mensagens apenas faz *borrowing* de
`tx`, porque enviar uma mensagem não exige *ownership*; mas, se pudermos *mover*
`tx` para dentro daquele bloco `async`, ele sofrerá *drop* assim que o bloco
terminar. Na seção [“Capturando Referências ou Movendo
Ownership”][capture-or-move]<!-- ignore --> no Capítulo 13, você aprendeu a
usar a palavra-chave `move` com *closures* e, como discutido na seção [“Usando
Closures `move` com Threads”][move-threads]<!-- ignore --> no Capítulo 16, muitas
vezes precisamos mover dados para dentro de *closures* ao trabalhar com
*threads*. As mesmas dinâmicas básicas se aplicam a blocos `async`, então a
palavra-chave `move` funciona com blocos `async` exatamente como funciona com
*closures*.

Na Listagem 17-12, mudamos o bloco usado para enviar mensagens de `async` para
`async move`.

<Listing number="17-12" caption="Uma revisão do código da Listagem 17-11 que encerra corretamente quando concluído" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-12/src/main.rs:with-move}}
```

</Listing>

Quando executamos *esta* versão do código, ela encerra graciosamente após a
última mensagem ser enviada e recebida. Em seguida, vamos ver o que precisaria
mudar para enviar dados a partir de mais de uma *future*.

<a id="joining-a-number-of-futures-with-the-join-macro"></a>
#### Unindo Várias Futures com a Macro `join!`

Este canal assíncrono também é um canal de múltiplos produtores, então podemos
chamar `clone` em `tx` se quisermos enviar mensagens de múltiplas *futures*,
como mostrado na Listagem 17-13.

<Listing number="17-13" caption="Usando múltiplos produtores com blocos `async`" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-13/src/main.rs:here}}
```

</Listing>

Primeiro, clonamos `tx`, criando `tx1` fora do primeiro bloco `async`. Movemos
`tx1` para dentro desse bloco exatamente como fizemos antes com `tx`. Depois,
mais adiante, movemos o `tx` original para um *novo* bloco `async`, onde
enviamos mais mensagens com um atraso ligeiramente maior. Por acaso colocamos
esse novo bloco `async` depois do bloco `async` que recebe as mensagens, mas ele
poderia ficar antes dele sem problemas. O ponto-chave é a ordem em que as
*futures* são aguardadas, não a ordem em que são criadas.

Ambos os blocos `async` para envio de mensagens precisam ser blocos `async move`
para que tanto `tx` quanto `tx1` sofram *drop* quando esses blocos terminarem.
Caso contrário, voltaremos ao mesmo laço infinito em que começamos.

Por fim, trocamos `trpl::join` por `trpl::join!` para lidar com a *future*
adicional: a macro `join!` aguarda um número arbitrário de *futures* quando
sabemos a quantidade de *futures* em tempo de compilação. Discutiremos como
aguardar uma coleção de um número desconhecido de *futures* mais adiante neste
capítulo.

Agora vemos todas as mensagens de ambas as *futures* transmissoras e, como as
*futures* que enviam usam atrasos ligeiramente diferentes após o envio, as
mensagens também são recebidas nesses intervalos diferentes:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
received 'hi'
received 'more'
received 'from'
received 'the'
received 'messages'
received 'future'
received 'for'
received 'you'
```

Exploramos como usar troca de mensagens para enviar dados entre *futures*, como
o código dentro de um bloco `async` executa sequencialmente, como mover
*ownership* para dentro de um bloco `async` e como unir múltiplas *futures*. A
seguir, vamos discutir como e por que informar ao *runtime* que ele pode
alternar para outra tarefa.

[thread-spawn]: ch16-01-threads.html#creating-a-new-thread-with-spawn
[join-handles]: ch16-01-threads.html#waiting-for-all-threads-to-finish
[message-passing-threads]: ch16-02-message-passing.html
[if-let]: ch06-03-if-let.html
[capture-or-move]: ch13-01-closures.html#capturing-references-or-moving-ownership
[move-threads]: ch16-01-threads.html#using-move-closures-with-threads
