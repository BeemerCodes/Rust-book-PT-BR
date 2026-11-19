<a id="generic-data-types"></a>

## Tipos de Dados Genéricos

Usamos *generics* para criar definições para itens como assinaturas de função ou *structs*, que podemos então usar com muitos tipos de dados concretos diferentes. Vamos primeiro ver como definir funções, *structs*, *enums* e métodos usando *generics*. Em seguida, discutiremos como *generics* afetam o desempenho do código.

<a id="in-function-definitions"></a>

### Em Definições de Função

Ao definir uma função que usa *generics*, colocamos os tipos genéricos na assinatura da função onde normalmente especificaríamos os tipos de dados dos parâmetros e do valor de retorno. Fazer isso torna nosso código mais flexível e oferece mais funcionalidades aos chamadores da função, ao mesmo tempo em que evita a duplicação de código.

Continuando com a nossa função `largest`, a Listagem 10-4 mostra duas funções que encontram o maior valor em um *slice*. Depois, vamos combiná-las em uma única função que usa *generics*.

<Listing number="10-4" file-name="src/main.rs" caption="Duas funções que diferem apenas em seus nomes e nos tipos de suas assinaturas">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-04/src/main.rs:here}}
```

</Listing>

A função `largest_i32` é a que extraímos na Listagem 10-3, que encontra o maior `i32` em um *slice*. A função `largest_char` encontra o maior `char` em um *slice*. Os corpos das funções têm o mesmo código, então vamos eliminar a duplicação introduzindo um parâmetro de tipo genérico em uma única função.

Para parametrizar os tipos em uma nova função única, precisamos dar um nome ao parâmetro de tipo, assim como fazemos para os parâmetros de valor de uma função. Você pode usar qualquer identificador como nome de parâmetro de tipo. Mas usaremos `T` porque, por convenção, os nomes de parâmetros de tipo em Rust são curtos, geralmente com apenas uma letra, e a convenção de nomenclatura de tipos de Rust é UpperCamelCase. Abreviação de *type* (tipo), `T` é a escolha padrão da maioria dos programadores Rust.

Quando usamos um parâmetro no corpo da função, temos que declarar o nome do parâmetro na assinatura para que o compilador saiba o que esse nome significa. Da mesma forma, quando usamos um nome de parâmetro de tipo em uma assinatura de função, temos que declarar o nome do parâmetro de tipo antes de usá-lo. Para definir a função genérica `largest`, colocamos declarações de nomes de tipos dentro de colchetes angulares (sinais de menor e maior), `<>`, entre o nome da função e a lista de parâmetros, desta forma:

```rust,ignore
fn largest<T>(list: &[T]) -> &T {
```

Lemos essa definição como "A função `largest` é genérica sobre algum tipo `T`." Essa função tem um parâmetro chamado `list`, que é um *slice* de valores do tipo `T`. A função `largest` retornará uma referência a um valor do mesmo tipo `T`.

A Listagem 10-5 mostra a definição combinada da função `largest` usando o tipo de dados genérico em sua assinatura. A listagem também mostra como podemos chamar a função com um *slice* de valores `i32` ou valores `char`. Note que esse código ainda não vai compilar.

<Listing number="10-5" file-name="src/main.rs" caption="A função `largest` usando parâmetros de tipos genéricos; isso ainda não compila">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/src/main.rs}}
```

</Listing>

Se compilarmos esse código agora, obteremos este erro:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/output.txt}}
```

O texto de ajuda menciona `std::cmp::PartialOrd`, que é uma *trait*, e vamos falar sobre *traits* na próxima seção. Por enquanto, saiba que esse erro afirma que o corpo de `largest` não funcionará para todos os tipos possíveis que `T` poderia ser. Como queremos comparar valores do tipo `T` no corpo, só podemos usar tipos cujos valores possam ser ordenados. Para permitir comparações, a biblioteca padrão tem a *trait* `std::cmp::PartialOrd` que você pode implementar em tipos (veja o Apêndice C para mais detalhes sobre essa *trait*). Para corrigir a Listagem 10-5, podemos seguir a sugestão do texto de ajuda e restringir os tipos válidos para `T` apenas àqueles que implementam `PartialOrd`. A listagem então compilará, porque a biblioteca padrão implementa `PartialOrd` tanto para `i32` quanto para `char`.

<a id="in-struct-definitions"></a>

### Em Definições de Struct

Também podemos definir *structs* para usar um parâmetro de tipo genérico em um ou mais campos usando a sintaxe `<>`. A Listagem 10-6 define uma *struct* `Point<T>` para conter valores de coordenadas `x` e `y` de qualquer tipo.

<Listing number="10-6" file-name="src/main.rs" caption="Uma *struct* `Point<T>` que contém valores `x` e `y` do tipo `T`">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-06/src/main.rs}}
```

</Listing>

A sintaxe para usar *generics* em definições de *struct* é semelhante à usada em definições de função. Primeiro, declaramos o nome do parâmetro de tipo dentro de colchetes angulares logo após o nome da *struct*. Em seguida, usamos o tipo genérico na definição da *struct* onde, de outra forma, especificaríamos tipos de dados concretos.

Note que, como usamos apenas um tipo genérico para definir `Point<T>`, essa definição diz que a *struct* `Point<T>` é genérica sobre algum tipo `T`, e os campos `x` e `y` são *ambos* desse mesmo tipo, seja qual for esse tipo. Se criarmos uma instância de um `Point<T>` que contenha valores de tipos diferentes, como na Listagem 10-7, nosso código não compilará.

<Listing number="10-7" file-name="src/main.rs" caption="Os campos `x` e `y` devem ser do mesmo tipo porque ambos têm o mesmo tipo de dados genérico `T`.">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/src/main.rs}}
```

</Listing>

Neste exemplo, quando atribuímos o valor inteiro `5` a `x`, informamos ao compilador que o tipo genérico `T` será um inteiro para esta instância de `Point<T>`. Em seguida, quando especificamos `4.0` para `y`, que definimos como tendo o mesmo tipo de `x`, obteremos um erro de incompatibilidade de tipos como este:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/output.txt}}
```

Para definir uma *struct* `Point` onde `x` e `y` sejam ambos genéricos, mas possam ter tipos diferentes, podemos usar múltiplos parâmetros de tipo genérico. Por exemplo, na Listagem 10-8, alteramos a definição de `Point` para ser genérica sobre os tipos `T` e `U`, onde `x` é do tipo `T` e `y` é do tipo `U`.

<Listing number="10-8" file-name="src/main.rs" caption="Um `Point<T, U>` genérico sobre dois tipos para que `x` e `y` possam ser valores de tipos diferentes">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-08/src/main.rs}}
```

</Listing>

Agora todas as instâncias de `Point` mostradas são permitidas! Você pode usar quantos parâmetros de tipos genéricos desejar em uma definição, mas usar mais do que alguns torna seu código difícil de ler. Se você perceber que precisa de muitos tipos genéricos no seu código, isso pode indicar que seu código precisa ser reestruturado em partes menores.

<a id="in-enum-definitions"></a>

### Em Definições de Enum

Assim como fizemos com *structs*, podemos definir *enums* para conter tipos de dados genéricos em suas variantes. Vamos dar outra olhada no *enum* `Option<T>` fornecido pela biblioteca padrão, que usamos no Capítulo 6:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Essa definição agora deve fazer mais sentido para você. Como pode ver, o *enum* `Option<T>` é genérico sobre o tipo `T` e tem duas variantes: `Some`, que contém um valor do tipo `T`, e uma variante `None` que não contém nenhum valor. Ao usar o *enum* `Option<T>`, podemos expressar o conceito abstrato de um valor opcional e, como `Option<T>` é genérico, podemos usar essa abstração independentemente de qual seja o tipo do valor opcional.

*Enums* também podem usar múltiplos tipos genéricos. A definição do *enum* `Result` que usamos no Capítulo 9 é um exemplo:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

O *enum* `Result` é genérico sobre dois tipos, `T` e `E`, e possui duas variantes: `Ok`, que contém um valor do tipo `T`, e `Err`, que contém um valor do tipo `E`. Essa definição torna conveniente o uso do *enum* `Result` em qualquer lugar onde tenhamos uma operação que possa ser bem-sucedida (retornar um valor de algum tipo `T`) ou falhar (retornar um erro de algum tipo `E`). Na verdade, foi isso o que usamos para abrir um arquivo na Listagem 9-3, onde `T` foi preenchido com o tipo `std::fs::File` quando o arquivo foi aberto com sucesso e `E` foi preenchido com o tipo `std::io::Error` quando houve problemas ao abrir o arquivo.

Quando você reconhecer situações no seu código com múltiplas definições de *struct* ou *enum* que diferem apenas nos tipos dos valores que contêm, poderá evitar a duplicação usando tipos genéricos em vez disso.

<a id="in-method-definitions"></a>

### Em Definições de Método

Podemos implementar métodos em *structs* e *enums* (como fizemos no Capítulo 5) e usar tipos genéricos em suas definições também. A Listagem 10-9 mostra a *struct* `Point<T>` que definimos na Listagem 10-6 com um método chamado `x` implementado nela.

<Listing number="10-9" file-name="src/main.rs" caption="Implementando um método chamado `x` na *struct* `Point<T>` que retornará uma referência ao campo `x` do tipo `T`">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-09/src/main.rs}}
```

</Listing>

Aqui, definimos um método chamado `x` em `Point<T>` que retorna uma referência aos dados no campo `x`.

Note que temos que declarar `T` logo após `impl` para que possamos usar `T` para especificar que estamos implementando métodos no tipo `Point<T>`. Ao declarar `T` como um tipo genérico após `impl`, Rust pode identificar que o tipo dentro dos colchetes angulares em `Point` é um tipo genérico em vez de um tipo concreto. Poderíamos ter escolhido um nome diferente para esse parâmetro genérico em relação ao parâmetro genérico declarado na definição da *struct*, mas usar o mesmo nome é convencional. Se você escrever um método dentro de um `impl` que declara um tipo genérico, esse método será definido em qualquer instância do tipo, não importa qual tipo concreto acabe substituindo o tipo genérico.

Também podemos especificar restrições em tipos genéricos ao definir métodos no tipo. Poderíamos, por exemplo, implementar métodos apenas em instâncias de `Point<f32>` em vez de instâncias de `Point<T>` com qualquer tipo genérico. Na Listagem 10-10, usamos o tipo concreto `f32`, o que significa que não declaramos nenhum tipo após `impl`.

<Listing number="10-10" file-name="src/main.rs" caption="Um bloco `impl` que só se aplica a uma *struct* com um tipo concreto específico para o parâmetro de tipo genérico `T`">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-10/src/main.rs:here}}
```

</Listing>

Esse código significa que o tipo `Point<f32>` terá um método `distance_from_origin`; outras instâncias de `Point<T>` onde `T` não é do tipo `f32` não terão esse método definido. O método mede a distância do nosso ponto até o ponto nas coordenadas (0.0, 0.0) e usa operações matemáticas disponíveis apenas para tipos de ponto flutuante.

Parâmetros de tipos genéricos na definição de uma *struct* nem sempre são os mesmos usados nas assinaturas de métodos dessa mesma *struct*. A Listagem 10-11 usa os tipos genéricos `X1` e `Y1` para a *struct* `Point` e `X2` e `Y2` para a assinatura do método `mixup` para tornar o exemplo mais claro. O método cria uma nova instância de `Point` com o valor `x` do `Point` `self` (do tipo `X1`) e o valor `y` do `Point` passado (do tipo `Y2`).

<Listing number="10-11" file-name="src/main.rs" caption="Um método que usa tipos genéricos diferentes da definição de sua *struct*">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-11/src/main.rs}}
```

</Listing>

Em `main`, definimos um `Point` que tem um `i32` para `x` (com valor `5`) e um `f64` para `y` (com valor `10.4`). A variável `p2` é uma *struct* `Point` que tem um *string slice* para `x` (com valor `"Hello"`) e um `char` para `y` (com valor `c`). Chamar `mixup` em `p1` com o argumento `p2` nos dá `p3`, que terá um `i32` para `x` porque `x` veio de `p1`. A variável `p3` terá um `char` para `y` porque `y` veio de `p2`. A chamada da macro `println!` imprimirá `p3.x = 5, p3.y = c`.

O objetivo deste exemplo é demonstrar uma situação na qual alguns parâmetros genéricos são declarados com `impl` e outros são declarados com a definição do método. Aqui, os parâmetros genéricos `X1` e `Y1` são declarados após `impl` porque acompanham a definição da *struct*. Os parâmetros genéricos `X2` e `Y2` são declarados após `fn mixup` porque são relevantes apenas para o método.

<a id="performance-of-code-using-generics"></a>

### Desempenho do Código Usando Generics

Você pode estar se perguntando se há um custo de tempo de execução ao usar parâmetros de tipos genéricos. A boa notícia é que o uso de tipos genéricos não fará seu programa rodar mais devagar do que rodaria com tipos concretos.

Rust consegue isso realizando a monomorfização do código que usa *generics* em tempo de compilação. *Monomorfização* (*monomorphization*) é o processo de transformar código genérico em código específico, preenchendo os tipos concretos que são usados quando compilado. Nesse processo, o compilador faz o inverso dos passos que usamos para criar a função genérica na Listagem 10-5: o compilador examina todos os locais onde o código genérico é chamado e gera código para os tipos concretos com os quais o código genérico foi chamado.

Vamos ver como isso funciona usando o *enum* genérico `Option<T>` da biblioteca padrão:

```rust
let integer = Some(5);
let float = Some(5.0);
```

Quando Rust compila esse código, ele realiza a monomorfização. Durante esse processo, o compilador lê os valores usados nas instâncias de `Option<T>` e identifica dois tipos de `Option<T>`: um é `i32` e o outro é `f64`. Dessa forma, ele expande a definição genérica de `Option<T>` em duas definições especializadas para `i32` e `f64`, substituindo assim a definição genérica pelas específicas.

A versão monomorfizada do código fica semelhante ao seguinte (o compilador usa nomes diferentes dos que estamos usando aqui para ilustração):

<Listing file-name="src/main.rs">

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

</Listing>

O `Option<T>` genérico é substituído pelas definições específicas criadas pelo compilador. Como Rust compila código genérico em código que especifica o tipo em cada instância, não pagamos nenhum custo em tempo de execução por usar *generics*. Quando o código é executado, ele se comporta exatamente como se tivéssemos duplicado cada definição manualmente. O processo de monomorfização torna os *generics* de Rust extremamente eficientes em tempo de execução.
