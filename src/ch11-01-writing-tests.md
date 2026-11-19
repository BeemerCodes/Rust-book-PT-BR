<a id="how-to-write-tests"></a>

## Como Escrever Testes

_Testes_ são funções Rust que verificam se o código de produção (não de teste) está funcionando da maneira esperada. Os corpos das funções de teste normalmente realizam estas três ações:

- Configurar quaisquer dados ou estados necessários (*setup*).
- Executar o código que você quer testar.
- Assegurar (*assert*) que os resultados são o que você espera.

Vamos dar uma olhada nos recursos que o Rust oferece especificamente para escrever testes que realizam essas ações, incluindo o atributo `test`, algumas *macros* e o atributo `should_panic`.

<!-- Old headings. Do not remove or links may break. -->

<a id="the-anatomy-of-a-test-function"></a>
<a id="structuring-test-functions"></a>

### Estruturando Funções de Teste

Em sua forma mais simples, um teste em Rust é uma função anotada com o atributo `test`. Atributos são metadados sobre partes do código Rust; um exemplo é o atributo `derive` que usamos com *structs* no Capítulo 5. Para transformar uma função em uma função de teste, adicione `#[test]` na linha anterior a `fn`. Quando você executa seus testes com o comando `cargo test`, o Rust compila um binário executor de testes (*test runner*) que roda as funções anotadas e relata se cada função de teste passou ou falhou.

Sempre que criamos um novo projeto de biblioteca com o Cargo, um módulo de testes contendo uma função de teste é gerado automaticamente para nós. Esse módulo oferece um modelo (*template*) para escrever seus testes, evitando que você precise procurar a estrutura e a sintaxe exatas toda vez que iniciar um novo projeto. Você pode adicionar quantas funções e módulos de teste adicionais quiser!

Vamos explorar alguns aspectos de como os testes funcionam experimentando com esse teste modelo antes de testarmos qualquer código de fato. Em seguida, escreveremos alguns testes do mundo real que chamam códigos que escrevemos e verificam se o comportamento deles está correto.

Vamos criar um novo projeto de biblioteca chamado `adder` que somará dois números:

```console
$ cargo new adder --lib
     Created library `adder` project
$ cd adder
```

O conteúdo do arquivo _src/lib.rs_ na sua biblioteca `adder` deve ser semelhante à Listagem 11-1.

<Listing number="11-1" file-name="src/lib.rs" caption="O código gerado automaticamente pelo `cargo new`">

<!-- manual-regeneration
cd listings/ch11-writing-automated-tests
rm -rf listing-11-01
cargo new listing-11-01 --lib --name adder
cd listing-11-01
echo "$ cargo test" > output.txt
RUSTFLAGS="-A unused_variables -A dead_code" RUST_TEST_THREADS=1 cargo test >> output.txt 2>&1
git diff output.txt # commit any relevant changes; discard irrelevant ones
cd ../../..
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-01/src/lib.rs}}
```

</Listing>

O arquivo começa com um exemplo de função `add` para que tenhamos algo para testar.

Por enquanto, vamos nos concentrar unicamente na função `it_works`. Observe a anotação `#[test]`: este atributo indica que se trata de uma função de teste, para que o executor de testes saiba tratá-la como tal. Também podemos ter funções comuns (que não são testes) dentro do módulo `tests` para ajudar a configurar cenários comuns ou realizar operações compartilhadas, de modo que sempre precisamos indicar quais funções são testes de fato.

O corpo da função de exemplo usa a *macro* `assert_eq!` para assegurar que `result`, que contém o resultado da chamada de `add` com 2 e 2, seja igual a 4. Esta asserção serve como exemplo do formato de um teste típico. Vamos executá-lo para ver se esse teste passa.

O comando `cargo test` executa todos os testes do nosso projeto, conforme mostrado na Listagem 11-2.

<Listing number="11-2" caption="A saída da execução do teste gerado automaticamente">

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-01/output.txt}}
```

</Listing>

O Cargo compilou e executou o teste. Vemos a linha `running 1 test`. A linha seguinte mostra o nome da função de teste gerada, chamada `tests::it_works`, e que o resultado da execução desse teste foi `ok`. O resumo geral `test result: ok.` significa que todos os testes passaram, e a parte que diz `1 passed; 0 failed` totaliza o número de testes que passaram ou falharam.

É possível marcar um teste como ignorado para que ele não rode em uma execução específica; cobriremos isso na seção [“Ignorando Testes a Menos que Sejam Especificamente Solicitados”][ignoring]<!-- ignore --> mais adiante neste capítulo. Como não fizemos isso aqui, o resumo exibe `0 ignored`. Também podemos passar um argumento para o comando `cargo test` para rodar apenas os testes cujo nome corresponda a uma string; isso é chamado de _filtragem_, e nós o cobriremos na seção [“Executando um Subconjunto de Testes por Nome”][subset]<!-- ignore -->. Aqui, não filtramos os testes que estão sendo executados, portanto o final do resumo mostra `0 filtered out`.

A estatística `0 measured` é para testes de benchmark (*benchmark tests*) que medem desempenho. Testes de benchmark estão, no momento em que este livro foi escrito, disponíveis apenas no Rust nightly. Consulte [a documentação sobre testes de benchmark][bench] para saber mais.

A próxima parte da saída dos testes, começando em `Doc-tests adder`, é para os resultados de quaisquer testes de documentação (*documentation tests*). Ainda não temos nenhum teste de documentação, mas o Rust pode compilar quaisquer exemplos de código que apareçam na nossa documentação de API. Esse recurso ajuda a manter sua documentação e seu código sincronizados! Discutiremos como escrever testes de documentação na seção [“Comentários de Documentação como Testes”][doc-comments]<!-- ignore --> do Capítulo 14. Por enquanto, vamos ignorar a saída de `Doc-tests`.

Vamos começar a customizar o teste para as nossas próprias necessidades. Primeiro, mude o nome da função `it_works` para um nome diferente, como `exploration`, assim:

<span class="filename">Arquivo: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/src/lib.rs}}
```

Em seguida, execute `cargo test` novamente. A saída agora exibe `exploration` em vez de `it_works`:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/output.txt}}
```

Agora adicionaremos outro teste, mas desta vez faremos um teste que falha! Os testes falham quando algo na função de teste entra em pânico (*panic*). Cada teste é executado em uma nova *thread*, e quando a *thread* principal vê que uma *thread* de teste morreu, o teste é marcado como reprovado (*failed*). No Capítulo 9, falamos sobre como a maneira mais simples de causar um pânico é chamar a *macro* `panic!`. Insira o novo teste como uma função chamada `another`, para que seu arquivo _src/lib.rs_ fique parecido com a Listagem 11-3.

<Listing number="11-3" file-name="src/lib.rs" caption="Adicionando um segundo teste que falhará porque chamamos a macro `panic!`">

```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-03/src/lib.rs}}
```

</Listing>

Execute os testes novamente usando `cargo test`. A saída deve ser parecida com a Listagem 11-4, que mostra que nosso teste `exploration` passou e `another` falhou.

<Listing number="11-4" caption="Resultados dos testes quando um teste passa e outro falha">

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-03/output.txt}}
```

</Listing>

<!-- manual-regeneration
rg panicked listings/ch11-writing-automated-tests/listing-11-03/output.txt
check the line number of the panic matches the line number in the following paragraph
 -->

Em vez de `ok`, a linha `test tests::another` exibe `FAILED`. Duas novas seções aparecem entre os resultados individuais e o resumo: a primeira exibe o motivo detalhado de cada falha de teste. Neste caso, recebemos os detalhes de que `tests::another` falhou porque entrou em pânico com a mensagem `Make this test fail` na linha 17 do arquivo _src/lib.rs_. A seção seguinte lista apenas os nomes de todos os testes com falha, o que é útil quando há muitos testes e muita saída detalhada de testes reprovados. Podemos usar o nome de um teste com falha para rodar apenas esse teste e depurá-lo mais facilmente; falaremos mais sobre maneiras de executar testes na seção [“Controlando Como os Testes São Executados”][controlling-how-tests-are-run]<!-- ignore -->.

A linha de resumo é exibida no final: no geral, o resultado do nosso teste é `FAILED`. Tivemos um teste aprovado e um teste reprovado.

Agora que você viu como são os resultados de teste em diferentes cenários, vamos dar uma olhada em algumas *macros* além de `panic!` que são úteis em testes.

<!-- Old headings. Do not remove or links may break. -->

<a id="checking-results-with-the-assert-macro"></a>
<a id="checking-results-with-assert"></a>

### Checando Resultados com `assert!`

A *macro* `assert!`, fornecida pela biblioteca padrão, é útil quando você deseja garantir que alguma condição em um teste seja avaliada como `true`. Fornecemos à *macro* `assert!` um argumento que é avaliado como um booleano. Se o valor for `true`, nada acontece e o teste passa. Se o valor for `false`, a *macro* `assert!` chama `panic!` para fazer o teste falhar. O uso da *macro* `assert!` nos ajuda a verificar se o nosso código está funcionando da maneira que pretendemos.

No Capítulo 5, Listagem 5-15, usamos uma *struct* `Rectangle` e um método `can_hold`, que são repetidos aqui na Listagem 11-5. Vamos colocar esse código no arquivo _src/lib.rs_ e, em seguida, escrever alguns testes para ele usando a *macro* `assert!`.

<Listing number="11-5" file-name="src/lib.rs" caption="A struct `Rectangle` e seu método `can_hold` do Capítulo 5">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-05/src/lib.rs}}
```

</Listing>

O método `can_hold` retorna um booleano, o que significa que é um caso de uso perfeito para a *macro* `assert!`. Na Listagem 11-6, escrevemos um teste que exercita o método `can_hold` criando uma instância de `Rectangle` que tem largura 8 e altura 7, e assegurando que ela pode conter outra instância de `Rectangle` que tem largura 5 e altura 1.

<Listing number="11-6" file-name="src/lib.rs" caption="Um teste para `can_hold` que verifica se um retângulo maior pode, de fato, conter um retângulo menor">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-06/src/lib.rs:here}}
```

</Listing>

Observe a linha `use super::*;` dentro do módulo `tests`. O módulo `tests` é um módulo comum que segue as regras usuais de visibilidade que cobrimos no Capítulo 7 na seção [“Caminhos para Referenciar um Item na Árvore de Módulos”][paths-for-referring-to-an-item-in-the-module-tree]<!-- ignore -->. Como o módulo `tests` é um módulo interno, precisamos trazer o código sob teste do módulo externo para o escopo do módulo interno. Usamos um glob (`*`) aqui, de modo que tudo o que definirmos no módulo externo fique disponível para este módulo `tests`.

Nomeamos nosso teste como `larger_can_hold_smaller` e criamos as duas instâncias de `Rectangle` de que precisamos. Em seguida, chamamos a *macro* `assert!` e passamos para ela o resultado da chamada `larger.can_hold(&smaller)`. Essa expressão deve retornar `true`, portanto o nosso teste deve passar. Vamos descobrir!

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-06/output.txt}}
```

E ele passa! Vamos adicionar outro teste, desta vez assegurando que um retângulo menor não pode conter um retângulo maior:

<span class="filename">Arquivo: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/src/lib.rs:here}}
```

Como o resultado correto da função `can_hold` neste caso é `false`, precisamos negar esse resultado antes de passá-lo para a *macro* `assert!`. Como resultado, nosso teste passará se `can_hold` retornar `false`:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/output.txt}}
```

Dois testes aprovados! Agora vamos ver o que acontece com os resultados dos nossos testes quando introduzimos um bug em nosso código. Vamos alterar a implementação do método `can_hold` substituindo o sinal de maior que (`>`) por um sinal de menor que (`<`) ao comparar as larguras:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/src/lib.rs:here}}
```

Executar os testes agora produz o seguinte:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/output.txt}}
```

Nossos testes pegaram o bug! Como `larger.width` é `8` e `smaller.width` é `5`, a comparação das larguras em `can_hold` agora retorna `false`: 8 não é menor que 5.

<!-- Old headings. Do not remove or links may break. -->

<a id="testing-equality-with-the-assert_eq-and-assert_ne-macros"></a>
<a id="testing-equality-with-assert_eq-and-assert_ne"></a>

### Testando Igualdade com `assert_eq!` e `assert_ne!`

Uma forma comum de verificar a funcionalidade é testar a igualdade entre o resultado do código sob teste e o valor que você espera que o código retorne. Você poderia fazer isso usando a *macro* `assert!` e passando a ela uma expressão que utiliza o operador `==`. No entanto, esse é um teste tão comum que a biblioteca padrão fornece um par de *macros*—`assert_eq!` e `assert_ne!`—para realizar esse teste de maneira mais conveniente. Essas *macros* comparam dois argumentos quanto à igualdade ou desigualdade, respectivamente. Elas também imprimirão os dois valores se a asserção falhar, facilitando ver o _porquê_ de o teste ter falhado; por outro lado, a *macro* `assert!` apenas indica que obteve um valor `false` para a expressão `==`, sem imprimir os valores que levaram ao valor `false`.

Na Listagem 11-7, escrevemos uma função chamada `add_two` que adiciona `2` ao seu parâmetro e, em seguida, testamos essa função usando a *macro* `assert_eq!`.

<Listing number="11-7" file-name="src/lib.rs" caption="Testando a função `add_two` usando a macro `assert_eq!`">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-07/src/lib.rs}}
```

</Listing>

Vamos verificar se ela passa!

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-07/output.txt}}
```

Criamos uma variável chamada `result` que guarda o resultado da chamada `add_two(2)`. Depois, passamos `result` e `4` como argumentos para a *macro* `assert_eq!`. A linha de saída para esse teste é `test tests::it_adds_two ... ok`, e o texto `ok` indica que nosso teste passou!

Vamos introduzir um bug no nosso código para ver como fica o `assert_eq!` quando ele falha. Altere a implementação da função `add_two` para somar `3` em vez de 2:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/src/lib.rs:here}}
```

Execute os testes novamente:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/output.txt}}
```

Nosso teste pegou o bug! O teste `tests::it_adds_two` falhou, e a mensagem nos diz que a asserção que falhou foi `left == right` e quais são os valores de `left` e `right`. Essa mensagem nos ajuda a começar a depuração: o argumento `left`, onde tínhamos o resultado da chamada `add_two(2)`, era `5`, mas o argumento `right` era `4`. Você pode imaginar que isso seria especialmente útil quando temos muitos testes sendo executados.

Observe que, em algumas linguagens e frameworks de teste, os parâmetros para funções de asserção de igualdade são chamados de `expected` e `actual`, e a ordem em que especificamos os argumentos importa. No entanto, em Rust, eles são chamados de `left` e `right`, e a ordem na qual especificamos o valor esperado e o valor que o código produz não importa. Poderíamos escrever a asserção neste teste como `assert_eq!(4, result)`, o que resultaria na mesma mensagem de falha exibindo `` assertion `left == right` failed ``.

A *macro* `assert_ne!` passará se os dois valores passados não forem iguais e falhará se forem iguais. Esta *macro* é mais útil para os casos em que não temos certeza de qual valor _será_, mas sabemos com certeza qual valor _não deve_ ser. Por exemplo, se estivermos testando uma função que garantidamente modifica sua entrada de alguma forma, mas a maneira como a entrada é alterada depende do dia da semana em que rodamos nossos testes, a melhor coisa a assegurar pode ser que a saída da função não seja igual à entrada.

Por baixo dos panos, as *macros* `assert_eq!` e `assert_ne!` utilizam os operadores `==` e `!=`, respectivamente. Quando as asserções falham, essas *macros* imprimem seus argumentos usando formatação de depuração (*debug formatting*), o que significa que os valores sendo comparados devem implementar as *traits* `PartialEq` e `Debug`. Todos os tipos primitivos e a maioria dos tipos da biblioteca padrão implementam essas *traits*. Para *structs* e *enums* que você mesmo define, será necessário implementar `PartialEq` para assegurar a igualdade desses tipos. Você também precisará implementar `Debug` para imprimir os valores quando a asserção falhar. Como ambas são *traits* deriváveis, conforme mencionado na Listagem 5-12 do Capítulo 5, isso normalmente é tão direto quanto adicionar a anotação `#[derive(PartialEq, Debug)]` à definição da sua *struct* ou *enum*. Consulte o Apêndice C, [“Traits Deriváveis”][derivable-traits]<!-- ignore --> para mais detalhes sobre essas e outras *traits* deriváveis.

<a id="adding-custom-failure-messages"></a>

### Adicionando Mensagens de Falha Personalizadas

Você também pode adicionar uma mensagem personalizada para ser impressa com a mensagem de falha como argumentos opcionais para as *macros* `assert!`, `assert_eq!` e `assert_ne!`. Quaisquer argumentos especificados após os argumentos obrigatórios são repassados para a *macro* `format!` (discutida em [“Concatenando com `+` ou `format!`”][concatenating]<!-- ignore --> no Capítulo 8), para que você possa passar uma string de formatação contendo marcadores `{}` e valores para preencher esses marcadores. Mensagens personalizadas são úteis para documentar o significado de uma asserção; quando um teste falhar, você terá uma ideia melhor de qual é o problema com o código.

Por exemplo, digamos que temos uma função que cumprimenta as pessoas pelo nome e queremos testar se o nome que passamos para a função aparece na saída:

<span class="filename">Arquivo: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-05-greeter/src/lib.rs}}
```

Os requisitos para esse programa ainda não foram acordados e temos quase certeza de que o texto `Hello` no início da saudação vai mudar. Decidimos que não queremos ter que atualizar o teste quando os requisitos mudarem, então, em vez de verificar a igualdade exata com o valor retornado pela função `greeting`, vamos apenas assegurar que a saída contém o texto do parâmetro de entrada.

Agora vamos introduzir um bug nesse código alterando `greeting` para excluir `name`, a fim de ver como é a falha de teste padrão:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/src/lib.rs:here}}
```

A execução desse teste produz o seguinte:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/output.txt}}
```

Esse resultado apenas indica que a asserção falhou e em qual linha a asserção está. Uma mensagem de falha mais útil imprimiria o valor da função `greeting`. Vamos adicionar uma mensagem de falha personalizada composta por uma string de formatação com um marcador preenchido com o valor real obtido da função `greeting`:

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/src/lib.rs:here}}
```

Agora, quando executarmos o teste, obteremos uma mensagem de erro mais informativa:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/output.txt}}
```

Podemos ver o valor que realmente obtivemos na saída do teste, o que nos ajudará a depurar o que aconteceu em vez do que esperávamos que acontecesse.

<a id="checking-for-panics-with-should_panic"></a>

### Verificando Pânicos com `should_panic`

Além de verificar valores de retorno, é importante verificar se o nosso código trata condições de erro como esperamos. Por exemplo, considere o tipo `Guess` que criamos no Capítulo 9, Listagem 9-13. Outro código que usa `Guess` depende da garantia de que instâncias de `Guess` conterão apenas valores entre 1 e 100. Podemos escrever um teste que garanta que tentar criar uma instância de `Guess` com um valor fora desse intervalo entre em pânico.

Fazemos isso adicionando o atributo `should_panic` à nossa função de teste. O teste passa se o código dentro da função entrar em pânico; o teste falha se o código dentro da função não entrar em pânico.

A Listagem 11-8 mostra um teste que verifica se as condições de erro de `Guess::new` acontecem quando esperamos.

<Listing number="11-8" file-name="src/lib.rs" caption="Testando se uma condição causará um `panic!`">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-08/src/lib.rs}}
```

</Listing>

Colocamos o atributo `#[should_panic]` após o atributo `#[test]` e antes da função de teste à qual ele se aplica. Vamos dar uma olhada no resultado quando esse teste passa:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-08/output.txt}}
```

Parece ótimo! Agora vamos introduzir um bug em nosso código removendo a condição de que a função `new` entrará em pânico se o valor for maior que 100:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/src/lib.rs:here}}
```

Quando executamos o teste na Listagem 11-8, ele falhará:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/output.txt}}
```

Não recebemos uma mensagem muito útil neste caso, mas quando olhamos para a função de teste, vemos que ela está anotada com `#[should_panic]`. A falha que obtivemos significa que o código na função de teste não causou um pânico.

Testes que usam `should_panic` podem ser imprecisos. Um teste `should_panic` passaria mesmo se o teste entrasse em pânico por um motivo diferente daquele que esperávamos. Para tornar os testes `should_panic` mais precisos, podemos adicionar um parâmetro opcional `expected` ao atributo `should_panic`. O framework de teste garantirá que a mensagem de falha contenha o texto fornecido. Por exemplo, considere o código modificado para `Guess` na Listagem 11-9, onde a função `new` entra em pânico com mensagens diferentes dependendo de se o valor é muito pequeno ou muito grande.

<Listing number="11-9" file-name="src/lib.rs" caption="Testando para um `panic!` com uma mensagem de pânico contendo uma substring especificada">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-09/src/lib.rs:here}}
```

</Listing>

Este teste passará porque o valor colocado no parâmetro `expected` do atributo `should_panic` é uma substring da mensagem com a qual a função `Guess::new` entra em pânico. Poderíamos ter especificado a mensagem inteira de pânico esperada, que neste caso seria `Guess value must be less than or equal to 100, got 200`. O que você escolhe especificar depende de quanto da mensagem de pânico é único ou dinâmico e de quão preciso você quer que seu teste seja. Neste caso, uma substring da mensagem de pânico é suficiente para garantir que o código na função de teste execute o caso `else if value > 100`.

Para ver o que acontece quando um teste `should_panic` com uma mensagem `expected` falha, vamos introduzir novamente um bug no nosso código trocando os corpos dos blocos `if value < 1` e `else if value > 100`:

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/src/lib.rs:here}}
```

Desta vez, quando executarmos o teste `should_panic`, ele falhará:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/output.txt}}
```

A mensagem de falha indica que este teste de fato entrou em pânico como esperávamos, mas a mensagem de pânico não incluiu a string esperada `less than or equal to 100`. A mensagem de pânico que recebemos neste caso foi `Guess value must be greater than or equal to 1, got 200`. Agora podemos começar a descobrir onde está o nosso bug!

<a id="using-resultt-e-in-tests"></a>

### Usando `Result<T, E>` em Testes

Todos os nossos testes até agora entram em pânico quando falham. Também podemos escrever testes que usam `Result<T, E>`! Aqui está o teste da Listagem 11-1, reescrito para usar `Result<T, E>` e retornar um `Err` em vez de entrar em pânico:

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-10-result-in-tests/src/lib.rs:here}}
```

A função `it_works` agora tem o tipo de retorno `Result<(), String>`. No corpo da função, em vez de chamar a *macro* `assert_eq!`, retornamos `Ok(())` quando o teste passa e um `Err` contendo uma `String` quando o teste falha.

Escrever testes para que retornem um `Result<T, E>` permite que você use o operador de interrogação (`?`) no corpo dos testes, o que pode ser uma maneira conveniente de escrever testes que devem falhar se qualquer operação dentro deles retornar uma variante `Err`.

Você não pode usar a anotação `#[should_panic]` em testes que usam `Result<T, E>`. Para assegurar que uma operação retorna uma variante `Err`, _não_ use o operador de interrogação no valor `Result<T, E>`. Em vez disso, use `assert!(value.is_err())`.

Agora que você conhece várias maneiras de escrever testes, vamos ver o que está acontecendo quando executamos nossos testes e explorar as diferentes opções que podemos usar com `cargo test`.

[concatenating]: ch08-02-strings.html#concatenating-with--or-format
[bench]: ../unstable-book/library-features/test.html
[ignoring]: ch11-02-running-tests.html#ignoring-tests-unless-specifically-requested
[subset]: ch11-02-running-tests.html#running-a-subset-of-tests-by-name
[controlling-how-tests-are-run]: ch11-02-running-tests.html#controlling-how-tests-are-run
[derivable-traits]: appendix-03-derivable-traits.html
[doc-comments]: ch14-02-publishing-to-crates-io.html#documentation-comments-as-tests
[paths-for-referring-to-an-item-in-the-module-tree]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
