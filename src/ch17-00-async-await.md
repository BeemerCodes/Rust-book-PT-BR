<a id="fundamentals-of-asynchronous-programming-async-await-futures-and-streams"></a>
# Fundamentos de Programação Assíncrona: Async, Await, Futures e Streams

Muitas operações que pedimos para o computador realizar podem demorar um pouco
para terminar. Seria ótimo se pudéssemos fazer outra coisa enquanto aguardamos a
conclusão desses processos demorados. Computadores modernos oferecem duas
técnicas para trabalhar em mais de uma operação ao mesmo tempo: paralelismo e
concorrência. A lógica dos nossos programas, no entanto, é escrita em sua maior
parte de forma linear. Gostaríamos de poder especificar as operações que um
programa deve executar e os pontos em que uma função pode pausar para que outra
parte do programa rode em seu lugar, sem precisar definir de antemão exatamente
a ordem e a maneira como cada trecho de código deve executar. A *programação
assíncrona* é uma abstração que nos permite expressar nosso código em termos de
pontos de pausa potenciais e resultados eventuais, cuidando dos detalhes de
coordenação para nós.

Este capítulo parte do uso de *threads* para paralelismo e concorrência visto no
Capítulo 16 e apresenta uma abordagem alternativa para escrever código: as
*futures*, as *streams* e a sintaxe `async` e `await` do Rust, que nos permitem
expressar como as operações podem ser assíncronas, além dos *crates* de
terceiros que implementam *runtimes* assíncronos — código que gerencia e
coordena a execução de operações assíncronas.

Vamos considerar um exemplo. Suponha que você esteja exportando um vídeo que
criou de uma festa em família, uma operação que pode levar de minutos a horas.
A exportação de vídeo usará o máximo de poder de CPU e GPU possível. Se você
tivesse apenas um núcleo de CPU e seu sistema operacional não pausasse essa
exportação até ela terminar — isto é, se ele executasse a exportação de forma
*síncrona* — você não conseguiria fazer mais nada no computador enquanto essa
tarefa estivesse rodando. Essa seria uma experiência bem frustrante. Felizmente,
o sistema operacional do seu computador pode interromper, e interrompe, a
exportação de forma invisível com frequência suficiente para permitir que você
faça outras coisas simultaneamente.

Agora suponha que você esteja baixando um vídeo compartilhado por outra pessoa,
o que também pode levar algum tempo, mas não consome tanto tempo de CPU. Nesse
caso, a CPU precisa esperar os dados chegarem pela rede. Embora você possa
começar a ler os dados assim que eles começam a chegar, pode levar algum tempo
até que tudo apareça. Mesmo depois que todos os dados estiverem presentes, se o
vídeo for bem grande, pode levar pelo menos um ou dois segundos para carregar
tudo. Isso pode não parecer muito, mas é uma eternidade para um processador
moderno, que consegue realizar bilhões de operações por segundo. Mais uma vez,
seu sistema operacional interromperá seu programa de forma invisível para
permitir que a CPU realize outro trabalho enquanto aguarda a conclusão da
chamada de rede.

A exportação de vídeo é um exemplo de operação limitada por CPU (*CPU-bound*) ou
limitada por computação (*compute-bound*). Ela é limitada pela velocidade
potencial de processamento de dados do computador dentro da CPU ou da GPU, e por
quanto dessa velocidade ele pode dedicar à operação. O download do vídeo é um
exemplo de operação limitada por E/S (*I/O-bound*), porque é limitada pela
velocidade de *entrada e saída* (*input and output*) do computador; ela só pode
ir tão rápido quanto os dados conseguem ser enviados pela rede.

Em ambos os exemplos, as interrupções invisíveis do sistema operacional
fornecem uma forma de concorrência. Essa concorrência acontece apenas no nível
do programa inteiro, porém: o sistema operacional interrompe um programa para
deixar outros programas trabalharem. Em muitos casos, como entendemos nossos
programas em um nível muito mais granular do que o sistema operacional, podemos
identificar oportunidades de concorrência que o sistema operacional não
consegue enxergar.

Por exemplo, se estivermos construindo uma ferramenta para gerenciar downloads
de arquivos, devemos conseguir escrever nosso programa de modo que iniciar um
download não trave a interface do usuário, e os usuários possam iniciar vários
downloads ao mesmo tempo. Muitas APIs de sistema operacional para interagir com
a rede são *bloqueantes* (*blocking*), no entanto; ou seja, elas bloqueiam o
progresso do programa até que os dados que estão processando estejam
completamente prontos.

> Note: Pensando bem, é assim que a *maioria* das chamadas de função funciona. No
> entanto, o termo *bloqueante* geralmente é reservado para chamadas de função
> que interagem com arquivos, a rede ou outros recursos do computador, porque
> esses são os casos em que um programa individual se beneficiaria se a operação
> fosse *não* bloqueante.

Poderíamos evitar bloquear nossa *thread* principal criando uma *thread* dedicada
para baixar cada arquivo. Contudo, a sobrecarga dos recursos de sistema usados
por essas *threads* acabaria se tornando um problema. Seria preferível se a
chamada nem bloqueasse para começo de conversa e, em vez disso, pudéssemos
definir várias tarefas que gostaríamos que nosso programa concluísse e permitir
que o *runtime* escolhesse a melhor ordem e maneira de executá-las.

Isso é exatamente o que a abstração *async* (abreviação de *asynchronous*,
assíncrono) do Rust nos oferece. Neste capítulo, você aprenderá tudo sobre
`async` à medida que cobrimos os seguintes tópicos:

- Como usar a sintaxe `async` e `await` do Rust e executar funções assíncronas
  com um *runtime*
- Como usar o modelo assíncrono para resolver alguns dos mesmos desafios que
  vimos no Capítulo 16
- Como *multithreading* e `async` fornecem soluções complementares que você
  pode combinar em muitos casos

Antes de vermos como `async` funciona na prática, porém, precisamos fazer um
breve desvio para discutir as diferenças entre paralelismo e concorrência.

<a id="parallelism-and-concurrency"></a>
## Paralelismo e Concorrência

Até aqui, tratamos paralelismo e concorrência de maneira praticamente
intercambiável. Agora precisamos distingui-los com mais precisão, porque as
diferenças ficarão evidentes à medida que começarmos a trabalhar.

Considere as diferentes maneiras pelas quais uma equipe pode dividir o trabalho
em um projeto de software. Você pode atribuir várias tarefas a um único membro,
atribuir uma tarefa a cada membro ou usar uma mistura das duas abordagens.

Quando um indivíduo trabalha em várias tarefas diferentes antes que qualquer
uma delas seja concluída, isso é *concorrência*. Uma forma de implementar a
concorrência é semelhante a ter dois projetos diferentes abertos no seu
computador e, quando você fica entediado ou travado em um projeto, alterna para
o outro. Você é apenas uma pessoa, portanto não consegue avançar em ambas as
tarefas exatamente ao mesmo tempo, mas pode fazer multitarefa, avançando em uma
de cada vez ao alternar entre elas (veja a Figura 17-1).

<figure>

<img src="img/trpl17-01.svg" class="center" alt="Um diagrama com caixas empilhadas rotuladas como Task A e Task B, com losangos dentro delas representando subtarefas. Setas apontam de A1 para B1, B1 para A2, A2 para B2, B2 para A3, A3 para A4 e A4 para B3. As setas entre as subtarefas cruzam entre as caixas de Task A e Task B." />

<figcaption>Figura 17-1: Um fluxo de trabalho concorrente, alternando entre a Tarefa A e a Tarefa B</figcaption>

</figure>

Quando a equipe divide um grupo de tarefas fazendo com que cada membro assuma
uma tarefa e trabalhe nela sozinho, isso é *paralelismo*. Cada pessoa da equipe
consegue avançar exatamente ao mesmo tempo (veja a Figura 17-2).

<figure>

<img src="img/trpl17-02.svg" class="center" alt="Um diagrama com caixas empilhadas rotuladas como Task A e Task B, com losangos dentro delas representando subtarefas. Setas apontam de A1 para A2, A2 para A3, A3 para A4, B1 para B2 e B2 para B3. Nenhuma seta cruza entre as caixas de Task A e Task B." />

<figcaption>Figura 17-2: Um fluxo de trabalho paralelo, onde o trabalho acontece na Tarefa A e na Tarefa B de forma independente</figcaption>

</figure>

Em ambos os fluxos de trabalho, você pode ter que coordenar entre tarefas
diferentes. Talvez você achasse que a tarefa atribuída a uma pessoa era
totalmente independente do trabalho de todo mundo, mas na verdade ela exige que
outra pessoa da equipe termine a tarefa dela primeiro. Parte do trabalho podia
ser feita em paralelo, mas parte dele era na verdade *serial*: só podia
acontecer em série, uma tarefa após a outra, como na Figura 17-3.

<figure>

<img src="img/trpl17-03.svg" class="center" alt="Um diagrama com caixas empilhadas rotuladas como Task A e Task B, com losangos dentro delas representando subtarefas. Em Task A, setas apontam de A1 para A2, de A2 para um par de linhas verticais espessas parecido com um símbolo de “pausa”, e desse símbolo para A3. Em Task B, setas apontam de B1 para B2, de B2 para B3, de B3 para A3 e de B3 para B4." />

<figcaption>Figura 17-3: Um fluxo de trabalho parcialmente paralelo, onde o trabalho acontece na Tarefa A e na Tarefa B de forma independente até que a Tarefa A3 fica bloqueada esperando os resultados da Tarefa B3.</figcaption>

</figure>

Da mesma forma, você pode perceber que uma das suas próprias tarefas depende de
outra tarefa sua. Agora o seu trabalho concorrente também se tornou serial.

Paralelismo e concorrência também podem se cruzar. Se você descobrir que um
colega está travado até que você termine uma das suas tarefas, provavelmente vai
concentrar todos os seus esforços nessa tarefa para “desbloquear” seu colega.
Você e seu colega não conseguem mais trabalhar em paralelo, e você também não
consegue mais trabalhar de forma concorrente nas suas próprias tarefas.

As mesmas dinâmicas básicas entram em cena com software e hardware. Em uma
máquina com um único núcleo de CPU, a CPU consegue executar apenas uma operação
por vez, mas ainda assim pode trabalhar de forma concorrente. Usando ferramentas
como *threads*, processos e `async`, o computador pode pausar uma atividade e
alternar para outras antes de finalmente voltar para aquela primeira atividade.
Em uma máquina com vários núcleos de CPU, ela também pode fazer trabalho em
paralelo. Um núcleo pode estar executando uma tarefa enquanto outro núcleo
executa uma tarefa completamente diferente, e essas operações realmente
acontecem ao mesmo tempo.

Executar código `async` em Rust geralmente acontece de forma concorrente.
Dependendo do hardware, do sistema operacional e do *runtime* assíncrono que
estivermos usando (falaremos mais sobre *runtimes* assíncronos em breve), essa
concorrência também pode usar paralelismo por baixo dos panos.

Agora, vamos mergulhar em como a programação assíncrona em Rust realmente
funciona.
