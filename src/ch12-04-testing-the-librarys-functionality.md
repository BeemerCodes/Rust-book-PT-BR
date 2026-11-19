<!-- Old headings. Do not remove or links may break. -->
<a id="developing-the-librarys-functionality-with-test-driven-development"></a>

## Adicionando Funcionalidades com Desenvolvimento Orientado a Testes

Agora que temos a lógica de busca em _src/lib.rs_ separada da função `main`, é muito mais fácil escrever testes para a funcionalidade central do nosso código. Podemos chamar funções diretamente com vários argumentos e verificar os valores de retorno sem precisar chamar nosso binário a partir da linha de comando.

Nesta seção, adicionaremos a lógica de busca ao programa `minigrep` usando o processo de desenvolvimento orientado a testes (TDD — *Test-Driven Development*) com as seguintes etapas:

1. Escreva um teste que falha e execute-o para garantir que ele falha pelo motivo que você espera.
2. Escreva ou modifique apenas código suficiente para fazer o novo teste passar.
3. Refatore o código que você acabou de adicionar ou modificar e garanta que os testes continuem passando.
4. Repita a partir do passo 1!

Embora seja apenas uma das muitas formas de escrever software, o TDD pode ajudar a guiar o design do código. Escrever o teste antes de escrever o código que faz o teste passar ajuda a manter uma cobertura de testes elevada ao longo do processo.

Vamos guiar com testes a implementação da funcionalidade que realmente realizará a busca pela string de consulta no conteúdo do arquivo e produzirá uma lista de linhas que correspondem à consulta. Adicionaremos essa funcionalidade em uma função chamada `search`.

<a id="writing-a-failing-test"></a>
### Escrevendo um Teste que Falha

Em _src/lib.rs_, adicionaremos um módulo `tests` com uma função de teste, como fizemos no [Capítulo 11][ch11-anatomy]<!-- ignore -->. A função de teste especifica o comportamento que queremos que a função `search` tenha: ela receberá uma consulta e o texto a ser pesquisado, e retornará apenas as linhas do texto que contêm a consulta. A Listagem 12-15 mostra esse teste.

<Listing number="12-15" file-name="src/lib.rs" caption="Criando um teste que falha para a função `search` com a funcionalidade que desejamos ter">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-15/src/lib.rs:here}}
```

</Listing>

Esse teste busca pela string `"duct"`. O texto que estamos pesquisando tem três linhas, e apenas uma delas contém `"duct"` (observe que a barra invertida após a aspas de abertura diz ao Rust para não colocar um caractere de nova linha no início do conteúdo deste literal de string). Verificamos que o valor retornado pela função `search` contém apenas a linha que esperamos.

Se executarmos esse teste agora, ele falhará porque a *macro* `unimplemented!` entra em pânico com a mensagem "not implemented". De acordo com os princípios do TDD, daremos um pequeno passo para adicionar código suficiente para que o teste não entre em pânico ao chamar a função — definindo a função `search` para sempre retornar um vetor vazio, como mostrado na Listagem 12-16. Então, o teste deve compilar e falhar porque um vetor vazio não corresponde a um vetor contendo a linha `"safe, fast, productive."`.

<Listing number="12-16" file-name="src/lib.rs" caption="Definindo apenas o suficiente da função `search` para que chamá-la não cause pânico">

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-16/src/lib.rs:here}}
```

</Listing>

Agora vamos discutir por que precisamos definir um *lifetime* explícito `'a` na assinatura de `search` e usar esse *lifetime* com o argumento `contents` e o valor de retorno. Lembre-se do [Capítulo 10][ch10-lifetimes]<!-- ignore --> que os parâmetros de *lifetime* especificam qual *lifetime* de argumento está conectado ao *lifetime* do valor de retorno. Nesse caso, indicamos que o vetor retornado deve conter *slices* de string que referenciam *slices* do argumento `contents` (e não do argumento `query`).

Em outras palavras, dizemos ao Rust que os dados retornados pela função `search` viverão enquanto os dados passados para `search` no argumento `contents`. Isso é importante! Os dados referenciados *por* uma *slice* precisam ser válidos para que a referência seja válida; se o compilador assumir que estamos criando *slices* de string de `query` em vez de `contents`, ele fará a verificação de segurança incorretamente.

Se esquecermos as anotações de *lifetime* e tentarmos compilar essa função, obteremos este erro:

```console
{{#include ../listings/ch12-an-io-project/output-only-02-missing-lifetimes/output.txt}}
```

O Rust não consegue saber qual dos dois parâmetros precisamos para a saída, então precisamos informá-lo explicitamente. Observe que o texto de ajuda sugere especificar o mesmo parâmetro de *lifetime* para todos os parâmetros e o tipo de saída, o que está incorreto! Como `contents` é o parâmetro que contém todo o nosso texto e queremos retornar as partes desse texto que correspondem, sabemos que `contents` é o único parâmetro que deve ser conectado ao valor de retorno usando a sintaxe de *lifetime*.

Outras linguagens de programação não exigem que você conecte argumentos a valores de retorno na assinatura, mas essa prática ficará mais natural com o tempo. Você pode querer comparar este exemplo com os exemplos na seção ["Validando Referências com Lifetimes"][validating-references-with-lifetimes]<!-- ignore --> no Capítulo 10.

<a id="writing-code-to-pass-the-test"></a>
### Escrevendo Código para Passar no Teste

Atualmente, nosso teste está falhando porque sempre retornamos um vetor vazio. Para corrigir isso e implementar `search`, nosso programa precisa seguir estas etapas:

1. Iterar por cada linha do conteúdo.
2. Verificar se a linha contém a string de consulta.
3. Se contiver, adicioná-la à lista de valores que estamos retornando.
4. Se não contiver, não fazer nada.
5. Retornar a lista de resultados que correspondem.

Vamos trabalhar em cada etapa, começando pela iteração pelas linhas.

<a id="iterating-through-lines-with-the-lines-method"></a>
#### Iterando pelas Linhas com o Método `lines`

O Rust tem um método útil para lidar com a iteração linha por linha de strings, convenientemente chamado `lines`, que funciona como mostrado na Listagem 12-17. Observe que isso ainda não compilará.

<Listing number="12-17" file-name="src/lib.rs" caption="Iterando por cada linha em `contents`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-17/src/lib.rs:here}}
```

</Listing>

O método `lines` retorna um iterador. Falaremos sobre iteradores em profundidade no [Capítulo 13][ch13-iterators]<!-- ignore -->. Mas lembre-se de que você viu essa forma de usar um iterador na [Listagem 3-5][ch3-iter]<!-- ignore -->, onde usamos um laço `for` com um iterador para executar algum código em cada item de uma coleção.

<a id="searching-each-line-for-the-query"></a>
#### Buscando a Consulta em Cada Linha

A seguir, verificaremos se a linha atual contém a string de consulta. Felizmente, strings têm um método útil chamado `contains` que faz isso por nós! Adicione uma chamada ao método `contains` na função `search`, como mostrado na Listagem 12-18. Observe que isso ainda não compilará.

<Listing number="12-18" file-name="src/lib.rs" caption="Adicionando funcionalidade para verificar se a linha contém a string em `query`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-18/src/lib.rs:here}}
```

</Listing>

No momento, estamos construindo a funcionalidade. Para que o código compile, precisamos retornar um valor do corpo como indicamos na assinatura da função.

<a id="storing-matching-lines"></a>
#### Armazenando as Linhas Correspondentes

Para concluir essa função, precisamos de uma forma de armazenar as linhas correspondentes que queremos retornar. Para isso, podemos criar um vetor mutável antes do laço `for` e chamar o método `push` para armazenar uma `line` no vetor. Após o laço `for`, retornamos o vetor, como mostrado na Listagem 12-19.

<Listing number="12-19" file-name="src/lib.rs" caption="Armazenando as linhas que correspondem para podermos retorná-las">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:here}}
```

</Listing>

Agora a função `search` deve retornar apenas as linhas que contêm `query`, e nosso teste deve passar. Vamos executar o teste:

```console
{{#include ../listings/ch12-an-io-project/listing-12-19/output.txt}}
```

Nosso teste passou, então sabemos que funciona!

Neste ponto, poderíamos considerar oportunidades de refatorar a implementação da função de busca, mantendo os testes passando para preservar a mesma funcionalidade. O código na função de busca não está ruim, mas não aproveita alguns recursos úteis dos iteradores. Voltaremos a esse exemplo no [Capítulo 13][ch13-iterators]<!-- ignore -->, onde exploraremos iteradores em detalhes e veremos como melhorá-lo.

Agora o programa completo deve funcionar! Vamos testá-lo, primeiro com uma palavra que deve retornar exatamente uma linha do poema de Emily Dickinson: _frog_.

```console
{{#include ../listings/ch12-an-io-project/no-listing-02-using-search-in-run/output.txt}}
```

Que legal! Agora vamos tentar uma palavra que corresponda a várias linhas, como _body_:

```console
{{#include ../listings/ch12-an-io-project/output-only-03-multiple-matches/output.txt}}
```

E finalmente, vamos garantir que não obtemos nenhuma linha quando buscamos por uma palavra que não está em nenhum lugar do poema, como _monomorphization_:

```console
{{#include ../listings/ch12-an-io-project/output-only-04-no-matches/output.txt}}
```

Excelente! Construímos nossa própria versão mini de uma ferramenta clássica e aprendemos muito sobre como estruturar aplicações. Também aprendemos um pouco sobre entrada e saída de arquivos, *lifetimes*, testes e análise de argumentos de linha de comando.

Para completar este projeto, demonstraremos brevemente como trabalhar com variáveis de ambiente e como imprimir na saída de erro padrão — ambos úteis quando você está escrevendo programas de linha de comando.

[validating-references-with-lifetimes]: ch10-03-lifetime-syntax.html#validating-references-with-lifetimes
[ch11-anatomy]: ch11-01-writing-tests.html#the-anatomy-of-a-test-function
[ch10-lifetimes]: ch10-03-lifetime-syntax.html
[ch3-iter]: ch03-05-control-flow.html#looping-through-a-collection-with-for
[ch13-iterators]: ch13-02-iterators.html
