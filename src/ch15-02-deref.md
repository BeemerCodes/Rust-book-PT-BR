<!-- Old headings. Do not remove or links may break. -->

<a id="treating-smart-pointers-like-regular-references-with-the-deref-trait"></a>
<a id="treating-smart-pointers-like-regular-references-with-deref"></a>

<a id="treating-smart-pointers-like-regular-references"></a>
## Tratando *Smart Pointers* como Referências Regulares

Implementar a *trait* `Deref` permite que você personalize o comportamento do
*operador de desreferência* `*` (não confundir com o operador de multiplicação
ou *glob*). Ao implementar `Deref` de tal forma que um *smart pointer* possa ser
tratado como uma referência regular, você pode escrever código que opere com
referências e usar esse código também com *smart pointers*.

Vamos primeiro ver como o operador de desreferência funciona com referências
regulares. Em seguida, tentaremos definir um tipo personalizado que se comporte
como `Box<T>` e ver por que o operador de desreferência não funciona como uma
referência em nosso tipo recém-definido. Exploraremos como implementar a *trait*
`Deref` torna possível que *smart pointers* funcionem de maneiras similares às
referências. Em seguida, veremos o recurso de *coerção deref* (*deref coercion*)
do Rust e como ele nos permite trabalhar tanto com referências quanto com *smart
pointers*.

<!-- Old headings. Do not remove or links may break. -->

<a id="following-the-pointer-to-the-value-with-the-dereference-operator"></a>
<a id="following-the-pointer-to-the-value"></a>

<a id="following-the-reference-to-the-value"></a>
### Seguindo a Referência até o Valor

Uma referência regular é um tipo de ponteiro, e uma maneira de pensar em um
ponteiro é como uma seta para um valor armazenado em outro lugar. Na Listagem
15-6, criamos uma referência para um valor `i32` e então usamos o operador de
desreferência para seguir a referência até o valor.

<Listing number="15-6" file-name="src/main.rs" caption="Usando o operador de desreferência para seguir uma referência a um valor `i32`">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-06/src/main.rs}}
```

</Listing>

A variável `x` armazena um valor `i32` igual a `5`. Definimos `y` como uma
referência a `x`. Podemos afirmar que `x` é igual a `5`. No entanto, se
quisermos fazer uma asserção sobre o valor em `y`, temos que usar `*y` para
seguir a referência até o valor para o qual ela aponta (portanto, *desreferenciar*)
para que o compilador possa comparar o valor real. Assim que desreferenciamos `y`,
temos acesso ao valor inteiro para o qual `y` aponta, que podemos comparar com `5`.

Se tentássemos escrever `assert_eq!(5, y);` em vez disso, obteríamos este erro
de compilação:

```console
{{#include ../listings/ch15-smart-pointers/output-only-01-comparing-to-reference/output.txt}}
```

Comparar um número e uma referência a um número não é permitido porque são tipos
diferentes. Precisamos usar o operador de desreferência para seguir a referência
até o valor para o qual ela aponta.

<a id="using-boxt-like-a-reference"></a>
### Usando `Box<T>` como uma Referência

Podemos reescrever o código da Listagem 15-6 para usar um `Box<T>` em vez de uma
referência; o operador de desreferência usado no `Box<T>` na Listagem 15-7
funciona da mesma forma que o operador de desreferência usado na referência da
Listagem 15-6.

<Listing number="15-7" file-name="src/main.rs" caption="Usando o operador de desreferência em um `Box<i32>`">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-07/src/main.rs}}
```

</Listing>

A principal diferença entre a Listagem 15-7 e a Listagem 15-6 é que aqui
definimos `y` como uma instância de um *box* apontando para um valor copiado de
`x`, em vez de uma referência apontando para o valor de `x`. Na última asserção,
podemos usar o operador de desreferência para seguir o ponteiro do *box* da mesma
forma que fizemos quando `y` era uma referência. Em seguida, exploraremos o que
é especial no `Box<T>` que nos permite usar o operador de desreferência,
definindo nosso próprio tipo de *box*.

<a id="defining-our-own-smart-pointer"></a>
### Definindo Nosso Próprio *Smart Pointer*

Vamos criar um tipo *wrapper* semelhante ao tipo `Box<T>` fornecido pela
biblioteca padrão, para experimentar como os tipos de *smart pointer* se
comportam de forma diferente das referências por padrão. Então, veremos como
adicionar a capacidade de usar o operador de desreferência.

> Nota: Há uma grande diferença entre o tipo `MyBox<T>` que estamos prestes a
> criar e o `Box<T>` real: nossa versão não armazenará seus dados no *heap*.
> Estamos focando este exemplo no `Deref`, portanto onde os dados são realmente
> armazenados é menos importante do que o comportamento semelhante a um ponteiro.

O tipo `Box<T>` é definido em última análise como uma *struct* de tupla com um
elemento, portanto a Listagem 15-8 define um tipo `MyBox<T>` da mesma forma.
Também definiremos uma função `new` para corresponder à função `new` definida em
`Box<T>`.

<Listing number="15-8" file-name="src/main.rs" caption="Definindo um tipo `MyBox<T>`">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-08/src/main.rs:here}}
```

</Listing>

Definimos uma *struct* chamada `MyBox` e declaramos um parâmetro genérico `T`
porque queremos que nosso tipo armazene valores de qualquer tipo. O tipo `MyBox`
é uma *struct* de tupla com um elemento do tipo `T`. A função `MyBox::new` recebe
um parâmetro do tipo `T` e retorna uma instância de `MyBox` que armazena o valor
passado.

Vamos tentar adicionar a função `main` da Listagem 15-7 à Listagem 15-8 e
alterá-la para usar o tipo `MyBox<T>` que definimos em vez de `Box<T>`. O código
da Listagem 15-9 não compilará, porque o Rust não sabe como desreferenciar
`MyBox`.

<Listing number="15-9" file-name="src/main.rs" caption="Tentando usar `MyBox<T>` da mesma forma que usamos referências e `Box<T>`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-09/src/main.rs:here}}
```

</Listing>

Aqui está o erro de compilação resultante:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-09/output.txt}}
```

Nosso tipo `MyBox<T>` não pode ser desreferenciado porque não implementamos essa
capacidade em nosso tipo. Para habilitar a desreferência com o operador `*`,
implementamos a *trait* `Deref`.

<!-- Old headings. Do not remove or links may break. -->

<a id="treating-a-type-like-a-reference-by-implementing-the-deref-trait"></a>

<a id="implementing-the-deref-trait"></a>
### Implementando a *Trait* `Deref`

Conforme discutido em ["Implementando uma *Trait* em um Tipo"][impl-trait]<!-- ignore -->
no Capítulo 10, para implementar uma *trait* precisamos fornecer implementações
para os métodos exigidos pela *trait*. A *trait* `Deref`, fornecida pela
biblioteca padrão, nos exige que implementemos um método chamado `deref` que faz
*borrowing* de `self` e retorna uma referência para os dados internos. A Listagem
15-10 contém uma implementação de `Deref` para adicionar à definição de
`MyBox<T>`.

<Listing number="15-10" file-name="src/main.rs" caption="Implementando `Deref` em `MyBox<T>`">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-10/src/main.rs:here}}
```

</Listing>

A sintaxe `type Target = T;` define um tipo associado para a *trait* `Deref`
usar. Tipos associados são uma forma ligeiramente diferente de declarar um
parâmetro genérico, mas você não precisa se preocupar com eles agora; os
abordaremos com mais detalhes no Capítulo 20.

Preenchemos o corpo do método `deref` com `&self.0` para que `deref` retorne uma
referência ao valor que queremos acessar com o operador `*`; lembre-se de
["Criando Tipos Diferentes com *Structs* de Tupla"][tuple-structs]<!-- ignore -->
no Capítulo 5 que `.0` acessa o primeiro valor em uma *struct* de tupla. A
função `main` da Listagem 15-9 que chama `*` no valor `MyBox<T>` agora compila,
e as asserções passam!

Sem a *trait* `Deref`, o compilador só pode desreferenciar referências `&`. O
método `deref` dá ao compilador a capacidade de pegar um valor de qualquer tipo
que implementa `Deref` e chamar o método `deref` para obter uma referência que
ele sabe como desreferenciar.

Quando inserimos `*y` na Listagem 15-9, por baixo dos panos o Rust na verdade
executou este código:

```rust,ignore
*(y.deref())
```

O Rust substitui o operador `*` por uma chamada ao método `deref` e então uma
desreferência simples, para que não precisemos pensar se precisamos ou não chamar
o método `deref`. Esse recurso do Rust nos permite escrever código que funciona
de forma idêntica, seja com uma referência regular ou um tipo que implementa
`Deref`.

A razão pela qual o método `deref` retorna uma referência a um valor, e que a
desreferência simples fora dos parênteses em `*(y.deref())` ainda é necessária,
tem a ver com o sistema de *ownership*. Se o método `deref` retornasse o valor
diretamente em vez de uma referência ao valor, o valor seria movido para fora de
`self`. Não queremos assumir a propriedade (*ownership*) do valor interno dentro
de `MyBox<T>` nesse caso nem na maioria dos casos em que usamos o operador de
desreferência.

Observe que o operador `*` é substituído por uma chamada ao método `deref` e
depois uma chamada ao operador `*` apenas uma vez, cada vez que usamos um `*` em
nosso código. Como a substituição do operador `*` não recursa infinitamente,
terminamos com dados do tipo `i32`, que corresponde ao `5` em `assert_eq!` na
Listagem 15-9.

<!-- Old headings. Do not remove or links may break. -->

<a id="implicit-deref-coercions-with-functions-and-methods"></a>
<a id="using-deref-coercions-in-functions-and-methods"></a>

<a id="using-deref-coercion-in-functions-and-methods"></a>
### Usando *Deref Coercion* em Funções e Métodos

*Deref coercion* converte uma referência a um tipo que implementa a *trait*
`Deref` em uma referência a outro tipo. Por exemplo, *deref coercion* pode
converter `&String` em `&str` porque `String` implementa a *trait* `Deref` de
forma que retorna `&str`. *Deref coercion* é uma conveniência que o Rust realiza
nos argumentos de funções e métodos, e funciona apenas em tipos que implementam a
*trait* `Deref`. Acontece automaticamente quando passamos uma referência a um
valor de um tipo específico como argumento para uma função ou método que não
corresponde ao tipo de parâmetro na definição da função ou método. Uma sequência
de chamadas ao método `deref` converte o tipo que fornecemos no tipo que o
parâmetro precisa.

A *deref coercion* foi adicionada ao Rust para que programadores que escrevem
chamadas de funções e métodos não precisem adicionar tantas referências e
desreferências explícitas com `&` e `*`. O recurso de *deref coercion* também nos
permite escrever mais código que pode funcionar tanto com referências quanto com
*smart pointers*.

Para ver a *deref coercion* em ação, vamos usar o tipo `MyBox<T>` que definimos
na Listagem 15-8, bem como a implementação de `Deref` que adicionamos na Listagem
15-10. A Listagem 15-11 mostra a definição de uma função que tem um parâmetro do
tipo *slice* de *string*.

<Listing number="15-11" file-name="src/main.rs" caption="Uma função `hello` que tem o parâmetro `name` do tipo `&str`">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-11/src/main.rs:here}}
```

</Listing>

Podemos chamar a função `hello` com um *slice* de *string* como argumento, como
`hello("Rust");`, por exemplo. A *deref coercion* torna possível chamar `hello`
com uma referência a um valor do tipo `MyBox<String>`, como mostrado na Listagem
15-12.

<Listing number="15-12" file-name="src/main.rs" caption="Chamando `hello` com uma referência a um valor `MyBox<String>`, que funciona devido à *deref coercion*">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-12/src/main.rs:here}}
```

</Listing>

Aqui estamos chamando a função `hello` com o argumento `&m`, que é uma referência
a um valor `MyBox<String>`. Como implementamos a *trait* `Deref` em `MyBox<T>` na
Listagem 15-10, o Rust pode transformar `&MyBox<String>` em `&String` chamando
`deref`. A biblioteca padrão fornece uma implementação de `Deref` em `String` que
retorna um *slice* de *string*, e isso está na documentação da API de `Deref`. O
Rust chama `deref` novamente para transformar `&String` em `&str`, que corresponde
à definição da função `hello`.

Se o Rust não implementasse *deref coercion*, teríamos que escrever o código da
Listagem 15-13 em vez do código da Listagem 15-12 para chamar `hello` com um
valor do tipo `&MyBox<String>`.

<Listing number="15-13" file-name="src/main.rs" caption="O código que teríamos que escrever se o Rust não tivesse *deref coercion*">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-13/src/main.rs:here}}
```

</Listing>

O `(*m)` desreferencia o `MyBox<String>` em um `String`. Em seguida, o `&` e
`[..]` pegam um *slice* de *string* do `String` que é igual à *string* inteira
para corresponder à assinatura de `hello`. Esse código sem *deref coercions* é
mais difícil de ler, escrever e entender com todos esses símbolos envolvidos. A
*deref coercion* permite que o Rust lide com essas conversões para nós
automaticamente.

Quando a *trait* `Deref` é definida para os tipos envolvidos, o Rust analisará os
tipos e usará `Deref::deref` quantas vezes forem necessárias para obter uma
referência que corresponda ao tipo do parâmetro. O número de vezes que
`Deref::deref` precisa ser inserido é resolvido em tempo de compilação, portanto
não há penalidade em tempo de execução por tirar proveito da *deref coercion*!

<!-- Old headings. Do not remove or links may break. -->

<a id="how-deref-coercion-interacts-with-mutability"></a>

<a id="handling-deref-coercion-with-mutable-references"></a>
### Lidando com *Deref Coercion* e Referências Mutáveis

De maneira semelhante a como você usa a *trait* `Deref` para sobrescrever o
operador `*` em referências imutáveis, você pode usar a *trait* `DerefMut` para
sobrescrever o operador `*` em referências mutáveis.

O Rust realiza *deref coercion* quando encontra tipos e implementações de *traits*
em três casos:

1. De `&T` para `&U` quando `T: Deref<Target=U>`
2. De `&mut T` para `&mut U` quando `T: DerefMut<Target=U>`
3. De `&mut T` para `&U` quando `T: Deref<Target=U>`

Os dois primeiros casos são iguais, exceto que o segundo implementa mutabilidade.
O primeiro caso afirma que se você tem um `&T`, e `T` implementa `Deref` para
algum tipo `U`, você pode obter um `&U` de forma transparente. O segundo caso
afirma que a mesma *deref coercion* acontece para referências mutáveis.

O terceiro caso é mais complicado: o Rust também fará a coerção de uma referência
mutável para uma imutável. Mas o inverso *não* é possível: referências imutáveis
nunca farão coerção para referências mutáveis. Devido às regras de *borrowing*,
se você tem uma referência mutável, essa referência mutável deve ser a única
referência para aqueles dados (caso contrário, o programa não compilaria).
Converter uma referência mutável em uma referência imutável nunca quebrará as
regras de *borrowing*. Converter uma referência imutável em uma referência mutável
exigiria que a referência imutável inicial fosse a única referência imutável para
aqueles dados, mas as regras de *borrowing* não garantem isso. Portanto, o Rust
não pode presumir que converter uma referência imutável em uma referência mutável
é possível.

[impl-trait]: ch10-02-traits.html#implementing-a-trait-on-a-type
[tuple-structs]: ch05-01-defining-structs.html#creating-different-types-with-tuple-structs
