<a id="accepting-command-line-arguments"></a>
## Aceitando Argumentos de Linha de Comando

Vamos criar um novo projeto com o comando de sempre, `cargo new`. Chamaremos nosso projeto de `minigrep` para diferenciá-lo da ferramenta `grep` que você talvez já tenha no seu sistema:

```console
$ cargo new minigrep
     Created binary (application) `minigrep` project
$ cd minigrep
```

A primeira tarefa é fazer o `minigrep` aceitar seus dois argumentos de linha de comando: o caminho do arquivo e a string a ser buscada. Ou seja, queremos conseguir executar nosso programa com `cargo run`, dois hifens para indicar que os argumentos seguintes são para o nosso programa e não para o `cargo`, a string de busca e o caminho do arquivo onde buscar, assim:

```console
$ cargo run -- searchstring example-filename.txt
```

No momento, o programa gerado pelo `cargo new` não consegue processar os argumentos que passamos para ele. Algumas bibliotecas existentes no [crates.io](https://crates.io/) podem ajudar a escrever um programa que aceita argumentos de linha de comando, mas como você está aprendendo esse conceito agora, vamos implementar essa capacidade por conta própria.

<a id="reading-the-argument-values"></a>
### Lendo os Valores dos Argumentos

Para permitir que o `minigrep` leia os valores dos argumentos de linha de comando que passamos para ele, precisaremos da função `std::env::args` fornecida pela biblioteca padrão do Rust. Essa função retorna um iterador dos argumentos de linha de comando passados para o `minigrep`. Abordaremos iteradores em detalhes no [Capítulo 13][ch13]<!-- ignore
-->. Por enquanto, você só precisa saber dois detalhes sobre iteradores: eles produzem uma série de valores, e podemos chamar o método `collect` em um iterador para transformá-lo em uma coleção, como um vetor, contendo todos os elementos produzidos pelo iterador.

O código na Listagem 12-1 permite que o programa `minigrep` leia quaisquer argumentos de linha de comando passados a ele e, em seguida, colete os valores em um vetor.

<Listing number="12-1" file-name="src/main.rs" caption="Coletando os argumentos de linha de comando em um vetor e imprimindo-os">

```rust
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-01/src/main.rs}}
```

</Listing>

Primeiro, trazemos o módulo `std::env` para o escopo com uma instrução `use` para podermos usar sua função `args`. Note que a função `std::env::args` está aninhada em dois níveis de módulos. Como discutimos no [Capítulo
7][ch7-idiomatic-use]<!-- ignore -->, nos casos em que a função desejada está aninhada em mais de um módulo, optamos por trazer o módulo pai para o escopo em vez da função. Fazendo isso, podemos usar facilmente outras funções de `std::env`. Também é menos ambíguo do que adicionar `use std::env::args` e depois chamar a função apenas como `args`, porque `args` poderia ser facilmente confundido com uma função definida no módulo atual.

<a id="the-args-function-and-invalid-unicode"></a>
> ### A Função `args` e Unicode Inválido
>
> Note que `std::env::args` entrará em *panic* se algum argumento contiver Unicode inválido. Se o seu programa precisar aceitar argumentos com Unicode inválido, use `std::env::args_os` em seu lugar. Essa função retorna um iterador que produz valores `OsString` em vez de valores `String`. Escolhemos usar `std::env::args` aqui por simplicidade, pois valores `OsString` variam conforme a plataforma e são mais complexos de manipular do que valores `String`.

Na primeira linha de `main`, chamamos `env::args` e imediatamente usamos `collect` para transformar o iterador em um vetor contendo todos os valores produzidos por ele. Podemos usar a função `collect` para criar vários tipos de coleções, então anotamos explicitamente o tipo de `args` para especificar que queremos um vetor de strings. Embora você raramente precise anotar tipos em Rust, `collect` é uma função em que frequentemente é necessário anotar porque o Rust não consegue inferir o tipo de coleção desejado.

Por fim, imprimimos o vetor usando a macro de depuração. Vamos tentar executar o código primeiro sem argumentos e depois com dois argumentos:

```console
{{#include ../listings/ch12-an-io-project/listing-12-01/output.txt}}
```

```console
{{#include ../listings/ch12-an-io-project/output-only-01-with-args/output.txt}}
```

Observe que o primeiro valor no vetor é `"target/debug/minigrep"`, que é o nome do nosso binário. Isso corresponde ao comportamento da lista de argumentos em C, permitindo que os programas usem o nome pelo qual foram invocados durante sua execução. Muitas vezes é conveniente ter acesso ao nome do programa caso você queira imprimi-lo em mensagens ou alterar o comportamento do programa com base no alias de linha de comando usado para invocá-lo. Porém, para os propósitos deste capítulo, vamos ignorá-lo e salvar apenas os dois argumentos de que precisamos.

<a id="saving-the-argument-values-in-variables"></a>
### Salvando os Valores dos Argumentos em Variáveis

O programa já consegue acessar os valores especificados como argumentos de linha de comando. Agora precisamos salvar os valores dos dois argumentos em variáveis para podermos usá-los ao longo do restante do programa. Fazemos isso na Listagem 12-2.

<Listing number="12-2" file-name="src/main.rs" caption="Criando variáveis para armazenar o argumento de consulta e o argumento de caminho do arquivo">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-02/src/main.rs}}
```

</Listing>

Como vimos quando imprimimos o vetor, o nome do programa ocupa o primeiro valor do vetor em `args[0]`, então começamos os argumentos no índice 1. O primeiro argumento que o `minigrep` recebe é a string que estamos buscando, então colocamos uma referência ao primeiro argumento na variável `query`. O segundo argumento será o caminho do arquivo, então colocamos uma referência ao segundo argumento na variável `file_path`.

Imprimimos temporariamente os valores dessas variáveis para comprovar que o código está funcionando como pretendemos. Vamos executar o programa novamente com os argumentos `test` e `sample.txt`:

```console
{{#include ../listings/ch12-an-io-project/listing-12-02/output.txt}}
```

Ótimo, o programa está funcionando! Os valores dos argumentos de que precisamos estão sendo salvos nas variáveis certas. Mais adiante adicionaremos tratamento de erros para lidar com certas situações errôneas em potencial, como quando o usuário não fornece argumentos; por enquanto, vamos ignorar essa situação e trabalhar na adição da funcionalidade de leitura de arquivos.

[ch13]: ch13-00-functional-features.html
[ch7-idiomatic-use]: ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#creating-idiomatic-use-paths
