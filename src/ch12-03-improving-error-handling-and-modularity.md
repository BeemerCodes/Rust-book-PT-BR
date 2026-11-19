<a id="refactoring-to-improve-modularity-and-error-handling"></a>
## Refatorando para Melhorar a Modularidade e o Tratamento de Erros

Para melhorar o nosso programa, vamos corrigir quatro problemas relacionados à estrutura do programa e à forma como ele trata possíveis erros. Primeiro, nossa função `main` agora realiza duas tarefas: ela analisa os argumentos e lê arquivos. À medida que o programa cresce, o número de tarefas distintas que a função `main` precisa gerenciar vai aumentar. Quanto mais responsabilidades uma função acumula, mais difícil fica de raciocinar sobre ela, testá-la e modificá-la sem quebrar alguma parte. O ideal é separar as funcionalidades de forma que cada função seja responsável por uma única tarefa.

Esse problema também está ligado ao segundo: Embora `query` e `file_path` sejam variáveis de configuração do programa, variáveis como `contents` são usadas para executar a lógica do programa. Quanto mais `main` cresce, mais variáveis precisamos trazer para o escopo; quanto mais variáveis no escopo, mais difícil fica de acompanhar o propósito de cada uma. É melhor agrupar as variáveis de configuração em uma *struct* para deixar o propósito delas claro.

O terceiro problema é que usamos `expect` para imprimir uma mensagem de erro quando a leitura do arquivo falha, mas a mensagem apenas diz `Should have been able to read the file`. A leitura de um arquivo pode falhar de várias formas: por exemplo, o arquivo pode não existir, ou podemos não ter permissão para abrí-lo. Hoje, independente da situação, imprimimos a mesma mensagem de erro para tudo — o que não dá ao usuário nenhuma informação útil!

Quarto, usamos `expect` para tratar um erro e, se o usuário executar nosso programa sem especificar argumentos suficientes, ele receberá um erro `index out of bounds` do Rust que não explica claramente o problema. O ideal seria que todo o código de tratamento de erros ficasse em um único lugar, para que futuros mantenedores tivessem somente um local a consultar caso a lógica de tratamento de erros precisasse mudar. Centralizar o código de tratamento de erros também garante que estamos exibindo mensagens que fazem sentido para nossos usuários finais.

Vamos resolver esses quatro problemas refatorando o projeto.

<!-- Old headings. Do not remove or links may break. -->

<a id="separation-of-concerns-for-binary-projects"></a>

### Separando Responsabilidades em Projetos Binários

O problema organizacional de atribuir muitas responsabilidades à função `main` é comum em muitos projetos binários. Por isso, a comunidade Rust desenvolveu um conjunto de boas práticas para dividir as responsabilidades de um programa binário quando a função `main` começa a crescer. Esse processo envolve as seguintes etapas:

- Divida o programa em um arquivo _main.rs_ e um arquivo _lib.rs_, e mova a lógica do programa para _lib.rs_.
- Enquanto a lógica de análise de argumentos de linha de comando for simples, ela pode permanecer na função `main`.
- Quando a lógica de análise de argumentos de linha de comando começar a ficar complexa, extraia-a da função `main` para outras funções ou tipos.

As responsabilidades que devem permanecer na função `main` após esse processo são:

- Chamar a lógica de análise de argumentos de linha de comando com os valores dos argumentos
- Configurar qualquer outra configuração necessária
- Chamar uma função `run` em _lib.rs_
- Tratar o erro caso `run` retorne um erro

Esse padrão trata da separação de responsabilidades: _main.rs_ cuida da execução do programa e _lib.rs_ cuida de toda a lógica da tarefa em questão. Como você não pode testar a função `main` diretamente, essa estrutura permite que você teste toda a lógica do programa movendo-a para fora da função `main`. O código que fica em `main` será pequeno o suficiente para verificar sua corretude apenas lendo-o. Vamos reescrever nosso programa seguindo esse processo.

<a id="extracting-the-argument-parser"></a>
#### Extraindo o Analisador de Argumentos

Vamos extrair a funcionalidade de análise de argumentos para uma função que `main` irá chamar. A Listagem 12-5 mostra o novo início da função `main`, que chama uma nova função `parse_config` — que definiremos em _src/main.rs_.

<Listing number="12-5" file-name="src/main.rs" caption="Extraindo uma função `parse_config` de `main`">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-05/src/main.rs:here}}
```

</Listing>

Ainda estamos coletando os argumentos de linha de comando em um vetor, mas em vez de atribuir o valor do argumento no índice 1 à variável `query` e o valor no índice 2 à variável `file_path` dentro da função `main`, passamos o vetor inteiro para a função `parse_config`. A função `parse_config` contém a lógica que determina qual argumento vai para qual variável e retorna os valores para `main`. Ainda criamos as variáveis `query` e `file_path` em `main`, mas `main` não tem mais a responsabilidade de determinar como os argumentos de linha de comando e as variáveis se correspondem.

Essa refatoração pode parecer excessiva para um programa tão pequeno, mas estamos refatorando em pequenos passos incrementais. Após fazer essa mudança, execute o programa novamente para verificar que a análise de argumentos ainda funciona. É bom checar o progresso com frequência para identificar a causa dos problemas quando eles ocorrem.

<a id="grouping-configuration-values"></a>
#### Agrupando Valores de Configuração

Podemos dar mais um pequeno passo para melhorar ainda mais a função `parse_config`. Por enquanto, estamos retornando uma tupla, mas logo em seguida a desmontamos em partes individuais. Isso é um sinal de que talvez ainda não tenhamos a abstração certa.

Outro indicador de que há espaço para melhoria é a parte `config` do nome `parse_config`, que implica que os dois valores retornados são relacionados e fazem parte de um único valor de configuração. Atualmente não estamos transmitindo esse significado na estrutura dos dados — apenas agrupamos os dois valores em uma tupla. Em vez disso, vamos colocar os dois valores em uma *struct* e dar a cada campo da *struct* um nome significativo. Isso tornará mais fácil para futuros mantenedores entenderem como os diferentes valores se relacionam e qual é o propósito de cada um.

A Listagem 12-6 mostra as melhorias na função `parse_config`.

<Listing number="12-6" file-name="src/main.rs" caption="Refatorando `parse_config` para retornar uma instância de uma *struct* `Config`">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-06/src/main.rs:here}}
```

</Listing>

Adicionamos uma *struct* chamada `Config` com campos `query` e `file_path`. A assinatura de `parse_config` agora indica que ela retorna um valor `Config`. No corpo de `parse_config`, onde antes retornávamos *slices* de string que referenciavam valores `String` em `args`, agora definimos `Config` para conter valores `String` próprios. A variável `args` em `main` é a proprietária dos valores dos argumentos e está apenas permitindo que a função `parse_config` faça *borrowing* deles — o que significa que violaríamos as regras de *borrowing* do Rust se `Config` tentasse assumir a *ownership* dos valores em `args`.

Existem várias maneiras de gerenciar os dados `String`; a mais simples, embora um pouco ineficiente, é chamar o método `clone` nos valores. Isso fará uma cópia completa dos dados para a instância de `Config` possuir, o que consome mais tempo e memória do que armazenar uma referência aos dados da string. Porém, clonar os dados também torna nosso código mais simples, porque não precisamos gerenciar os *lifetimes* das referências; nessa circunstância, abrir mão de um pouco de desempenho para ganhar simplicidade é uma troca que vale a pena.

> ### As Compensações de Usar `clone`
>
> Muitos Rustaceans têm o hábito de evitar o uso de `clone` para resolver problemas de *ownership* por causa do seu custo em tempo de execução. No
> [Capítulo 13][ch13]<!-- ignore -->, você aprenderá a usar métodos mais eficientes nesse tipo de situação. Mas por enquanto, não tem problema copiar algumas strings para continuar avançando, porque você fará essas cópias apenas uma vez e o caminho do arquivo e a string de consulta são muito pequenos. É melhor ter um programa funcional que seja um pouco ineficiente do que tentar hiper-otimizar o código na primeira passagem. Com mais experiência em Rust, ficará mais fácil partir diretamente para a solução mais eficiente, mas por enquanto chamar `clone` é perfeitamente aceitável.

Atualizamos `main` para colocar a instância de `Config` retornada por `parse_config` em uma variável chamada `config`, e atualizamos o código que antes usava as variáveis separadas `query` e `file_path` para agora usar os campos da *struct* `Config`.

Agora nosso código transmite com mais clareza que `query` e `file_path` são relacionados e que seu propósito é configurar como o programa funcionará. Qualquer código que use esses valores sabe onde encontrá-los na instância `config`, nos campos com nomes que descrevem seu propósito.

<a id="creating-a-constructor-for-config"></a>
#### Criando um Construtor para `Config`

Até agora, extraímos a lógica responsável por analisar os argumentos de linha de comando de `main` e a colocamos na função `parse_config`. Isso nos ajudou a ver que os valores `query` e `file_path` são relacionados, e que essa relação deve ser expressa em nosso código. Em seguida, adicionamos uma *struct* `Config` para nomear o propósito relacionado de `query` e `file_path` e para poder retornar os nomes dos valores como nomes de campos da *struct* a partir da função `parse_config`.

Então, agora que o propósito da função `parse_config` é criar uma instância de `Config`, podemos mudar `parse_config` de uma função comum para uma função chamada `new` associada à *struct* `Config`. Fazer essa mudança tornará o código mais idiomático. Podemos criar instâncias de tipos da biblioteca padrão, como `String`, chamando `String::new`. Da mesma forma, ao mudar `parse_config` para uma função `new` associada a `Config`, poderemos criar instâncias de `Config` chamando `Config::new`. A Listagem 12-7 mostra as mudanças necessárias.

<Listing number="12-7" file-name="src/main.rs" caption="Transformando `parse_config` em `Config::new`">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-07/src/main.rs:here}}
```

</Listing>

Atualizamos `main` para chamar `Config::new` em vez de `parse_config`. Renomeamos `parse_config` para `new` e a movemos para dentro de um bloco `impl`, que associa a função `new` ao `Config`. Tente compilar esse código novamente para verificar que funciona.

<a id="fixing-the-error-handling"></a>
### Corrigindo o Tratamento de Erros

Agora vamos trabalhar na correção do nosso tratamento de erros. Lembre-se de que tentar acessar os valores no vetor `args` nos índices 1 ou 2 fará o programa entrar em pânico se o vetor contiver menos de três itens. Tente executar o programa sem nenhum argumento; ficará assim:

```console
{{#include ../listings/ch12-an-io-project/listing-12-07/output.txt}}
```

A linha `index out of bounds: the len is 1 but the index is 1` é uma mensagem de erro destinada a programadores. Ela não ajudará nossos usuários finais a entender o que devem fazer. Vamos corrigir isso agora.

<a id="improving-the-error-message"></a>
#### Melhorando a Mensagem de Erro

Na Listagem 12-8, adicionamos uma verificação na função `new` que checará se a *slice* é longa o suficiente antes de acessar os índices 1 e 2. Se a *slice* não for longa o suficiente, o programa entra em pânico e exibe uma mensagem de erro melhor.

<Listing number="12-8" file-name="src/main.rs" caption="Adicionando uma verificação para o número de argumentos">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-08/src/main.rs:here}}
```

</Listing>

Esse código é similar à [função `Guess::new` que escrevemos na Listagem
9-13][ch9-custom-types]<!-- ignore -->, onde chamávamos `panic!` quando o argumento `value` estava fora do intervalo de valores válidos. Em vez de verificar um intervalo de valores aqui, estamos verificando que o comprimento de `args` é pelo menos `3` e o resto da função pode operar sob o pressuposto de que essa condição foi atendida. Se `args` tiver menos de três itens, essa condição será `true`, e chamamos a *macro* `panic!` para encerrar o programa imediatamente.

Com essas poucas linhas extras de código em `new`, vamos executar o programa sem argumentos novamente para ver como o erro ficou agora:

```console
{{#include ../listings/ch12-an-io-project/listing-12-08/output.txt}}
```

Essa saída é melhor: agora temos uma mensagem de erro razoável. Porém, também temos informações desnecessárias que não queremos dar aos nossos usuários. Talvez a técnica que usamos na Listagem 9-13 não seja a melhor aqui: uma chamada a `panic!` é mais adequada para um problema de programação do que para um problema de uso, [como discutido no Capítulo 9][ch9-error-guidelines]<!-- ignore -->. Em vez disso, usaremos a outra técnica que você aprendeu no Capítulo 9 — [retornar um `Result`][ch9-result]<!-- ignore --> que indica sucesso ou erro.

<!-- Old headings. Do not remove or links may break. -->

<a id="returning-a-result-from-new-instead-of-calling-panic"></a>

#### Retornando um `Result` em Vez de Chamar `panic!`

Podemos, em vez disso, retornar um valor `Result` que conterá uma instância de `Config` em caso de sucesso e descreverá o problema em caso de erro. Também vamos mudar o nome da função de `new` para `build` porque muitos programadores esperam que funções `new` nunca falhem. Quando `Config::build` se comunica com `main`, podemos usar o tipo `Result` para sinalizar que houve um problema. Então podemos modificar `main` para converter um *variant* `Err` em um erro mais prático para nossos usuários, sem o texto ao redor sobre `thread 'main'` e `RUST_BACKTRACE` que uma chamada a `panic!` causa.

A Listagem 12-9 mostra as mudanças que precisamos fazer no valor de retorno da função que agora chamamos `Config::build` e no corpo da função para retornar um `Result`. Observe que isso não compilará até que também atualizemos `main`, o que faremos na próxima listagem.

<Listing number="12-9" file-name="src/main.rs" caption="Retornando um `Result` de `Config::build`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-09/src/main.rs:here}}
```

</Listing>

Nossa função `build` retorna um `Result` com uma instância de `Config` no caso de sucesso e um literal de string no caso de erro. Nossos valores de erro serão sempre literais de string com *lifetime* `'static`.

Fizemos duas mudanças no corpo da função: Em vez de chamar `panic!` quando o usuário não passa argumentos suficientes, agora retornamos um valor `Err`, e embrulhamos o valor de retorno `Config` em um `Ok`. Essas mudanças fazem a função se conformar com sua nova assinatura de tipo.

Retornar um valor `Err` de `Config::build` permite que a função `main` trate o valor `Result` retornado pela função `build` e encerre o processo de forma mais limpa no caso de erro.

<!-- Old headings. Do not remove or links may break. -->

<a id="calling-confignew-and-handling-errors"></a>

#### Chamando `Config::build` e Tratando Erros

Para tratar o caso de erro e imprimir uma mensagem amigável ao usuário, precisamos atualizar `main` para tratar o `Result` retornado por `Config::build`, como mostrado na Listagem 12-10. Também vamos assumir a responsabilidade de encerrar a ferramenta de linha de comando com um código de erro diferente de zero — que era tratado por `panic!` — e implementá-la manualmente. Um status de saída diferente de zero é uma convenção para sinalizar ao processo que chamou nosso programa que ele terminou com um estado de erro.

<Listing number="12-10" file-name="src/main.rs" caption="Encerrando com um código de erro se a criação de um `Config` falhar">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-10/src/main.rs:here}}
```

</Listing>

Nessa listagem, usamos um método que ainda não cobrimos em detalhes: `unwrap_or_else`, definido em `Result<T, E>` pela biblioteca padrão. Usar `unwrap_or_else` nos permite definir um tratamento de erro personalizado sem usar `panic!`. Se o `Result` for um valor `Ok`, o comportamento desse método é similar ao de `unwrap`: ele retorna o valor interno que `Ok` está embrulhando. Porém, se o valor for um `Err`, esse método chama o código na *closure* — uma função anônima que definimos e passamos como argumento para `unwrap_or_else`. Vamos cobrir *closures* em mais detalhes no [Capítulo 13][ch13]<!-- ignore -->. Por enquanto, apenas saiba que `unwrap_or_else` passará o valor interno de `Err` — que nesse caso é a string estática `"not enough arguments"` que adicionamos na Listagem 12-9 — para nossa *closure* no argumento `err` que aparece entre as barras verticais. O código na *closure* pode então usar o valor `err` quando executado.

Adicionamos uma nova linha `use` para trazer `process` da biblioteca padrão para o escopo. O código na *closure* que será executado no caso de erro tem apenas duas linhas: imprimimos o valor `err` e chamamos `process::exit`. A função `process::exit` encerrará o programa imediatamente e retornará o número passado como código de status de saída. Isso é similar ao tratamento baseado em `panic!` que usamos na Listagem 12-8, mas não recebemos mais toda a saída extra. Vamos testar:

```console
{{#include ../listings/ch12-an-io-project/listing-12-10/output.txt}}
```

Ótimo! Essa saída é muito mais amigável para nossos usuários.

<!-- Old headings. Do not remove or links may break. -->

<a id="extracting-logic-from-the-main-function"></a>

### Extraindo Lógica de `main`

Agora que terminamos de refatorar a análise de configuração, vamos cuidar da lógica do programa. Como afirmamos em ["Separando Responsabilidades em Projetos Binários"](#separation-of-concerns-for-binary-projects)<!-- ignore -->, vamos extrair uma função chamada `run` que conterá toda a lógica atualmente em `main` que não está relacionada com a configuração ou o tratamento de erros. Quando terminarmos, a função `main` será concisa e fácil de verificar por inspeção, e poderemos escrever testes para toda a outra lógica.

A Listagem 12-11 mostra a pequena melhoria incremental de extrair a função `run`.

<Listing number="12-11" file-name="src/main.rs" caption="Extraindo uma função `run` contendo o restante da lógica do programa">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-11/src/main.rs:here}}
```

</Listing>

A função `run` agora contém toda a lógica restante de `main`, começando pela leitura do arquivo. A função `run` recebe a instância de `Config` como argumento.

<!-- Old headings. Do not remove or links may break. -->

<a id="returning-errors-from-the-run-function"></a>

#### Retornando Erros da Função `run`

Com a lógica restante do programa separada na função `run`, podemos melhorar o tratamento de erros, como fizemos com `Config::build` na Listagem 12-9. Em vez de permitir que o programa entre em pânico chamando `expect`, a função `run` retornará um `Result<T, E>` quando algo der errado. Isso nos permitirá consolidar ainda mais a lógica de tratamento de erros em `main` de forma amigável ao usuário. A Listagem 12-12 mostra as mudanças que precisamos fazer na assinatura e no corpo de `run`.

<Listing number="12-12" file-name="src/main.rs" caption="Mudando a função `run` para retornar `Result`">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-12/src/main.rs:here}}
```

</Listing>

Fizemos três mudanças significativas aqui. Primeiro, mudamos o tipo de retorno da função `run` para `Result<(), Box<dyn Error>>`. Essa função antes retornava o tipo unidade `()`, e mantemos esse como o valor retornado no caso `Ok`.

Para o tipo de erro, usamos o objeto de *trait* `Box<dyn Error>` (e trouxemos `std::error::Error` para o escopo com uma instrução `use` no topo). Vamos cobrir objetos de *trait* no [Capítulo 18][ch18]<!-- ignore -->. Por enquanto, apenas saiba que `Box<dyn Error>` significa que a função retornará um tipo que implementa o *trait* `Error`, mas não precisamos especificar qual tipo particular o valor de retorno será. Isso nos dá flexibilidade para retornar valores de erro que podem ser de tipos diferentes em casos de erro diferentes. A palavra-chave `dyn` é uma abreviação de *dynamic* (dinâmico).

Segundo, removemos a chamada a `expect` em favor do operador `?`, como discutimos no [Capítulo 9][ch9-question-mark]<!-- ignore -->. Em vez de `panic!` em caso de erro, `?` retornará o valor de erro da função atual para o chamador tratar.

Terceiro, a função `run` agora retorna um valor `Ok` no caso de sucesso. Declaramos o tipo de sucesso da função `run` como `()` na assinatura, o que significa que precisamos embrulhar o valor do tipo unidade no valor `Ok`. Essa sintaxe `Ok(())` pode parecer um pouco estranha no início. Mas usar `()` dessa forma é a maneira idiomática de indicar que estamos chamando `run` apenas pelos seus efeitos colaterais; ela não retorna um valor que precisamos.

Ao executar esse código, ele compilará mas exibirá um aviso:

```console
{{#include ../listings/ch12-an-io-project/listing-12-12/output.txt}}
```

O Rust nos diz que nosso código ignorou o valor `Result` e que o valor `Result` pode indicar que ocorreu um erro. Mas não estamos verificando se houve ou não um erro, e o compilador nos lembra que provavelmente precisávamos ter algum código de tratamento de erros aqui! Vamos corrigir esse problema agora.

<a id="handling-errors-returned-from-run-in-main"></a>
#### Tratando Erros Retornados de `run` em `main`

Verificaremos os erros e os trataremos usando uma técnica similar à que usamos com `Config::build` na Listagem 12-10, mas com uma pequena diferença:

<span class="filename">Arquivo: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/no-listing-01-handling-errors-in-main/src/main.rs:here}}
```

Usamos `if let` em vez de `unwrap_or_else` para verificar se `run` retorna um valor `Err` e chamar `process::exit(1)` se isso acontecer. A função `run` não retorna um valor que queremos usar `unwrap` da mesma forma que `Config::build` retorna a instância de `Config`. Como `run` retorna `()` no caso de sucesso, só precisamos detectar um erro — não precisamos `unwrap` para obter o valor embrulhado, que seria apenas `()`.

Os corpos do `if let` e das funções `unwrap_or_else` são os mesmos nos dois casos: imprimimos o erro e encerramos.

<a id="splitting-code-into-a-library-crate"></a>
### Dividindo o Código em um *Crate* de Biblioteca

Nosso projeto `minigrep` está ficando bom até agora! Agora vamos dividir o arquivo _src/main.rs_ e colocar algum código no arquivo _src/lib.rs_. Dessa forma, podemos testar o código e ter um arquivo _src/main.rs_ com menos responsabilidades.

Vamos mover para _src/lib.rs_ o código responsável por buscar texto — em vez de deixá-lo em _src/main.rs_. Assim, nós (ou qualquer outro usuário da nossa biblioteca `minigrep`) podemos chamar a função de busca a partir de mais contextos do que apenas nosso binário `minigrep`.

Primeiro, vamos definir a assinatura da função `search` em _src/lib.rs_, como mostrado na Listagem 12-13, com um corpo que chama a *macro* `unimplemented!`. Explicaremos a assinatura em mais detalhes quando preenchermos a implementação.

<Listing number="12-13" file-name="src/lib.rs" caption="Definindo a função `search` em *src/lib.rs*">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-13/src/lib.rs}}
```

</Listing>

Usamos a palavra-chave `pub` na definição da função para designar `search` como parte da API pública do nosso *crate* de biblioteca. Agora temos um *crate* de biblioteca que podemos usar a partir do nosso *crate* binário e que podemos testar!

Agora precisamos trazer o código definido em _src/lib.rs_ para o escopo do *crate* binário em _src/main.rs_ e chamá-lo, como mostrado na Listagem 12-14.

<Listing number="12-14" file-name="src/main.rs" caption="Usando a função `search` do *crate* de biblioteca `minigrep` em *src/main.rs*">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-14/src/main.rs:here}}
```

</Listing>

Adicionamos uma linha `use minigrep::search` para trazer a função `search` do *crate* de biblioteca para o escopo do *crate* binário. Então, na função `run`, em vez de imprimir o conteúdo do arquivo, chamamos a função `search` e passamos o valor `config.query` e `contents` como argumentos. Em seguida, `run` usará um laço `for` para imprimir cada linha retornada por `search` que correspondeu à consulta. Este também é um bom momento para remover as chamadas `println!` em `main` que exibiam a consulta e o caminho do arquivo, para que nosso programa imprima apenas os resultados da busca (caso não haja erros).

Observe que a função `search` coletará todos os resultados em um vetor antes de qualquer impressão acontecer. Essa implementação pode ser lenta para exibir resultados ao buscar em arquivos grandes, pois os resultados não são impressos à medida que são encontrados; discutiremos uma possível forma de corrigir isso usando iteradores no Capítulo 13.

Ufa! Foi bastante trabalho, mas nos preparamos bem para o sucesso no futuro. Agora é muito mais fácil tratar erros, e tornamos o código mais modular. Quase todo o nosso trabalho daqui em diante será feito em _src/lib.rs_.

Vamos aproveitar essa nova modularidade fazendo algo que teria sido difícil com o código antigo, mas que é fácil com o novo: vamos escrever alguns testes!

[ch13]: ch13-00-functional-features.html
[ch9-custom-types]: ch09-03-to-panic-or-not-to-panic.html#creating-custom-types-for-validation
[ch9-error-guidelines]: ch09-03-to-panic-or-not-to-panic.html#guidelines-for-error-handling
[ch9-result]: ch09-02-recoverable-errors-with-result.html
[ch18]: ch18-00-oop.html
[ch9-question-mark]: ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator
