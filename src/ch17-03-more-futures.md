
<!-- Old headings. Do not remove or links may break. -->

<a id="yielding"></a>
<a id="yielding-control-to-the-runtime"></a>
<a id="working-with-any-number-of-futures"></a>
### Cedendo o Controle ao Runtime

Lembre-se da seção [“Nosso Primeiro Programa Assíncrono”][async-program]<!-- ignore -->
de que, em cada ponto de `await`, o Rust dá ao *runtime* a chance de pausar a
tarefa e alternar para outra caso a *future* aguardada não esteja pronta. O
inverso também é verdadeiro: o Rust *só* pausa blocos `async` e devolve o
controle a um *runtime* em um ponto de `await`. Tudo o que está entre pontos de
`await` é síncrono.

Isso significa que, se você fizer um monte de trabalho em um bloco `async` sem
um ponto de `await`, essa *future* impedirá quaisquer outras *futures* de
avançarem. Às vezes você pode ouvir isso ser chamado de uma *future* causando
*starvation* (inanição) em outras *futures*. Em alguns casos, isso pode não ser
um grande problema. No entanto, se você estiver fazendo algum tipo de
preparação custosa ou trabalho demorado, ou se tiver uma *future* que continuará
realizando alguma tarefa específica indefinidamente, precisará pensar em quando
e onde devolver o controle ao *runtime*.

Vamos simular uma operação demorada para ilustrar o problema de *starvation* e,
em seguida, explorar como resolvê-lo. A Listagem 17-14 apresenta uma função
`slow`.

<Listing number="17-14" caption="Usando `thread::sleep` para simular operações lentas" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-14/src/main.rs:slow}}
```

</Listing>

Este código usa `std::thread::sleep` em vez de `trpl::sleep`, de modo que chamar
`slow` bloqueará a *thread* atual por uma certa quantidade de milissegundos.
Podemos usar `slow` para representar operações do mundo real que são ao mesmo
tempo demoradas e bloqueantes.

Na Listagem 17-15, usamos `slow` para emular esse tipo de trabalho limitado por
CPU em um par de *futures*.

<Listing number="17-15" caption="Chamando a função `slow` para simular operações lentas" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-15/src/main.rs:slow-futures}}
```

</Listing>

Cada *future* devolve o controle ao *runtime* somente *depois* de realizar uma
série de operações lentas. Se você executar este código, verá esta saída:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-15/
cargo run
copy just the output
-->

```text
'a' started.
'a' ran for 30ms
'a' ran for 10ms
'a' ran for 20ms
'b' started.
'b' ran for 75ms
'b' ran for 10ms
'b' ran for 15ms
'b' ran for 350ms
'a' finished.
```

Assim como na Listagem 17-5, onde usamos `trpl::select` para colocar em disputa
*futures* que buscavam duas URLs, `select` ainda termina assim que `a` conclui.
Não há intercalação entre as chamadas a `slow` nas duas *futures*, porém. A
*future* `a` faz todo o seu trabalho até que a chamada a `trpl::sleep` seja
aguardada, depois a *future* `b` faz todo o seu trabalho até que sua própria
chamada a `trpl::sleep` seja aguardada e, por fim, a *future* `a` termina. Para
permitir que ambas as *futures* avancem entre suas tarefas lentas, precisamos de
pontos de `await` para podermos devolver o controle ao *runtime*. Isso significa
que precisamos de algo que possamos aguardar!

Já podemos ver esse tipo de passagem de controle acontecendo na Listagem 17-15:
se removêssemos o `trpl::sleep` no final da *future* `a`, ela terminaria sem que
a *future* `b` executasse *nem uma vez*. Vamos tentar usar a função
`trpl::sleep` como ponto de partida para permitir que as operações se alternem
ao avançar, como mostrado na Listagem 17-16.

<Listing number="17-16" caption="Usando `trpl::sleep` para permitir que as operações se alternem ao avançar" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-16/src/main.rs:here}}
```

</Listing>

Adicionamos chamadas a `trpl::sleep` com pontos de `await` entre cada chamada a
`slow`. Agora o trabalho das duas *futures* é intercalado:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-16
cargo run
copy just the output
-->

```text
'a' started.
'a' ran for 30ms
'b' started.
'b' ran for 75ms
'a' ran for 10ms
'b' ran for 10ms
'a' ran for 20ms
'b' ran for 15ms
'a' finished.
```

A *future* `a` ainda executa um pouco antes de passar o controle para `b`,
porque ela chama `slow` antes de chamar `trpl::sleep` pela primeira vez, mas
depois disso as *futures* alternam entre si cada vez que uma delas atinge um
ponto de `await`. Neste caso, fizemos isso após cada chamada a `slow`, mas
poderíamos dividir o trabalho da maneira que fizesse mais sentido para nós.

Na verdade, não queremos *dormir* aqui: queremos avançar o mais rápido possível.
Só precisamos devolver o controle ao *runtime*. Podemos fazer isso diretamente
usando a função `trpl::yield_now`. Na Listagem 17-17, substituímos todas aquelas
chamadas a `trpl::sleep` por `trpl::yield_now`.

<Listing number="17-17" caption="Usando `yield_now` para permitir que as operações se alternem ao avançar" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-17/src/main.rs:yields}}
```

</Listing>

Este código é mais claro quanto à intenção real e pode ser significativamente
mais rápido do que usar `sleep`, porque temporizadores como o usado por `sleep`
muitas vezes têm limites em quão granulares podem ser. A versão de `sleep` que
estamos usando, por exemplo, sempre dormirá por pelo menos um milissegundo,
mesmo se passarmos a ela uma `Duration` de um nanossegundo. Mais uma vez,
computadores modernos são *rápidos*: eles conseguem fazer muita coisa em um
milissegundo!

Isso significa que `async` pode ser útil mesmo para tarefas limitadas por
computação (*compute-bound*), dependendo do que mais o seu programa estiver
fazendo, porque oferece uma ferramenta útil para estruturar as relações entre
diferentes partes do programa (ao custo da sobrecarga da máquina de estados
assíncrona). Essa é uma forma de *multitarefa cooperativa* (*cooperative
multitasking*), na qual cada *future* tem o poder de determinar quando entrega o
controle por meio de pontos de `await`. Cada *future*, portanto, também tem a
responsabilidade de evitar bloquear por tempo demais. Em alguns sistemas
operacionais embarcados baseados em Rust, esse é o *único* tipo de multitarefa!

Em código do mundo real, você normalmente não ficará alternando chamadas de
função com pontos de `await` em toda linha, é claro. Embora ceder o controle
dessa forma seja relativamente barato, não é gratuito. Em muitos casos, tentar
dividir uma tarefa limitada por computação pode torná-la significativamente mais
lenta, então às vezes é melhor para o desempenho *geral* deixar uma operação
bloquear brevemente. Sempre meça para ver quais são os gargalos reais de
desempenho do seu código. A dinâmica subjacente é importante de se ter em mente,
porém, se você *estiver* vendo muito trabalho acontecendo em série quando
esperava que acontecesse de forma concorrente!

<a id="building-our-own-async-abstractions"></a>
### Construindo Nossas Próprias Abstrações Assíncronas

Também podemos compor *futures* entre si para criar novos padrões. Por exemplo,
podemos construir uma função `timeout` com os blocos de construção assíncronos
que já temos. Quando terminarmos, o resultado será outro bloco de construção que
poderemos usar para criar ainda mais abstrações assíncronas.

A Listagem 17-18 mostra como esperaríamos que esse `timeout` funcionasse com
uma *future* lenta.

<Listing number="17-18" caption="Usando nossa função `timeout` imaginada para executar uma operação lenta com um limite de tempo" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-18/src/main.rs:here}}
```

</Listing>

Vamos implementar isso! Para começar, vamos pensar na API de `timeout`:

- Ela própria precisa ser uma função `async` para que possamos aguardá-la.
- Seu primeiro parâmetro deve ser uma *future* a ser executada. Podemos torná-lo
  genérico para permitir que funcione com qualquer *future*.
- Seu segundo parâmetro será o tempo máximo de espera. Se usarmos uma
  `Duration`, será fácil repassá-la para `trpl::sleep`.
- Ela deve retornar um `Result`. Se a *future* concluir com sucesso, o `Result`
  será `Ok` com o valor produzido pela *future*. Se o tempo limite esgotar
  primeiro, o `Result` será `Err` com a duração pela qual o *timeout* esperou.

A Listagem 17-19 mostra essa declaração.

<!-- This is not tested because it intentionally does not compile. -->

<Listing number="17-19" caption="Definindo a assinatura de `timeout`" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-19/src/main.rs:declaration}}
```

</Listing>

Isso atende aos nossos objetivos para os tipos. Agora vamos pensar no
*comportamento* de que precisamos: queremos colocar a *future* passada em
disputa contra a duração. Podemos usar `trpl::sleep` para criar uma *future* de
temporizador a partir da duração e usar `trpl::select` para executar esse
temporizador junto com a *future* que o chamador passar.

Na Listagem 17-20, implementamos `timeout` fazendo `match` no resultado de
aguardar `trpl::select`.

<Listing number="17-20" caption="Definindo `timeout` com `select` e `sleep`" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-20/src/main.rs:implementation}}
```

</Listing>

A implementação de `trpl::select` não é justa: ela sempre faz *polling* dos
argumentos na ordem em que são passados (outras implementações de `select`
escolhem aleatoriamente qual argumento consultar primeiro). Assim, passamos
`future_to_try` primeiro para `select` para que ela tenha a chance de concluir
mesmo se `max_time` for uma duração muito curta. Se `future_to_try` terminar
primeiro, `select` retornará `Left` com a saída de `future_to_try`. Se `timer`
terminar primeiro, `select` retornará `Right` com a saída do temporizador, que
é `()`.

Se `future_to_try` tiver sucesso e obtivermos um `Left(output)`, retornamos
`Ok(output)`. Se, em vez disso, o temporizador de espera esgotar e obtivermos
um `Right(())`, ignoramos o `()` com `_` e retornamos `Err(max_time)`.

Com isso, temos uma função `timeout` funcional construída a partir de outros
dois auxiliares assíncronos. Se executarmos nosso código, ele imprimirá o modo
de falha após o tempo limite:

```text
Failed after 2 seconds
```

Como *futures* se compõem com outras *futures*, você pode construir ferramentas
realmente poderosas usando blocos de construção assíncronos menores. Por
exemplo, você pode usar essa mesma abordagem para combinar *timeouts* com novas
tentativas (*retries*) e, por sua vez, usá-los com operações como chamadas de
rede (como as da Listagem 17-5).

Na prática, você normalmente trabalhará diretamente com `async` e `await` e,
secundariamente, com funções como `select` e macros como a macro `join!` para
controlar como as *futures* mais externas são executadas.

Já vimos várias maneiras de trabalhar com múltiplas *futures* ao mesmo tempo. A
seguir, veremos como podemos trabalhar com múltiplas *futures* em sequência ao
longo do tempo usando *streams*.

[async-program]: ch17-01-futures-and-syntax.html#our-first-async-program
