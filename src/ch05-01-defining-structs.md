<a id="defining-and-instantiating-structs"></a>
## Definindo e Instanciando Structs

Structs são parecidas com tuplas, discutidas na seção [“O Tipo Tupla”][tuples]<!--
ignore -->, pois ambas armazenam múltiplos valores relacionados. Assim como nas
tuplas, os elementos de uma struct podem ter tipos diferentes. A diferença é
que, em uma struct, você dá um nome a cada dado para deixar claro o que cada
valor significa. Adicionar esses nomes torna as structs mais flexíveis que
tuplas: você não precisa depender da ordem dos dados para especificar ou acessar
os valores de uma instância.

Para definir uma struct, usamos a palavra-chave `struct` e damos um nome a ela.
O nome de uma struct deve descrever o significado do conjunto de dados que está
sendo agrupado. Em seguida, entre chaves, definimos os nomes e os tipos de cada
dado, que chamamos de _campos_ (_fields_). Por exemplo, a Listagem 5-1 mostra
uma struct que armazena informações sobre uma conta de usuário.

<Listing number="5-1" file-name="src/main.rs" caption="Definição de uma struct `User`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-01/src/main.rs:here}}
```

</Listing>

Para usar uma struct depois de defini-la, criamos uma _instância_ dela
especificando valores concretos para cada um dos campos. Criamos uma instância
escrevendo o nome da struct seguido de chaves contendo pares _`chave: valor`_,
onde as chaves são os nomes dos campos e os valores são os dados que queremos
armazenar nesses campos. Não precisamos especificar os campos na mesma ordem em
que foram declarados na struct. Em outras palavras, a definição da struct
funciona como um modelo geral para o tipo, e as instâncias preenchem esse modelo
com dados específicos para criar valores desse tipo. Por exemplo, podemos
declarar um usuário específico como mostrado na Listagem 5-2.

<Listing number="5-2" file-name="src/main.rs" caption="Criando uma instância da struct `User`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-02/src/main.rs:here}}
```

</Listing>

Para obter um valor específico de uma struct, usamos a notação de ponto. Por
exemplo, para acessar o endereço de e-mail desse usuário, usamos `user1.email`.
Se a instância for mutável, podemos alterar um valor usando a notação de ponto e
atribuindo um novo valor a um campo específico. A Listagem 5-3 mostra como
alterar o valor do campo `email` de uma instância mutável de `User`.

<Listing number="5-3" file-name="src/main.rs" caption="Alterando o valor no campo `email` de uma instância de `User`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-03/src/main.rs:here}}
```

</Listing>

Note que a instância inteira precisa ser mutável; o Rust não permite marcar
apenas alguns campos como mutáveis. Assim como acontece com qualquer expressão,
podemos construir uma nova instância da struct como a última expressão do corpo
de uma função para retorná-la implicitamente.

A Listagem 5-4 mostra uma função `build_user` que retorna uma instância de
`User` com o e-mail e o nome de usuário fornecidos. O campo `active` recebe o
valor `true` e `sign_in_count` recebe o valor `1`.

<Listing number="5-4" file-name="src/main.rs" caption="Uma função `build_user` que recebe um e-mail e um nome de usuário e retorna uma instância de `User`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-04/src/main.rs:here}}
```

</Listing>

Faz sentido dar aos parâmetros da função os mesmos nomes dos campos da struct,
mas ter que repetir os nomes dos campos e das variáveis `email` e `username` é
um pouco tedioso. Se a struct tivesse mais campos, repetir cada nome ficaria
ainda mais chato. Felizmente, existe um atalho bem prático!

<!-- Old headings. Do not remove or links may break. -->

<a id="using-the-field-init-shorthand-when-variables-and-fields-have-the-same-name"></a>

<a id="using-the-field-init-shorthand"></a>
### Usando o Atalho de Inicialização de Campos

Como os nomes dos parâmetros e dos campos da struct são exatamente iguais na
Listagem 5-4, podemos usar a sintaxe de _atalho de inicialização de campos_
(_field init shorthand_) para reescrever `build_user` de forma que ela se
comporte exatamente igual, mas sem a repetição de `username` e `email`, como
mostrado na Listagem 5-5.

<Listing number="5-5" file-name="src/main.rs" caption="Uma função `build_user` que usa o atalho de inicialização de campos porque os parâmetros `username` e `email` têm o mesmo nome dos campos da struct">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-05/src/main.rs:here}}
```

</Listing>

Aqui, estamos criando uma nova instância da struct `User`, que tem um campo
chamado `email`. Queremos definir o valor do campo `email` com o valor do
parâmetro `email` da função `build_user`. Como o campo `email` e o parâmetro
`email` têm o mesmo nome, só precisamos escrever `email` em vez de
`email: email`.

<!-- Old headings. Do not remove or links may break. -->

<a id="creating-instances-from-other-instances-with-struct-update-syntax"></a>

<a id="creating-instances-with-struct-update-syntax"></a>
### Criando Instâncias com a Sintaxe de Atualização de Struct

Muitas vezes é útil criar uma nova instância de uma struct que aproveite a maior
parte dos valores de outra instância do mesmo tipo, alterando apenas alguns
deles. Você pode fazer isso usando a sintaxe de atualização de struct (_struct
update syntax_).

Primeiro, na Listagem 5-6 mostramos como criar uma nova instância de `User` em
`user2` da forma tradicional, sem a sintaxe de atualização. Definimos um novo
valor para `email`, mas de resto usamos os mesmos valores de `user1` que criamos
na Listagem 5-2.

<Listing number="5-6" file-name="src/main.rs" caption="Criando uma nova instância de `User` usando todos os valores de `user1`, exceto um">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-06/src/main.rs:here}}
```

</Listing>

Usando a sintaxe de atualização de struct, conseguimos o mesmo efeito com menos
código, como mostrado na Listagem 5-7. A sintaxe `..` especifica que os campos
restantes que não foram definidos explicitamente devem receber os mesmos valores
dos campos correspondentes na instância informada.

<Listing number="5-7" file-name="src/main.rs" caption="Usando a sintaxe de atualização de struct para definir um novo valor de `email` para uma instância de `User`, reaproveitando o restante dos valores de `user1`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-07/src/main.rs:here}}
```

</Listing>

O código na Listagem 5-7 também cria uma instância em `user2` que tem um valor
diferente para `email`, mas mantém os mesmos valores de `user1` para os campos
`username`, `active` e `sign_in_count`. O `..user1` precisa vir por último para
indicar que quaisquer campos restantes devem obter seus valores dos campos
correspondentes em `user1`, mas podemos especificar valores para quantos campos
quisermos em qualquer ordem, independentemente da ordem dos campos na definição
da struct.

Note que a sintaxe de atualização de struct usa `=` como uma atribuição; isso
acontece porque ela move os dados, exatamente como vimos na seção [“Variáveis e
Dados Interagindo com Move”][move]<!-- ignore -->. Nesse exemplo, não podemos
mais usar `user1` depois de criar `user2`, porque a `String` no campo `username`
de `user1` foi movida para `user2`. Se tivéssemos dado a `user2` novos valores
de `String` tanto para `email` quanto para `username` — usando de `user1` apenas
os valores de `active` e `sign_in_count` — então `user1` continuaria válido após
a criação de `user2`. Tanto `active` quanto `sign_in_count` são tipos que
implementam a _trait_ `Copy`, então se aplicaria o comportamento que discutimos
na seção [“Dados Apenas na Stack: Copy”][copy]<!-- ignore -->. Também ainda
podemos usar `user1.email` neste exemplo, porque seu valor não foi movido para
fora de `user1`.

<!-- Old headings. Do not remove or links may break. -->

<a id="using-tuple-structs-without-named-fields-to-create-different-types"></a>

<a id="creating-different-types-with-tuple-structs"></a>
### Criando Tipos Diferentes com Tuple Structs

O Rust também suporta structs parecidas com tuplas, chamadas de _tuple structs_.
Tuple structs trazem o significado extra que o nome da struct proporciona, mas
não têm nomes associados aos seus campos; em vez disso, têm apenas os tipos dos
campos. Elas são úteis quando você quer dar um nome à tupla inteira e torná-la
um tipo diferente de outras tuplas, e quando nomear cada campo como em uma
struct comum seria verboso ou redundante.

Para definir uma tuple struct, comece com a palavra-chave `struct` e o nome da
struct seguido pelos tipos da tupla. Por exemplo, aqui definimos e usamos duas
tuple structs chamadas `Color` e `Point`:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-01-tuple-structs/src/main.rs}}
```

</Listing>

Note que os valores `black` e `origin` têm tipos diferentes porque são
instâncias de tuple structs diferentes. Cada struct que você define é um tipo
próprio, mesmo que os campos dentro dela tenham os mesmos tipos. Por exemplo,
uma função que recebe um parâmetro do tipo `Color` não pode receber um `Point`
como argumento, mesmo que ambos os tipos sejam compostos por três valores `i32`.
De resto, instâncias de tuple structs funcionam como tuplas: você pode
desestruturá-las em suas partes individuais e usar `.` seguido do índice para
acessar um valor específico. Diferente das tuplas, porém, tuple structs exigem
que você informe o nome do tipo da struct ao desestruturá-las. Por exemplo,
escreveríamos `let Point(x, y, z) = origin;` para desestruturar os valores do
ponto `origin` em variáveis chamadas `x`, `y` e `z`.

<!-- Old headings. Do not remove or links may break. -->

<a id="unit-like-structs-without-any-fields"></a>

<a id="defining-unit-like-structs"></a>
### Definindo Unit-Like Structs

Você também pode definir structs que não têm nenhum campo! Elas são chamadas de
_unit-like structs_ porque se comportam de maneira semelhante a `()`, o tipo
unitário que mencionamos na seção [“O Tipo Tupla”][tuples]<!-- ignore -->.
Unit-like structs podem ser úteis quando você precisa implementar uma trait em
algum tipo, mas não tem nenhum dado que queira armazenar no tipo em si. Vamos
falar sobre traits no Capítulo 10. Aqui está um exemplo de como declarar e
instanciar uma unit-like struct chamada `AlwaysEqual`:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-04-unit-like-structs/src/main.rs}}
```

</Listing>

Para definir `AlwaysEqual`, usamos a palavra-chave `struct`, o nome que queremos
e um ponto e vírgula. Não precisa de chaves nem parênteses! Depois, podemos
obter uma instância de `AlwaysEqual` na variável `subject` de forma parecida:
usando o nome que definimos, sem chaves nem parênteses. Imagine que mais tarde
vamos implementar um comportamento para esse tipo de modo que toda instância de
`AlwaysEqual` seja sempre igual a qualquer instância de qualquer outro tipo,
talvez para ter um resultado previsível em testes. Não precisaríamos de nenhum
dado para implementar esse comportamento! Você verá no Capítulo 10 como definir
traits e implementá-las em qualquer tipo, incluindo unit-like structs.

> <a id="ownership-of-struct-data"></a>
> ### Ownership dos Dados de uma Struct
>
> Na definição da struct `User` na Listagem 5-1, usamos o tipo `String` (com
> _ownership_) em vez do tipo de _slice_ de string `&str`. Essa foi uma escolha
> deliberada, porque queremos que cada instância dessa struct seja dona de todos
> os seus dados e que esses dados permaneçam válidos enquanto a struct inteira
> for válida.
>
> Também é possível que structs armazenem referências para dados pertencentes a
> outra coisa, mas fazer isso exige o uso de _lifetimes_, um recurso do Rust que
> discutiremos no Capítulo 10. Lifetimes garantem que os dados referenciados por
> uma struct permaneçam válidos enquanto a struct existir. Digamos que você tente
> armazenar uma referência em uma struct sem especificar lifetimes, como no
> exemplo a seguir em *src/main.rs*; isso não vai funcionar:
>
> <Listing file-name="src/main.rs">
>
> <!-- CAN'T EXTRACT SEE https://github.com/rust-lang/mdBook/issues/1127 -->
>
> ```rust,ignore,does_not_compile
> struct User {
>     active: bool,
>     username: &str,
>     email: &str,
>     sign_in_count: u64,
> }
>
> fn main() {
>     let user1 = User {
>         active: true,
>         username: "someusername123",
>         email: "someone@example.com",
>         sign_in_count: 1,
>     };
> }
> ```
>
> </Listing>
>
> O compilador vai reclamar que precisa de especificadores de lifetime:
>
> ```console
> $ cargo run
>    Compiling structs v0.1.0 (file:///projects/structs)
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:3:15
>   |
> 3 |     username: &str,
>   |               ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 ~     username: &'a str,
>   |
>
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:4:12
>   |
> 4 |     email: &str,
>   |            ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 |     username: &str,
> 4 ~     email: &'a str,
>   |
>
> For more information about this error, try `rustc --explain E0106`.
> error: could not compile `structs` (bin "structs") due to 2 previous errors
> ```
>
> No Capítulo 10, vamos ver como corrigir esses erros para poder armazenar
> referências em structs, mas, por enquanto, vamos resolver erros assim usando
> tipos com ownership, como `String`, em vez de referências como `&str`.

<!-- manual-regeneration
for the error above
after running update-rustc.sh:
pbcopy < listings/ch05-using-structs-to-structure-related-data/no-listing-02-reference-in-struct/output.txt
paste above
add `> ` before every line -->

[tuples]: ch03-02-data-types.html#the-tuple-type
[move]: ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move
[copy]: ch04-01-what-is-ownership.html#stack-only-data-copy
