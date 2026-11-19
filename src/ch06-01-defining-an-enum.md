<a id="defining-an-enum"></a>
## Definindo um Enum

Enquanto *structs* oferecem uma forma de agrupar campos e dados relacionados,
como um `Rectangle` com sua `width` e `height`, *enums* oferecem uma forma de
dizer que um valor é um entre um conjunto possível de valores. Por exemplo,
podemos querer dizer que `Rectangle` é uma entre um conjunto de formas possíveis
que também inclui `Circle` e `Triangle`. Para fazer isso, Rust nos permite
codificar essas possibilidades como um enum.

Vamos analisar uma situação que podemos querer expressar em código e ver por que
enums são úteis e mais apropriados que structs nesse caso. Digamos que
precisamos trabalhar com endereços IP. Atualmente, dois padrões principais são
usados para endereços IP: versão quatro e versão seis. Como essas são as únicas
possibilidades para um endereço IP que nosso programa vai encontrar, podemos
*enumerar* todas as variantes possíveis, que é de onde vem o nome enumeração.

Qualquer endereço IP pode ser um endereço versão quatro ou versão seis, mas não
ambos ao mesmo tempo. Essa propriedade dos endereços IP torna a estrutura de
dados enum apropriada, porque um valor de enum só pode ser uma de suas
variantes. Tanto endereços versão quatro quanto versão seis ainda são
fundamentalmente endereços IP, então eles devem ser tratados como o mesmo tipo
quando o código estiver lidando com situações que se aplicam a qualquer tipo de
endereço IP.

Podemos expressar esse conceito em código definindo uma enumeração `IpAddrKind`
e listando os tipos possíveis que um endereço IP pode ser, `V4` e `V6`. Essas
são as variantes do enum:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:def}}
```

`IpAddrKind` agora é um tipo de dados personalizado que podemos usar em outras
partes do nosso código.

<a id="enum-values"></a>
### Valores de Enum

Podemos criar instâncias de cada uma das duas variantes de `IpAddrKind` assim:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:instance}}
```

Note que as variantes do enum ficam no namespace sob o seu identificador, e
usamos dois-pontos duplos (`::`) para separar os dois. Isso é útil porque agora
ambos os valores `IpAddrKind::V4` e `IpAddrKind::V6` são do mesmo tipo:
`IpAddrKind`. Podemos então, por exemplo, definir uma função que recebe qualquer
`IpAddrKind`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:fn}}
```

E podemos chamar essa função com qualquer uma das variantes:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:fn_call}}
```

Usar enums tem ainda mais vantagens. Pensando mais sobre o nosso tipo de
endereço IP, no momento não temos uma forma de armazenar os *dados* reais do
endereço IP; só sabemos qual *tipo* ele é. Como você acabou de aprender sobre
structs no Capítulo 5, pode ficar tentado a resolver esse problema com structs,
como mostrado na Listagem 6-1.

<Listing number="6-1" caption="Armazenando os dados e a variante `IpAddrKind` de um endereço IP usando uma `struct`">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-01/src/main.rs:here}}
```

</Listing>

Aqui, definimos uma struct `IpAddr` que tem dois campos: um campo `kind` do tipo
`IpAddrKind` (o enum que definimos anteriormente) e um campo `address` do tipo
`String`. Temos duas instâncias dessa struct. A primeira é `home`, que tem o
valor `IpAddrKind::V4` como seu `kind` com os dados de endereço associados
`127.0.0.1`. A segunda instância é `loopback`. Ela tem a outra variante de
`IpAddrKind` como seu valor `kind`, `V6`, e tem o endereço `::1` associado a
ela. Usamos uma struct para agrupar os valores `kind` e `address`, então agora a
variante está associada ao valor.

No entanto, representar o mesmo conceito usando apenas um enum é mais conciso:
em vez de um enum dentro de uma struct, podemos colocar dados diretamente em
cada variante do enum. Essa nova definição do enum `IpAddr` diz que ambas as
variantes `V4` e `V6` terão valores `String` associados:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-02-enum-with-data/src/main.rs:here}}
```

Anexamos dados a cada variante do enum diretamente, então não há necessidade de
uma struct extra. Aqui também fica mais fácil ver outro detalhe de como enums
funcionam: o nome de cada variante de enum que definimos também se torna uma
função que constrói uma instância do enum. Ou seja, `IpAddr::V4()` é uma chamada
de função que recebe um argumento `String` e retorna uma instância do tipo
`IpAddr`. Ganhamos essa função construtora automaticamente como resultado de
definir o enum.

Há outra vantagem em usar um enum em vez de uma struct: cada variante pode ter
diferentes tipos e quantidades de dados associados. Endereços IP versão quatro
sempre terão quatro componentes numéricos com valores entre 0 e 255. Se
quiséssemos armazenar endereços `V4` como quatro valores `u8`, mas ainda
expressar endereços `V6` como um único valor `String`, não conseguiríamos fazer
isso com uma struct. Enums lidam com esse caso com facilidade:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-03-variants-with-different-data/src/main.rs:here}}
```

Mostramos várias formas diferentes de definir estruturas de dados para armazenar
endereços IP versão quatro e versão seis. Contudo, acontece que querer armazenar
endereços IP e codificar de qual tipo eles são é tão comum que [a biblioteca
padrão tem uma definição que podemos usar!][IpAddr]<!-- ignore --> Vamos ver
como a biblioteca padrão define `IpAddr`. Ela tem exatamente o enum e as
variantes que definimos e usamos, mas incorpora os dados do endereço dentro das
variantes na forma de duas structs diferentes, definidas de forma distinta para
cada variante:

```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

Esse código ilustra que você pode colocar qualquer tipo de dado dentro de uma
variante de enum: strings, tipos numéricos ou structs, por exemplo. Você pode
até incluir outro enum! Além disso, os tipos da biblioteca padrão muitas vezes
não são muito mais complicados do que o que você mesmo criaria.

Note que, mesmo que a biblioteca padrão contenha uma definição para `IpAddr`,
ainda podemos criar e usar nossa própria definição sem conflito porque não
trouxemos a definição da biblioteca padrão para o nosso escopo. Falaremos mais
sobre trazer tipos para o escopo no Capítulo 7.

Vamos ver outro exemplo de enum na Listagem 6-2: este tem uma grande variedade
de tipos incorporados em suas variantes.

<Listing number="6-2" caption="Um enum `Message` cujas variantes armazenam diferentes quantidades e tipos de valores">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-02/src/main.rs:here}}
```

</Listing>

Esse enum tem quatro variantes com tipos diferentes:

- `Quit`: Não tem nenhum dado associado a ela
- `Move`: Tem campos nomeados, assim como uma struct
- `Write`: Inclui uma única `String`
- `ChangeColor`: Inclui três valores `i32`

Definir um enum com variantes como as da Listagem 6-2 é semelhante a definir
diferentes tipos de structs, exceto que o enum não usa a palavra-chave `struct`
e todas as variantes ficam agrupadas sob o tipo `Message`. As structs a seguir
poderiam guardar os mesmos dados que as variantes do enum anterior guardam:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-04-structs-similar-to-message-enum/src/main.rs:here}}
```

Mas se usássemos as diferentes structs, cada uma com seu próprio tipo, não
poderíamos definir tão facilmente uma função que recebesse qualquer um desses
tipos de mensagens como conseguimos com o enum `Message` definido na Listagem
6-2, que é um único tipo.

Há mais uma semelhança entre enums e structs: assim como podemos definir métodos
em structs usando `impl`, também podemos definir métodos em enums. Aqui está um
método chamado `call` que poderíamos definir no nosso enum `Message`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-05-methods-on-enums/src/main.rs:here}}
```

O corpo do método usaria `self` para obter o valor no qual chamamos o método.
Neste exemplo, criamos uma variável `m` que tem o valor
`Message::Write(String::from("hello"))`, e é isso que `self` será no corpo do
método `call` quando `m.call()` for executado.

Vamos olhar para outro enum na biblioteca padrão que é muito comum e útil:
`Option`.

<!-- Old headings. Do not remove or links may break. -->

<a id="the-option-enum-and-its-advantages-over-null-values"></a>
<a id="the-option-enum"></a>
### O Enum `Option`

Esta seção explora um estudo de caso de `Option`, que é outro enum definido pela
biblioteca padrão. O tipo `Option` codifica o cenário muito comum em que um
valor pode ser alguma coisa ou pode ser nada.

Por exemplo, se você pedir o primeiro item de uma lista não vazia, receberá um
valor. Se pedir o primeiro item de uma lista vazia, não receberá nada. Expressar
esse conceito em termos do sistema de tipos significa que o compilador pode
verificar se você tratou todos os casos que deveria tratar; essa funcionalidade
pode evitar bugs extremamente comuns em outras linguagens de programação.

O design de linguagens de programação é frequentemente pensado em termos de
quais recursos você inclui, mas os recursos que você exclui também são
importantes. Rust não tem o recurso de nulo (*null*) que muitas outras
linguagens têm. *Null* é um valor que significa que não há nenhum valor ali. Em
linguagens com null, variáveis podem estar sempre em um de dois estados: nulo ou
não nulo.

Em sua apresentação de 2009 “Null References: The Billion Dollar Mistake”
(“Referências Nulas: O Erro de Um Bilhão de Dólares”), Tony Hoare, o inventor do
null, disse o seguinte:

> Eu chamo isso de meu erro de um bilhão de dólares. Naquela época, eu estava
> projetando o primeiro sistema de tipos abrangente para referências em uma
> linguagem orientada a objetos. Meu objetivo era garantir que todo uso de
> referências fosse absolutamente seguro, com verificação feita automaticamente
> pelo compilador. Mas não resisti à tentação de colocar uma referência nula,
> simplesmente porque era muito fácil de implementar. Isso levou a inúmeros
> erros, vulnerabilidades e falhas de sistema, que provavelmente causaram um
> bilhão de dólares em dor e prejuízo nos últimos quarenta anos.

O problema com valores nulos é que, se você tentar usar um valor nulo como um
valor não nulo, receberá algum tipo de erro. Como essa propriedade de nulo ou
não nulo é onipresente, é extremamente fácil cometer esse tipo de erro.

No entanto, o conceito que o null está tentando expressar ainda é útil: um null
é um valor que está atualmente inválido ou ausente por algum motivo.

O problema não está realmente no conceito, mas na implementação específica. Sendo
assim, Rust não tem nulos, mas tem um enum que pode codificar o conceito de um
valor estar presente ou ausente. Esse enum é `Option<T>`, e ele é [definido pela
biblioteca padrão][option]<!-- ignore --> da seguinte forma:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

O enum `Option<T>` é tão útil que até está incluído no prelúdio (*prelude*);
você não precisa trazê-lo para o escopo explicitamente. Suas variantes também
estão incluídas no prelúdio: você pode usar `Some` e `None` diretamente sem o
prefixo `Option::`. O enum `Option<T>` ainda é apenas um enum normal, e
`Some(T)` e `None` ainda são variantes do tipo `Option<T>`.

A sintaxe `<T>` é um recurso de Rust sobre o qual ainda não falamos. É um
parâmetro de tipo genérico, e vamos abordar genéricos com mais detalhes no
Capítulo 10. Por enquanto, tudo o que você precisa saber é que `<T>` significa
que a variante `Some` do enum `Option` pode guardar um dado de qualquer tipo, e
que cada tipo concreto usado no lugar de `T` torna o tipo `Option<T>` geral um
tipo diferente. Aqui estão alguns exemplos de uso de valores `Option` para
guardar tipos numéricos e tipos char:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-06-option-examples/src/main.rs:here}}
```

O tipo de `some_number` é `Option<i32>`. O tipo de `some_char` é
`Option<char>`, que é um tipo diferente. Rust consegue inferir esses tipos
porque especificamos um valor dentro da variante `Some`. Para `absent_number`,
Rust exige que anotemos o tipo `Option` completo: o compilador não consegue
inferir o tipo que a variante `Some` correspondente guardará olhando apenas para
um valor `None`. Aqui, dizemos ao Rust que queremos que `absent_number` seja do
tipo `Option<i32>`.

Quando temos um valor `Some`, sabemos que um valor está presente e que esse
valor está guardado dentro do `Some`. Quando temos um valor `None`, de certa
forma significa a mesma coisa que null: não temos um valor válido. Então, por
que ter `Option<T>` é melhor do que ter null?

Em resumo, porque `Option<T>` e `T` (onde `T` pode ser qualquer tipo) são tipos
diferentes, o compilador não nos deixará usar um valor `Option<T>` como se ele
fosse definitivamente um valor válido. Por exemplo, este código não compila,
porque está tentando somar um `i8` com um `Option<i8>`:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-07-cant-use-option-directly/src/main.rs:here}}
```

Se executarmos esse código, recebemos uma mensagem de erro como esta:

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-07-cant-use-option-directly/output.txt}}
```

Intenso! Na prática, essa mensagem de erro significa que Rust não entende como
somar um `i8` e um `Option<i8>`, porque eles são tipos diferentes. Quando temos
um valor de um tipo como `i8` em Rust, o compilador garante que sempre temos um
valor válido. Podemos prosseguir com confiança sem precisar verificar se é nulo
antes de usar esse valor. Somente quando temos um `Option<i8>` (ou qualquer
outro tipo de valor com o qual estejamos trabalhando) é que precisamos nos
preocupar com a possibilidade de não ter um valor, e o compilador garantirá que
tratemos esse caso antes de usar o valor.

Em outras palavras, você precisa converter um `Option<T>` em um `T` antes de
poder realizar operações de `T` com ele. Em geral, isso ajuda a evitar um dos
problemas mais comuns com null: assumir que algo não é nulo quando na verdade é.

Eliminar o risco de assumir incorretamente um valor não nulo ajuda você a ter
mais confiança no seu código. Para ter um valor que pode possivelmente ser nulo,
você precisa optar explicitamente por isso tornando o tipo desse valor
`Option<T>`. Então, ao usar esse valor, você é obrigado a tratar explicitamente
o caso em que o valor é nulo. Em qualquer lugar onde um valor tenha um tipo que
não seja `Option<T>`, você *pode* assumir com segurança que o valor não é nulo.
Essa foi uma decisão de design deliberada do Rust para limitar a onipresença do
null e aumentar a segurança do código Rust.

Então, como você extrai o valor `T` de uma variante `Some` quando tem um valor
do tipo `Option<T>` para poder usá-lo? O enum `Option<T>` tem um grande número
de métodos que são úteis em diversas situações; você pode conferi-los em [sua
documentação][docs]<!-- ignore -->. Familiarizar-se com os métodos de
`Option<T>` será extremamente útil na sua jornada com Rust.

Em geral, para usar um valor `Option<T>`, você quer ter um código que trate cada
variante. Você quer um código que execute apenas quando tiver um valor
`Some(T)`, e esse código pode usar o `T` interno. Você quer outro código que
execute apenas se tiver um valor `None`, e esse código não tem um valor `T`
disponível. A expressão `match` é uma construção de fluxo de controle que faz
exatamente isso quando usada com enums: ela executará códigos diferentes
dependendo de qual variante do enum possui, e esse código pode usar os dados
dentro do valor correspondente.

[IpAddr]: ../std/net/enum.IpAddr.html
[option]: ../std/option/enum.Option.html
[docs]: ../std/option/enum.Option.html
