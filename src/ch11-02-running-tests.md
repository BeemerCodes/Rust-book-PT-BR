<a id="controlling-how-tests-are-run"></a>

## Controlando Como os Testes São Executados

Assim como `cargo run` compila o seu código e depois roda o binário resultante, `cargo test` compila o seu código em modo de teste e roda o binário de teste resultante. O comportamento padrão do binário produzido por `cargo test` é rodar todos os testes em paralelo e capturar a saída gerada durante a execução dos testes, impedindo que ela seja exibida e facilitando a leitura da saída relacionada aos resultados dos testes. No entanto, você pode especificar opções de linha de comando para alterar esse comportamento padrão.

Algumas opções de linha de comando vão para o `cargo test`, e algumas vão para o binário de teste resultante. Para separar esses dois tipos de argumentos, você lista os argumentos destinados ao `cargo test` seguidos pelo separador `--` e, em seguida, aqueles destinados ao binário de teste. Executar `cargo test --help` exibe as opções que você pode usar com `cargo test`, e executar `cargo test -- --help` exibe as opções que você pode usar após o separador. Essas opções também estão documentadas na [seção “Tests” do livro _The `rustc` Book_][tests].

[tests]: https://doc.rust-lang.org/rustc/tests/index.html

<a id="running-tests-in-parallel-or-consecutively"></a>

### Executando Testes em Paralelo ou Consecutivamente

Quando você executa múltiplos testes, por padrão eles rodam em paralelo usando *threads*, o que significa que eles terminam mais rápido e você obtém feedback mais cedo. Como os testes estão rodando ao mesmo tempo, você deve garantir que seus testes não dependam uns dos outros nem de qualquer estado compartilhado, incluindo um ambiente compartilhado, como o diretório de trabalho atual ou variáveis de ambiente.

Por exemplo, digamos que cada um dos seus testes execute algum código que cria um arquivo no disco chamado _test-output.txt_ e grava alguns dados nele. Em seguida, cada teste lê os dados desse arquivo e assegura que ele contenha um determinado valor, que é diferente em cada teste. Como os testes rodam ao mesmo tempo, um teste pode sobrescrever o arquivo no intervalo de tempo entre outro teste gravar e ler o arquivo. O segundo teste falhará, não porque o código está incorreto, mas porque os testes interferiram uns com os outros enquanto rodavam em paralelo. Uma solução é garantir que cada teste grave em um arquivo diferente; outra solução é rodar os testes um de cada vez.

Se você não quiser rodar os testes em paralelo ou se quiser um controle mais refinado sobre o número de *threads* utilizadas, você pode enviar a *flag* `--test-threads` e o número de *threads* que deseja usar para o binário de teste. Veja o seguinte exemplo:

```console
$ cargo test -- --test-threads=1
```

Definimos o número de *threads* de teste para `1`, dizendo ao programa para não usar nenhum paralelismo. Executar os testes usando uma única *thread* levará mais tempo do que executá-los em paralelo, mas os testes não interferirão uns com os outros caso compartilhem estado.

<a id="showing-function-output"></a>

### Exibindo a Saída de Funções

Por padrão, se um teste passar, a biblioteca de testes do Rust captura qualquer coisa impressa na saída padrão. Por exemplo, se chamarmos `println!` em um teste e o teste passar, não veremos a saída de `println!` no terminal; veremos apenas a linha que indica que o teste passou. Se um teste falhar, veremos o que quer que tenha sido impresso na saída padrão junto com o restante da mensagem de falha.

Como exemplo, a Listagem 11-10 possui uma função boba que imprime o valor do seu parâmetro e retorna 10, bem como um teste que passa e um teste que falha.

<Listing number="11-10" file-name="src/lib.rs" caption="Testes para uma função que chama `println!`">

```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-10/src/lib.rs}}
```

</Listing>

Quando executamos esses testes com `cargo test`, vemos a seguinte saída:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-10/output.txt}}
```

Observe que em nenhum lugar dessa saída vemos `I got the value 4`, que é impresso quando o teste que passa executa. Essa saída foi capturada. A saída do teste que falhou, `I got the value 8`, aparece na seção do resumo de saída dos testes, que também mostra a causa da falha do teste.

Se quisermos ver os valores impressos também para testes que passam, podemos dizer ao Rust para exibir a saída de testes bem-sucedidos com `--show-output`:

```console
$ cargo test -- --show-output
```

Quando executamos os testes na Listagem 11-10 novamente com a *flag* `--show-output`, vemos a seguinte saída:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-01-show-output/output.txt}}
```

<a id="running-a-subset-of-tests-by-name"></a>

### Executando um Subconjunto de Testes por Nome

Executar uma suíte de testes inteira pode às vezes demorar bastante. Se você estiver trabalhando no código de uma área específica, pode querer rodar apenas os testes referentes àquele código. Você pode escolher quais testes rodar passando para o `cargo test` o nome ou os nomes do(s) teste(s) que deseja executar como argumento.

Para demonstrar como executar um subconjunto de testes, primeiro criaremos três testes para a nossa função `add_two`, como mostrado na Listagem 11-11, e escolheremos quais deles executar.

<Listing number="11-11" file-name="src/lib.rs" caption="Três testes com três nomes diferentes">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-11/src/lib.rs}}
```

</Listing>

Se executarmos os testes sem passar nenhum argumento, como vimos antes, todos os testes rodarão em paralelo:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-11/output.txt}}
```

<a id="running-single-tests"></a>

#### Executando Testes Individuais

Podemos passar o nome de qualquer função de teste para `cargo test` para rodar apenas aquele teste:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-02-single-test/output.txt}}
```

Apenas o teste com o nome `one_hundred` rodou; os outros dois testes não corresponderam a esse nome. A saída do teste nos informa que tivemos mais testes que não foram executados exibindo `2 filtered out` no final.

Não podemos especificar os nomes de múltiplos testes dessa maneira; apenas o primeiro valor fornecido ao `cargo test` será usado. Mas existe uma maneira de rodar múltiplos testes.

<a id="filtering-to-run-multiple-tests"></a>

#### Filtrando para Executar Múltiplos Testes

Podemos especificar parte de um nome de teste, e qualquer teste cujo nome corresponda a esse valor será executado. Por exemplo, como dois dos nomes dos nossos testes contêm `add`, podemos rodar esses dois executando `cargo test add`:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-03-multiple-tests/output.txt}}
```

Este comando executou todos os testes com `add` no nome e filtrou o teste chamado `one_hundred`. Observe também que o módulo em que um teste aparece se torna parte do nome do teste, portanto podemos rodar todos os testes em um módulo filtrando pelo nome do módulo.

<!-- Old headings. Do not remove or links may break. -->

<a id="ignoring-some-tests-unless-specifically-requested"></a>
<a id="ignoring-tests-unless-specifically-requested"></a>

### Ignorando Testes a Menos que Sejam Especificamente Solicitados

Às vezes, alguns testes específicos podem ser muito demorados para serem executados, portanto você pode querer excluí-los durante a maioria das execuções do `cargo test`. Em vez de listar como argumentos todos os testes que você de fato deseja rodar, você pode, em vez disso, anotar os testes demorados usando o atributo `ignore` para excluí-los, como mostrado aqui:

<span class="filename">Arquivo: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-11-ignore-a-test/src/lib.rs:here}}
```

Após `#[test]`, adicionamos a linha `#[ignore]` ao teste que queremos excluir. Agora, quando rodamos nossos testes, `it_works` é executado, mas `expensive_test` não:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-11-ignore-a-test/output.txt}}
```

A função `expensive_test` é listada como `ignored`. Se quisermos executar apenas os testes ignorados, podemos usar `cargo test -- --ignored`:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-04-running-ignored/output.txt}}
```

Ao controlar quais testes são executados, você pode garantir que os resultados do seu `cargo test` retornem rapidamente. Quando você estiver em um ponto onde faça sentido verificar os resultados dos testes `ignored` e tiver tempo para esperar pelos resultados, poderá executar `cargo test -- --ignored`. Se quiser rodar todos os testes, estejam eles ignorados ou não, você pode executar `cargo test -- --include-ignored`.
