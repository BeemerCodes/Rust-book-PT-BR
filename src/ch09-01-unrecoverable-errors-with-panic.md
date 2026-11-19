<a id="unrecoverable-errors-with-panic"></a>

## Erros Irrecuperáveis com `panic!`

Às vezes, coisas ruins acontecem no seu código e não há nada que você possa fazer a respeito. Nessas situações, o Rust disponibiliza a *macro* `panic!`. Existem duas formas de provocar um *panic* na prática: realizando uma ação que cause o pânico no código (como acessar um vetor além do seu limite) ou chamando a *macro* `panic!` explicitamente. Em ambos os casos, causamos um pânico no programa. Por padrão, esses pânicos imprimem uma mensagem de falha, desfazem a *stack*, limpam os dados e encerram o programa. Por meio de uma variável de ambiente, você também pode fazer o Rust exibir a pilha de chamadas quando ocorre um pânico, facilitando a identificação da origem do problema.

> ### Desfazer a *Stack* ou Abortar em Resposta a um Pânico
>
> Por padrão, quando um pânico ocorre, o programa começa a _desfazer a stack_ (*unwinding*), o que significa que o Rust percorre a pilha de volta e limpa os dados de cada função que encontra pelo caminho. No entanto, percorrer e limpar tudo isso dá bastante trabalho. Por isso, o Rust permite que você escolha a alternativa de _abortar_ imediatamente, encerrando o programa sem fazer limpeza.
>
> A memória que o programa estava usando precisará, então, ser liberada pelo sistema operacional. Se no seu projeto for necessário tornar o binário gerado o menor possível, você pode trocar o comportamento de desfazer a stack pelo de abortar em caso de pânico, adicionando `panic = 'abort'` às seções `[profile]` apropriadas no seu arquivo _Cargo.toml_. Por exemplo, se quiser abortar em caso de pânico no modo *release*, adicione:
>
> ```toml
> [profile.release]
> panic = 'abort'
> ```

Vamos tentar chamar `panic!` em um programa simples:

<Listing file-name="src/main.rs">

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-01-panic/src/main.rs}}
```

</Listing>

Ao executar o programa, você verá algo assim:

```console
{{#include ../listings/ch09-error-handling/no-listing-01-panic/output.txt}}
```

A chamada ao `panic!` gera a mensagem de erro exibida nas duas últimas linhas. A primeira linha mostra nossa mensagem de pânico e o local no código-fonte onde ocorreu: _src/main.rs:2:5_ indica que é a segunda linha, quinto caractere do nosso arquivo _src/main.rs_.

Nesse caso, a linha indicada faz parte do nosso código, e se formos até ela veremos a chamada à *macro* `panic!`. Em outros casos, a chamada a `panic!` pode estar em código que o nosso código chama, e o nome de arquivo e o número de linha reportados na mensagem de erro serão de código de outra pessoa onde a *macro* `panic!` é chamada, não a linha do nosso código que eventualmente levou à chamada a `panic!`.

<!-- Old headings. Do not remove or links may break. -->

<a id="using-a-panic-backtrace"></a>

Podemos usar o *backtrace* das funções das quais a chamada a `panic!` veio para descobrir qual parte do nosso código está causando o problema. Para entender como usar um *backtrace* do `panic!`, vamos ver outro exemplo e observar como é quando uma chamada a `panic!` vem de uma biblioteca por causa de um bug no nosso código, em vez de vir do nosso código chamando a *macro* diretamente. A Listagem 9-1 tem um código que tenta acessar um índice em um vetor além do intervalo de índices válidos.

<Listing number="9-1" file-name="src/main.rs" caption="Tentativa de acessar um elemento além do final de um vetor, o que causará uma chamada a `panic!`">

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-01/src/main.rs}}
```

</Listing>

Aqui, estamos tentando acessar o 100º elemento do vetor (que fica no índice 99, pois a indexação começa em zero), mas o vetor tem apenas três elementos. Nessa situação, o Rust entrará em pânico. Usar `[]` deveria retornar um elemento, mas se você passar um índice inválido, não há nenhum elemento que o Rust possa retornar corretamente.

Em C, tentar ler além do final de uma estrutura de dados é um comportamento indefinido. Você pode obter o que estiver na posição de memória que corresponderia ao elemento naquela estrutura, mesmo que a memória não pertença àquela estrutura. Isso é chamado de _buffer overread_ e pode levar a vulnerabilidades de segurança se um atacante conseguir manipular o índice de forma a ler dados que não deveria, armazenados após a estrutura de dados.

Para proteger seu programa desse tipo de vulnerabilidade, se você tentar ler um elemento em um índice inexistente, o Rust interromperá a execução e se recusará a continuar. Vamos tentar e ver o que acontece:

```console
{{#include ../listings/ch09-error-handling/listing-09-01/output.txt}}
```

Esse erro aponta para a linha 4 do nosso _main.rs_, onde tentamos acessar o índice 99 do vetor `v`.

A linha `note:` nos diz que podemos definir a variável de ambiente `RUST_BACKTRACE` para obter um *backtrace* do exato que aconteceu para causar o erro. Um _backtrace_ é uma lista de todas as funções que foram chamadas até chegar a esse ponto. *Backtraces* no Rust funcionam como em outras linguagens: a chave para ler o *backtrace* é começar do topo e ler até você ver arquivos que você escreveu. Esse é o ponto de origem do problema. As linhas acima desse ponto são código que o seu código chamou; as linhas abaixo são código que chamou o seu código. Essas linhas antes e depois podem incluir código do núcleo do Rust, código da biblioteca padrão ou *crates* que você está usando. Vamos tentar obter um *backtrace* definindo a variável de ambiente `RUST_BACKTRACE` com qualquer valor diferente de `0`. A Listagem 9-2 mostra uma saída semelhante ao que você verá.

<!-- manual-regeneration
cd listings/ch09-error-handling/listing-09-01
RUST_BACKTRACE=1 cargo run
copy the backtrace output below
check the backtrace number mentioned in the text below the listing
-->

<Listing number="9-2" caption="O *backtrace* gerado por uma chamada a `panic!`, exibido quando a variável de ambiente `RUST_BACKTRACE` está definida">

```console
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at src/main.rs:4:6:
index out of bounds: the len is 3 but the index is 99
stack backtrace:
   0: rust_begin_unwind
             at /rustc/4d91de4e48198da2e33413efdcd9cd2cc0c46688/library/std/src/panicking.rs:692:5
   1: core::panicking::panic_fmt
             at /rustc/4d91de4e48198da2e33413efdcd9cd2cc0c46688/library/core/src/panicking.rs:75:14
   2: core::panicking::panic_bounds_check
             at /rustc/4d91de4e48198da2e33413efdcd9cd2cc0c46688/library/core/src/panicking.rs:273:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/slice/index.rs:274:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/slice/index.rs:16:9
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3361:9
   6: panic::main
             at ./src/main.rs:4:6
   7: core::ops::function::FnOnce::call_once
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

</Listing>

São muitas informações! A saída exata que você vê pode ser diferente dependendo do seu sistema operacional e da versão do Rust. Para obter *backtraces* com essas informações, os símbolos de depuração devem estar habilitados. Os símbolos de depuração estão habilitados por padrão ao usar `cargo build` ou `cargo run` sem o sinalizador `--release`, como fizemos aqui.

Na saída da Listagem 9-2, a linha 6 do *backtrace* aponta para a linha do nosso projeto que está causando o problema: a linha 4 de _src/main.rs_. Se não quisermos que nosso programa entre em pânico, devemos começar nossa investigação pelo local apontado pela primeira linha que menciona um arquivo que escrevemos. Na Listagem 9-1, onde deliberadamente escrevemos código que entraria em pânico, a forma de corrigir o pânico é não requisitar um elemento além do intervalo de índices do vetor. Quando seu código entrar em pânico no futuro, você precisará descobrir qual ação o código está executando com quais valores para causar o pânico e o que o código deveria fazer em vez disso.

Voltaremos ao `panic!` e a quando devemos e não devemos usá-lo para tratar condições de erro na seção ["Usar `panic!` ou Não Usar `panic!`"][to-panic-or-not-to-panic]<!-- ignore --> mais adiante neste capítulo. A seguir, veremos como se recuperar de um erro usando `Result`.

[to-panic-or-not-to-panic]: ch09-03-to-panic-or-not-to-panic.html#to-panic-or-not-to-panic
