<a id="using-threads-to-run-code-simultaneously"></a>
## Usando *Threads* para Executar Código Simultaneamente

Na maioria dos sistemas operacionais atuais, o código de um programa em execução
roda em um *processo*, e o sistema operacional gerencia múltiplos processos ao
mesmo tempo. Dentro de um programa, você também pode ter partes independentes
que executam simultaneamente. Os recursos que executam essas partes
independentes são chamados de *threads*. Por exemplo, um servidor web pode ter
múltiplas *threads* para responder a mais de uma requisição ao mesmo tempo.

Dividir a computação no seu programa em múltiplas *threads* para executar
múltiplas tarefas ao mesmo tempo pode melhorar o desempenho, mas também adiciona
complexidade. Como as *threads* podem executar simultaneamente, não há garantia
inerente sobre a ordem em que partes do seu código em *threads* diferentes serão
executadas. Isso pode levar a problemas, como:

- Condições de corrida (*race conditions*), em que *threads* acessam dados ou
  recursos em uma ordem inconsistente
- *Deadlocks*, em que duas *threads* ficam esperando uma pela outra, impedindo
  que ambas continuem
- Bugs que só acontecem em certas situações e são difíceis de reproduzir e
  corrigir de forma confiável

O Rust tenta mitigar os efeitos negativos de usar *threads*, mas programar em um
contexto *multithreaded* ainda exige cuidado e requer uma estrutura de código
diferente daquela usada em programas que executam em uma única *thread*.

Linguagens de programação implementam *threads* de algumas maneiras diferentes,
e muitos sistemas operacionais fornecem uma API que a linguagem de programação
pode chamar para criar novas *threads*. A biblioteca padrão do Rust usa um
modelo de implementação de *threads* *1:1*, no qual um programa usa uma *thread*
do sistema operacional para cada *thread* da linguagem. Existem *crates* que
implementam outros modelos de *threading* que fazem contrapartidas diferentes em
relação ao modelo 1:1. (O sistema assíncrono do Rust, que veremos no próximo
capítulo, também fornece outra abordagem para concorrência.)

<a id="creating-a-new-thread-with-spawn"></a>
### Criando uma Nova *Thread* com `spawn`

Para criar uma nova *thread*, chamamos a função `thread::spawn` e passamos a ela
uma *closure* (falamos sobre *closures* no Capítulo 13) contendo o código que
queremos executar na nova *thread*. O exemplo na Listagem 16-1 imprime um texto
a partir da *thread* principal e outro texto a partir de uma nova *thread*.

<Listing number="16-1" file-name="src/main.rs" caption="Criando uma nova *thread* para imprimir uma coisa enquanto a *thread* principal imprime outra">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-01/src/main.rs}}
```

</Listing>

Observe que, quando a *thread* principal de um programa Rust termina, todas as
*threads* criadas (*spawned*) são encerradas, tenham elas terminado de executar
ou não. A saída deste programa pode ser um pouco diferente a cada execução, mas
será parecida com a seguinte:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
```

As chamadas a `thread::sleep` forçam uma *thread* a interromper sua execução por
um curto período, permitindo que uma *thread* diferente seja executada. As
*threads* provavelmente se alternarão, mas isso não é garantido: depende de como
o seu sistema operacional escalona as *threads*. Nesta execução, a *thread*
principal imprimiu primeiro, mesmo que a instrução de impressão da *thread*
criada apareça primeiro no código. E, embora tenhamos dito à *thread* criada
para imprimir até que `i` fosse `9`, ela só chegou até `5` antes que a *thread*
principal fosse encerrada.

Se você executar esse código e vir apenas a saída da *thread* principal, ou não
vir nenhuma intercalação, tente aumentar os números nos intervalos para criar
mais oportunidades para o sistema operacional alternar entre as *threads*.

<!-- Old headings. Do not remove or links may break. -->

<a id="waiting-for-all-threads-to-finish-using-join-handles"></a>

<a id="waiting-for-all-threads-to-finish"></a>
### Aguardando Todas as *Threads* Terminarem

O código da Listagem 16-1 não só interrompe a *thread* criada prematuramente na
maioria das vezes devido ao encerramento da *thread* principal, mas, como não há
garantia sobre a ordem em que as *threads* executam, também não podemos garantir
que a *thread* criada chegará a executar!

Podemos corrigir o problema de a *thread* criada não executar ou terminar
prematuramente salvando o valor de retorno de `thread::spawn` em uma variável. O
tipo de retorno de `thread::spawn` é `JoinHandle<T>`. Um `JoinHandle<T>` é um
valor possuído que, quando chamamos o método `join` nele, aguardará sua *thread*
terminar. A Listagem 16-2 mostra como usar o `JoinHandle<T>` da *thread* que
criamos na Listagem 16-1 e como chamar `join` para garantir que a *thread*
criada termine antes de `main` sair.

<Listing number="16-2" file-name="src/main.rs" caption="Salvando um `JoinHandle<T>` de `thread::spawn` para garantir que a *thread* execute até o fim">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-02/src/main.rs}}
```

</Listing>

Chamar `join` no *handle* bloqueia a *thread* atualmente em execução até que a
*thread* representada pelo *handle* termine. *Bloquear* uma *thread* significa
que essa *thread* fica impedida de realizar trabalho ou sair. Como colocamos a
chamada a `join` após o laço `for` da *thread* principal, executar a Listagem
16-2 deve produzir uma saída semelhante a esta:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 1 from the spawned thread!
hi number 3 from the main thread!
hi number 2 from the spawned thread!
hi number 4 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
```

As duas *threads* continuam se alternando, mas a *thread* principal espera por
causa da chamada a `handle.join()` e não termina até que a *thread* criada tenha
finalizado.

Mas vamos ver o que acontece quando, em vez disso, movemos `handle.join()` para
antes do laço `for` em `main`, assim:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/no-listing-01-join-too-early/src/main.rs}}
```

</Listing>

A *thread* principal aguardará a *thread* criada terminar e só então executará
seu laço `for`, de modo que a saída não ficará mais intercalada, como mostrado
aqui:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
hi number 1 from the spawned thread!
hi number 2 from the spawned thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 3 from the main thread!
hi number 4 from the main thread!
```

Pequenos detalhes, como onde `join` é chamado, podem afetar se suas *threads*
executam ao mesmo tempo ou não.

<a id="using-move-closures-with-threads"></a>
### Usando *Closures* `move` com *Threads*

Frequentemente usaremos a palavra-chave `move` com *closures* passadas para
`thread::spawn` porque a *closure* então assumirá a propriedade (*ownership*)
dos valores que usa do ambiente, transferindo assim a propriedade desses valores
de uma *thread* para outra. Em ["Capturando Referências ou Movendo
*Ownership*"][capture]<!-- ignore --> no Capítulo 13, discutimos `move` no
contexto de *closures*. Agora vamos nos concentrar mais na interação entre
`move` e `thread::spawn`.

Observe na Listagem 16-1 que a *closure* que passamos para `thread::spawn` não
recebe argumentos: não estamos usando nenhum dado da *thread* principal no
código da *thread* criada. Para usar dados da *thread* principal na *thread*
criada, a *closure* da *thread* criada precisa capturar os valores de que
precisa. A Listagem 16-3 mostra uma tentativa de criar um vetor na *thread*
principal e usá-lo na *thread* criada. No entanto, isso ainda não funcionará,
como você verá em breve.

<Listing number="16-3" file-name="src/main.rs" caption="Tentando usar um vetor criado pela *thread* principal em outra *thread*">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-03/src/main.rs}}
```

</Listing>

A *closure* usa `v`, então ela capturará `v` e o tornará parte do ambiente da
*closure*. Como `thread::spawn` executa essa *closure* em uma nova *thread*,
deveríamos conseguir acessar `v` dentro dessa nova *thread*. Mas quando
compilamos este exemplo, obtemos o seguinte erro:

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-03/output.txt}}
```

O Rust *infere* como capturar `v` e, como `println!` precisa apenas de uma
referência para `v`, a *closure* tenta fazer *borrowing* de `v`. No entanto, há
um problema: o Rust não consegue dizer por quanto tempo a *thread* criada será
executada, portanto não sabe se a referência a `v` será sempre válida.

A Listagem 16-4 apresenta um cenário com maior probabilidade de ter uma
referência para `v` que não será válida.

<Listing number="16-4" file-name="src/main.rs" caption="Uma *thread* com uma *closure* que tenta capturar uma referência para `v` de uma *thread* principal que libera `v`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-04/src/main.rs}}
```

</Listing>

Se o Rust nos permitisse executar esse código, haveria a possibilidade de a
*thread* criada ser colocada imediatamente em segundo plano sem executar nada. A
*thread* criada tem uma referência a `v` dentro dela, mas a *thread* principal
libera `v` imediatamente, usando a função `drop` que discutimos no Capítulo 15.
Então, quando a *thread* criada começar a executar, `v` não será mais válido,
portanto uma referência a ele também será inválida. Oh não!

Para corrigir o erro de compilação na Listagem 16-3, podemos usar o conselho da
mensagem de erro:

<!-- manual-regeneration
after automatic regeneration, look at listings/ch16-fearless-concurrency/listing-16-03/output.txt and copy the relevant part
-->

```text
help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
  |
6 |     let handle = thread::spawn(move || {
  |                                ++++
```

Ao adicionar a palavra-chave `move` antes da *closure*, forçamos a *closure* a
assumir a propriedade dos valores que está usando, em vez de permitir que o Rust
infira que ela deve fazer *borrowing* dos valores. A modificação na Listagem
16-3 mostrada na Listagem 16-5 compilará e executará como pretendemos.

<Listing number="16-5" file-name="src/main.rs" caption="Usando a palavra-chave `move` para forçar uma *closure* a assumir a propriedade dos valores que ela usa">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-05/src/main.rs}}
```

</Listing>

Poderíamos ficar tentados a tentar a mesma coisa para corrigir o código da
Listagem 16-4, em que a *thread* principal chamou `drop`, usando uma *closure*
`move`. No entanto, essa correção não funcionará porque o que a Listagem 16-4
está tentando fazer não é permitido por um motivo diferente. Se adicionássemos
`move` à *closure*, moveríamos `v` para o ambiente da *closure* e não
poderíamos mais chamar `drop` nele na *thread* principal. Em vez disso,
obteríamos este erro de compilação:

```console
{{#include ../listings/ch16-fearless-concurrency/output-only-01-move-drop/output.txt}}
```

As regras de *ownership* do Rust nos salvaram novamente! Recebemos um erro do
código na Listagem 16-3 porque o Rust estava sendo conservador e apenas fazendo
*borrowing* de `v` para a *thread*, o que significava que a *thread* principal
poderia teoricamente invalidar a referência da *thread* criada. Ao dizer ao Rust
para mover a propriedade de `v` para a *thread* criada, estamos garantindo ao
Rust que a *thread* principal não usará mais `v`. Se alterarmos a Listagem 16-4
da mesma forma, estaremos violando as regras de *ownership* ao tentar usar `v`
na *thread* principal. A palavra-chave `move` sobrescreve o padrão conservador
de *borrowing* do Rust; ela não nos permite violar as regras de *ownership*.

Agora que abordamos o que são *threads* e os métodos fornecidos pela API de
*threads*, vamos ver algumas situações em que podemos usar *threads*.

[capture]: ch13-01-closures.html#capturing-references-or-moving-ownership
