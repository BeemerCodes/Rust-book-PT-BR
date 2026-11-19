<a id="working-with-environment-variables"></a>
## Trabalhando com Variáveis de Ambiente

Vamos melhorar o binário `minigrep` adicionando um recurso extra: uma opção de busca sem distinção de maiúsculas e minúsculas que o usuário pode ativar por meio de uma variável de ambiente. Poderíamos tornar esse recurso uma opção de linha de comando e exigir que os usuários a informem toda vez que quiserem aplicá-la, mas ao transformá-lo em uma variável de ambiente, permitimos que nossos usuários definam a variável de ambiente uma vez e que todas as suas buscas sejam sem diferenciação de maiúsculas/minúsculas naquela sessão do terminal.

<!-- Old headings. Do not remove or links may break. -->
<a id="writing-a-failing-test-for-the-case-insensitive-search-function"></a>

### Escrevendo um Teste que Falha para a Busca sem Distinção de Maiúsculas/Minúsculas

Primeiro adicionamos uma nova função `search_case_insensitive` à biblioteca `minigrep` que será chamada quando a variável de ambiente tiver um valor. Continuaremos seguindo o processo TDD, então o primeiro passo é novamente escrever um teste que falha. Adicionaremos um novo teste para a nova função `search_case_insensitive` e renomearemos o teste antigo de `one_result` para `case_sensitive` para esclarecer as diferenças entre os dois testes, como mostrado na Listagem 12-20.

<Listing number="12-20" file-name="src/lib.rs" caption="Adicionando um novo teste que falha para a função sem distinção de maiúsculas/minúsculas que estamos prestes a adicionar">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-20/src/lib.rs:here}}
```

</Listing>

Observe que também editamos o `contents` do teste antigo. Adicionamos uma nova linha com o texto `"Duct tape."` usando um _D_ maiúsculo que não deve corresponder à consulta `"duct"` quando fazemos uma busca com diferenciação de maiúsculas/minúsculas. Mudar o teste antigo dessa forma ajuda a garantir que não quebremos acidentalmente a funcionalidade de busca com distinção de maiúsculas/minúsculas que já implementamos. Esse teste deve passar agora e deve continuar passando enquanto trabalhamos na busca sem distinção de maiúsculas/minúsculas.

O novo teste para a busca _sem_ distinção de maiúsculas/minúsculas usa `"rUsT"` como consulta. Na função `search_case_insensitive` que estamos prestes a adicionar, a consulta `"rUsT"` deve corresponder à linha contendo `"Rust:"` com um _R_ maiúsculo e à linha `"Trust me."` mesmo que ambas tenham capitalização diferente da consulta. Esse é nosso teste que falha, e ele falhará ao compilar porque ainda não definimos a função `search_case_insensitive`. Fique à vontade para adicionar uma implementação esqueleto que sempre retorna um vetor vazio, da mesma forma que fizemos para a função `search` na Listagem 12-16, para ver o teste compilar e falhar.

<a id="implementing-the-search_case_insensitive-function"></a>
### Implementando a Função `search_case_insensitive`

A função `search_case_insensitive`, mostrada na Listagem 12-21, será quase idêntica à função `search`. A única diferença é que converteremos `query` e cada `line` para minúsculas, de modo que, independentemente da capitalização dos argumentos de entrada, eles terão a mesma capitalização quando verificarmos se a linha contém a consulta.

<Listing number="12-21" file-name="src/lib.rs" caption="Definindo a função `search_case_insensitive` para converter a consulta e a linha para minúsculas antes de compará-las">

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-21/src/lib.rs:here}}
```

</Listing>

Primeiro, convertemos a string `query` para minúsculas e a armazenamos em uma nova variável com o mesmo nome, sombreando a `query` original. Chamar `to_lowercase` na consulta é necessário para que, independentemente de a consulta do usuário ser `"rust"`, `"RUST"`, `"Rust"` ou `"rUsT"`, tratemos a consulta como se fosse `"rust"` e sejamos insensíveis à capitalização. Embora `to_lowercase` lide com Unicode básico, não será 100% preciso. Se estivéssemos escrevendo uma aplicação real, precisaríamos fazer um pouco mais de trabalho aqui, mas esta seção é sobre variáveis de ambiente, não sobre Unicode, então deixaremos assim por ora.

Observe que `query` agora é uma `String` em vez de uma *slice* de string, porque chamar `to_lowercase` cria novos dados em vez de referenciar dados existentes. Digamos que a consulta seja `"rUsT"`, por exemplo: essa *slice* de string não contém um `u` ou `t` minúsculo para usarmos, então precisamos alocar uma nova `String` contendo `"rust"`. Quando passamos `query` como argumento para o método `contains` agora, precisamos adicionar um `&` porque a assinatura de `contains` é definida para receber uma *slice* de string.

Em seguida, adicionamos uma chamada a `to_lowercase` em cada `line` para converter todos os caracteres para minúsculas. Agora que convertemos `line` e `query` para minúsculas, encontraremos correspondências independentemente da capitalização da consulta.

Vamos ver se essa implementação passa nos testes:

```console
{{#include ../listings/ch12-an-io-project/listing-12-21/output.txt}}
```

Ótimo! Passaram. Agora vamos chamar a nova função `search_case_insensitive` a partir da função `run`. Primeiro, adicionaremos uma opção de configuração à *struct* `Config` para alternar entre busca com e sem distinção de maiúsculas/minúsculas. Adicionar esse campo causará erros de compilação porque não estamos inicializando esse campo em nenhum lugar ainda:

<span class="filename">Arquivo: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-22/src/main.rs:here}}
```

Adicionamos o campo `ignore_case` que armazena um booleano. Em seguida, precisamos que a função `run` verifique o valor do campo `ignore_case` e use isso para decidir se deve chamar a função `search` ou a função `search_case_insensitive`, como mostrado na Listagem 12-22. Isso ainda não compilará.

<Listing number="12-22" file-name="src/main.rs" caption="Chamando `search` ou `search_case_insensitive` com base no valor em `config.ignore_case`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-22/src/main.rs:there}}
```

</Listing>

Por fim, precisamos verificar a variável de ambiente. As funções para trabalhar com variáveis de ambiente estão no módulo `env` da biblioteca padrão, que já está no escopo no topo de _src/main.rs_. Usaremos a função `var` do módulo `env` para verificar se algum valor foi definido para uma variável de ambiente chamada `IGNORE_CASE`, como mostrado na Listagem 12-23.

<Listing number="12-23" file-name="src/main.rs" caption="Verificando se há algum valor em uma variável de ambiente chamada `IGNORE_CASE`">

```rust,ignore,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-23/src/main.rs:here}}
```

</Listing>

Aqui, criamos uma nova variável `ignore_case`. Para definir seu valor, chamamos a função `env::var` e passamos o nome da variável de ambiente `IGNORE_CASE`. A função `env::var` retorna um `Result` que será o *variant* `Ok` bem-sucedido contendo o valor da variável de ambiente se ela estiver definida com algum valor. Ela retornará o *variant* `Err` se a variável de ambiente não estiver definida.

Usamos o método `is_ok` no `Result` para verificar se a variável de ambiente está definida, o que significa que o programa deve fazer uma busca sem distinção de maiúsculas/minúsculas. Se a variável de ambiente `IGNORE_CASE` não estiver definida com nenhum valor, `is_ok` retornará `false` e o programa realizará uma busca com distinção de maiúsculas/minúsculas. Não nos importamos com o _valor_ da variável de ambiente, apenas se ela está definida ou não, então verificamos `is_ok` em vez de usar `unwrap`, `expect` ou qualquer outro método que vimos em `Result`.

Passamos o valor na variável `ignore_case` para a instância de `Config` para que a função `run` possa ler esse valor e decidir se deve chamar `search_case_insensitive` ou `search`, como implementamos na Listagem 12-22.

Vamos experimentar! Primeiro, executaremos nosso programa sem a variável de ambiente definida e com a consulta `to`, que deve corresponder a qualquer linha que contenha a palavra _to_ em letras minúsculas:

```console
{{#include ../listings/ch12-an-io-project/listing-12-23/output.txt}}
```

Parece que ainda funciona! Agora vamos executar o programa com `IGNORE_CASE` definido como `1` mas com a mesma consulta `to`:

```console
$ IGNORE_CASE=1 cargo run -- to poem.txt
```

Se você estiver usando o PowerShell, precisará definir a variável de ambiente e executar o programa como comandos separados:

```console
PS> $Env:IGNORE_CASE=1; cargo run -- to poem.txt
```

Isso fará com que `IGNORE_CASE` persista pelo restante da sua sessão do shell. Pode ser removido com o cmdlet `Remove-Item`:

```console
PS> Remove-Item Env:IGNORE_CASE
```

Devemos obter linhas que contêm _to_ que podem ter letras maiúsculas:

<!-- manual-regeneration
cd listings/ch12-an-io-project/listing-12-23
IGNORE_CASE=1 cargo run -- to poem.txt
can't extract because of the environment variable
-->

```console
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

Excelente, também obtivemos linhas contendo _To_! Nosso programa `minigrep` agora pode fazer buscas sem distinção de maiúsculas/minúsculas controladas por uma variável de ambiente. Agora você sabe como gerenciar opções definidas usando argumentos de linha de comando ou variáveis de ambiente.

Alguns programas permitem argumentos _e_ variáveis de ambiente para a mesma configuração. Nesses casos, os programas decidem que um ou outro tem prioridade. Para outro exercício por conta própria, tente controlar a sensibilidade de maiúsculas/minúsculas por meio de um argumento de linha de comando ou de uma variável de ambiente. Decida se o argumento de linha de comando ou a variável de ambiente deve ter prioridade se o programa for executado com um definido para sensível a maiúsculas/minúsculas e outro definido para ignorar.

O módulo `std::env` contém muitos outros recursos úteis para lidar com variáveis de ambiente: consulte a documentação para ver o que está disponível.
