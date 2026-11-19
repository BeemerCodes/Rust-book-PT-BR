<!-- Old headings. Do not remove or links may break. -->

<a id="turning-our-single-threaded-server-into-a-multithreaded-server"></a>
<a id="from-single-threaded-to-multithreaded-server"></a>
<a id="from-a-single-threaded-to-a-multithreaded-server"></a>

## De um Servidor Single-Threaded para um Servidor Multithreaded

No momento, o servidor processará cada requisição por vez, o que significa que
ele não processará uma segunda conexão até que a primeira conexão termine de
ser processada. Se o servidor recebesse mais e mais requisições, essa execução
serial seria cada vez menos ideal. Se o servidor receber uma requisição que leva
muito tempo para ser processada, as requisições subsequentes terão de esperar
até que a requisição demorada termine, mesmo que as novas requisições possam ser
processadas rapidamente. Precisaremos corrigir isso, mas primeiro veremos o
problema em ação.

<!-- Old headings. Do not remove or links may break. -->

<a id="simulating-a-slow-request-in-the-current-server-implementation"></a>
<a id="simulating-a-slow-request"></a>

### Simulando uma Requisição Lenta

Veremos como uma requisição processada lentamente pode afetar outras requisições
feitas à nossa implementação atual do servidor. A Listagem 21-10 implementa o
tratamento de uma requisição para _/sleep_ com uma resposta lenta simulada que
fará o servidor dormir por cinco segundos antes de responder.

<Listing number="21-10" file-name="src/main.rs" caption="Simulando uma requisição lenta dormindo por cinco segundos">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-10/src/main.rs:here}}
```

</Listing>

Trocamos `if` por `match` agora que temos três casos. Precisamos fazer o
*match* explicitamente em um *slice* de `request_line` para usar *pattern
matching* contra os valores de string literal; `match` não faz referenciação e
desreferenciação automáticas, como o método de igualdade faz.

O primeiro braço é igual ao bloco `if` da Listagem 21-9. O segundo braço
corresponde a uma requisição para _/sleep_. Quando essa requisição for recebida,
o servidor dormirá por cinco segundos antes de renderizar a página HTML de
sucesso. O terceiro braço é igual ao bloco `else` da Listagem 21-9.

Você pode ver o quão primitivo é o nosso servidor: bibliotecas reais lidariam
com o reconhecimento de múltiplas requisições de uma maneira muito menos
verbosa!

Inicie o servidor usando `cargo run`. Em seguida, abra duas janelas do
navegador: uma para _http://127.0.0.1:7878_ e a outra para
_http://127.0.0.1:7878/sleep_. Se você acessar o URI _/_ algumas vezes, como
antes, verá que ele responde rapidamente. Mas se você acessar _/sleep_ e depois
carregar _/_, verá que _/_ espera até que `sleep` tenha dormido por todos os
cinco segundos antes de carregar.

Existem várias técnicas que poderíamos usar para evitar que requisições fiquem
acumuladas atrás de uma requisição lenta, incluindo o uso de *async* como
fizemos no Capítulo 17; a que implementaremos é um *thread pool*.

<a id="improving-throughput-with-a-thread-pool"></a>
### Melhorando a Vazão com um Thread Pool

Um _thread pool_ é um grupo de *threads* criadas (*spawned*) que estão prontas
e esperando para lidar com uma tarefa. Quando o programa recebe uma nova tarefa,
ele atribui uma das *threads* do pool à tarefa, e essa *thread* processará a
tarefa. As *threads* restantes no pool ficam disponíveis para lidar com
quaisquer outras tarefas que cheguem enquanto a primeira *thread* está
processando. Quando a primeira *thread* termina de processar sua tarefa, ela
retorna ao pool de *threads* ociosas, pronta para lidar com uma nova tarefa. Um
*thread pool* permite processar conexões concorrentemente, aumentando a vazão
(*throughput*) do seu servidor.

Limitaremos o número de *threads* no pool a um número pequeno para nos proteger
de ataques DoS; se fizéssemos nosso programa criar uma nova *thread* para cada
requisição que chegasse, alguém fazendo 10 milhões de requisições ao nosso
servidor poderia causar estragos consumindo todos os recursos do servidor e
paralisando o processamento de requisições.

Em vez de criar *threads* ilimitadas, portanto, teremos um número fixo de
*threads* esperando no pool. As requisições que chegam são enviadas para o pool
para processamento. O pool manterá uma fila de requisições recebidas. Cada uma
das *threads* no pool retirará uma requisição dessa fila, tratará a requisição
e, em seguida, pedirá outra requisição à fila. Com esse design, podemos
processar até _`N`_ requisições concorrentemente, onde _`N`_ é o número de
*threads*. Se cada *thread* estiver respondendo a uma requisição demorada,
requisições subsequentes ainda podem se acumular na fila, mas aumentamos o
número de requisições demoradas que podemos tratar antes de atingir esse ponto.

Essa técnica é apenas uma das muitas maneiras de melhorar a vazão de um
servidor web. Outras opções que você pode explorar são o modelo *fork/join*, o
modelo de E/S assíncrona *single-threaded* e o modelo de E/S assíncrona
*multithreaded*. Se você tiver interesse nesse tópico, pode ler mais sobre
outras soluções e tentar implementá-las; com uma linguagem de baixo nível como
Rust, todas essas opções são possíveis.

Antes de começarmos a implementar um *thread pool*, vamos falar sobre como deve
ser o uso do pool. Quando você está tentando projetar código, escrever primeiro
a interface do cliente pode ajudar a guiar o seu design. Escreva a API do
código de forma que ela fique estruturada do jeito que você quer chamá-la;
depois, implemente a funcionalidade dentro dessa estrutura, em vez de
implementar a funcionalidade e só então projetar a API pública.

De forma semelhante a como usamos o desenvolvimento orientado a testes no
projeto do Capítulo 12, usaremos aqui o desenvolvimento orientado pelo
compilador (*compiler-driven development*). Escreveremos o código que chama as
funções que queremos e, em seguida, olharemos os erros do compilador para
determinar o que devemos mudar a seguir para fazer o código funcionar. Antes de
fazer isso, porém, exploraremos como ponto de partida a técnica que não vamos
usar.

<!-- Old headings. Do not remove or links may break. -->

<a id="code-structure-if-we-could-spawn-a-thread-for-each-request"></a>
<a id="spawning-a-thread-for-each-request"></a>

#### Criando uma Thread para Cada Requisição

Primeiro, vamos explorar como ficaria nosso código se ele criasse uma nova
*thread* para cada conexão. Como mencionado antes, esse não é nosso plano final
devido aos problemas de potencialmente criar um número ilimitado de *threads*,
mas é um ponto de partida para ter um servidor *multithreaded* funcionando
primeiro. Depois, adicionaremos o *thread pool* como uma melhoria, e comparar
as duas soluções será mais fácil.

A Listagem 21-11 mostra as alterações a serem feitas em `main` para criar uma
nova *thread* para tratar cada *stream* dentro do loop `for`.

<Listing number="21-11" file-name="src/main.rs" caption="Criando uma nova *thread* para cada *stream*">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-11/src/main.rs:here}}
```

</Listing>

Como você aprendeu no Capítulo 16, `thread::spawn` criará uma nova *thread* e
então executará o código da *closure* na nova *thread*. Se você rodar este
código e carregar _/sleep_ no navegador, e depois _/_ em mais duas abas do
navegador, verá de fato que as requisições para _/_ não precisam esperar _/sleep_
terminar. Contudo, como mencionamos, isso acabará sobrecarregando o sistema
porque você estaria criando novas *threads* sem nenhum limite.

Você também deve se lembrar do Capítulo 17 que essa é exatamente a situação em
que *async* e *await* realmente brilham! Tenha isso em mente enquanto construímos
o *thread pool* e pense em como as coisas seriam diferentes ou iguais com
*async*.

<!-- Old headings. Do not remove or links may break. -->

<a id="creating-a-similar-interface-for-a-finite-number-of-threads"></a>
<a id="creating-a-finite-number-of-threads"></a>

#### Criando um Número Finito de Threads

Queremos que nosso *thread pool* funcione de maneira semelhante e familiar para
que a troca de *threads* para um *thread pool* não exija grandes alterações no
código que usa nossa API. A Listagem 21-12 mostra a interface hipotética para
uma *struct* `ThreadPool` que queremos usar no lugar de `thread::spawn`.

<Listing number="21-12" file-name="src/main.rs" caption="Nossa interface ideal de `ThreadPool`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch21-web-server/listing-21-12/src/main.rs:here}}
```

</Listing>

Usamos `ThreadPool::new` para criar um novo *thread pool* com um número
configurável de *threads*, neste caso quatro. Então, no loop `for`,
`pool.execute` tem uma interface semelhante à de `thread::spawn`, pois recebe
uma *closure* que o pool deve executar para cada *stream*. Precisamos
implementar `pool.execute` de modo que ele receba a *closure* e a entregue a
uma *thread* no pool para executar. Este código ainda não compilará, mas
tentaremos compilá-lo para que o compilador possa nos guiar sobre como
corrigi-lo.

<!-- Old headings. Do not remove or links may break. -->

<a id="building-the-threadpool-struct-using-compiler-driven-development"></a>
<a id="building-threadpool-using-compiler-driven-development"></a>

#### Construindo `ThreadPool` Usando Desenvolvimento Orientado pelo Compilador

Faça as alterações da Listagem 21-12 em _src/main.rs_ e, em seguida, vamos usar
os erros do compilador vindos de `cargo check` para conduzir nosso
desenvolvimento. Aqui está o primeiro erro que obtemos:

```console
{{#include ../listings/ch21-web-server/listing-21-12/output.txt}}
```

Ótimo! Esse erro nos diz que precisamos de um tipo ou módulo `ThreadPool`, então
vamos construir um agora. Nossa implementação de `ThreadPool` será independente
do tipo de trabalho que nosso servidor web está fazendo. Portanto, vamos mudar a
*crate* `hello` de uma *crate* binária para uma *crate* de biblioteca para
abrigar nossa implementação de `ThreadPool`. Depois de mudarmos para uma
*crate* de biblioteca, poderemos também usar a biblioteca separada de *thread
pool* para qualquer trabalho que quisermos fazer usando um *thread pool*, não
apenas para servir requisições web.

Crie um arquivo _src/lib.rs_ que contenha o seguinte código, que é a definição
mais simples de uma *struct* `ThreadPool` que podemos ter por enquanto:

<Listing file-name="src/lib.rs">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-01-define-threadpool-struct/src/lib.rs}}
```

</Listing>

Em seguida, edite o arquivo _main.rs_ para trazer `ThreadPool` para o escopo a
partir da *crate* de biblioteca adicionando o seguinte código ao topo de
_src/main.rs_:

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch21-web-server/no-listing-01-define-threadpool-struct/src/main.rs:here}}
```

</Listing>

Este código ainda não funcionará, mas vamos checá-lo novamente para obter o
próximo erro que precisamos resolver:

```console
{{#include ../listings/ch21-web-server/no-listing-01-define-threadpool-struct/output.txt}}
```

Esse erro indica que, em seguida, precisamos criar uma função associada chamada
`new` para `ThreadPool`. Também sabemos que `new` precisa ter um parâmetro que
possa aceitar `4` como argumento e deve retornar uma instância de `ThreadPool`.
Vamos implementar a função `new` mais simples que terá essas características:

<Listing file-name="src/lib.rs">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-02-impl-threadpool-new/src/lib.rs}}
```

</Listing>

Escolhemos `usize` como o tipo do parâmetro `size` porque sabemos que um número
negativo de *threads* não faz nenhum sentido. Também sabemos que usaremos esse
`4` como o número de elementos em uma coleção de *threads*, que é para o que o
tipo `usize` serve, conforme discutido na seção [“Tipos Inteiros”][integer-types]<!--
ignore --> no Capítulo 3.

Vamos checar o código novamente:

```console
{{#include ../listings/ch21-web-server/no-listing-02-impl-threadpool-new/output.txt}}
```

Agora o erro ocorre porque não temos um método `execute` em `ThreadPool`.
Lembre-se da seção [“Criando um Número Finito de
Threads”](#creating-a-finite-number-of-threads)<!-- ignore --> que decidimos que
nosso *thread pool* deve ter uma interface semelhante a `thread::spawn`. Além
disso, implementaremos a função `execute` de modo que ela receba a *closure*
que lhe for dada e a entregue a uma *thread* ociosa no pool para executar.

Definiremos o método `execute` em `ThreadPool` para receber uma *closure* como
parâmetro. Lembre-se da seção [“Movendo Valores Capturados para Fora das
Closures”][moving-out-of-closures]<!-- ignore --> no Capítulo 13 que podemos
receber *closures* como parâmetros com três *traits* diferentes: `Fn`, `FnMut` e
`FnOnce`. Precisamos decidir qual tipo de *closure* usar aqui. Sabemos que
acabaremos fazendo algo semelhante à implementação de `thread::spawn` da
biblioteca padrão, então podemos olhar quais limites (*bounds*) a assinatura de
`thread::spawn` tem em seu parâmetro. A documentação nos mostra o seguinte:

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```

O parâmetro de tipo `F` é aquele com o qual estamos preocupados aqui; o
parâmetro de tipo `T` está relacionado ao valor de retorno, e não estamos
preocupados com isso. Podemos ver que `spawn` usa `FnOnce` como limite de
*trait* (*trait bound*) em `F`. Isso provavelmente é o que queremos também,
porque eventualmente passaremos o argumento que recebemos em `execute` para
`spawn`. Podemos ter ainda mais certeza de que `FnOnce` é a *trait* que
queremos usar porque a *thread* que executa uma requisição só executará a
*closure* daquela requisição uma vez, o que combina com o `Once` em `FnOnce`.

O parâmetro de tipo `F` também tem o limite de *trait* `Send` e o limite de
*lifetime* `'static`, que são úteis na nossa situação: precisamos de `Send` para
transferir a *closure* de uma *thread* para outra e de `'static` porque não
sabemos quanto tempo a *thread* levará para executar. Vamos criar um método
`execute` em `ThreadPool` que receberá um parâmetro genérico do tipo `F` com
esses limites:

<Listing file-name="src/lib.rs">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-03-define-execute/src/lib.rs:here}}
```

</Listing>

Ainda usamos o `()` após `FnOnce` porque esse `FnOnce` representa uma *closure*
que não recebe parâmetros e retorna o tipo unitário `()`. Assim como nas
definições de função, o tipo de retorno pode ser omitido da assinatura, mas
mesmo que não tenhamos parâmetros, ainda precisamos dos parênteses.

Novamente, esta é a implementação mais simples do método `execute`: ele não faz
nada, mas estamos apenas tentando fazer nosso código compilar. Vamos checá-lo
novamente:

```console
{{#include ../listings/ch21-web-server/no-listing-03-define-execute/output.txt}}
```

Ele compila! Mas observe que, se você tentar `cargo run` e fizer uma requisição
no navegador, verá no navegador os erros que vimos no início do capítulo. Nossa
biblioteca ainda não está de fato chamando a *closure* passada para `execute`!

> Note: Um ditado que você pode ouvir sobre linguagens com compiladores
> rigorosos, como Haskell e Rust, é “Se o código compila, ele funciona”. Mas
> esse ditado não é universalmente verdadeiro. Nosso projeto compila, mas não
> faz absolutamente nada! Se estivéssemos construindo um projeto real e
> completo, este seria um bom momento para começar a escrever testes unitários
> para verificar se o código compila _e_ tem o comportamento que queremos.

Reflita: o que seria diferente aqui se fôssemos executar uma *future* em vez de
uma *closure*?

<a id="validating-the-number-of-threads-in-new"></a>
#### Validando o Número de Threads em `new`

Não estamos fazendo nada com os parâmetros de `new` e `execute`. Vamos
implementar os corpos dessas funções com o comportamento que queremos. Para
começar, vamos pensar em `new`. Antes escolhemos um tipo sem sinal (*unsigned*)
para o parâmetro `size` porque um pool com um número negativo de *threads* não
faz sentido. No entanto, um pool com zero *threads* também não faz sentido, mas
zero é um `usize` perfeitamente válido. Adicionaremos código para verificar se
`size` é maior que zero antes de retornarmos uma instância de `ThreadPool` e
faremos o programa entrar em pânico (*panic*) se ele receber zero usando a
*macro* `assert!`, conforme mostrado na Listagem 21-13.

<Listing number="21-13" file-name="src/lib.rs" caption="Implementando `ThreadPool::new` para entrar em pânico se `size` for zero">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-13/src/lib.rs:here}}
```

</Listing>

Também adicionamos alguma documentação para o nosso `ThreadPool` com comentários
de documentação (*doc comments*). Observe que seguimos boas práticas de
documentação adicionando uma seção que destaca as situações em que nossa função
pode entrar em pânico, conforme discutido no Capítulo 14. Tente rodar
`cargo doc --open` e clicar na *struct* `ThreadPool` para ver como ficou a
documentação gerada para `new`!

Em vez de adicionar a *macro* `assert!` como fizemos aqui, poderíamos mudar `new`
para `build` e retornar um `Result`, como fizemos com `Config::build` no projeto
de E/S na Listagem 12-9. Mas decidimos neste caso que tentar criar um *thread
pool* sem nenhuma *thread* deve ser um erro irrecuperável. Se você estiver se
sentindo ambicioso, tente escrever uma função chamada `build` com a seguinte
assinatura para comparar com a função `new`:

```rust,ignore
pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
```

<a id="creating-space-to-store-the-threads"></a>
#### Criando Espaço para Armazenar as Threads

Agora que temos uma maneira de saber que temos um número válido de *threads*
para armazenar no pool, podemos criar essas *threads* e armazená-las na *struct*
`ThreadPool` antes de retornar a *struct*. Mas como “armazenamos” uma *thread*?
Vamos dar outra olhada na assinatura de `thread::spawn`:

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```

A função `spawn` retorna um `JoinHandle<T>`, onde `T` é o tipo que a *closure*
retorna. Vamos tentar usar `JoinHandle` também e ver o que acontece. No nosso
caso, as *closures* que estamos passando para o *thread pool* tratarão a
conexão e não retornarão nada, então `T` será o tipo unitário `()`.

O código da Listagem 21-14 compilará, mas ainda não cria nenhuma *thread*.
Mudamos a definição de `ThreadPool` para guardar um vetor de instâncias
`thread::JoinHandle<()>`, inicializamos o vetor com uma capacidade de `size`,
configuramos um loop `for` que executará algum código para criar as *threads* e
retornamos uma instância de `ThreadPool` contendo-as.

<Listing number="21-14" file-name="src/lib.rs" caption="Criando um vetor para `ThreadPool` guardar as *threads*">

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch21-web-server/listing-21-14/src/lib.rs:here}}
```

</Listing>

Trouxemos `std::thread` para o escopo na *crate* de biblioteca porque estamos
usando `thread::JoinHandle` como o tipo dos itens no vetor em `ThreadPool`.

Assim que um tamanho válido é recebido, nosso `ThreadPool` cria um novo vetor
que pode conter `size` itens. A função `with_capacity` realiza a mesma tarefa
que `Vec::new`, mas com uma diferença importante: ela pré-aloca espaço no
vetor. Como sabemos que precisamos armazenar `size` elementos no vetor, fazer
essa alocação antecipadamente é ligeiramente mais eficiente do que usar
`Vec::new`, que se redimensiona à medida que elementos são inseridos.

Quando você rodar `cargo check` novamente, ele deverá ser bem-sucedido.

<!-- Old headings. Do not remove or links may break. -->
<a id ="a-worker-struct-responsible-for-sending-code-from-the-threadpool-to-a-thread"></a>
<a id="sending-code-from-the-threadpool-to-a-thread"></a>

#### Enviando Código do `ThreadPool` para uma Thread

Deixamos um comentário no loop `for` da Listagem 21-14 sobre a criação de
*threads*. Aqui, veremos como realmente criar *threads*. A biblioteca padrão
fornece `thread::spawn` como uma forma de criar *threads*, e `thread::spawn`
espera receber algum código que a *thread* deve executar assim que for criada.
No entanto, no nosso caso, queremos criar as *threads* e fazê-las _esperar_
pelo código que enviaremos depois. A implementação de *threads* da biblioteca
padrão não inclui nenhuma maneira de fazer isso; temos que implementá-la
manualmente.

Implementaremos esse comportamento introduzindo uma nova estrutura de dados entre
o `ThreadPool` e as *threads* que gerenciará esse novo comportamento. Chamaremos
essa estrutura de dados de _Worker_ (Trabalhador), que é um termo comum em
implementações de pooling. O `Worker` pega o código que precisa ser executado e
executa o código em sua *thread*.

Pense nas pessoas trabalhando na cozinha de um restaurante: os trabalhadores
esperam até que os pedidos cheguem dos clientes e, então, são responsáveis por
pegar esses pedidos e prepará-los.

Em vez de armazenar um vetor de instâncias de `JoinHandle<()>` no *thread
pool*, armazenaremos instâncias da *struct* `Worker`. Cada `Worker` armazenará
uma única instância de `JoinHandle<()>`. Depois, implementaremos um método em
`Worker` que receberá uma *closure* de código para executar e a enviará para a
*thread* que já está rodando para execução. Também daremos a cada `Worker` um
`id` para que possamos distinguir entre as diferentes instâncias de `Worker` no
pool ao registrar logs ou depurar.

Aqui está o novo processo que acontecerá quando criarmos um `ThreadPool`.
Implementaremos o código que envia a *closure* para a *thread* depois de
termos `Worker` configurado desta forma:

1. Definir uma *struct* `Worker` que guarda um `id` e um `JoinHandle<()>`.
2. Mudar `ThreadPool` para guardar um vetor de instâncias de `Worker`.
3. Definir uma função `Worker::new` que recebe um número `id` e retorna uma
   instância de `Worker` que guarda o `id` e uma *thread* criada com uma
   *closure* vazia.
4. Em `ThreadPool::new`, usar o contador do loop `for` para gerar um `id`, criar
   um novo `Worker` com esse `id` e armazenar o `Worker` no vetor.

Se você estiver pronto para um desafio, tente implementar essas alterações por
conta própria antes de olhar o código na Listagem 21-15.

Pronto? Aqui está a Listagem 21-15 com uma maneira de fazer as modificações
anteriores.

<Listing number="21-15" file-name="src/lib.rs" caption="Modificando `ThreadPool` para guardar instâncias de `Worker` em vez de guardar *threads* diretamente">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-15/src/lib.rs:here}}
```

</Listing>

Mudamos o nome do campo em `ThreadPool` de `threads` para `workers` porque ele
agora está guardando instâncias de `Worker` em vez de instâncias de
`JoinHandle<()>`. Usamos o contador no loop `for` como argumento para
`Worker::new`, e armazenamos cada novo `Worker` no vetor chamado `workers`.

Código externo (como o nosso servidor em _src/main.rs_) não precisa saber dos
detalhes de implementação referentes ao uso de uma *struct* `Worker` dentro de
`ThreadPool`, então tornamos a *struct* `Worker` e sua função `new` privadas. A
função `Worker::new` usa o `id` que lhe damos e armazena uma instância de
`JoinHandle<()>` criada ao iniciar uma nova *thread* usando uma *closure* vazia.

> Note: Se o sistema operacional não puder criar uma *thread* porque não há
> recursos de sistema suficientes, `thread::spawn` entrará em pânico. Isso fará
> com que todo o nosso servidor entre em pânico, mesmo que a criação de algumas
> *threads* possa ter sucesso. Por simplicidade, esse comportamento está bom, mas
> em uma implementação de *thread pool* para produção, você provavelmente
> gostaria de usar [`std::thread::Builder`][builder]<!-- ignore --> e seu método
> [`spawn`][builder-spawn]<!-- ignore --> que retorna um `Result` em vez disso.

Este código compilará e armazenará o número de instâncias de `Worker` que
especificamos como argumento para `ThreadPool::new`. Mas _ainda_ não estamos
processando a *closure* que recebemos em `execute`. Vamos ver como fazer isso a
seguir.

<a id="sending-requests-to-threads-via-channels"></a>
#### Enviando Requisições para Threads via Canais

O próximo problema que enfrentaremos é que as *closures* dadas a `thread::spawn`
não fazem absolutamente nada. Atualmente, recebemos a *closure* que queremos
executar no método `execute`. Mas precisamos dar a `thread::spawn` uma *closure*
para executar quando criamos cada `Worker` durante a criação do `ThreadPool`.

Queremos que as *structs* `Worker` que acabamos de criar busquem o código a ser
executado em uma fila mantida no `ThreadPool` e enviem esse código para sua
*thread* executar.

Os canais (*channels*) que aprendemos no Capítulo 16 — uma forma simples de
comunicação entre duas *threads* — seriam perfeitos para esse caso de uso.
Usaremos um canal para funcionar como a fila de trabalhos (*jobs*), e `execute`
enviará um trabalho do `ThreadPool` para as instâncias de `Worker`, que enviarão
o trabalho para sua *thread*. Aqui está o plano:

1. O `ThreadPool` criará um canal e guardará o transmissor (*sender*).
2. Cada `Worker` guardará o receptor (*receiver*).
3. Criaremos uma nova *struct* `Job` que guardará as *closures* que queremos
   enviar pelo canal.
4. O método `execute` enviará o trabalho que deseja executar através do
   transmissor.
5. Em sua *thread*, o `Worker` fará um loop sobre seu receptor e executará as
   *closures* de quaisquer trabalhos que receber.

Vamos começar criando um canal em `ThreadPool::new` e guardando o transmissor
na instância de `ThreadPool`, conforme mostrado na Listagem 21-16. A *struct*
`Job` não guarda nada por enquanto, mas será o tipo de item que estamos
enviando pelo canal.

<Listing number="21-16" file-name="src/lib.rs" caption="Modificando `ThreadPool` para armazenar o transmissor de um canal que transmite instâncias de `Job`">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-16/src/lib.rs:here}}
```

</Listing>

Em `ThreadPool::new`, criamos nosso novo canal e fazemos o pool guardar o
transmissor. Isso compilará com sucesso.

Vamos tentar passar um receptor do canal para cada `Worker` à medida que o
*thread pool* cria o canal. Sabemos que queremos usar o receptor na *thread* que
as instâncias de `Worker` criam, então referenciaremos o parâmetro `receiver` na
*closure*. O código da Listagem 21-17 ainda não compilará.

<Listing number="21-17" file-name="src/lib.rs" caption="Passando o receptor para cada `Worker`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch21-web-server/listing-21-17/src/lib.rs:here}}
```

</Listing>

Fizemos algumas alterações pequenas e diretas: passamos o receptor para
`Worker::new` e então o usamos dentro da *closure*.

Quando tentamos verificar este código, obtemos este erro:

```console
{{#include ../listings/ch21-web-server/listing-21-17/output.txt}}
```

O código está tentando passar `receiver` para múltiplas instâncias de `Worker`.
Isso não funcionará, como você se lembrará do Capítulo 16: a implementação de
canal que o Rust fornece é de múltiplos _produtores_, único _consumidor_
(*multiple producer, single consumer*). Isso significa que não podemos
simplesmente clonar a extremidade consumidora do canal para corrigir este código.
Também não queremos enviar uma mensagem várias vezes para vários consumidores;
queremos uma única lista de mensagens com múltiplas instâncias de `Worker`, de
modo que cada mensagem seja processada uma única vez.

Além disso, retirar um trabalho da fila do canal envolve mutar `receiver`, então
as *threads* precisam de uma maneira segura de compartilhar e modificar
`receiver`; caso contrário, poderíamos ter condições de corrida (*race
conditions*, como abordado no Capítulo 16).

Lembre-se dos *smart pointers* seguros para *threads* discutidos no Capítulo 16:
para compartilhar *ownership* entre múltiplas *threads* e permitir que as
*threads* mutem o valor, precisamos usar `Arc<Mutex<T>>`. O tipo `Arc` permitirá
que múltiplas instâncias de `Worker` possuam o receptor, e `Mutex` garantirá que
apenas um `Worker` obtenha um trabalho do receptor por vez. A Listagem 21-18
mostra as alterações que precisamos fazer.

<Listing number="21-18" file-name="src/lib.rs" caption="Compartilhando o receptor entre as instâncias de `Worker` usando `Arc` e `Mutex`">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-18/src/lib.rs:here}}
```

</Listing>

Em `ThreadPool::new`, colocamos o receptor em um `Arc` e um `Mutex`. Para cada
novo `Worker`, clonamos o `Arc` para incrementar a contagem de referências para
que as instâncias de `Worker` possam compartilhar o *ownership* do receptor.

Com essas alterações, o código compila! Estamos chegando lá!

<a id="implementing-the-execute-method"></a>
#### Implementando o Método `execute`

Vamos finalmente implementar o método `execute` em `ThreadPool`. Também
mudaremos `Job` de uma *struct* para um apelido de tipo (*type alias*) para um
*trait object* que guarda o tipo de *closure* que `execute` recebe. Conforme
discutido na seção [“Sinônimos de Tipos e Type Aliases”][type-aliases]<!--
ignore --> no Capítulo 20, *type aliases* nos permitem tornar tipos longos mais
curtos para facilitar o uso. Veja a Listagem 21-19.

<Listing number="21-19" file-name="src/lib.rs" caption="Criando um *type alias* `Job` para um `Box` que guarda cada *closure* e então enviando o trabalho pelo canal">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-19/src/lib.rs:here}}
```

</Listing>

Depois de criar uma nova instância de `Job` usando a *closure* que recebemos em
`execute`, enviamos esse trabalho pela extremidade transmissora do canal. Estamos
chamando `unwrap` em `send` para o caso de o envio falhar. Isso pode acontecer
se, por exemplo, pararmos todas as nossas *threads* de executar, o que significa
que a extremidade receptora parou de receber novas mensagens. No momento, não
podemos parar nossas *threads* de executar: nossas *threads* continuam
executando enquanto o pool existir. O motivo pelo qual usamos `unwrap` é que
sabemos que o caso de falha não acontecerá, mas o compilador não sabe disso.

Mas ainda não terminamos! No `Worker`, nossa *closure* sendo passada para
`thread::spawn` ainda apenas _referencia_ a extremidade receptora do canal. Em
vez disso, precisamos que a *closure* faça um loop infinito, pedindo à
extremidade receptora do canal um trabalho e executando o trabalho quando obtiver
um. Vamos fazer a alteração mostrada na Listagem 21-20 em `Worker::new`.

<Listing number="21-20" file-name="src/lib.rs" caption="Recebendo e executando os trabalhos na *thread* da instância de `Worker`">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-20/src/lib.rs:here}}
```

</Listing>

Aqui, primeiro chamamos `lock` em `receiver` para adquirir o mutex e, em
seguida, chamamos `unwrap` para entrar em pânico em caso de erros. Adquirir um
bloqueio (*lock*) pode falhar se o mutex estiver em um estado _envenenado_
(*poisoned*), o que pode acontecer se alguma outra *thread* entrou em pânico
enquanto segurava o bloqueio em vez de liberá-lo. Nessa situação, chamar `unwrap`
para fazer esta *thread* entrar em pânico é a ação correta a tomar. Sinta-se à
vontade para mudar esse `unwrap` para um `expect` com uma mensagem de erro que
seja significativa para você.

Se conseguirmos o bloqueio no mutex, chamamos `recv` para receber um `Job` do
canal. Um `unwrap` final também trata quaisquer erros aqui, os quais podem
ocorrer se a *thread* que detém o transmissor tiver sido encerrada, de forma
semelhante a como o método `send` retorna `Err` se o receptor for encerrado.

A chamada a `recv` bloqueia a execução, portanto, se ainda não houver nenhum
trabalho, a *thread* atual esperará até que um trabalho fique disponível. O
`Mutex<T>` garante que apenas uma *thread* de `Worker` por vez esteja tentando
requisitar um trabalho.

Nosso *thread pool* agora está funcionando! Rode um `cargo run` e faça algumas
requisições:

<!-- manual-regeneration
cd listings/ch21-web-server/listing-21-20
cargo run
make some requests to 127.0.0.1:7878
Can't automate because the output depends on making requests
-->

```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
warning: field `workers` is never read
 --> src/lib.rs:7:5
  |
6 | pub struct ThreadPool {
  |            ---------- field in this struct
7 |     workers: Vec<Worker>,
  |     ^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: fields `id` and `thread` are never read
  --> src/lib.rs:48:5
   |
47 | struct Worker {
   |        ------ fields in this struct
48 |     id: usize,
   |     ^^
49 |     thread: thread::JoinHandle<()>,
   |     ^^^^^^

warning: `hello` (lib) generated 2 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.91s
     Running `target/debug/hello`
Worker 0 got a job; executing.
Worker 2 got a job; executing.
Worker 1 got a job; executing.
Worker 3 got a job; executing.
Worker 0 got a job; executing.
Worker 2 got a job; executing.
Worker 1 got a job; executing.
Worker 3 got a job; executing.
Worker 0 got a job; executing.
Worker 2 got a job; executing.
```

Sucesso! Agora temos um *thread pool* que executa conexões assincronamente.
Nunca são criadas mais de quatro *threads*, então nosso sistema não ficará
sobrecarregado se o servidor receber muitas requisições. Se fizermos uma
requisição para _/sleep_, o servidor conseguirá atender outras requisições
fazendo outra *thread* executá-las.

> Note: Se você abrir _/sleep_ em várias janelas do navegador simultaneamente,
> elas podem carregar uma de cada vez em intervalos de cinco segundos. Alguns
> navegadores web executam múltiplas instâncias da mesma requisição
> sequencialmente por motivos de cache. Essa limitação não é causada pelo nosso
> servidor web.

Este é um bom momento para parar e refletir sobre como o código nas Listagens
21-18, 21-19 e 21-20 seria diferente se estivéssemos usando *futures* em vez de
uma *closure* para o trabalho a ser feito. Quais tipos mudariam? Como as
assinaturas dos métodos seriam diferentes, se é que seriam? Quais partes do
código permaneceriam iguais?

Depois de aprender sobre o loop `while let` no Capítulo 17 e no Capítulo 19,
você pode estar se perguntando por que não escrevemos o código da *thread* de
`Worker` como mostrado na Listagem 21-21.

<Listing number="21-21" file-name="src/lib.rs" caption="Uma implementação alternativa de `Worker::new` usando `while let`">

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch21-web-server/listing-21-21/src/lib.rs:here}}
```

</Listing>

Este código compila e roda, mas não resulta no comportamento de *threads*
desejado: uma requisição lenta ainda fará outras requisições esperarem para serem
processadas. O motivo é um tanto sutil: a *struct* `Mutex` não tem um método
público `unlock` porque o *ownership* do bloqueio é baseado no *lifetime* do
`MutexGuard<T>` dentro do `LockResult<MutexGuard<T>>` que o método `lock`
retorna. Em tempo de compilação, o *borrow checker* pode então impor a regra de
que um recurso protegido por um `Mutex` não pode ser acessado a menos que
detenhamos o bloqueio. No entanto, essa implementação também pode fazer com que o
bloqueio seja mantido por mais tempo do que o pretendido se não estivermos
atentos ao *lifetime* do `MutexGuard<T>`.

O código da Listagem 21-20 que usa `let job =
receiver.lock().unwrap().recv().unwrap();` funciona porque, com `let`, quaisquer
valores temporários usados na expressão do lado direito do sinal de igual são
imediatamente descartados (*dropped*) quando a instrução `let` termina. Contudo,
`while let` (assim como `if let` e `match`) não descarta valores temporários até
o final do bloco associado. Na Listagem 21-21, o bloqueio permanece mantido
durante toda a chamada a `job()`, o que significa que outras instâncias de
`Worker` não conseguem receber trabalhos.

[type-aliases]: ch20-03-advanced-types.html#type-synonyms-and-type-aliases
[integer-types]: ch03-02-data-types.html#integer-types
[moving-out-of-closures]: ch13-01-closures.html#moving-captured-values-out-of-closures
[builder]: ../std/thread/struct.Builder.html
[builder-spawn]: ../std/thread/struct.Builder.html#method.spawn
