<!-- Old headings. Do not remove or links may break. -->

<a id="closures-anonymous-functions-that-can-capture-their-environment"></a>
<a id="closures-anonymous-functions-that-capture-their-environment"></a>

<a id="closures"></a>
## Closures

As *closures* em Rust são funções anônimas que você pode salvar em uma variável ou passar como argumentos para outras funções. Você pode criar a *closure* em um lugar e chamá-la em outro para avaliá-la em um contexto diferente. Diferente das funções comuns, as *closures* podem capturar valores do escopo em que foram definidas. Vamos demonstrar como esses recursos permitem reutilização de código e personalização de comportamento.

<!-- Old headings. Do not remove or links may break. -->

<a id="creating-an-abstraction-of-behavior-with-closures"></a>
<a id="refactoring-using-functions"></a>
<a id="refactoring-with-closures-to-store-code"></a>
<a id="capturing-the-environment-with-closures"></a>

<a id="capturing-the-environment"></a>
### Capturando o Ambiente

Primeiro, vamos ver como usar *closures* para capturar valores do ambiente em que são definidas para uso posterior. Imagine o seguinte cenário: de tempos em tempos, nossa loja de camisetas sorteia uma camiseta exclusiva de edição limitada para alguém da nossa lista de e-mails como promoção. As pessoas na lista podem, opcionalmente, adicionar sua cor favorita no perfil. Se a pessoa sorteada tiver definido sua cor favorita, ela recebe a camiseta dessa cor. Se ela não especificou nenhuma cor favorita, ela recebe a cor que a empresa tiver em maior quantidade no estoque no momento.

Existem várias formas de implementar isso. Para este exemplo, vamos usar um *enum* chamado `ShirtColor` com as variantes `Red` e `Blue` (limitando o número de cores para manter simples). Representamos o estoque da empresa com uma *struct* `Inventory` que tem um campo `shirts` contendo um `Vec<ShirtColor>` representando as cores das camisetas em estoque. O método `giveaway`, definido em `Inventory`, recebe a preferência opcional de cor do ganhador e retorna a cor da camiseta que a pessoa vai ganhar. Essa estrutura está na Listagem 13-1.

<Listing number="13-1" file-name="src/main.rs" caption="Cenário de sorteio da loja de camisetas">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-01/src/main.rs}}
```

</Listing>

A variável `store` definida em `main` tem duas camisetas azuis e uma vermelha restantes para distribuir nessa promoção de edição limitada. Chamamos o método `giveaway` para um usuário com preferência pela camiseta vermelha e para outro sem preferência.

Novamente, esse código poderia ser implementado de várias formas, mas aqui, para focar nas *closures*, usamos apenas conceitos que você já aprendeu — exceto no corpo do método `giveaway`, que usa uma *closure*. No método `giveaway`, recebemos a preferência do usuário como um parâmetro do tipo `Option<ShirtColor>` e chamamos o método `unwrap_or_else` em `user_preference`. O [método `unwrap_or_else` em `Option<T>`][unwrap-or-else]<!-- ignore --> é definido pela biblioteca padrão. Ele recebe um argumento: uma *closure* sem parâmetros que retorna um valor `T` (o mesmo tipo armazenado na variante `Some` do `Option<T>`, neste caso `ShirtColor`). Se o `Option<T>` for a variante `Some`, `unwrap_or_else` retorna o valor de dentro do `Some`. Se for a variante `None`, `unwrap_or_else` chama a *closure* e retorna o valor retornado por ela.

Especificamos a expressão de *closure* `|| self.most_stocked()` como argumento para `unwrap_or_else`. Essa é uma *closure* que não recebe parâmetros (se tivesse parâmetros, eles apareceriam entre as duas barras verticais). O corpo da *closure* chama `self.most_stocked()`. Estamos definindo a *closure* aqui, e a implementação de `unwrap_or_else` vai avaliá-la depois, apenas se o resultado for necessário.

Ao executar esse código, a saída será:

```console
{{#include ../listings/ch13-functional-features/listing-13-01/output.txt}}
```

Um ponto interessante aqui é que passamos uma *closure* que chama `self.most_stocked()` na instância atual de `Inventory`. A biblioteca padrão não precisou saber nada sobre os tipos `Inventory` ou `ShirtColor` que definimos, nem sobre a lógica que queremos usar nesse cenário. A *closure* captura uma referência imutável à instância `self` de `Inventory` e a passa junto com o código que especificamos para o método `unwrap_or_else`. Funções comuns, por outro lado, não conseguem capturar o ambiente dessa maneira.

<!-- Old headings. Do not remove or links may break. -->

<a id="closure-type-inference-and-annotation"></a>

<a id="inferring-and-annotating-closure-types"></a>
### Inferência e Anotação de Tipos em Closures

Existem outras diferenças entre funções e *closures*. As *closures* geralmente não exigem que você anote os tipos dos parâmetros nem o tipo de retorno como as funções `fn` exigem. Anotações de tipo são obrigatórias em funções porque os tipos fazem parte de uma interface explícita exposta aos usuários. Definir essa interface de forma rígida é importante para garantir que todos concordem sobre quais tipos de valores uma função usa e retorna. Já as *closures* não são usadas em uma interface exposta assim: elas são armazenadas em variáveis e usadas sem precisar nomeá-las ou expô-las aos usuários da nossa biblioteca.

As *closures* costumam ser curtas e relevantes apenas dentro de um contexto restrito, em vez de em qualquer cenário arbitrário. Dentro desses contextos limitados, o compilador consegue inferir os tipos dos parâmetros e do retorno, de forma parecida com a inferência de tipos da maioria das variáveis (embora existam casos raros em que o compilador também precisa de anotações de tipo na *closure*).

Assim como acontece com variáveis, podemos adicionar anotações de tipo se quisermos deixar tudo mais explícito e claro, ao custo de ser um pouco mais verboso do que o estritamente necessário. Anotar os tipos de uma *closure* ficaria como na definição mostrada na Listagem 13-2. Neste exemplo, estamos definindo uma *closure* e armazenando-a em uma variável em vez de defini-la diretamente no ponto onde a passamos como argumento, como fizemos na Listagem 13-1.

<Listing number="13-2" file-name="src/main.rs" caption="Adicionando anotações opcionais de tipo para o parâmetro e o valor de retorno na *closure*">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-02/src/main.rs:here}}
```

</Listing>

Com as anotações de tipo adicionadas, a sintaxe das *closures* fica mais parecida com a sintaxe das funções. Abaixo, definimos uma função que soma 1 ao seu parâmetro e uma *closure* com o mesmo comportamento, para comparação. Adicionamos alguns espaços para alinhar as partes relevantes. Isso ilustra como a sintaxe de *closure* é semelhante à de função, exceto pelo uso das barras verticais (`|`) e pela quantidade de sintaxe que é opcional:

```rust,ignore
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

A primeira linha mostra uma definição de função e a segunda mostra uma definição de *closure* totalmente anotada. Na terceira linha, removemos as anotações de tipo da *closure*. Na quarta linha, removemos as chaves, que são opcionais porque o corpo da *closure* tem apenas uma expressão. Todas essas são definições válidas que produzem o mesmo comportamento quando chamadas. As linhas `add_one_v3` e `add_one_v4` exigem que as *closures* sejam avaliadas para conseguir compilar, pois os tipos serão inferidos a partir do seu uso. Isso é parecido com `let v = Vec::new();`, que precisa de anotações de tipo ou de valores inseridos no `Vec` para que o Rust consiga inferir o tipo.

Para definições de *closure*, o compilador vai inferir um único tipo concreto para cada parâmetro e para o valor de retorno. Por exemplo, a Listagem 13-3 mostra uma *closure* curta que apenas retorna o valor recebido como parâmetro. Essa *closure* não é muito útil além de servir para este exemplo. Note que não adicionamos nenhuma anotação de tipo à definição. Como não há anotações de tipo, podemos chamar a *closure* com qualquer tipo — o que fazemos aqui com `String` na primeira vez. Se depois tentarmos chamar `example_closure` com um inteiro, teremos um erro.

<Listing number="13-3" file-name="src/main.rs" caption="Tentando chamar uma *closure* cujos tipos são inferidos com dois tipos diferentes">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-03/src/main.rs:here}}
```

</Listing>

O compilador nos dá este erro:

```console
{{#include ../listings/ch13-functional-features/listing-13-03/output.txt}}
```

Na primeira vez que chamamos `example_closure` com o valor `String`, o compilador infere que o tipo de `x` e o tipo de retorno da *closure* são `String`. Esses tipos ficam então travados na *closure* em `example_closure`, e recebemos um erro de tipo quando tentamos usar um tipo diferente com a mesma *closure*.

<a id="capturing-references-or-moving-ownership"></a>
### Capturando Referências ou Movendo Ownership

As *closures* podem capturar valores do ambiente de três maneiras, que correspondem diretamente às três maneiras pelas quais uma função pode receber um parâmetro: emprestando imutavelmente (*borrowing* imutável), emprestando mutavelmente (*borrowing* mutável) e assumindo *ownership*. A *closure* decide qual delas usar com base no que o seu corpo faz com os valores capturados.

Na Listagem 13-4, definimos uma *closure* que captura uma referência imutável ao vetor chamado `list`, porque ela só precisa de uma referência imutável para imprimir o valor.

<Listing number="13-4" file-name="src/main.rs" caption="Definindo e chamando uma *closure* que captura uma referência imutável">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-04/src/main.rs}}
```

</Listing>

Este exemplo também ilustra que uma variável pode ser associada a uma definição de *closure*, e podemos chamá-la depois usando o nome da variável seguido de parênteses, como se fosse o nome de uma função.

Como podemos ter múltiplas referências imutáveis para `list` ao mesmo tempo, `list` continua acessível no código antes da definição da *closure*, depois da definição mas antes da chamada, e também depois que a *closure* é chamada. Esse código compila, executa e imprime:

```console
{{#include ../listings/ch13-functional-features/listing-13-04/output.txt}}
```

Em seguida, na Listagem 13-5, alteramos o corpo da *closure* para adicionar um elemento ao vetor `list`. Agora a *closure* captura uma referência mutável.

<Listing number="13-5" file-name="src/main.rs" caption="Definindo e chamando uma *closure* que captura uma referência mutável">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-05/src/main.rs}}
```

</Listing>

Esse código compila, executa e imprime:

```console
{{#include ../listings/ch13-functional-features/listing-13-05/output.txt}}
```

Note que não há mais um `println!` entre a definição e a chamada da *closure* `borrows_mutably`: quando `borrows_mutably` é definida, ela captura uma referência mutável para `list`. Não usamos a *closure* novamente depois que ela é chamada, então o empréstimo mutável termina ali. Entre a definição e a chamada da *closure*, um empréstimo imutável para imprimir não é permitido, pois nenhum outro empréstimo é permitido enquanto existe um empréstimo mutável ativo. Experimente adicionar um `println!` ali para ver qual mensagem de erro você recebe!

Se você quiser forçar a *closure* a assumir *ownership* dos valores que usa do ambiente — mesmo que o corpo da *closure* não precise estritamente de *ownership* —, você pode usar a palavra-chave `move` antes da lista de parâmetros.

Essa técnica é especialmente útil ao passar uma *closure* para uma nova *thread*, movendo os dados para que passem a pertencer à nova *thread*. Vamos falar em detalhes sobre *threads* e por que usá-las no Capítulo 16, quando tratarmos de concorrência, mas por enquanto vamos explorar brevemente a criação de uma nova *thread* usando uma *closure* que precisa da palavra-chave `move`. A Listagem 13-6 mostra a Listagem 13-4 modificada para imprimir o vetor em uma nova *thread* em vez da *thread* principal.

<Listing number="13-6" file-name="src/main.rs" caption="Usando `move` para forçar a *closure* da *thread* a assumir *ownership* de `list`">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-06/src/main.rs}}
```

</Listing>

Criamos uma nova *thread*, passando uma *closure* como argumento para ela executar. O corpo da *closure* imprime a lista. Na Listagem 13-4, a *closure* só capturava `list` usando uma referência imutável, porque esse é o menor nível de acesso necessário para imprimi-la. Neste exemplo, mesmo que o corpo da *closure* ainda só precise de uma referência imutável, precisamos especificar que `list` deve ser movida para dentro da *closure* colocando a palavra-chave `move` no início da definição. Se a *thread* principal realizasse mais operações antes de chamar `join` na nova *thread*, a nova *thread* poderia terminar antes do restante da *thread* principal, ou a *thread* principal poderia terminar primeiro. Se a *thread* principal mantivesse a *ownership* de `list`, mas terminasse antes da nova *thread* e descartasse `list`, a referência imutável na nova *thread* ficaria inválida. Portanto, o compilador exige que `list` seja movida para dentro da *closure* entregue à nova *thread* para garantir que a referência seja válida. Tente remover a palavra-chave `move` ou usar `list` na *thread* principal depois da definição da *closure* para ver os erros do compilador!

<!-- Old headings. Do not remove or links may break. -->

<a id="storing-closures-using-generic-parameters-and-the-fn-traits"></a>
<a id="limitations-of-the-cacher-implementation"></a>
<a id="moving-captured-values-out-of-the-closure-and-the-fn-traits"></a>
<a id="moving-captured-values-out-of-closures-and-the-fn-traits"></a>

<a id="moving-captured-values-out-of-closures"></a>
### Movendo Valores Capturados para Fora de Closures

Depois que uma *closure* captura uma referência ou a *ownership* de um valor do ambiente onde foi definida (afetando o que, se houver algo, é movido *para dentro* da *closure*), o código no corpo da *closure* define o que acontece com essas referências ou valores quando ela é avaliada mais tarde (afetando o que, se houver algo, é movido *para fora* da *closure*).

O corpo de uma *closure* pode fazer qualquer uma das seguintes ações: mover um valor capturado para fora da *closure*, modificar o valor capturado, não mover nem modificar o valor, ou nem sequer capturar nada do ambiente.

A forma como uma *closure* captura e manipula valores do ambiente determina quais *traits* ela implementa, e os *traits* são o mecanismo pelo qual funções e *structs* especificam quais tipos de *closures* podem aceitar. As *closures* implementam automaticamente um, dois ou todos os três *traits* `Fn`, de forma aditiva, dependendo de como o corpo da *closure* lida com os valores:

* `FnOnce` se aplica a *closures* que podem ser chamadas uma vez. Todas as *closures* implementam pelo menos esse *trait*, porque todas podem ser chamadas. Uma *closure* que move valores capturados para fora do seu corpo implementará apenas `FnOnce` e nenhum dos outros *traits* `Fn`, pois só pode ser chamada uma vez.
* `FnMut` se aplica a *closures* que não movem valores capturados para fora do seu corpo, mas podem modificá-los. Essas *closures* podem ser chamadas mais de uma vez.
* `Fn` se aplica a *closures* que não movem valores capturados para fora do seu corpo e não os modificam, bem como a *closures* que não capturam nada do ambiente. Essas *closures* podem ser chamadas mais de uma vez sem alterar o ambiente, o que é importante em casos como chamar uma *closure* várias vezes de forma concorrente.

Vamos observar a definição do método `unwrap_or_else` em `Option<T>` que usamos na Listagem 13-1:

```rust,ignore
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

Lembre-se de que `T` é o tipo genérico que representa o tipo do valor na variante `Some` de um `Option`. Esse tipo `T` também é o tipo de retorno da função `unwrap_or_else`: um código que chama `unwrap_or_else` em um `Option<String>`, por exemplo, receberá uma `String`.

Em seguida, note que a função `unwrap_or_else` tem o parâmetro de tipo genérico adicional `F`. O tipo `F` é o tipo do parâmetro chamado `f`, que é a *closure* que fornecemos ao chamar `unwrap_or_else`.

A restrição de *trait* (*trait bound*) especificada no tipo genérico `F` é `FnOnce() -> T`, o que significa que `F` precisa poder ser chamada uma vez, não receber argumentos e retornar um `T`. Usar `FnOnce` na restrição expressa a garantia de que `unwrap_or_else` não chamará `f` mais de uma vez. No corpo de `unwrap_or_else`, podemos ver que se o `Option` for `Some`, `f` não será chamada. Se for `None`, `f` será chamada uma única vez. Como todas as *closures* implementam `FnOnce`, `unwrap_or_else` aceita todos os três tipos de *closures* e é o mais flexível possível.

> Nota: Se o que queremos fazer não exige capturar nenhum valor do ambiente, podemos usar o nome de uma função em vez de uma *closure* sempre que precisarmos de algo que implemente um dos *traits* `Fn`. Por exemplo, em um valor `Option<Vec<T>>`, poderíamos chamar `unwrap_or_else(Vec::new)` para obter um novo vetor vazio se o valor for `None`. O compilador implementa automaticamente o *trait* `Fn` aplicável para definições de funções.

Agora vamos dar uma olhada no método `sort_by_key` da biblioteca padrão, definido em *slices*, para ver como ele difere de `unwrap_or_else` e por que `sort_by_key` usa `FnMut` em vez de `FnOnce` na restrição de *trait*. A *closure* recebe um argumento na forma de uma referência ao item atual da *slice* sendo analisada e retorna um valor do tipo `K` que pode ser ordenado. Essa função é útil quando você quer ordenar uma *slice* por um atributo específico de cada item. Na Listagem 13-7, temos uma lista de instâncias de `Rectangle` e usamos `sort_by_key` para ordená-las pelo atributo `width`, do menor para o maior.

<Listing number="13-7" file-name="src/main.rs" caption="Usando `sort_by_key` para ordenar retângulos pela largura">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-07/src/main.rs}}
```

</Listing>

Esse código imprime:

```console
{{#include ../listings/ch13-functional-features/listing-13-07/output.txt}}
```

O motivo pelo qual `sort_by_key` é definido para receber uma *closure* `FnMut` é que ele chama a *closure* várias vezes: uma vez para cada item na *slice*. A *closure* `|r| r.width` não captura, não modifica nem move nada para fora do ambiente, portanto atende aos requisitos da restrição de *trait*.

Por outro lado, a Listagem 13-8 mostra um exemplo de uma *closure* que implementa apenas o *trait* `FnOnce`, porque move um valor para fora do ambiente. O compilador não nos deixará usar essa *closure* com `sort_by_key`.

<Listing number="13-8" file-name="src/main.rs" caption="Tentando usar uma *closure* `FnOnce` com `sort_by_key`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-08/src/main.rs}}
```

</Listing>

Essa é uma forma artificial e complicada (que não funciona) de tentar contar quantas vezes `sort_by_key` chama a *closure* ao ordenar `list`. O código tenta fazer essa contagem inserindo `value` — uma `String` do ambiente da *closure* — no vetor `sort_operations`. A *closure* captura `value` e depois move `value` para fora dela, transferindo a *ownership* de `value` para o vetor `sort_operations`. Essa *closure* só pode ser chamada uma vez; tentar chamá-la uma segunda vez não funcionaria, porque `value` não estaria mais no ambiente para ser inserida novamente em `sort_operations`! Portanto, essa *closure* implementa apenas `FnOnce`. Ao tentar compilar esse código, recebemos um erro dizendo que `value` não pode ser movido para fora da *closure* porque ela precisa implementar `FnMut`:

```console
{{#include ../listings/ch13-functional-features/listing-13-08/output.txt}}
```

O erro aponta para a linha no corpo da *closure* que move `value` para fora do ambiente. Para corrigir isso, precisamos alterar o corpo da *closure* para que ele não mova valores para fora do ambiente. Manter um contador no ambiente e incrementar seu valor no corpo da *closure* é uma maneira muito mais direta de contar quantas vezes a *closure* é chamada. A *closure* na Listagem 13-9 funciona com `sort_by_key` porque está apenas capturando uma referência mutável ao contador `num_sort_operations` e, portanto, pode ser chamada mais de uma vez.

<Listing number="13-9" file-name="src/main.rs" caption="Usar uma *closure* `FnMut` com `sort_by_key` é permitido.">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-09/src/main.rs}}
```

</Listing>

Os *traits* `Fn` são importantes ao definir ou usar funções ou tipos que fazem uso de *closures*. Na próxima seção, vamos falar sobre iteradores. Muitos métodos de iteradores recebem *closures* como argumentos, então mantenha esses detalhes em mente enquanto continuamos!

[unwrap-or-else]: ../std/option/enum.Option.html#method.unwrap_or_else
