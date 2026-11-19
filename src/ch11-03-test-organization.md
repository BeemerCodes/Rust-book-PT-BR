<a id="test-organization"></a>

## Organização dos Testes

Como mencionado no início do capítulo, testar é uma disciplina complexa, e pessoas diferentes usam terminologias e organizações diferentes. A comunidade Rust pensa sobre testes em termos de duas categorias principais: testes unitários e testes de integração. Os _testes unitários_ são pequenos e mais focados, testando um módulo isoladamente por vez, e podem testar interfaces privadas. Os _testes de integração_ são inteiramente externos à sua biblioteca e usam o seu código da mesma forma que qualquer outro código externo usaria, utilizando apenas a interface pública e potencialmente exercitando múltiplos módulos por teste.

Escrever ambos os tipos de testes é importante para garantir que as partes da sua biblioteca estejam fazendo o que você espera que façam, tanto separadamente quanto em conjunto.

<a id="unit-tests"></a>

### Testes Unitários

O objetivo dos testes unitários é testar cada unidade de código isoladamente do restante do código, para identificar rapidamente onde o código está e onde não está funcionando como esperado. Você colocará os testes unitários no diretório _src_, em cada arquivo com o código que eles estão testando. A convenção é criar um módulo chamado `tests` em cada arquivo para conter as funções de teste e anotar o módulo com `cfg(test)`.

<a id="the-tests-module-and-cfgtest"></a>

#### O Módulo `tests` e `#[cfg(test)]`

A anotação `#[cfg(test)]` no módulo `tests` diz ao Rust para compilar e rodar o código de teste apenas quando você executa `cargo test`, não quando você roda `cargo build`. Isso economiza tempo de compilação quando você só quer construir a biblioteca e economiza espaço no artefato compilado resultante, pois os testes não são incluídos. Você verá que, como os testes de integração ficam em um diretório diferente, eles não precisam da anotação `#[cfg(test)]`. No entanto, como os testes unitários ficam nos mesmos arquivos que o código, você usará `#[cfg(test)]` para especificar que eles não devem ser incluídos no resultado compilado.

Lembre-se de que quando geramos o novo projeto `adder` na primeira seção deste capítulo, o Cargo gerou este código para nós:

<span class="filename">Arquivo: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-01/src/lib.rs}}
```

No módulo `tests` gerado automaticamente, o atributo `cfg` significa _configuração_ (*configuration*) e informa ao Rust que o item a seguir só deve ser incluído de acordo com uma determinada opção de configuração. Neste caso, a opção de configuração é `test`, que é fornecida pelo Rust para compilar e executar testes. Ao usar o atributo `cfg`, o Cargo compila nosso código de teste apenas se executarmos ativamente os testes com `cargo test`. Isso inclui quaisquer funções auxiliares que possam estar dentro deste módulo, além das funções anotadas com `#[test]`.

<!-- Old headings. Do not remove or links may break. -->

<a id="testing-private-functions"></a>
<a id="private-function-tests"></a>

#### Testes de Funções Privadas

Existe um debate dentro da comunidade de testes sobre se funções privadas devem ou não ser testadas diretamente, e outras linguagens tornam difícil ou impossível testar funções privadas. Independentemente de qual ideologia de teste você siga, as regras de privacidade do Rust permitem sim que você teste funções privadas. Considere o código na Listagem 11-12 com a função privada `internal_adder`.

<Listing number="11-12" file-name="src/lib.rs" caption="Testando uma função privada">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-12/src/lib.rs}}
```

</Listing>

Observe que a função `internal_adder` não está marcada como `pub`. Testes são apenas código Rust, e o módulo `tests` é apenas mais um módulo. Como discutimos em [“Caminhos para Referenciar um Item na Árvore de Módulos”][paths]<!-- ignore -->, itens em módulos filhos podem usar os itens em seus módulos ancestrais. Neste teste, trazemos todos os itens pertencentes ao módulo pai do módulo `tests` para o escopo com `use super::*`, e então o teste pode chamar `internal_adder`. Se você acha que funções privadas não devem ser testadas, não há nada no Rust que o force a fazer isso.

<a id="integration-tests"></a>

### Testes de Integração

No Rust, os testes de integração são inteiramente externos à sua biblioteca. Eles usam sua biblioteca da mesma forma que qualquer outro código usaria, o que significa que eles só podem chamar funções que fazem parte da API pública da sua biblioteca. Seu objetivo é testar se muitas partes da sua biblioteca funcionam juntas corretamente. Unidades de código que funcionam corretamente por conta própria podem ter problemas quando integradas, por isso a cobertura de testes do código integrado também é importante. Para criar testes de integração, você precisa primeiro de um diretório _tests_.

<a id="the-tests-directory"></a>

#### O Diretório _tests_

Criamos um diretório _tests_ no nível superior do diretório do nosso projeto, ao lado de _src_. O Cargo sabe procurar arquivos de teste de integração nesse diretório. Podemos então criar quantos arquivos de teste quisermos, e o Cargo compilará cada um dos arquivos como uma *crate* individual.

Vamos criar um teste de integração. Com o código da Listagem 11-12 ainda no arquivo _src/lib.rs_, crie um diretório _tests_ e crie um novo arquivo chamado _tests/integration_test.rs_. Sua estrutura de diretórios deve ficar assim:

```text
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```

Insira o código da Listagem 11-13 no arquivo _tests/integration_test.rs_.

<Listing number="11-13" file-name="tests/integration_test.rs" caption="Um teste de integração de uma função na crate `adder`">

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-13/tests/integration_test.rs}}
```

</Listing>

Cada arquivo no diretório _tests_ é uma *crate* separada, portanto precisamos trazer nossa biblioteca para o escopo da *crate* de cada teste. Por esse motivo, adicionamos `use adder::add_two;` no topo do código, o que não precisávamos nos testes unitários.

Não precisamos anotar nenhum código em _tests/integration_test.rs_ com `#[cfg(test)]`. O Cargo trata o diretório _tests_ de forma especial e compila arquivos neste diretório apenas quando executamos `cargo test`. Execute `cargo test` agora:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-13/output.txt}}
```

As três seções de saída incluem os testes unitários, o teste de integração e os testes de documentação. Observe que se qualquer teste em uma seção falhar, as seções seguintes não serão executadas. Por exemplo, se um teste unitário falhar, não haverá nenhuma saída para testes de integração e testes de documentação, porque esses testes só serão executados se todos os testes unitários estiverem passando.

A primeira seção para os testes unitários é a mesma que vimos acompanhando: uma linha para cada teste unitário (um chamado `internal` que adicionamos na Listagem 11-12) e depois uma linha de resumo para os testes unitários.

A seção de testes de integração começa com a linha `Running tests/integration_test.rs`. Em seguida, há uma linha para cada função de teste naquele teste de integração e uma linha de resumo para os resultados do teste de integração logo antes de começar a seção `Doc-tests adder`.

Cada arquivo de teste de integração tem sua própria seção, portanto se adicionarmos mais arquivos no diretório _tests_, haverá mais seções de testes de integração.

Ainda podemos executar uma função de teste de integração específica informando o nome da função de teste como um argumento para `cargo test`. Para rodar todos os testes em um arquivo de teste de integração específico, use o argumento `--test` de `cargo test` seguido pelo nome do arquivo:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-05-single-integration/output.txt}}
```

Este comando roda apenas os testes no arquivo _tests/integration_test.rs_.

<a id="submodules-in-integration-tests"></a>

#### Submódulos em Testes de Integração

Conforme você adiciona mais testes de integração, pode querer criar mais arquivos no diretório _tests_ para ajudar a organizá-los; por exemplo, você pode agrupar as funções de teste pela funcionalidade que estão testando. Como mencionado anteriormente, cada arquivo no diretório _tests_ é compilado como sua própria *crate* separada, o que é útil para criar escopos separados para imitar mais de perto a maneira como os usuários finais usarão sua *crate*. No entanto, isso significa que arquivos no diretório _tests_ não compartilham o mesmo comportamento que arquivos em _src_, como você aprendeu no Capítulo 7 sobre como separar código em módulos e arquivos.

O comportamento diferente dos arquivos do diretório _tests_ é mais perceptível quando você tem um conjunto de funções auxiliares para usar em múltiplos arquivos de teste de integração e tenta seguir os passos da seção [“Separando Módulos em Arquivos Diferentes”][separating-modules-into-files]<!-- ignore --> do Capítulo 7 para extraí-los em um módulo comum. Por exemplo, se criarmos _tests/common.rs_ e colocarmos uma função chamada `setup` nele, podemos adicionar algum código a `setup` que queremos chamar a partir de múltiplas funções de teste em múltiplos arquivos de teste:

<span class="filename">Arquivo: tests/common.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-12-shared-test-code-problem/tests/common.rs}}
```

Quando rodamos os testes novamente, vemos uma nova seção na saída do teste para o arquivo _common.rs_, mesmo que esse arquivo não contenha nenhuma função de teste nem tenhamos chamado a função `setup` de nenhum lugar:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-12-shared-test-code-problem/output.txt}}
```

Ter `common` aparecendo nos resultados de teste com `running 0 tests` exibido para ele não é o que queríamos. Queríamos apenas compartilhar algum código com os outros arquivos de teste de integração. Para evitar que `common` apareça na saída dos testes, em vez de criar _tests/common.rs_, criaremos _tests/common/mod.rs_. O diretório do projeto agora fica assim:

```text
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs
```

Esta é a convenção de nomenclatura mais antiga que o Rust também compreende e que mencionamos em [“Caminhos de Arquivos Alternativos”][alt-paths]<!-- ignore --> no Capítulo 7. Nomear o arquivo dessa maneira diz ao Rust para não tratar o módulo `common` como um arquivo de teste de integração. Quando movemos o código da função `setup` para _tests/common/mod.rs_ e excluímos o arquivo _tests/common.rs_, a seção na saída do teste não aparecerá mais. Arquivos em subdiretórios do diretório _tests_ não são compilados como *crates* separadas nem têm seções na saída do teste.

Depois de criarmos _tests/common/mod.rs_, podemos usá-lo a partir de qualquer um dos arquivos de teste de integração como um módulo. Aqui está um exemplo de chamada da função `setup` a partir do teste `it_adds_two` em _tests/integration_test.rs_:

<span class="filename">Arquivo: tests/integration_test.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-13-fix-shared-test-code-problem/tests/integration_test.rs}}
```

Observe que a declaração `mod common;` é a mesma declaração de módulo que demonstramos na Listagem 7-21. Então, na função de teste, podemos chamar a função `common::setup()`.

<a id="integration-tests-for-binary-crates"></a>

#### Testes de Integração para Crates Binárias

Se nosso projeto for uma *crate* binária que contém apenas um arquivo _src/main.rs_ e não possui um arquivo _src/lib.rs_, não podemos criar testes de integração no diretório _tests_ e trazer funções definidas no arquivo _src/main.rs_ para o escopo com uma declaração `use`. Apenas *crates* de biblioteca expõem funções que outras *crates* podem usar; *crates* binárias foram feitas para serem executadas por conta própria.

Essa é uma das razões pelas quais projetos Rust que fornecem um binário têm um arquivo _src/main.rs_ direto e simples que chama a lógica que reside no arquivo _src/lib.rs_. Usando essa estrutura, os testes de integração _podem_ testar a *crate* de biblioteca com `use` para disponibilizar a funcionalidade importante. Se a funcionalidade importante funcionar, a pequena quantidade de código no arquivo _src/main.rs_ funcionará também, e essa pequena quantidade de código não precisa ser testada.

<a id="summary"></a>

## Resumo

Os recursos de teste do Rust fornecem uma maneira de especificar como o código deve funcionar para garantir que ele continue a funcionar como você espera, mesmo quando você faz alterações. Os testes unitários exercitam diferentes partes de uma biblioteca separadamente e podem testar detalhes de implementação privados. Os testes de integração verificam se muitas partes da biblioteca funcionam juntas corretamente e usam a API pública da biblioteca para testar o código da mesma maneira que o código externo o usará. Mesmo que o sistema de tipos do Rust e as regras de *ownership* ajudem a prevenir alguns tipos de bugs, os testes ainda são importantes para reduzir bugs de lógica relacionados a como o seu código deve se comportar.

Vamos combinar o conhecimento que você aprendeu neste capítulo e nos capítulos anteriores para trabalhar em um projeto!

[paths]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
[separating-modules-into-files]: ch07-05-separating-modules-into-different-files.html
[alt-paths]: ch07-05-separating-modules-into-different-files.html#alternate-file-paths
