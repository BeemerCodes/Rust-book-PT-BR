<a id="an-example-program-using-structs"></a>
## Um Programa de Exemplo Usando Structs

Para entender quando faz sentido usar structs, vamos escrever um programa que
calcula a área de um retângulo. Vamos começar usando variáveis avulsas e depois
refatorar o programa até estarmos usando structs.

Vamos criar um novo projeto binário com o Cargo chamado _rectangles_, que vai
receber a largura e a altura de um retângulo em pixels e calcular a sua área. A
Listagem 5-8 mostra um programa curto com uma forma de fazer exatamente isso no
arquivo _src/main.rs_ do nosso projeto.

<Listing number="5-8" file-name="src/main.rs" caption="Calculando a área de um retângulo especificado por variáveis separadas de largura e altura">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:all}}
```

</Listing>

Agora, execute o programa usando `cargo run`:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/output.txt}}
```

Esse código consegue calcular a área do retângulo chamando a função `area` com
cada dimensão, mas podemos fazer melhor para deixá-lo mais claro e legível.

O problema desse código fica evidente na assinatura de `area`:

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:here}}
```

A função `area` deveria calcular a área de um único retângulo, mas a função que
escrevemos tem dois parâmetros, e não fica claro em nenhum lugar do programa que
esses parâmetros estão relacionados. Seria mais legível e mais fácil de manter
se agrupássemos largura e altura juntos. Já vimos uma forma de fazer isso na
seção [“O Tipo Tupla”][the-tuple-type]<!-- ignore --> do Capítulo 3: usando
tuplas.

<a id="refactoring-with-tuples"></a>
### Refatorando com Tuplas

A Listagem 5-9 mostra outra versão do nosso programa usando tuplas.

<Listing number="5-9" file-name="src/main.rs" caption="Especificando a largura e a altura do retângulo com uma tupla">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-09/src/main.rs}}
```

</Listing>

Por um lado, esse programa está melhor. As tuplas nos permitem adicionar um
pouco de estrutura, e agora passamos apenas um argumento. Por outro lado, essa
versão é menos clara: tuplas não dão nome aos seus elementos, então precisamos
acessar as partes da tupla por índice, o que torna o cálculo menos óbvio.

Trocar largura por altura não faria diferença no cálculo da área, mas se
quiséssemos desenhar o retângulo na tela, faria toda a diferença! Teríamos que
lembrar que `width` é o índice `0` da tupla e `height` é o índice `1`. Isso
seria ainda mais difícil para outra pessoa descobrir e manter em mente caso
fosse usar nosso código. Como não expressamos o significado dos nossos dados no
código, fica mais fácil introduzir erros.

<!-- Old headings. Do not remove or links may break. -->

<a id="refactoring-with-structs-adding-more-meaning"></a>

<a id="refactoring-with-structs"></a>
### Refatorando com Structs

Usamos structs para adicionar significado dando rótulos aos dados. Podemos
transformar a tupla que estamos usando em uma struct com um nome para o todo e
nomes para cada parte, como mostrado na Listagem 5-10.

<Listing number="5-10" file-name="src/main.rs" caption="Definindo uma struct `Rectangle`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-10/src/main.rs}}
```

</Listing>

Aqui, definimos uma struct e a chamamos de `Rectangle`. Dentro das chaves,
definimos os campos `width` e `height`, ambos do tipo `u32`. Então, em `main`,
criamos uma instância específica de `Rectangle` com largura `30` e altura `50`.

Nossa função `area` agora é definida com um único parâmetro, que chamamos de
`rectangle`, cujo tipo é um empréstimo (*borrowing*) imutável de uma instância
da struct `Rectangle`. Como mencionamos no Capítulo 4, queremos pegar a struct
emprestada em vez de assumir seu *ownership*. Dessa forma, `main` mantém o
ownership e pode continuar usando `rect1`, que é o motivo de usarmos `&` na
assinatura da função e no momento da chamada.

A função `area` acessa os campos `width` e `height` da instância de `Rectangle`
(note que acessar campos de uma instância de struct emprestada não move os
valores dos campos, motivo pelo qual você frequentemente verá empréstimos de
structs). Nossa assinatura de `area` agora diz exatamente o que queremos:
calcular a área de um `Rectangle` usando seus campos `width` e `height`. Isso
deixa claro que largura e altura estão relacionadas entre si e dá nomes
descritivos aos valores em vez de usar os índices `0` e `1` de uma tupla. É um
ganho enorme em clareza.

<!-- Old headings. Do not remove or links may break. -->

<a id="adding-useful-functionality-with-derived-traits"></a>

<a id="adding-functionality-with-derived-traits"></a>
### Adicionando Funcionalidades com Traits Derivadas

Seria útil poder imprimir uma instância de `Rectangle` enquanto estamos
depurando o programa e ver os valores de todos os seus campos. A Listagem 5-11
tenta usar a [_macro_ `println!`][println]<!-- ignore --> como fizemos nos
capítulos anteriores. Porém, isso não vai funcionar.

<Listing number="5-11" file-name="src/main.rs" caption="Tentando imprimir uma instância de `Rectangle`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/src/main.rs}}
```

</Listing>

Ao compilar esse código, recebemos um erro com esta mensagem principal:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:3}}
```

A macro `println!` pode fazer vários tipos de formatação e, por padrão, as
chaves dizem a `println!` para usar a formatação conhecida como `Display`: uma
saída voltada diretamente para o usuário final. Os tipos primitivos que vimos
até agora implementam `Display` por padrão porque só existe uma maneira óbvia de
mostrar um `1` ou qualquer outro tipo primitivo para um usuário. Mas com
structs, a forma como `println!` deve formatar a saída é menos óbvia porque há
várias possibilidades: você quer vírgulas ou não? Quer imprimir as chaves? Todos
os campos devem aparecer? Por causa dessa ambiguidade, o Rust não tenta
adivinhar o que queremos, e structs não vêm com uma implementação padrão de
`Display` para usar com `println!` e o marcador `{}`.

Se continuarmos lendo as mensagens de erro, encontraremos esta nota útil:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:9:10}}
```

Vamos testar! A chamada da macro `println!` agora ficará assim:
`println!("rect1 is {rect1:?}");`. Colocar o especificador `:?` dentro das
chaves diz a `println!` que queremos usar um formato de saída chamado `Debug`. A
trait `Debug` permite imprimir nossa struct de uma forma útil para
desenvolvedores, para que possamos ver seu valor enquanto depuramos o código.

Compile o código com essa alteração. Puxa! Ainda recebemos um erro:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:3}}
```

Mas, novamente, o compilador nos dá uma nota útil:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:9:10}}
```

O Rust _inclui_, sim, a funcionalidade de imprimir informações de depuração, mas
precisamos optar explicitamente por disponibilizá-la para a nossa struct. Para
fazer isso, adicionamos o atributo externo `#[derive(Debug)]` logo antes da
definição da struct, como mostrado na Listagem 5-12.

<Listing number="5-12" file-name="src/main.rs" caption="Adicionando o atributo para derivar a trait `Debug` e imprimindo a instância de `Rectangle` usando formatação de depuração">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/src/main.rs}}
```

</Listing>

Agora, ao executar o programa, não teremos nenhum erro e veremos a seguinte
saída:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/output.txt}}
```

Legal! Não é a saída mais bonita do mundo, mas mostra os valores de todos os
campos dessa instância, o que certamente ajuda durante a depuração. Quando temos
structs maiores, é útil ter uma saída um pouco mais fácil de ler; nesses casos,
podemos usar `{:#?}` em vez de `{:?}` na string de `println!`. Neste exemplo,
usar o estilo `{:#?}` produzirá a seguinte saída:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-02-pretty-debug/output.txt}}
```

Outra maneira de imprimir um valor usando o formato `Debug` é com a [macro
`dbg!`][dbg]<!-- ignore -->, que assume o ownership de uma expressão (ao
contrário de `println!`, que recebe uma referência), imprime o arquivo e o
número da linha onde a chamada de `dbg!` ocorre no código junto com o valor
resultante dessa expressão, e devolve o ownership do valor.

> Nota: Chamar a macro `dbg!` imprime no fluxo de saída de erro padrão
> (`stderr`), ao contrário de `println!`, que imprime no fluxo de saída padrão
> (`stdout`). Falaremos mais sobre `stderr` e `stdout` na seção
> [“Redirecionando Erros para a Saída de Erro Padrão” no Capítulo 12][err]<!--
> ignore -->.

Aqui está um exemplo em que estamos interessados tanto no valor atribuído ao
campo `width` quanto no valor da struct inteira em `rect1`:

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/src/main.rs}}
```

Podemos envolver a expressão `30 * scale` com `dbg!` e, como `dbg!` devolve o
ownership do valor da expressão, o campo `width` receberá exatamente o mesmo
valor que receberia se não tivéssemos colocado a chamada a `dbg!` ali. Não
queremos que `dbg!` assuma o ownership de `rect1`, então passamos uma referência
a `rect1` na chamada seguinte. Veja como fica a saída desse exemplo:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/output.txt}}
```

Podemos ver que a primeira parte da saída veio da linha 10 de _src/main.rs_,
onde estamos depurando a expressão `30 * scale`, e seu valor resultante é `60`
(a formatação `Debug` implementada para inteiros imprime apenas o valor deles).
A chamada a `dbg!` na linha 14 de _src/main.rs_ imprime o valor de `&rect1`, que
é a struct `Rectangle`. Essa saída usa a formatação `Debug` indentada do tipo
`Rectangle`. A macro `dbg!` pode ser muito útil quando você está tentando
entender o que o seu código está fazendo!

Além da trait `Debug`, o Rust fornece várias outras traits que podemos usar com
o atributo `derive` para adicionar comportamentos úteis aos nossos tipos
personalizados. Essas traits e seus comportamentos estão listados no [Apêndice
C][app-c]<!-- ignore -->. Vamos ver como implementar essas traits com
comportamentos personalizados e como criar suas próprias traits no Capítulo 10.
Também existem muitos outros atributos além de `derive`; para mais informações,
veja [a seção “Attributes” da Referência do Rust][attributes].

Nossa função `area` é bem específica: ela só calcula a área de retângulos. Seria
útil vincular esse comportamento mais estreitamente à nossa struct `Rectangle`,
já que ele não funciona com nenhum outro tipo. Vamos ver como podemos continuar
refatorando esse código transformando a função `area` em um método `area`
definido no nosso tipo `Rectangle`.

[the-tuple-type]: ch03-02-data-types.html#the-tuple-type
[app-c]: appendix-03-derivable-traits.md
[println]: ../std/macro.println.html
[dbg]: ../std/macro.dbg.html
[err]: ch12-06-writing-to-stderr-instead-of-stdout.html
[attributes]: ../reference/attributes.html
