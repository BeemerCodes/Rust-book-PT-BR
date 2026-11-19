<!-- Old headings. Do not remove or links may break. -->

<a id="using-message-passing-to-transfer-data-between-threads"></a>

<a id="transfer-data-between-threads-with-message-passing"></a>
## Transferindo Dados Entre *Threads* com Troca de Mensagens

Uma abordagem cada vez mais popular para garantir concorrência segura é a troca
de mensagens (*message passing*), em que *threads* ou atores se comunicam
enviando mensagens contendo dados uns aos outros. Aqui está a ideia em um slogan
da [documentação da linguagem Go](https://golang.org/doc/effective_go.html#concurrency):
"Não se comunique compartilhando memória; em vez disso, compartilhe memória se
comunicando."

Para realizar concorrência por envio de mensagens, a biblioteca padrão do Rust
fornece uma implementação de canais (*channels*). Um *canal* (*channel*) é um
conceito geral de programação pelo qual dados são enviados de uma *thread* para
outra.

Você pode imaginar um canal em programação como um canal direcional de água,
como um riacho ou um rio. Se você colocar algo como um patinho de borracha em um
rio, ele viajará correnteza abaixo até o final do curso d'água.

Um canal tem duas metades: um transmissor (*transmitter*) e um receptor
(*receiver*). A metade transmissora é o ponto rio acima onde você coloca o
patinho de borracha na água, e a metade receptora é onde o patinho de borracha
chega rio abaixo. Uma parte do seu código chama métodos no transmissor com os
dados que você deseja enviar, e outra parte verifica a extremidade receptora em
busca de mensagens que chegam. Diz-se que um canal está *fechado* se a metade
transmissora ou a metade receptora for liberada (*dropped*).

Aqui, vamos construir um programa que tem uma *thread* para gerar valores e
enviá-los por um canal, e outra *thread* que receberá os valores e os imprimirá.
Enviaremos valores simples entre *threads* usando um canal para ilustrar o
recurso. Assim que você estiver familiarizado com a técnica, poderá usar canais
para quaisquer *threads* que precisem se comunicar entre si, como um sistema de
chat ou um sistema em que muitas *threads* realizam partes de um cálculo e
enviam as partes para uma *thread* que agrega os resultados.

Primeiro, na Listagem 16-6, criaremos um canal, mas não faremos nada com ele.
Observe que isso ainda não compilará porque o Rust não consegue saber que tipo
de valores queremos enviar pelo canal.

<Listing number="16-6" file-name="src/main.rs" caption="Criando um canal e atribuindo as duas metades a `tx` e `rx`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-06/src/main.rs}}
```

</Listing>

Criamos um novo canal usando a função `mpsc::channel`; `mpsc` significa
*multiple producer, single consumer* (múltiplos produtores, único consumidor).
Em resumo, a forma como a biblioteca padrão do Rust implementa canais significa
que um canal pode ter múltiplas extremidades de *envio* que produzem valores,
mas apenas uma extremidade de *recebimento* que consome esses valores. Imagine
vários riachos fluindo juntos para um único grande rio: tudo o que for enviado
por qualquer um dos riachos acabará em um único rio no final. Começaremos com um
único produtor por enquanto, mas adicionaremos múltiplos produtores quando
fizermos este exemplo funcionar.

A função `mpsc::channel` retorna uma tupla, cujo primeiro elemento é a
extremidade de envio — o transmissor — e o segundo elemento é a extremidade de
recebimento — o receptor. As abreviações `tx` e `rx` são tradicionalmente usadas
em muitas áreas para *transmitter* e *receiver*, respectivamente, então nomeamos
nossas variáveis assim para indicar cada extremidade. Estamos usando uma
instrução `let` com um padrão que desestrutura a tupla; discutiremos o uso de
padrões em instruções `let` e desestruturação no Capítulo 19. Por enquanto,
saiba que usar uma instrução `let` dessa forma é uma abordagem conveniente para
extrair as partes da tupla retornada por `mpsc::channel`.

Vamos mover a extremidade transmissora para uma *thread* criada e fazê-la enviar
uma *string* para que a *thread* criada se comunique com a *thread* principal,
como mostrado na Listagem 16-7. Isso é como colocar um patinho de borracha no
rio acima ou enviar uma mensagem de chat de uma *thread* para outra.

<Listing number="16-7" file-name="src/main.rs" caption='Movendo `tx` para uma *thread* criada e enviando `"hi"`'>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-07/src/main.rs}}
```

</Listing>

Novamente, estamos usando `thread::spawn` para criar uma nova *thread* e depois
usando `move` para mover `tx` para dentro da *closure*, de modo que a *thread*
criada seja dona de `tx`. A *thread* criada precisa possuir o transmissor para
poder enviar mensagens pelo canal.

O transmissor tem um método `send` que recebe o valor que queremos enviar. O
método `send` retorna um tipo `Result<T, E>`, portanto, se o receptor já tiver
sido liberado e não houver para onde enviar um valor, a operação de envio
retornará um erro. Neste exemplo, estamos chamando `unwrap` para entrar em
*panic* em caso de erro. Mas em uma aplicação real, trataríamos isso
adequadamente: volte ao Capítulo 9 para revisar estratégias de tratamento de
erros adequado.

Na Listagem 16-8, obteremos o valor do receptor na *thread* principal. Isso é
como resgatar o patinho de borracha da água no final do rio ou receber uma
mensagem de chat.

<Listing number="16-8" file-name="src/main.rs" caption='Recebendo o valor `"hi"` na *thread* principal e imprimindo-o'>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-08/src/main.rs}}
```

</Listing>

O receptor tem dois métodos úteis: `recv` e `try_recv`. Estamos usando `recv`,
abreviação de *receive*, que bloqueará a execução da *thread* principal e
aguardará até que um valor seja enviado pelo canal. Assim que um valor for
enviado, `recv` o retornará em um `Result<T, E>`. Quando o transmissor fechar,
`recv` retornará um erro para sinalizar que não virão mais valores.

O método `try_recv` não bloqueia, mas em vez disso retorna um `Result<T, E>`
imediatamente: um valor `Ok` contendo uma mensagem se houver uma disponível e um
valor `Err` se não houver mensagens neste momento. Usar `try_recv` é útil se
esta *thread* tiver outro trabalho a fazer enquanto aguarda mensagens:
poderíamos escrever um laço que chama `try_recv` de tempos em tempos, lida com
uma mensagem se houver uma disponível e, caso contrário, faz outro trabalho por
um tempo antes de verificar novamente.

Usamos `recv` neste exemplo por simplicidade; não temos nenhum outro trabalho
para a *thread* principal fazer além de aguardar mensagens, portanto bloquear a
*thread* principal é apropriado.

Quando executarmos o código da Listagem 16-8, veremos o valor impresso a partir
da *thread* principal:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Got: hi
```

Perfeito!

<!-- Old headings. Do not remove or links may break. -->

<a id="channels-and-ownership-transference"></a>

<a id="transferring-ownership-through-channels"></a>
### Transferindo *Ownership* Através de Canais

As regras de *ownership* desempenham um papel vital no envio de mensagens porque
ajudam você a escrever código concorrente seguro. Prevenir erros na programação
concorrente é a vantagem de pensar em *ownership* em todos os seus programas
Rust. Vamos fazer um experimento para mostrar como canais e *ownership*
trabalham juntos para evitar problemas: tentaremos usar um valor `val` na
*thread* criada *depois* de já tê-lo enviado pelo canal. Tente compilar o código
da Listagem 16-9 para ver por que esse código não é permitido.

<Listing number="16-9" file-name="src/main.rs" caption="Tentando usar `val` depois de já tê-lo enviado pelo canal">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-09/src/main.rs}}
```

</Listing>

Aqui, tentamos imprimir `val` depois de já tê-lo enviado pelo canal via
`tx.send`. Permitir isso seria uma má ideia: uma vez que o valor foi enviado
para outra *thread*, essa *thread* poderia modificá-lo ou liberá-lo antes que
tentássemos usar o valor novamente. Potencialmente, as modificações da outra
*thread* poderiam causar erros ou resultados inesperados devido a dados
inconsistentes ou inexistentes. No entanto, o Rust nos dá um erro se tentarmos
compilar o código da Listagem 16-9:

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-09/output.txt}}
```

Nosso erro de concorrência causou um erro em tempo de compilação. A função
`send` assume a propriedade de seu parâmetro e, quando o valor é movido, o
receptor assume a propriedade dele. Isso nos impede de usar acidentalmente o
valor novamente após enviá-lo; o sistema de *ownership* verifica se está tudo
certo.

<!-- Old headings. Do not remove or links may break. -->

<a id="sending-multiple-values-and-seeing-the-receiver-waiting"></a>

<a id="sending-multiple-values"></a>
### Enviando Múltiplos Valores

O código da Listagem 16-8 compilou e executou, mas não nos mostrou claramente
que duas *threads* separadas estavam conversando entre si pelo canal.

Na Listagem 16-10, fizemos algumas modificações que provarão que o código da
Listagem 16-8 está executando de forma concorrente: a *thread* criada agora
enviará múltiplas mensagens e fará uma pausa de um segundo entre cada mensagem.

<Listing number="16-10" file-name="src/main.rs" caption="Enviando múltiplas mensagens e fazendo pausa entre cada uma">

```rust,noplayground
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-10/src/main.rs}}
```

</Listing>

Desta vez, a *thread* criada tem um vetor de *strings* que queremos enviar para
a *thread* principal. Iteramos sobre elas, enviando cada uma individualmente, e
fazemos uma pausa entre cada uma chamando a função `thread::sleep` com um valor
`Duration` de um segundo.

Na *thread* principal, não estamos mais chamando a função `recv` explicitamente:
em vez disso, estamos tratando `rx` como um iterador. Para cada valor recebido,
nós o imprimimos. Quando o canal for fechado, a iteração terminará.

Ao executar o código da Listagem 16-10, você deverá ver a seguinte saída com uma
pausa de um segundo entre cada linha:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Got: hi
Got: from
Got: the
Got: thread
```

Como não temos nenhum código que faça pausa ou atraso no laço `for` da *thread*
principal, podemos perceber que a *thread* principal está esperando para receber
valores da *thread* criada.

<!-- Old headings. Do not remove or links may break. -->

<a id="creating-multiple-producers-by-cloning-the-transmitter"></a>

<a id="creating-multiple-producers"></a>
### Criando Múltiplos Produtores

Anteriormente mencionamos que `mpsc` era um acrônimo para *multiple producer,
single consumer*. Vamos colocar `mpsc` em prática e expandir o código da
Listagem 16-10 para criar múltiplas *threads* que enviam valores para o mesmo
receptor. Podemos fazer isso clonando o transmissor, como mostrado na Listagem
16-11.

<Listing number="16-11" file-name="src/main.rs" caption="Enviando múltiplas mensagens de múltiplos produtores">

```rust,noplayground
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-11/src/main.rs:here}}
```

</Listing>

Desta vez, antes de criarmos a primeira *thread*, chamamos `clone` no
transmissor. Isso nos dará um novo transmissor que podemos passar para a
primeira *thread* criada. Passamos o transmissor original para uma segunda
*thread* criada. Isso nos dá duas *threads*, cada uma enviando mensagens
diferentes para o único receptor.

Quando você executar o código, sua saída deverá ficar parecida com esta:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Got: hi
Got: more
Got: from
Got: messages
Got: for
Got: the
Got: thread
Got: you
```

Você pode ver os valores em outra ordem, dependendo do seu sistema. É isso que
torna a concorrência interessante, além de difícil. Se você experimentar com
`thread::sleep`, dando a ele valores variados nas diferentes *threads*, cada
execução será mais não determinística e criará uma saída diferente a cada vez.

Agora que vimos como os canais funcionam, vamos ver um método diferente de
concorrência.
