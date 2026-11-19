<a id="graceful-shutdown-and-cleanup"></a>
## Desligamento Gracioso e Limpeza

O código da Listagem 21-20 está respondendo a requisições assincronamente por
meio de um *thread pool*, como pretendíamos. Recebemos alguns avisos sobre os
campos `workers`, `id` e `thread` que não estamos usando diretamente, o que nos
lembra de que não estamos fazendo nenhuma limpeza. Quando usamos o método menos
elegante de pressionar <kbd>ctrl</kbd>-<kbd>C</kbd> para interromper a *thread*
principal, todas as outras *threads* também são encerradas imediatamente, mesmo
que estejam no meio do atendimento de uma requisição.

Em seguida, portanto, implementaremos a *trait* `Drop` para chamar `join` em
cada uma das *threads* do pool, para que elas possam concluir as requisições em
que estão trabalhando antes de fechar. Depois, implementaremos uma forma de
dizer às *threads* que elas devem parar de aceitar novas requisições e desligar.
Para ver esse código em ação, modificaremos nosso servidor para aceitar apenas
duas requisições antes de desligar graciosamente seu *thread pool*.

Uma coisa a se notar à medida que avançamos: nada disso afeta as partes do
código que lidam com a execução das *closures*, portanto tudo aqui seria igual
se estivéssemos usando um *thread pool* para um runtime assíncrono.

<a id="implementing-the-drop-trait-on-threadpool"></a>
### Implementando a Trait `Drop` em `ThreadPool`

Vamos começar implementando `Drop` no nosso *thread pool*. Quando o pool for
descartado (*dropped*), todas as nossas *threads* devem fazer `join` para
garantir que terminem seu trabalho. A Listagem 21-22 mostra uma primeira
tentativa de implementação de `Drop`; este código ainda não funcionará.

<Listing number="21-22" file-name="src/lib.rs" caption="Fazendo `join` em cada *thread* quando o *thread pool* sai de escopo">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch21-web-server/listing-21-22/src/lib.rs:here}}
```

</Listing>

Primeiro, percorremos cada um dos `workers` do *thread pool* em um loop. Usamos
`&mut` para isso porque `self` é uma referência mutável e também precisamos
poder mutar `worker`. Para cada `worker`, imprimimos uma mensagem dizendo que
essa instância específica de `Worker` está sendo desligada e, em seguida,
chamamos `join` na *thread* dessa instância de `Worker`. Se a chamada a `join`
falhar, usamos `unwrap` para fazer o Rust entrar em pânico e realizar um
desligamento não gracioso.

Aqui está o erro que obtemos ao compilar este código:

```console
{{#include ../listings/ch21-web-server/listing-21-22/output.txt}}
```

O erro nos diz que não podemos chamar `join` porque temos apenas um empréstimo
(*borrow*) mutável de cada `worker`, e `join` toma *ownership* de seu
argumento. Para resolver esse problema, precisamos mover a *thread* para fora da
instância de `Worker` que detém o *ownership* de `thread`, de modo que `join`
possa consumir a *thread*. Uma maneira de fazer isso é adotar a mesma abordagem
que usamos na Listagem 18-15. Se `Worker` guardasse um
`Option<thread::JoinHandle<()>>`, poderíamos chamar o método `take` na `Option`
para mover o valor para fora da variante `Some` e deixar uma variante `None` em
seu lugar. Em outras palavras, um `Worker` em execução teria uma variante `Some`
em `thread` e, quando quiséssemos limpar um `Worker`, substituiríamos `Some`
por `None` para que o `Worker` não tivesse mais uma *thread* para executar.

No entanto, o _único_ momento em que isso surgiria seria ao descartar o
`Worker`. Em troca, teríamos que lidar com um `Option<thread::JoinHandle<()>>`
em qualquer lugar onde acessássemos `worker.thread`. Rust idiomático usa
`Option` bastante, mas quando você se pega envolvendo em uma `Option` algo que
sabe que sempre estará presente apenas como uma solução alternativa como essa, é
uma boa ideia procurar abordagens alternativas para tornar seu código mais limpo
e menos propenso a erros.

Neste caso, existe uma alternativa melhor: o método `Vec::drain`. Ele aceita um
parâmetro de intervalo (*range*) para especificar quais itens remover do vetor e
retorna um iterador desses itens. Passar a sintaxe de intervalo `..` removerá
*todos* os valores do vetor.

Portanto, precisamos atualizar a implementação de `drop` de `ThreadPool` assim:

<Listing file-name="src/lib.rs">

```rust
{{#rustdoc_include ../listings/ch21-web-server/no-listing-04-update-drop-definition/src/lib.rs:here}}
```

</Listing>

Isso resolve o erro do compilador e não exige nenhuma outra alteração no nosso
código. Observe que, como `drop` pode ser chamado durante um pânico, o `unwrap`
também poderia entrar em pânico e causar um pânico duplo (*double panic*), o
que trava imediatamente o programa e encerra qualquer limpeza em andamento. Isso
é aceitável para um programa de exemplo, mas não é recomendado para código em
produção.

<a id="signaling-to-the-threads-to-stop-listening-for-jobs"></a>
### Sinalizando às Threads para Pararem de Escutar por Trabalhos

Com todas as alterações que fizemos, nosso código compila sem nenhum aviso.
Contudo, a má notícia é que esse código ainda não funciona da maneira que
queremos. A chave está na lógica das *closures* executadas pelas *threads* das
instâncias de `Worker`: no momento, chamamos `join`, mas isso não encerrará as
*threads*, porque elas executam um `loop` infinito procurando trabalhos. Se
tentarmos descartar nosso `ThreadPool` com a nossa implementação atual de
`drop`, a *thread* principal ficará bloqueada para sempre, esperando a primeira
*thread* terminar.

Para corrigir esse problema, precisaremos de uma alteração na implementação de
`drop` de `ThreadPool` e, em seguida, de uma alteração no loop de `Worker`.

Primeiro, alteraremos a implementação de `drop` de `ThreadPool` para descartar
explicitamente `sender` antes de esperar as *threads* terminarem. A Listagem
21-23 mostra as alterações em `ThreadPool` para descartar `sender`
explicitamente. Diferentemente do caso da *thread*, aqui nós _precisamos_ usar
uma `Option` para poder mover `sender` para fora de `ThreadPool` com
`Option::take`.

<Listing number="21-23" file-name="src/lib.rs" caption="Descartando `sender` explicitamente antes de fazer `join` nas *threads* de `Worker`">

```rust,noplayground,not_desired_behavior
{{#rustdoc_include ../listings/ch21-web-server/listing-21-23/src/lib.rs:here}}
```

</Listing>

Descartar `sender` fecha o canal, o que indica que nenhuma outra mensagem será
enviada. Quando isso acontece, todas as chamadas a `recv` que as instâncias de
`Worker` fazem no loop infinito retornarão um erro. Na Listagem 21-24, mudamos o
loop de `Worker` para sair graciosamente do loop nesse caso, o que significa que
as *threads* terminarão quando a implementação de `drop` de `ThreadPool` chamar
`join` nelas.

<Listing number="21-24" file-name="src/lib.rs" caption="Saindo explicitamente do loop quando `recv` retorna um erro">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-24/src/lib.rs:here}}
```

</Listing>

Para ver este código em ação, vamos modificar `main` para aceitar apenas duas
requisições antes de desligar o servidor graciosamente, conforme mostrado na
Listagem 21-25.

<Listing number="21-25" file-name="src/main.rs" caption="Desligando o servidor após atender duas requisições saindo do loop">

```rust,ignore
{{#rustdoc_include ../listings/ch21-web-server/listing-21-25/src/main.rs:here}}
```

</Listing>

Você não iria querer que um servidor web do mundo real desligasse após atender
apenas duas requisições. Este código apenas demonstra que o desligamento
gracioso e a limpeza estão funcionando corretamente.

O método `take` é definido na *trait* `Iterator` e limita a iteração aos dois
primeiros itens no máximo. O `ThreadPool` sairá de escopo ao final de `main`, e
a implementação de `drop` será executada.

Inicie o servidor com `cargo run` e faça três requisições. A terceira requisição
deverá dar erro e, no seu terminal, você deverá ver uma saída semelhante a esta:

<!-- manual-regeneration
cd listings/ch21-web-server/listing-21-25
cargo run
curl http://127.0.0.1:7878
curl http://127.0.0.1:7878
curl http://127.0.0.1:7878
third request will error because server will have shut down
copy output below
Can't automate because the output depends on making requests
-->

```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.41s
     Running `target/debug/hello`
Worker 0 got a job; executing.
Shutting down.
Shutting down worker 0
Worker 3 got a job; executing.
Worker 1 disconnected; shutting down.
Worker 2 disconnected; shutting down.
Worker 3 disconnected; shutting down.
Worker 0 disconnected; shutting down.
Shutting down worker 1
Shutting down worker 2
Shutting down worker 3
```

Você pode ver uma ordenação diferente de IDs de `Worker` e mensagens impressas.
Podemos ver como esse código funciona a partir das mensagens: as instâncias 0 e
3 de `Worker` receberam as duas primeiras requisições. O servidor parou de
aceitar conexões após a segunda conexão, e a implementação de `Drop` em
`ThreadPool` começa a ser executada antes mesmo que `Worker 3` inicie seu
trabalho. Descartar `sender` desconecta todas as instâncias de `Worker` e diz a
elas para desligar. Cada instância de `Worker` imprime uma mensagem quando se
desconecta e, então, o *thread pool* chama `join` para esperar que cada *thread*
de `Worker` termine.

Observe um aspecto interessante dessa execução em particular: o `ThreadPool`
descartou `sender` e, antes que qualquer `Worker` recebesse um erro, tentamos
fazer `join` em `Worker 0`. O `Worker 0` ainda não havia recebido um erro de
`recv`, então a *thread* principal ficou bloqueada, esperando o `Worker 0`
terminar. Enquanto isso, `Worker 3` recebeu um trabalho e, em seguida, todas as
*threads* receberam um erro. Quando `Worker 0` terminou, a *thread* principal
esperou pelo restante das instâncias de `Worker` terminarem. Nesse ponto, todas
elas já haviam saído de seus loops e parado.

Parabéns! Agora concluímos nosso projeto; temos um servidor web básico que usa
um *thread pool* para responder assincronamente. Somos capazes de realizar um
desligamento gracioso do servidor, o que limpa todas as *threads* no pool.

Aqui está o código completo para referência:

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch21-web-server/no-listing-07-final-code/src/main.rs}}
```

</Listing>

<Listing file-name="src/lib.rs">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-07-final-code/src/lib.rs}}
```

</Listing>

Poderíamos fazer mais aqui! Se você quiser continuar aprimorando este projeto,
aqui estão algumas ideias:

- Adicionar mais documentação a `ThreadPool` e a seus métodos públicos.
- Adicionar testes para a funcionalidade da biblioteca.
- Trocar as chamadas a `unwrap` por um tratamento de erros mais robusto.
- Usar `ThreadPool` para realizar alguma tarefa diferente de servir requisições
  web.
- Encontrar uma *crate* de *thread pool* em [crates.io](https://crates.io/) e
  implementar um servidor web semelhante usando a *crate* em vez da nossa. Em
  seguida, compare a API e a robustez dela com o *thread pool* que
  implementamos.

<a id="summary"></a>
## Resumo

Muito bem! Você chegou ao final do livro! Queremos agradecer por se juntar a
nós nesta jornada por Rust. Agora você está pronto para implementar seus
próprios projetos em Rust e ajudar nos projetos de outras pessoas. Lembre-se de
que existe uma comunidade acolhedora de outros *Rustaceans* que adorariam
ajudar você com quaisquer desafios que encontrar em sua jornada com Rust.
