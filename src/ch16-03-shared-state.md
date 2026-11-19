<a id="shared-state-concurrency"></a>
## Concorrência com Estado Compartilhado

Troca de mensagens é uma ótima maneira de lidar com concorrência, mas não é a
única. Outro método é fazer com que múltiplas *threads* acessem os mesmos dados
compartilhados. Considere esta parte do slogan da documentação da linguagem Go
novamente: "Não se comunique compartilhando memória."

Como seria se comunicar compartilhando memória? Além disso, por que os
entusiastas da troca de mensagens alertam para não usar compartilhamento de
memória?

De certa forma, canais em qualquer linguagem de programação são semelhantes à
propriedade única (*single ownership*), porque uma vez que você transfere um
valor por um canal, não deve mais usar esse valor. A concorrência com memória
compartilhada é como múltiplos proprietários: múltiplas *threads* podem acessar
o mesmo local na memória ao mesmo tempo. Como você viu no Capítulo 15, em que
*smart pointers* tornaram possíveis múltiplos proprietários, múltiplos
proprietários podem adicionar complexidade porque esses diferentes proprietários
precisam ser gerenciados. O sistema de tipos e as regras de *ownership* do Rust
ajudam muito a fazer esse gerenciamento corretamente. Como exemplo, vamos ver os
*mutexes*, uma das primitivas de concorrência mais comuns para memória
compartilhada.

<!-- Old headings. Do not remove or links may break. -->

<a id="using-mutexes-to-allow-access-to-data-from-one-thread-at-a-time"></a>

<a id="controlling-access-with-mutexes"></a>
### Controlando o Acesso com *Mutexes*

*Mutex* é uma abreviação de *mutual exclusion* (exclusão mútua), pois um *mutex*
permite que apenas uma *thread* acesse alguns dados em um determinado momento.
Para acessar os dados em um *mutex*, uma *thread* deve primeiro sinalizar que
deseja acesso solicitando a aquisição do *lock* (trava) do *mutex*. O *lock* é
uma estrutura de dados que faz parte do *mutex* e mantém o controle de quem
atualmente tem acesso exclusivo aos dados. Portanto, diz-se que o *mutex*
*protege* (*guarding*) os dados que armazena por meio do sistema de *lock*.

*Mutexes* têm a reputação de serem difíceis de usar porque você precisa se
lembrar de duas regras:

1. Você deve tentar adquirir o *lock* antes de usar os dados.
2. Quando terminar de usar os dados que o *mutex* protege, você deve desbloquear
   os dados para que outras *threads* possam adquirir o *lock*.

Para uma metáfora do mundo real de um *mutex*, imagine um painel de discussão em
uma conferência com apenas um microfone. Antes que um painelista possa falar,
ele precisa pedir ou sinalizar que deseja usar o microfone. Quando consegue o
microfone, pode falar o quanto quiser e depois passar o microfone para o próximo
painelista que solicitar a palavra. Se um painelista esquecer de passar o
microfone quando terminar, ninguém mais conseguirá falar. Se o gerenciamento do
microfone compartilhado der errado, o painel não funcionará como planejado!

O gerenciamento de *mutexes* pode ser incrivelmente complicado de acertar, e é
por isso que tantas pessoas são entusiastas de canais. No entanto, graças ao
sistema de tipos e às regras de *ownership* do Rust, você não consegue errar no
bloqueio e desbloqueio.

<a id="the-api-of-mutext"></a>
#### A API de `Mutex<T>`

Como exemplo de como usar um *mutex*, vamos começar usando um *mutex* em um
contexto de *thread* única, como mostrado na Listagem 16-12.

<Listing number="16-12" file-name="src/main.rs" caption="Explorando a API de `Mutex<T>` em um contexto de *thread* única por simplicidade">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-12/src/main.rs}}
```

</Listing>

Assim como ocorre com muitos tipos, criamos um `Mutex<T>` usando a função
associada `new`. Para acessar os dados dentro do *mutex*, usamos o método `lock`
para adquirir o *lock*. Essa chamada bloqueará a *thread* atual para que ela não
possa fazer nenhum trabalho até que seja a nossa vez de ter o *lock*.

A chamada a `lock` falharia se outra *thread* segurando o *lock* entrasse em
*panic*. Nesse caso, ninguém jamais conseguiria obter o *lock*, então escolhemos
chamar `unwrap` e fazer esta *thread* entrar em *panic* se estivermos nessa
situação.

Depois de adquirirmos o *lock*, podemos tratar o valor de retorno, chamado `num`
neste caso, como uma referência mutável aos dados internos. O sistema de tipos
garante que adquiramos um *lock* antes de usar o valor em `m`. O tipo de `m` é
`Mutex<i32>`, não `i32`, portanto *precisamos* chamar `lock` para poder usar o
valor `i32`. Não podemos esquecer; o sistema de tipos não nos permitirá acessar
o `i32` interno de outra forma.

A chamada a `lock` retorna um tipo chamado `MutexGuard`, envolvido em um
`LockResult` que tratamos com a chamada a `unwrap`. O tipo `MutexGuard`
implementa `Deref` para apontar para nossos dados internos; o tipo também tem
uma implementação de `Drop` que libera o *lock* automaticamente quando um
`MutexGuard` sai de escopo, o que acontece no final do escopo interno. Como
resultado, não corremos o risco de esquecer de liberar o *lock* e bloquear o
*mutex* de ser usado por outras *threads*, porque a liberação do *lock* acontece
automaticamente.

Após liberar o *lock*, podemos imprimir o valor do *mutex* e ver que conseguimos
alterar o `i32` interno para `6`.

<!-- Old headings. Do not remove or links may break. -->

<a id="sharing-a-mutext-between-multiple-threads"></a>

<a id="shared-access-to-mutext"></a>
#### Acesso Compartilhado a `Mutex<T>`

Agora vamos tentar compartilhar um valor entre múltiplas *threads* usando
`Mutex<T>`. Criaremos 10 *threads* e faremos cada uma incrementar um valor de
contador em 1, de modo que o contador vá de 0 a 10. O exemplo na Listagem 16-13
terá um erro de compilação, e usaremos esse erro para aprender mais sobre como
usar `Mutex<T>` e como o Rust nos ajuda a usá-lo corretamente.

<Listing number="16-13" file-name="src/main.rs" caption="Dez *threads*, cada uma incrementando um contador protegido por um `Mutex<T>`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-13/src/main.rs}}
```

</Listing>

Criamos uma variável `counter` para armazenar um `i32` dentro de um `Mutex<T>`,
como fizemos na Listagem 16-12. Em seguida, criamos 10 *threads* iterando sobre
um intervalo de números. Usamos `thread::spawn` e passamos a mesma *closure*
para todas as *threads*: uma que move o contador para dentro da *thread*,
adquire um *lock* no `Mutex<T>` chamando o método `lock` e então soma 1 ao valor
no *mutex*. Quando uma *thread* termina de executar sua *closure*, `num` sai de
escopo e libera o *lock* para que outra *thread* possa adquiri-lo.

Na *thread* principal, coletamos todos os *join handles*. Então, como fizemos na
Listagem 16-2, chamamos `join` em cada *handle* para garantir que todas as
*threads* terminem. Nesse ponto, a *thread* principal adquirirá o *lock* e
imprimirá o resultado deste programa.

Já demos a dica de que este exemplo não compilaria. Agora vamos descobrir por
quê!

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-13/output.txt}}
```

A mensagem de erro afirma que o valor `counter` foi movido na iteração anterior
do laço. O Rust está nos dizendo que não podemos mover a propriedade do *lock*
`counter` para múltiplas *threads*. Vamos corrigir o erro de compilação com o
método de múltiplos proprietários que discutimos no Capítulo 15.

<a id="multiple-ownership-with-multiple-threads"></a>
#### Múltiplos Proprietários com Múltiplas *Threads*

No Capítulo 15, demos um valor a múltiplos proprietários usando o *smart
pointer* `Rc<T>` para criar um valor com contagem de referências. Vamos fazer o
mesmo aqui e ver o que acontece. Envolveremos o `Mutex<T>` em `Rc<T>` na
Listagem 16-14 e clonaremos o `Rc<T>` antes de mover a propriedade para a
*thread*.

<Listing number="16-14" file-name="src/main.rs" caption="Tentando usar `Rc<T>` para permitir que múltiplas *threads* possuam o `Mutex<T>`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-14/src/main.rs}}
```

</Listing>

Mais uma vez, compilamos e obtemos... erros diferentes! O compilador está nos
ensinando muito:

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-14/output.txt}}
```

Nossa, essa mensagem de erro é bem longa! Aqui está a parte importante em que
devemos focar: `` `Rc<Mutex<i32>>` cannot be sent between threads safely ``. O
compilador também está nos dizendo o motivo: `` the trait `Send` is not
implemented for `Rc<Mutex<i32>>` ``. Falaremos sobre `Send` na próxima seção:
ela é uma das *traits* que garantem que os tipos que usamos com *threads* são
feitos para uso em situações concorrentes.

Infelizmente, `Rc<T>` não é seguro para compartilhar entre *threads*. Quando
`Rc<T>` gerencia a contagem de referências, ele soma à contagem para cada
chamada a `clone` e subtrai da contagem quando cada clone é liberado. Mas ele
não usa nenhuma primitiva de concorrência para garantir que mudanças na contagem
não possam ser interrompidas por outra *thread*. Isso poderia levar a contagens
erradas — bugs sutis que, por sua vez, poderiam levar a vazamentos de memória ou
a um valor sendo liberado antes de terminarmos de usá-lo. O que precisamos é de
um tipo que seja exatamente como `Rc<T>`, mas que faça alterações na contagem de
referências de maneira *thread-safe*.

<a id="atomic-reference-counting-with-arct"></a>
#### Contagem de Referências Atômica com `Arc<T>`

Felizmente, `Arc<T>` *é* um tipo como `Rc<T>` que é seguro para usar em
situações concorrentes. O *a* significa *atomic* (atômico), indicando que é um
tipo *com contagem de referências atômica* (*atomically reference-counted*).
Atômicos são um tipo adicional de primitiva de concorrência que não abordaremos
em detalhes aqui: consulte a documentação da biblioteca padrão para
[`std::sync::atomic`][atomic]<!-- ignore --> para mais detalhes. Neste momento,
você só precisa saber que atômicos funcionam como tipos primitivos, mas são
seguros para compartilhar entre *threads*.

Você pode então se perguntar por que todos os tipos primitivos não são atômicos
e por que os tipos da biblioteca padrão não são implementados para usar `Arc<T>`
por padrão. A razão é que a segurança entre *threads* (*thread safety*) vem com
uma penalidade de desempenho que você só quer pagar quando realmente precisa. Se
você está apenas realizando operações em valores dentro de uma única *thread*,
seu código pode rodar mais rápido se não precisar impor as garantias que os
atômicos fornecem.

Vamos voltar ao nosso exemplo: `Arc<T>` e `Rc<T>` têm a mesma API, então
corrigimos nosso programa alterando a linha `use`, a chamada a `new` e a chamada
a `clone`. O código na Listagem 16-15 finalmente compilará e executará.

<Listing number="16-15" file-name="src/main.rs" caption="Usando um `Arc<T>` para envolver o `Mutex<T>` e poder compartilhar a propriedade entre múltiplas *threads*">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-15/src/main.rs}}
```

</Listing>

Esse código imprimirá o seguinte:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Result: 10
```

Conseguimos! Contamos de 0 a 10, o que pode não parecer muito impressionante,
mas nos ensinou muito sobre `Mutex<T>` e *thread safety*. Você também poderia
usar a estrutura desse programa para fazer operações mais complicadas do que
apenas incrementar um contador. Usando essa estratégia, você pode dividir um
cálculo em partes independentes, distribuir essas partes entre *threads* e então
usar um `Mutex<T>` para fazer cada *thread* atualizar o resultado final com sua
parte.

Observe que, se você estiver fazendo operações numéricas simples, existem tipos
mais simples do que `Mutex<T>` fornecidos pelo [módulo `std::sync::atomic` da
biblioteca padrão][atomic]<!-- ignore -->. Esses tipos fornecem acesso atômico,
concorrente e seguro a tipos primitivos. Escolhemos usar `Mutex<T>` com um tipo
primitivo para este exemplo para podermos nos concentrar em como `Mutex<T>`
funciona.

<!-- Old headings. Do not remove or links may break. -->

<a id="similarities-between-refcelltrct-and-mutextarct"></a>

<a id="comparing-refcelltrct-and-mutextarct"></a>
### Comparando `RefCell<T>`/`Rc<T>` e `Mutex<T>`/`Arc<T>`

Você pode ter notado que `counter` é imutável, mas conseguimos obter uma
referência mutável para o valor dentro dele; isso significa que `Mutex<T>`
fornece mutabilidade interior, assim como a família `Cell`. Da mesma forma que
usamos `RefCell<T>` no Capítulo 15 para nos permitir mutar o conteúdo dentro de
um `Rc<T>`, usamos `Mutex<T>` para mutar o conteúdo dentro de um `Arc<T>`.

Outro detalhe a notar é que o Rust não pode protegê-lo de todos os tipos de
erros lógicos quando você usa `Mutex<T>`. Lembre-se do Capítulo 15 que usar
`Rc<T>` trazia o risco de criar ciclos de referência, em que dois valores
`Rc<T>` se referenciam mutuamente, causando vazamentos de memória. Da mesma
forma, `Mutex<T>` traz o risco de criar *deadlocks*. Eles ocorrem quando uma
operação precisa bloquear dois recursos e duas *threads* adquiriram cada uma um
dos *locks*, fazendo com que fiquem esperando uma pela outra para sempre. Se
você se interessar por *deadlocks*, tente criar um programa Rust que tenha um
*deadlock*; depois, pesquise estratégias de mitigação de *deadlocks* para
*mutexes* em qualquer linguagem e tente implementá-las em Rust. A documentação da
API da biblioteca padrão para `Mutex<T>` e `MutexGuard` oferece informações
úteis.

Vamos encerrar este capítulo falando sobre as *traits* `Send` e `Sync` e como
podemos usá-las com tipos personalizados.

[atomic]: ../std/sync/atomic/index.html
