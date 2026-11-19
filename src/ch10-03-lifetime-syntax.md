<a id="validating-references-with-lifetimes"></a>

## Validando Referências com Lifetimes

*Lifetimes* são outro tipo de genérico que já estávamos usando. Em vez de garantir que um tipo tenha o comportamento que queremos, os *lifetimes* garantem que as referências sejam válidas enquanto precisarmos que elas sejam.

Um detalhe que não discutimos na seção [“Referências e Empréstimos (*Borrowing*)”][references-and-borrowing]<!-- ignore --> no Capítulo 4 é que toda referência em Rust tem um *lifetime*, que é o escopo pelo qual aquela referência é válida. Na maioria das vezes, os *lifetimes* são implícitos e inferidos, assim como na maioria das vezes os tipos são inferidos. Só somos obrigados a anotar tipos quando múltiplos tipos são possíveis. De maneira semelhante, devemos anotar *lifetimes* quando os *lifetimes* de referências puderem se relacionar de algumas formas diferentes. Rust exige que anotemos esses relacionamentos usando parâmetros de *lifetime* genéricos para garantir que as referências reais usadas em tempo de execução sejam definitivamente válidas.

Anotar *lifetimes* nem é um conceito que a maioria das outras linguagens de programação tem, então isso vai parecer pouco familiar. Embora não vamos cobrir *lifetimes* por completo neste capítulo, discutiremos formas comuns em que você pode encontrar a sintaxe de *lifetime* para que possa se familiarizar com o conceito.

<!-- Old headings. Do not remove or links may break. -->

<a id="preventing-dangling-references-with-lifetimes"></a>

### Referências Pendentes (*Dangling References*)

O principal objetivo dos *lifetimes* é evitar referências pendentes (*dangling references*), que, se pudessem existir, fariam um programa referenciar dados diferentes daqueles que pretendia referenciar. Considere o programa na Listagem 10-16, que tem um escopo externo e um escopo interno.

<Listing number="10-16" caption="Uma tentativa de usar uma referência cujo valor saiu de escopo">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-16/src/main.rs}}
```

</Listing>

> Nota: Os exemplos nas Listagens 10-16, 10-17 e 10-23 declaram variáveis sem atribuir a elas um valor inicial, de modo que o nome da variável existe no escopo externo. À primeira vista, isso pode parecer em conflito com o fato de Rust não ter valores nulos. No entanto, se tentarmos usar uma variável antes de atribuir a ela um valor, teremos um erro em tempo de compilação, o que mostra que, de fato, Rust não permite valores nulos.

O escopo externo declara uma variável chamada `r` sem valor inicial, e o escopo interno declara uma variável chamada `x` com o valor inicial de `5`. Dentro do escopo interno, tentamos definir o valor de `r` como uma referência a `x`. Em seguida, o escopo interno termina e tentamos imprimir o valor em `r`. Esse código não compilará, porque o valor ao qual `r` está se referindo saiu de escopo antes de tentarmos usá-lo. Aqui está a mensagem de erro:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-16/output.txt}}
```

A mensagem de erro diz que a variável `x` "não vive o suficiente" (`does not live long enough`). A razão é que `x` estará fora de escopo quando o escopo interno terminar na linha 7. Mas `r` ainda é válido para o escopo externo; como seu escopo é maior, dizemos que ele "vive mais". Se Rust permitisse que esse código funcionasse, `r` estaria referenciando memória que foi desalocada quando `x` saiu de escopo, e qualquer coisa que tentássemos fazer com `r` não funcionaria corretamente. Então, como Rust determina que esse código é inválido? Ele usa um verificador de empréstimos (*borrow checker*).

<a id="the-borrow-checker"></a>

### O Borrow Checker

O compilador Rust possui um *borrow checker* que compara escopos para determinar se todos os empréstimos (*borrows*) são válidos. A Listagem 10-17 mostra o mesmo código da Listagem 10-16, mas com anotações mostrando os *lifetimes* das variáveis.

<Listing number="10-17" caption="Anotações dos *lifetimes* de `r` e `x`, chamados de `'a` e `'b`, respectivamente">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-17/src/main.rs}}
```

</Listing>

Aqui, anotamos o *lifetime* de `r` com `'a` e o *lifetime* de `x` com `'b`. Como você pode ver, o bloco interno `'b` é muito menor que o bloco de *lifetime* externo `'a`. Em tempo de compilação, Rust compara o tamanho dos dois *lifetimes* e vê que `r` tem um *lifetime* de `'a`, mas que ele se refere a uma memória com um *lifetime* de `'b`. O programa é rejeitado porque `'b` é mais curto que `'a`: o objeto da referência não vive tanto quanto a referência.

A Listagem 10-18 corrige o código para que ele não tenha uma referência pendente e compile sem erros.

<Listing number="10-18" caption="Uma referência válida porque os dados têm um *lifetime* mais longo que a referência">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-18/src/main.rs}}
```

</Listing>

Aqui, `x` tem o *lifetime* `'b`, que neste caso é maior que `'a`. Isso significa que `r` pode referenciar `x` porque Rust sabe que a referência em `r` sempre será válida enquanto `x` for válido.

Agora que você sabe onde estão os *lifetimes* das referências e como Rust analisa os *lifetimes* para garantir que as referências sejam sempre válidas, vamos explorar *lifetimes* genéricos em parâmetros de função e valores de retorno.

<a id="generic-lifetimes-in-functions"></a>

### Lifetimes Genéricos em Funções

Escreveremos uma função que retorna a maior de duas fatias de string (*string slices*). Essa função receberá duas fatias de string e retornará uma única fatia de string. Depois de implementarmos a função `longest`, o código na Listagem 10-19 deverá imprimir `The longest string is abcd`.

<Listing number="10-19" file-name="src/main.rs" caption="Uma função `main` que chama a função `longest` para encontrar a mais longa de duas fatias de string">

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-19/src/main.rs}}
```

</Listing>

Note que queremos que a função receba *string slices*, que são referências, em vez de strings próprias, porque não queremos que a função `longest` tome a posse (*ownership*) de seus parâmetros. Consulte a seção [“Fatias de String como Parâmetros”][string-slices-as-parameters]<!-- ignore --> no Capítulo 4 para mais detalhes sobre por que os parâmetros que usamos na Listagem 10-19 são os que queremos.

Se tentarmos implementar a função `longest` como mostrado na Listagem 10-20, ela não compilará.

<Listing number="10-20" file-name="src/main.rs" caption="Uma implementação da função `longest` que retorna a mais longa de duas fatias de string, mas que ainda não compila">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-20/src/main.rs:here}}
```

</Listing>

Em vez disso, obtemos o seguinte erro que fala sobre *lifetimes*:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-20/output.txt}}
```

O texto de ajuda revela que o tipo de retorno precisa de um parâmetro de *lifetime* genérico porque Rust não consegue determinar se a referência que está sendo retornada se refere a `x` ou `y`. Na verdade, nós também não sabemos, porque o bloco `if` no corpo dessa função retorna uma referência a `x` e o bloco `else` retorna uma referência a `y`!

Quando estamos definindo esta função, não sabemos os valores concretos que serão passados para ela, então não sabemos se o caso `if` ou o caso `else` será executado. Também não sabemos os *lifetimes* concretos das referências que serão passadas, portanto, não podemos examinar os escopos como fizemos nas Listagens 10-17 e 10-18 para determinar se a referência que retornamos será sempre válida. O *borrow checker* também não pode determinar isso, porque não sabe como os *lifetimes* de `x` e `y` se relacionam com o *lifetime* do valor de retorno. Para corrigir esse erro, adicionaremos parâmetros de *lifetime* genéricos que definem a relação entre as referências para que o *borrow checker* possa realizar sua análise.

<a id="lifetime-annotation-syntax"></a>

### Sintaxe de Anotação de Lifetime

As anotações de *lifetime* não mudam o tempo de vida de nenhuma das referências. Em vez disso, elas descrevem os relacionamentos dos *lifetimes* de múltiplas referências entre si, sem afetar esses *lifetimes*. Assim como as funções podem aceitar qualquer tipo quando a assinatura especifica um parâmetro de tipo genérico, as funções podem aceitar referências com qualquer *lifetime* especificando um parâmetro de *lifetime* genérico.

Anotações de *lifetime* têm uma sintaxe ligeiramente incomum: os nomes dos parâmetros de *lifetime* devem começar com um apóstrofo (`'`) e geralmente são em letras minúsculas e muito curtos, como tipos genéricos. A maioria das pessoas usa o nome `'a` para a primeira anotação de *lifetime*. Colocamos as anotações de parâmetros de *lifetime* após o `&` de uma referência, usando um espaço para separar a anotação do tipo da referência.

Aqui estão alguns exemplos: uma referência a um `i32` sem um parâmetro de *lifetime*, uma referência a um `i32` que tem um parâmetro de *lifetime* chamado `'a` e uma referência mutável a um `i32` que também tem o *lifetime* `'a`:

```rust,ignore
&i32        // uma referência
&'a i32     // uma referência com um lifetime explícito
&'a mut i32 // uma referência mutável com um lifetime explícito
```

Uma anotação de *lifetime* por si só não tem muito significado, porque as anotações têm como objetivo informar a Rust como os parâmetros de *lifetime* genéricos de múltiplas referências se relacionam entre si. Vamos examinar como as anotações de *lifetime* se relacionam entre si no contexto da função `longest`.

<!-- Old headings. Do not remove or links may break. -->

<a id="lifetime-annotations-in-function-signatures"></a>

### Em Assinaturas de Função

Para usar anotações de *lifetime* em assinaturas de função, precisamos declarar os parâmetros de *lifetime* genéricos dentro de colchetes angulares entre o nome da função e a lista de parâmetros, exatamente como fizemos com os parâmetros de tipos genéricos.

Queremos que a assinatura expresse a seguinte restrição: a referência retornada será válida enquanto ambos os parâmetros forem válidos. Essa é a relação entre os *lifetimes* dos parâmetros e o valor de retorno. Chamaremos o *lifetime* de `'a` e depois o adicionaremos a cada referência, conforme mostrado na Listagem 10-21.

<Listing number="10-21" file-name="src/main.rs" caption="A definição da função `longest` especificando que todas as referências na assinatura devem ter o mesmo *lifetime* `'a`">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-21/src/main.rs:here}}
```

</Listing>

Este código deve compilar e produzir o resultado que desejamos quando o usamos com a função `main` na Listagem 10-19.

A assinatura da função agora diz a Rust que, para algum *lifetime* `'a`, a função recebe dois parâmetros, ambos *string slices* que vivem pelo menos tanto quanto o *lifetime* `'a`. A assinatura da função também informa a Rust que o *string slice* retornado da função viverá pelo menos tanto quanto o *lifetime* `'a`. Na prática, isso significa que o *lifetime* da referência retornada pela função `longest` é o mesmo que o menor dos *lifetimes* dos valores referenciados pelos argumentos da função. Esses relacionamentos são o que queremos que Rust use ao analisar este código.

Lembre-se, quando especificamos os parâmetros de *lifetime* nesta assinatura de função, não estamos alterando os *lifetimes* de nenhum valor passado ou retornado. Em vez disso, estamos especificando que o *borrow checker* deve rejeitar quaisquer valores que não atendam a essas restrições. Note que a função `longest` não precisa saber exatamente quanto tempo `x` e `y` viverão, apenas que algum escopo pode ser substituído por `'a` que satisfará essa assinatura.

Ao anotar *lifetimes* em funções, as anotações vão na assinatura da função, não no corpo da função. As anotações de *lifetime* tornam-se parte do contrato da função, muito parecido com os tipos na assinatura. Ter assinaturas de funções contendo o contrato de *lifetime* significa que a análise feita pelo compilador Rust pode ser mais simples. Se houver um problema com a forma como uma função é anotada ou com a forma como ela é chamada, os erros do compilador podem apontar para a parte do nosso código e para as restrições com mais precisão. Se, em vez disso, o compilador Rust fizesse mais inferências sobre quais pretendíamos que fossem os relacionamentos dos *lifetimes*, o compilador poderia apontar apenas para um uso do nosso código muitos passos distante da causa do problema.

Quando passamos referências concretas para `longest`, o *lifetime* concreto que é substituído por `'a` é a parte do escopo de `x` que se sobrepõe ao escopo de `y`. Em outras palavras, o *lifetime* genérico `'a` receberá o *lifetime* concreto que é igual ao menor dos *lifetimes* de `x` e `y`. Como anotamos a referência retornada com o mesmo parâmetro de *lifetime* `'a`, a referência retornada também será válida pela duração do menor dos *lifetimes* de `x` e `y`.

Vamos ver como as anotações de *lifetime* restringem a função `longest` passando referências que têm *lifetimes* concretos diferentes. A Listagem 10-22 é um exemplo direto.

<Listing number="10-22" file-name="src/main.rs" caption="Usando a função `longest` com referências a valores `String` que têm diferentes *lifetimes* concretos">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-22/src/main.rs:here}}
```

</Listing>

Neste exemplo, `string1` é válida até o final do escopo externo, `string2` é válida até o final do escopo interno e `result` referencia algo que é válido até o final do escopo interno. Execute este código e verá que o *borrow checker* o aprova; ele compilará e imprimirá `The longest string is long string is long`.

Em seguida, vamos tentar um exemplo que mostra que o *lifetime* da referência em `result` deve ser o menor *lifetime* dos dois argumentos. Moveremos a declaração da variável `result` para fora do escopo interno, mas deixaremos a atribuição do valor à variável `result` dentro do escopo com `string2`. Em seguida, moveremos o `println!` que usa `result` para fora do escopo interno, após o término do escopo interno. O código na Listagem 10-23 não compilará.

<Listing number="10-23" file-name="src/main.rs" caption="Tentando usar `result` depois que `string2` saiu de escopo">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-23/src/main.rs:here}}
```

</Listing>

Quando tentamos compilar este código, obtemos este erro:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-23/output.txt}}
```

O erro mostra que, para `result` ser válido para a instrução `println!`, `string2` precisaria ser válida até o final do escopo externo. Rust sabe disso porque anotamos os *lifetimes* dos parâmetros da função e os valores de retorno usando o mesmo parâmetro de *lifetime* `'a`.

Como humanos, podemos olhar para esse código e ver que `string1` é mais longa que `string2` e, portanto, `result` conterá uma referência a `string1`. Como `string1` ainda não saiu do escopo, uma referência a `string1` ainda será válida para a instrução `println!`. No entanto, o compilador não pode ver que a referência é válida neste caso. Dissemos a Rust que o *lifetime* da referência retornada pela função `longest` é o mesmo que o menor dos *lifetimes* das referências passadas. Portanto, o *borrow checker* não permite o código na Listagem 10-23 por possivelmente ter uma referência inválida.

Tente criar mais experimentos que variem os valores e os *lifetimes* das referências passadas para a função `longest` e como a referência retornada é usada. Faça hipóteses sobre se seus experimentos passarão ou não no *borrow checker* antes de compilar; depois, verifique se você estava certo!

<!-- Old headings. Do not remove or links may break. -->

<a id="thinking-in-terms-of-lifetimes"></a>

### Pensando em Termos de Lifetimes

A maneira pela qual você precisa especificar parâmetros de *lifetime* depende do que sua função está fazendo. Por exemplo, se mudássemos a implementação da função `longest` para sempre retornar o primeiro parâmetro em vez do maior *string slice*, não precisaríamos especificar um *lifetime* no parâmetro `y`. O código a seguir compilará:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-08-only-one-reference-with-lifetime/src/main.rs:here}}
```

</Listing>

Especificamos um parâmetro de *lifetime* `'a` para o parâmetro `x` e o tipo de retorno, mas não para o parâmetro `y`, porque o *lifetime* de `y` não tem nenhuma relação com o *lifetime* de `x` ou o valor de retorno.

Ao retornar uma referência de uma função, o parâmetro de *lifetime* para o tipo de retorno precisa corresponder ao parâmetro de *lifetime* de um dos parâmetros. Se a referência retornada *não* se referir a um dos parâmetros, ela deve se referir a um valor criado dentro dessa função. No entanto, essa seria uma referência pendente, porque o valor sairá de escopo no final da função. Considere esta tentativa de implementação da função `longest` que não compilará:

<Listing file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-09-unrelated-lifetime/src/main.rs:here}}
```

</Listing>

Aqui, embora tenhamos especificado um parâmetro de *lifetime* `'a` para o tipo de retorno, essa implementação não conseguirá compilar porque o *lifetime* do valor de retorno não está relacionado com o *lifetime* dos parâmetros de forma alguma. Aqui está a mensagem de erro que obtemos:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-09-unrelated-lifetime/output.txt}}
```

O problema é que `result` sai de escopo e é limpo ao final da função `longest`. Também estamos tentando retornar uma referência a `result` a partir da função. Não há como especificar parâmetros de *lifetime* que mudem essa referência pendente, e Rust não nos deixará criar uma referência pendente. Nesse caso, a melhor solução seria retornar um tipo de dado com posse (*owned data type*) em vez de uma referência, para que a função chamadora seja então responsável por limpar o valor.

Em última análise, a sintaxe de *lifetime* trata de conectar os *lifetimes* de vários parâmetros e valores de retorno de funções. Uma vez conectados, Rust tem informações suficientes para permitir operações seguras para a memória e proibir operações que criariam ponteiros pendentes ou violariam de outra forma a segurança de memória.

<!-- Old headings. Do not remove or links may break. -->

<a id="lifetime-annotations-in-struct-definitions"></a>

### Em Definições de Struct

Até agora, as *structs* que definimos guardavam apenas tipos com posse. Podemos definir *structs* para guardar referências, mas, nesse caso, precisaríamos adicionar uma anotação de *lifetime* em cada referência na definição da *struct*. A Listagem 10-24 tem uma *struct* chamada `ImportantExcerpt` que guarda um *string slice*.

<Listing number="10-24" file-name="src/main.rs" caption="Uma *struct* que guarda uma referência, exigindo uma anotação de *lifetime*">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-24/src/main.rs}}
```

</Listing>

Essa *struct* tem um único campo `part` que guarda um *string slice*, que é uma referência. Assim como nos tipos de dados genéricos, declaramos o nome do parâmetro de *lifetime* genérico dentro de colchetes angulares após o nome da *struct* para que possamos usar o parâmetro de *lifetime* no corpo da definição da *struct*. Essa anotação significa que uma instância de `ImportantExcerpt` não pode viver mais do que a referência que ela guarda em seu campo `part`.

A função `main` aqui cria uma instância da *struct* `ImportantExcerpt` que guarda uma referência à primeira frase da `String` pertencente à variável `novel`. Os dados em `novel` existem antes da criação da instância de `ImportantExcerpt`. Além disso, `novel` não sai de escopo até depois que `ImportantExcerpt` sai de escopo, de modo que a referência na instância de `ImportantExcerpt` é válida.

<a id="lifetime-elision"></a>

### Elisão de Lifetime

Você aprendeu que toda referência tem um *lifetime* e que precisa especificar parâmetros de *lifetime* para funções ou *structs* que usam referências. No entanto, tínhamos uma função na Listagem 4-9, mostrada novamente na Listagem 10-25, que compilava sem anotações de *lifetime*.

<Listing number="10-25" file-name="src/lib.rs" caption="Uma função que definimos na Listagem 4-9 que compilou sem anotações de *lifetime*, mesmo que o parâmetro e o tipo de retorno sejam referências">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-25/src/main.rs:here}}
```

</Listing>

A razão pela qual essa função compila sem anotações de *lifetime* é histórica: nas primeiras versões (pré-1.0) de Rust, esse código não teria compilado, porque toda referência precisava de um *lifetime* explícito. Naquela época, a assinatura da função teria sido escrita assim:

```rust,ignore
fn first_word<'a>(s: &'a str) -> &'a str {
```

Depois de escrever muito código Rust, a equipe do Rust descobriu que os programadores Rust estavam digitando as mesmas anotações de *lifetime* repetidamente em situações particulares. Essas situações eram previsíveis e seguiam alguns padrões determinísticos. Os desenvolvedores programaram esses padrões no código do compilador para que o *borrow checker* pudesse inferir os *lifetimes* nessas situações e não precisasse de anotações explícitas.

Esta parte da história do Rust é relevante porque é possível que mais padrões determinísticos surjam e sejam adicionados ao compilador. No futuro, ainda menos anotações de *lifetime* podem ser necessárias.

Os padrões programados na análise de referências de Rust são chamados de *regras de elisão de lifetime* (*lifetime elision rules*). Essas não são regras para os programadores seguirem; são um conjunto de casos particulares que o compilador considerará e, se o seu código se enquadrar nesses casos, você não precisará escrever os *lifetimes* explicitamente.

As regras de elisão não fornecem uma inferência completa. Se ainda houver ambiguidade sobre quais *lifetimes* as referências têm depois que Rust aplicar as regras, o compilador não adivinhará qual deve ser o *lifetime* das referências restantes. Em vez de adivinhar, o compilador fornecerá um erro que você pode resolver adicionando as anotações de *lifetime*.

*Lifetimes* em parâmetros de função ou método são chamados de *lifetimes de entrada* (*input lifetimes*), e *lifetimes* em valores de retorno são chamados de *lifetimes de saída* (*output lifetimes*).

O compilador usa três regras para descobrir os *lifetimes* das referências quando não há anotações explícitas. A primeira regra se aplica a *lifetimes* de entrada, e a segunda e a terceira regras se aplicam a *lifetimes* de saída. Se o compilador chegar ao final das três regras e ainda houver referências para as quais ele não consegue descobrir os *lifetimes*, o compilador irá parar com um erro. Essas regras se aplicam a definições `fn`, bem como a blocos `impl`.

A primeira regra é que o compilador atribui um parâmetro de *lifetime* a cada parâmetro que seja uma referência. Em outras palavras, uma função com um parâmetro recebe um parâmetro de *lifetime*: `fn foo<'a>(x: &'a i32)`; uma função com dois parâmetros recebe dois parâmetros de *lifetime* separados: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`; e assim por diante.

A segunda regra é que, se houver exatamente um parâmetro de *lifetime* de entrada, esse *lifetime* será atribuído a todos os parâmetros de *lifetime* de saída: `fn foo<'a>(x: &'a i32) -> &'a i32`.

A terceira regra é que, se houver múltiplos parâmetros de *lifetime* de entrada, mas um deles for `&self` ou `&mut self` por se tratar de um método, o *lifetime* de `self` será atribuído a todos os parâmetros de *lifetime* de saída. Essa terceira regra torna os métodos muito mais agradáveis de ler e escrever porque menos símbolos são necessários.

Vamos fingir que somos o compilador. Vamos aplicar essas regras para descobrir os *lifetimes* das referências na assinatura da função `first_word` na Listagem 10-25. A assinatura começa sem nenhum *lifetime* associado às referências:

```rust,ignore
fn first_word(s: &str) -> &str {
```

Em seguida, o compilador aplica a primeira regra, que especifica que cada parâmetro recebe seu próprio *lifetime*. Nós o chamaremos de `'a` como de costume, então agora a assinatura é esta:

```rust,ignore
fn first_word<'a>(s: &'a str) -> &str {
```

A segunda regra se aplica porque há exatamente um *lifetime* de entrada. A segunda regra especifica que o *lifetime* do único parâmetro de entrada é atribuído ao *lifetime* de saída, portanto a assinatura agora é esta:

```rust,ignore
fn first_word<'a>(s: &'a str) -> &'a str {
```

Agora todas as referências nesta assinatura de função têm *lifetimes*, e o compilador pode continuar sua análise sem precisar que o programador anote os *lifetimes* nesta assinatura de função.

Vamos olhar outro exemplo, desta vez usando a função `longest` que não tinha parâmetros de *lifetime* quando começamos a trabalhar com ela na Listagem 10-20:

```rust,ignore
fn longest(x: &str, y: &str) -> &str {
```

Vamos aplicar a primeira regra: cada parâmetro recebe seu próprio *lifetime*. Desta vez, temos dois parâmetros em vez de um, então temos dois *lifetimes*:

```rust,ignore
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```

Você pode ver que a segunda regra não se aplica, porque há mais de um *lifetime* de entrada. A terceira regra também não se aplica, porque `longest` é uma função e não um método, portanto nenhum dos parâmetros é `self`. Depois de trabalhar com todas as três regras, ainda não descobrimos qual é o *lifetime* do tipo de retorno. É por isso que recebemos um erro ao tentar compilar o código na Listagem 10-20: o compilador aplicou as regras de elisão de *lifetime*, mas ainda assim não conseguiu descobrir todos os *lifetimes* das referências na assinatura.

Como a terceira regra realmente se aplica apenas em assinaturas de métodos, veremos os *lifetimes* nesse contexto a seguir para entender por que a terceira regra significa que não precisamos anotar *lifetimes* em assinaturas de métodos com muita frequência.

<!-- Old headings. Do not remove or links may break. -->

<a id="lifetime-annotations-in-method-definitions"></a>

### Em Definições de Método

Quando implementamos métodos em uma *struct* com *lifetimes*, usamos a mesma sintaxe dos parâmetros de tipos genéricos, como mostrado na Listagem 10-11. Onde declaramos e usamos os parâmetros de *lifetime* depende se eles estão relacionados aos campos da *struct* ou aos parâmetros e valores de retorno do método.

Nomes de *lifetime* para campos de *struct* sempre precisam ser declarados após a palavra-chave `impl` e, em seguida, usados após o nome da *struct*, porque esses *lifetimes* fazem parte do tipo da *struct*.

Nas assinaturas de métodos dentro do bloco `impl`, as referências podem estar vinculadas ao *lifetime* das referências nos campos da *struct* ou podem ser independentes. Além disso, as regras de elisão de *lifetime* geralmente fazem com que anotações de *lifetime* não sejam necessárias em assinaturas de métodos. Vamos ver alguns exemplos usando a *struct* chamada `ImportantExcerpt` que definimos na Listagem 10-24.

Primeiro, usaremos um método chamado `level` cujo único parâmetro é uma referência a `self` e cujo valor de retorno é um `i32`, que não é uma referência a nada:

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-10-lifetimes-on-methods/src/main.rs:1st}}
```

A declaração do parâmetro de *lifetime* após `impl` e seu uso após o nome do tipo são obrigatórios, mas por causa da primeira regra de elisão, não somos obrigados a anotar o *lifetime* da referência a `self`.

Aqui está um exemplo em que a terceira regra de elisão de *lifetime* se aplica:

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-10-lifetimes-on-methods/src/main.rs:3rd}}
```

Há dois *lifetimes* de entrada, então Rust aplica a primeira regra de elisão de *lifetime* e atribui a `&self` e `announcement` seus próprios *lifetimes*. Em seguida, como um dos parâmetros é `&self`, o tipo de retorno recebe o *lifetime* de `&self`, e todos os *lifetimes* foram determinados.

<a id="the-static-lifetime"></a>

### O Lifetime Estático ('static)

Um *lifetime* especial que precisamos discutir é o `'static`, que denota que a referência afetada *pode* viver por toda a duração do programa. Todas as strings literais têm o *lifetime* `'static`, que podemos anotar da seguinte maneira:

```rust
let s: &'static str = "I have a static lifetime.";
```

O texto dessa string é armazenado diretamente no binário do programa, que está sempre disponível. Portanto, o *lifetime* de todas as strings literais é `'static`.

Você pode ver sugestões em mensagens de erro para usar o *lifetime* `'static`. Mas antes de especificar `'static` como o *lifetime* para uma referência, pense se a referência que você tem realmente vive por toda a duração do seu programa e se você realmente deseja que seja assim. Na maioria das vezes, uma mensagem de erro sugerindo o *lifetime* `'static` decorre da tentativa de criar uma referência pendente ou de uma incompatibilidade dos *lifetimes* disponíveis. Nesses casos, a solução é corrigir esses problemas, não especificar o *lifetime* `'static`.

<!-- Old headings. Do not remove or links may break. -->

<a id="generic-type-parameters-trait-bounds-and-lifetimes-together"></a>

## Parâmetros de Tipos Genéricos, Trait Bounds e Lifetimes Juntos

Vamos dar uma olhada rápida na sintaxe para especificar parâmetros de tipos genéricos, *trait bounds* e *lifetimes*, tudo em uma única função!

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-11-generics-traits-and-lifetimes/src/main.rs:here}}
```

Esta é a função `longest` da Listagem 10-21 que retorna a maior de duas fatias de string. Mas agora ela tem um parâmetro extra chamado `ann` do tipo genérico `T`, que pode ser preenchido por qualquer tipo que implemente a *trait* `Display`, conforme especificado pela cláusula `where`. Esse parâmetro extra será impresso usando `{}`, razão pela qual o *trait bound* `Display` é necessário. Como *lifetimes* são um tipo de genérico, as declarações do parâmetro de *lifetime* `'a` e do parâmetro de tipo genérico `T` ficam na mesma lista dentro dos colchetes angulares após o nome da função.

<a id="summary"></a>

## Resumo

Cobrimos muita coisa neste capítulo! Agora que você conhece parâmetros de tipos genéricos, *traits* e *trait bounds*, e parâmetros de *lifetime* genéricos, você está pronto para escrever código sem repetição que funcione em muitas situações diferentes. Parâmetros de tipos genéricos permitem aplicar o código a tipos diferentes. *Traits* e *trait bounds* garantem que, mesmo que os tipos sejam genéricos, eles terão o comportamento de que o código precisa. Você aprendeu a usar anotações de *lifetime* para garantir que esse código flexível não terá referências pendentes. E toda essa análise acontece em tempo de compilação, o que não afeta o desempenho em tempo de execução!

Acredite ou não, há muito mais a aprender sobre os tópicos que discutimos neste capítulo: o Capítulo 18 aborda *trait objects*, que são outra maneira de usar *traits*. Também existem cenários mais complexos envolvendo anotações de *lifetime* que você só precisará em casos muito avançados; para esses, você deve ler a [Referência do Rust (*Rust Reference*)][reference]. Mas a seguir, você aprenderá a escrever testes em Rust para ter certeza de que seu código está funcionando da forma como deveria.

[references-and-borrowing]: ch04-02-references-and-borrowing.html#references-and-borrowing
[string-slices-as-parameters]: ch04-03-slices.html#string-slices-as-parameters
[reference]: ../reference/trait-bounds.html
