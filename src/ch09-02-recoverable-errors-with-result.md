<a id="recoverable-errors-with-result"></a>

## Erros Recuperáveis com `Result`

A maioria dos erros não é grave o suficiente para exigir que o programa pare completamente. Às vezes, quando uma função falha, é por um motivo que você pode facilmente interpretar e contornar. Por exemplo, se você tentar abrir um arquivo e essa operação falhar porque o arquivo não existe, talvez você queira criar o arquivo em vez de encerrar o processo.

Lembre-se de ["Lidando com Falhas Potenciais com `Result`"][handle_failure]<!-- ignore --> no Capítulo 2 que o *enum* `Result` é definido com duas variantes, `Ok` e `Err`, da seguinte forma:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Os parâmetros `T` e `E` são parâmetros de tipo genérico: discutiremos os genéricos com mais detalhes no Capítulo 10. O que você precisa saber agora é que `T` representa o tipo do valor que será retornado em caso de sucesso dentro da variante `Ok`, e `E` representa o tipo do erro que será retornado em caso de falha dentro da variante `Err`. Como `Result` tem esses parâmetros de tipo genérico, podemos usar o tipo `Result` e as funções definidas sobre ele em muitas situações diferentes, onde o valor de sucesso e o valor de erro que queremos retornar podem variar.

Vamos chamar uma função que retorna um valor `Result` porque ela pode falhar. Na Listagem 9-3, tentamos abrir um arquivo.

<Listing number="9-3" file-name="src/main.rs" caption="Abrindo um arquivo">

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-03/src/main.rs}}
```

</Listing>

O tipo de retorno de `File::open` é `Result<T, E>`. O parâmetro genérico `T` foi preenchido pela implementação de `File::open` com o tipo do valor de sucesso, `std::fs::File`, que é um *handle* de arquivo. O tipo de `E` usado no valor de erro é `std::io::Error`. Esse tipo de retorno significa que a chamada a `File::open` pode ter sucesso e retornar um *handle* de arquivo que podemos usar para leitura ou escrita. A chamada à função também pode falhar: por exemplo, o arquivo pode não existir, ou pode não termos permissão para acessá-lo. A função `File::open` precisa de uma maneira de nos informar se teve sucesso ou falhou e, ao mesmo tempo, nos fornecer o *handle* do arquivo ou as informações do erro. Essas informações são exatamente o que o *enum* `Result` transmite.

No caso em que `File::open` tem sucesso, o valor na variável `greeting_file_result` será uma instância de `Ok` contendo um *handle* de arquivo. No caso em que falha, o valor em `greeting_file_result` será uma instância de `Err` contendo mais informações sobre o tipo de erro ocorrido.

Precisamos adicionar ao código da Listagem 9-3 diferentes ações a tomar dependendo do valor que `File::open` retorna. A Listagem 9-4 mostra uma maneira de tratar o `Result` usando uma ferramenta básica, a expressão *`match`* que discutimos no Capítulo 6.

<Listing number="9-4" file-name="src/main.rs" caption="Usando uma expressão `match` para tratar as variantes de `Result` que podem ser retornadas">

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-04/src/main.rs}}
```

</Listing>

Note que, assim como o *enum* `Option`, o *enum* `Result` e suas variantes já são importados pelo prelúdio, portanto não precisamos especificar `Result::` antes das variantes `Ok` e `Err` nos braços do *match*.

Quando o resultado é `Ok`, esse código retornará o valor interno `file` de dentro da variante `Ok`, e então atribuímos esse valor de *handle* de arquivo à variável `greeting_file`. Após o *match*, podemos usar o *handle* de arquivo para leitura ou escrita.

O outro braço do *match* trata o caso em que obtemos um valor `Err` de `File::open`. Neste exemplo, optamos por chamar a *macro* `panic!`. Se não houver nenhum arquivo chamado _hello.txt_ no nosso diretório atual e executarmos esse código, veremos a seguinte saída da *macro* `panic!`:

```console
{{#include ../listings/ch09-error-handling/listing-09-04/output.txt}}
```

Como de costume, essa saída nos diz exatamente o que deu errado.

<a id="matching-on-different-errors"></a>

### Tratando Diferentes Tipos de Erros

O código da Listagem 9-4 chamará `panic!` não importa o motivo pelo qual `File::open` falhou. No entanto, queremos tomar ações diferentes para diferentes motivos de falha. Se `File::open` falhou porque o arquivo não existe, queremos criar o arquivo e retornar o *handle* do novo arquivo. Se `File::open` falhou por qualquer outro motivo — por exemplo, porque não temos permissão para abri-lo — ainda queremos que o código entre em `panic!` da mesma forma que na Listagem 9-4. Para isso, adicionamos uma expressão *match* interna, mostrada na Listagem 9-5.

<Listing number="9-5" file-name="src/main.rs" caption="Tratando diferentes tipos de erros de formas diferentes">

<!-- ignore this test because otherwise it creates hello.txt which causes other
tests to fail lol -->

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-05/src/main.rs}}
```

</Listing>

O tipo do valor que `File::open` retorna dentro da variante `Err` é `io::Error`, que é uma *struct* fornecida pela biblioteca padrão. Essa *struct* tem um método, `kind`, que podemos chamar para obter um valor do tipo `io::ErrorKind`. O *enum* `io::ErrorKind` é fornecido pela biblioteca padrão e tem variantes representando os diferentes tipos de erros que podem resultar de uma operação `io`. A variante que queremos usar é `ErrorKind::NotFound`, que indica que o arquivo que estamos tentando abrir ainda não existe. Então fazemos *match* em `greeting_file_result`, mas também temos um *match* interno em `error.kind()`.

A condição que queremos verificar no *match* interno é se o valor retornado por `error.kind()` é a variante `NotFound` do *enum* `ErrorKind`. Se for, tentamos criar o arquivo com `File::create`. No entanto, como `File::create` também pode falhar, precisamos de um segundo braço na expressão *match* interna. Quando o arquivo não pode ser criado, uma mensagem de erro diferente é exibida. O segundo braço do *match* externo permanece igual, portanto o programa entra em pânico para qualquer erro além do erro de arquivo inexistente.

> #### Alternativas ao Uso de `match` com `Result<T, E>`
>
> São muitos *matches*! A expressão *match* é muito útil mas também bastante primitiva. No Capítulo 13, você aprenderá sobre *closures*, que são usadas com muitos dos métodos definidos em `Result<T, E>`. Esses métodos podem ser mais concisos do que usar *match* ao tratar valores `Result<T, E>` no seu código.
>
> Por exemplo, aqui está outra maneira de escrever a mesma lógica da Listagem 9-5, desta vez usando *closures* e o método `unwrap_or_else`:
>
> <!-- CAN'T EXTRACT SEE https://github.com/rust-lang/mdBook/issues/1127 -->
>
> ```rust,ignore
> use std::fs::File;
> use std::io::ErrorKind;
>
> fn main() {
>     let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
>         if error.kind() == ErrorKind::NotFound {
>             File::create("hello.txt").unwrap_or_else(|error| {
>                 panic!("Problem creating the file: {error:?}");
>             })
>         } else {
>             panic!("Problem opening the file: {error:?}");
>         }
>     });
> }
> ```
>
> Embora esse código tenha o mesmo comportamento que a Listagem 9-5, ele não contém nenhuma expressão *match* e é mais limpo de ler. Volte a este exemplo depois de ler o Capítulo 13 e consulte o método `unwrap_or_else` na documentação da biblioteca padrão. Muitos outros desses métodos podem simplificar expressões *match* grandes e aninhadas quando você está lidando com erros.

<!-- Old headings. Do not remove or links may break. -->

<a id="shortcuts-for-panic-on-error-unwrap-and-expect"></a>

#### Atalhos para Pânico em Caso de Erro

Usar *match* funciona bem, mas pode ser um pouco verboso e nem sempre comunica a intenção com clareza. O tipo `Result<T, E>` tem muitos métodos auxiliares definidos para realizar tarefas diversas e mais específicas. O método `unwrap` é um método de atalho implementado de forma parecida com a expressão *match* que escrevemos na Listagem 9-4. Se o valor de `Result` for a variante `Ok`, `unwrap` retornará o valor dentro de `Ok`. Se o `Result` for a variante `Err`, `unwrap` chamará a *macro* `panic!` para nós. Veja um exemplo de `unwrap` em ação:

<Listing file-name="src/main.rs">

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-04-unwrap/src/main.rs}}
```

</Listing>

Se executarmos esse código sem um arquivo _hello.txt_, veremos uma mensagem de erro da chamada a `panic!` que o método `unwrap` faz:

<!-- manual-regeneration
cd listings/ch09-error-handling/no-listing-04-unwrap
cargo run
copy and paste relevant text
-->

```text
thread 'main' panicked at src/main.rs:4:49:
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

Da mesma forma, o método `expect` nos permite escolher a mensagem de erro do `panic!`. Usar `expect` em vez de `unwrap` e fornecer boas mensagens de erro pode transmitir sua intenção e facilitar o rastreamento da origem de um pânico. A sintaxe de `expect` é assim:

<Listing file-name="src/main.rs">

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-05-expect/src/main.rs}}
```

</Listing>

Usamos `expect` da mesma forma que `unwrap`: para retornar o *handle* de arquivo ou chamar a *macro* `panic!`. A mensagem de erro usada por `expect` em sua chamada a `panic!` será o parâmetro que passamos para `expect`, em vez da mensagem padrão do `panic!` que `unwrap` usa. Veja como fica:

<!-- manual-regeneration
cd listings/ch09-error-handling/no-listing-05-expect
cargo run
copy and paste relevant text
-->

```text
thread 'main' panicked at src/main.rs:5:10:
hello.txt should be included in this project: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

Em código de qualidade de produção, a maioria dos Rustaceans escolhe `expect` em vez de `unwrap` e fornece mais contexto sobre por que se espera que a operação sempre tenha sucesso. Assim, se suas suposições forem algum dia provadas erradas, você terá mais informações para usar na depuração.

<a id="propagating-errors"></a>

### Propagando Erros

Quando a implementação de uma função chama algo que pode falhar, em vez de tratar o erro dentro da própria função, você pode retornar o erro para o código chamador para que ele decida o que fazer. Isso é conhecido como _propagação_ do erro e dá mais controle ao código chamador, onde pode haver mais informações ou lógica que ditam como o erro deve ser tratado do que o que você tem disponível no contexto do seu código.

Por exemplo, a Listagem 9-6 mostra uma função que lê um nome de usuário de um arquivo. Se o arquivo não existir ou não puder ser lido, essa função retornará esses erros ao código que chamou a função.

<Listing number="9-6" file-name="src/main.rs" caption="Uma função que retorna erros ao código chamador usando `match`">

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-06/src/main.rs:here}}
```

</Listing>

Essa função pode ser escrita de forma muito mais curta, mas vamos começar fazendo grande parte dela manualmente para explorar o tratamento de erros; no final, mostraremos a forma mais curta. Vamos olhar primeiro para o tipo de retorno da função: `Result<String, io::Error>`. Isso significa que a função está retornando um valor do tipo `Result<T, E>`, onde o parâmetro genérico `T` foi preenchido com o tipo concreto `String` e o tipo genérico `E` foi preenchido com o tipo concreto `io::Error`.

Se essa função tiver sucesso sem nenhum problema, o código que a chama receberá um valor `Ok` contendo uma `String` — o `username` que essa função leu do arquivo. Se essa função encontrar algum problema, o código chamador receberá um valor `Err` contendo uma instância de `io::Error` com mais informações sobre quais foram os problemas. Escolhemos `io::Error` como tipo de retorno dessa função porque esse é o tipo de erro retornado de ambas as operações que podem falhar no corpo dessa função: a função `File::open` e o método `read_to_string`.

O corpo da função começa chamando a função `File::open`. Em seguida, tratamos o valor `Result` com um *match* semelhante ao *match* da Listagem 9-4. Se `File::open` tiver sucesso, o *handle* de arquivo na variável de padrão `file` se torna o valor na variável mutável `username_file` e a função continua. No caso `Err`, em vez de chamar `panic!`, usamos a palavra-chave `return` para retornar antecipadamente da função inteira e passar o valor de erro de `File::open`, agora na variável de padrão `e`, de volta ao código chamador como o valor de erro dessa função.

Portanto, se tivermos um *handle* de arquivo em `username_file`, a função então cria uma nova `String` na variável `username` e chama o método `read_to_string` no *handle* de arquivo em `username_file` para ler o conteúdo do arquivo em `username`. O método `read_to_string` também retorna um `Result` porque pode falhar, mesmo que `File::open` tenha tido sucesso. Portanto, precisamos de outro *match* para tratar esse `Result`: se `read_to_string` tiver sucesso, então nossa função teve sucesso, e retornamos o nome de usuário do arquivo que agora está em `username` envolvido em um `Ok`. Se `read_to_string` falhar, retornamos o valor de erro da mesma forma que retornamos o valor de erro no *match* que tratou o valor de retorno de `File::open`. No entanto, não precisamos dizer explicitamente `return`, porque essa é a última expressão da função.

O código que chama este código tratará, então, de obter um valor `Ok` contendo um nome de usuário ou um valor `Err` contendo um `io::Error`. Cabe ao código chamador decidir o que fazer com esses valores. Se o código chamador obtiver um valor `Err`, ele poderá chamar `panic!` e encerrar o programa, usar um nome de usuário padrão ou procurar o nome de usuário em outro lugar que não seja um arquivo, por exemplo. Não temos informações suficientes sobre o que o código chamador está realmente tentando fazer, por isso propagamos todas as informações de sucesso ou erro para cima para que ele as trate adequadamente.

Esse padrão de propagação de erros é tão comum em Rust que o Rust fornece o operador de interrogação `?` para facilitar isso.

<!-- Old headings. Do not remove or links may break. -->

<a id="a-shortcut-for-propagating-errors-the--operator"></a>

#### O Atalho do Operador `?`

A Listagem 9-7 mostra uma implementação de `read_username_from_file` que tem a mesma funcionalidade que na Listagem 9-6, mas esta implementação usa o operador `?`.

<Listing number="9-7" file-name="src/main.rs" caption="Uma função que retorna erros ao código chamador usando o operador `?`">

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-07/src/main.rs:here}}
```

</Listing>

O `?` colocado após um valor `Result` é definido para funcionar de quase a mesma forma que as expressões *match* que definimos para tratar os valores `Result` na Listagem 9-6. Se o valor do `Result` for um `Ok`, o valor dentro do `Ok` será retornado a partir dessa expressão, e o programa continuará. Se o valor for um `Err`, o `Err` será retornado de toda a função como se tivéssemos usado a palavra-chave `return`, para que o valor de erro seja propagado ao código chamador.

Há uma diferença entre o que a expressão *match* da Listagem 9-6 faz e o que o operador `?` faz: os valores de erro sobre os quais o operador `?` é chamado passam pela função `from`, definida na *trait* `From` da biblioteca padrão, que é usada para converter valores de um tipo para outro. Quando o operador `?` chama a função `from`, o tipo de erro recebido é convertido no tipo de erro definido no tipo de retorno da função atual. Isso é útil quando uma função retorna um tipo de erro único para representar todas as formas pelas quais uma função pode falhar, mesmo que partes possam falhar por muitos motivos diferentes.

Por exemplo, poderíamos alterar a função `read_username_from_file` da Listagem 9-7 para retornar um tipo de erro personalizado chamado `OurError` que definimos. Se também definirmos `impl From<io::Error> for OurError` para construir uma instância de `OurError` a partir de um `io::Error`, então as chamadas ao operador `?` no corpo de `read_username_from_file` chamarão `from` e converterão os tipos de erro sem precisar adicionar mais código à função.

No contexto da Listagem 9-7, o `?` ao final da chamada a `File::open` retornará o valor dentro de um `Ok` para a variável `username_file`. Se ocorrer um erro, o operador `?` retornará antecipadamente de toda a função e dará qualquer valor `Err` ao código chamador. O mesmo se aplica ao `?` ao final da chamada a `read_to_string`.

O operador `?` elimina muito código repetitivo (*boilerplate*) e torna a implementação dessa função mais simples. Poderíamos até encurtar ainda mais esse código encadeando chamadas de método imediatamente após o `?`, como mostrado na Listagem 9-8.

<Listing number="9-8" file-name="src/main.rs" caption="Encadeando chamadas de método após o operador `?`">

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-08/src/main.rs:here}}
```

</Listing>

Movemos a criação da nova `String` em `username` para o início da função; essa parte não mudou. Em vez de criar uma variável `username_file`, encadeamos a chamada a `read_to_string` diretamente no resultado de `File::open("hello.txt")?`. Ainda temos um `?` ao final da chamada a `read_to_string`, e ainda retornamos um valor `Ok` contendo `username` quando tanto `File::open` quanto `read_to_string` têm sucesso, em vez de retornar erros. A funcionalidade é novamente a mesma que nas Listagens 9-6 e 9-7; esta é apenas uma forma diferente e mais ergonômica de escrevê-la.

A Listagem 9-9 mostra uma forma de tornar isso ainda mais curto usando `fs::read_to_string`.

<Listing number="9-9" file-name="src/main.rs" caption="Usando `fs::read_to_string` em vez de abrir e depois ler o arquivo">

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-09/src/main.rs:here}}
```

</Listing>

Ler um arquivo em uma *string* é uma operação bastante comum, então a biblioteca padrão fornece a conveniente função `fs::read_to_string` que abre o arquivo, cria uma nova `String`, lê o conteúdo do arquivo, coloca o conteúdo nessa `String` e a retorna. É claro que usar `fs::read_to_string` não nos dá a oportunidade de explicar todo o tratamento de erros, por isso fizemos da forma mais longa primeiro.

<!-- Old headings. Do not remove or links may break. -->

<a id="where-the--operator-can-be-used"></a>

#### Onde o Operador `?` Pode Ser Usado

O operador `?` só pode ser usado em funções cujo tipo de retorno seja compatível com o valor sobre o qual `?` é usado. Isso porque o operador `?` é definido para realizar um retorno antecipado de um valor da função, da mesma maneira que a expressão *match* que definimos na Listagem 9-6. Na Listagem 9-6, o *match* estava usando um valor `Result`, e o braço de retorno antecipado retornava um valor `Err(e)`. O tipo de retorno da função tem que ser um `Result` para ser compatível com esse `return`.

Na Listagem 9-10, vamos ver o erro que obteremos se usarmos o operador `?` em uma função `main` com um tipo de retorno incompatível com o tipo do valor sobre o qual usamos `?`.

<Listing number="9-10" file-name="src/main.rs" caption="Tentar usar `?` na função `main` que retorna `()` não compila.">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-10/src/main.rs}}
```

</Listing>

Esse código abre um arquivo, o que pode falhar. O operador `?` segue o valor `Result` retornado por `File::open`, mas essa função `main` tem o tipo de retorno `()`, não `Result`. Quando compilamos esse código, recebemos a seguinte mensagem de erro:

```console
{{#include ../listings/ch09-error-handling/listing-09-10/output.txt}}
```

Esse erro aponta que só podemos usar o operador `?` em uma função que retorna `Result`, `Option` ou outro tipo que implemente `FromResidual`.

Para corrigir o erro, você tem duas escolhas. Uma é alterar o tipo de retorno da sua função para ser compatível com o valor sobre o qual você está usando o operador `?`, desde que não haja restrições que impeçam isso. A outra é usar um *match* ou um dos métodos de `Result<T, E>` para tratar o `Result<T, E>` da maneira adequada.

A mensagem de erro também mencionou que `?` pode ser usado com valores `Option<T>`. Assim como ao usar `?` em `Result`, você só pode usar `?` em `Option` em uma função que retorne `Option`. O comportamento do operador `?` quando chamado sobre um `Option<T>` é semelhante ao seu comportamento quando chamado sobre um `Result<T, E>`: se o valor for `None`, o `None` será retornado antecipadamente da função naquele ponto. Se o valor for `Some`, o valor dentro de `Some` é o valor resultante da expressão, e a função continua. A Listagem 9-11 tem um exemplo de uma função que encontra o último caractere da primeira linha do texto fornecido.

<Listing number="9-11" caption="Usando o operador `?` em um valor `Option<T>`">

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-11/src/main.rs:here}}
```

</Listing>

Essa função retorna `Option<char>` porque é possível que haja um caractere lá, mas também é possível que não haja. Esse código recebe o argumento *slice* de *string* `text` e chama o método `lines` sobre ele, que retorna um iterador sobre as linhas da *string*. Como essa função quer examinar a primeira linha, ela chama `next` no iterador para obter o primeiro valor. Se `text` for a *string* vazia, essa chamada a `next` retornará `None`, e nesse caso usamos `?` para parar e retornar `None` de `last_char_of_first_line`. Se `text` não for a *string* vazia, `next` retornará um valor `Some` contendo um *slice* de *string* da primeira linha em `text`.

O `?` extrai o *slice* de *string*, e podemos chamar `chars` sobre esse *slice* de *string* para obter um iterador de seus caracteres. Estamos interessados no último caractere dessa primeira linha, por isso chamamos `last` para retornar o último item no iterador. Isso é um `Option` porque é possível que a primeira linha seja a *string* vazia; por exemplo, se `text` começar com uma linha em branco mas tiver caracteres em outras linhas, como em `"\nhi"`. No entanto, se houver um último caractere na primeira linha, ele será retornado na variante `Some`. O operador `?` no meio nos dá uma forma concisa de expressar essa lógica, permitindo implementar a função em uma única linha. Se não pudéssemos usar o operador `?` em `Option`, teríamos que implementar essa lógica usando mais chamadas de método ou uma expressão *match*.

Note que você pode usar o operador `?` em um `Result` em uma função que retorna `Result`, e pode usar o operador `?` em um `Option` em uma função que retorna `Option`, mas não pode misturar os dois. O operador `?` não converte automaticamente um `Result` em um `Option` nem vice-versa; nesses casos, você pode usar métodos como o método `ok` em `Result` ou o método `ok_or` em `Option` para fazer a conversão explicitamente.

Até agora, todas as funções `main` que usamos retornam `()`. A função `main` é especial porque é o ponto de entrada e saída de um programa executável, e há restrições sobre qual pode ser o seu tipo de retorno para que o programa se comporte conforme esperado.

Felizmente, `main` também pode retornar um `Result<(), E>`. A Listagem 9-12 tem o código da Listagem 9-10, mas alteramos o tipo de retorno de `main` para `Result<(), Box<dyn Error>>` e adicionamos um valor de retorno `Ok(())` ao final. Esse código agora compilará.

<Listing number="9-12" file-name="src/main.rs" caption="Mudar `main` para retornar `Result<(), E>` permite o uso do operador `?` em valores `Result`.">

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-12/src/main.rs}}
```

</Listing>

O tipo `Box<dyn Error>` é um objeto de *trait*, sobre o qual falaremos na seção ["Usando Objetos de *Trait* para Abstrair Comportamentos Compartilhados"][trait-objects]<!-- ignore --> no Capítulo 18. Por enquanto, você pode ler `Box<dyn Error>` como "qualquer tipo de erro". Usar `?` em um valor `Result` em uma função `main` com o tipo de erro `Box<dyn Error>` é permitido porque permite que qualquer valor `Err` seja retornado antecipadamente. Embora o corpo dessa função `main` vá retornar apenas erros do tipo `std::io::Error`, ao especificar `Box<dyn Error>`, essa assinatura continuará correta mesmo se mais código que retorne outros erros for adicionado ao corpo de `main`.

Quando uma função `main` retorna um `Result<(), E>`, o executável sairá com um valor de `0` se `main` retornar `Ok(())` e sairá com um valor diferente de zero se `main` retornar um valor `Err`. Executáveis escritos em C retornam inteiros ao sair: programas que saem com sucesso retornam o inteiro `0`, e programas com erro retornam algum inteiro diferente de `0`. O Rust também retorna inteiros de executáveis para ser compatível com essa convenção.

A função `main` pode retornar qualquer tipo que implemente [a *trait* `std::process::Termination`][termination]<!-- ignore -->, que contém uma função `report` que retorna um `ExitCode`. Consulte a documentação da biblioteca padrão para mais informações sobre como implementar a *trait* `Termination` para seus próprios tipos.

Agora que discutimos os detalhes de chamar `panic!` ou retornar `Result`, vamos voltar ao tópico de como decidir qual é o mais adequado para usar em cada situação.

[handle_failure]: ch02-00-guessing-game-tutorial.html#handling-potential-failure-with-result
[trait-objects]: ch18-02-trait-objects.html#using-trait-objects-to-abstract-over-shared-behavior
[termination]: ../std/process/trait.Termination.html
