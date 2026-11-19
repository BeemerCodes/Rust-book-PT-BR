<a id="storing-lists-of-values-with-vectors"></a>

## Armazenando Listas de Valores com Vetores

O primeiro tipo de coleção que veremos é `Vec<T>`, também conhecido como vetor (*vector*).
Vetores permitem armazenar mais de um valor em uma única estrutura de dados que
coloca todos os valores lado a lado na memória. Vetores só podem armazenar valores
do mesmo tipo. Eles são úteis quando você tem uma lista de itens, como as
linhas de texto em um arquivo ou os preços dos itens em um carrinho de compras.

<a id="creating-a-new-vector"></a>

### Criando um Novo Vetor

Para criar um novo vetor vazio, chamamos a função `Vec::new`, como mostrado na
Listagem 8-1.

<Listing number="8-1" caption="Criando um novo vetor vazio para armazenar valores do tipo `i32`">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-01/src/main.rs:here}}
```

</Listing>

Observe que adicionamos uma anotação de tipo aqui. Como não estamos inserindo nenhum
valor nesse vetor, o Rust não sabe que tipo de elementos pretendemos
armazenar. Esse é um ponto importante. Vetores são implementados usando genéricos (*generics*);
veremos como usar tipos genéricos com seus próprios tipos no Capítulo 10. Por ora,
saiba que o tipo `Vec<T>` fornecido pela biblioteca padrão pode conter qualquer tipo.
Quando criamos um vetor para guardar um tipo específico, podemos especificar o tipo entre
parênteses angulares (ou sinais de menor e maior). Na Listagem 8-1, dissemos ao Rust que o `Vec<T>` em `v`
conterá elementos do tipo `i32`.

Na maioria das vezes, você criará um `Vec<T>` com valores iniciais, e o Rust inferirá
o tipo de valor que você quer armazenar, de modo que raramente precisará fazer essa anotação
de tipo. O Rust convenientemente fornece a macro `vec!`, que cria um
novo vetor contendo os valores que você fornecer a ela. A Listagem 8-2 cria um novo
`Vec<i32>` contendo os valores `1`, `2` e `3`. O tipo inteiro é `i32`
porque esse é o tipo inteiro padrão, como vimos na seção [“Tipos de
Dados”][data-types]<!-- ignore --> do Capítulo 3.

<Listing number="8-2" caption="Criando um novo vetor contendo valores">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-02/src/main.rs:here}}
```

</Listing>

Como fornecemos valores `i32` iniciais, o Rust pode inferir que o tipo de `v`
é `Vec<i32>`, tornando a anotação de tipo desnecessária. A seguir, veremos como
modificar um vetor.

<a id="updating-a-vector"></a>

### Atualizando um Vetor

Para criar um vetor e depois adicionar elementos a ele, podemos usar o método `push`,
como mostrado na Listagem 8-3.

<Listing number="8-3" caption="Usando o método `push` para adicionar valores a um vetor">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-03/src/main.rs:here}}
```

</Listing>

Assim como com qualquer variável, se quisermos ser capazes de alterar seu valor, precisamos
torná-la mutável usando a palavra-chave `mut`, como abordado no Capítulo 3. Os números
que colocamos dentro são todos do tipo `i32`, e o Rust infere isso a partir dos dados,
portanto não precisamos da anotação `Vec<i32>`.

<a id="reading-elements-of-vectors"></a>

### Lendo Elementos de Vetores

Existem duas maneiras de referenciar um valor armazenado em um vetor: via indexação ou
usando o método `get`. Nos exemplos a seguir, anotamos os tipos dos
valores retornados dessas funções para maior clareza.

A Listagem 8-4 mostra ambos os métodos de acesso a um valor em um vetor, com a sintaxe de indexação
e com o método `get`.

<Listing number="8-4" caption="Usando a sintaxe de indexação e o método `get` para acessar um item em um vetor">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-04/src/main.rs:here}}
```

</Listing>

Observe alguns detalhes aqui. Usamos o índice `2` para obter o terceiro elemento
porque vetores são indexados por número, começando do zero. Usar `&` e `[]`
nos dá uma referência ao elemento no valor de índice informado. Quando usamos o método
`get` passando o índice como argumento, obtemos um `Option<&T>` que podemos
usar com `match`.

O Rust fornece essas duas formas de referenciar um elemento para que você possa escolher como
o programa se comporta ao tentar usar um índice fora do intervalo dos
elementos existentes. Como exemplo, vamos ver o que acontece quando temos um vetor
de cinco elementos e tentamos acessar o elemento no índice 100 com cada
técnica, como mostrado na Listagem 8-5.

<Listing number="8-5" caption="Tentando acessar o elemento no índice 100 em um vetor contendo cinco elementos">

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-05/src/main.rs:here}}
```

</Listing>

Quando executamos esse código, o primeiro método com `[]` fará com que o programa entre em pânico (*panic*)
porque faz referência a um elemento inexistente. Esse método é mais indicado quando você
quer que seu programa falhe imediatamente se houver uma tentativa de acessar um elemento além do
fim do vetor.

Quando o método `get` recebe um índice que está fora do vetor, ele retorna
`None` sem causar pânico. Você usaria esse método se acessar um elemento
além do limite do vetor puder acontecer ocasionalmente em circunstâncias normais.
Seu código terá então uma lógica para tratar tanto `Some(&element)` quanto `None`,
conforme discutido no Capítulo 6. Por exemplo, o índice pode vir de uma pessoa digitando
um número. Se ela acidentalmente digitar um número muito grande e o programa receber um valor `None`,
você poderá informar ao usuário quantos itens existem no vetor atual e dar a ele outra chance de
digitar um valor válido. Isso seria bem mais amigável do que derrubar o programa
por causa de um erro de digitação!

Quando o programa tem uma referência válida, o verificador de empréstimos (*borrow checker*) impõe as
regras de *ownership* e de empréstimo (*borrowing*, cobertas no Capítulo 4) para garantir que essa
referência e quaisquer outras referências ao conteúdo do vetor permaneçam válidas.
Lembre-se da regra que afirma que você não pode ter referências mutáveis e imutáveis no
mesmo escopo. Essa regra se aplica na Listagem 8-6, onde mantemos uma referência
imutável para o primeiro elemento em um vetor e tentamos adicionar um elemento ao
final. Esse programa não funcionará se também tentarmos nos referir a esse elemento mais tarde
na função.

<Listing number="8-6" caption="Tentando adicionar um elemento a um vetor enquanto mantém uma referência a um item">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-06/src/main.rs:here}}
```

</Listing>

Compilar esse código resultará no seguinte erro:

```console
{{#include ../listings/ch08-common-collections/listing-08-06/output.txt}}
```

O código na Listagem 8-6 pode parecer que deveria funcionar: por que uma referência
ao primeiro elemento se importaria com alterações no final do vetor? Esse erro se
deve à maneira como os vetores funcionam: como os vetores colocam os valores lado a lado
na memória, adicionar um novo elemento ao final do vetor pode exigir a alocação de nova
memória e a cópia dos elementos antigos para o novo espaço, caso não haja espaço suficiente
para colocar todos os elementos contíguos onde o vetor está armazenado atualmente.
Nesse caso, a referência ao primeiro elemento estaria apontando para uma memória desalocada.
As regras de empréstimo (*borrowing*) evitam que os programas cheguem a essa situação.

> Nota: Para saber mais sobre os detalhes de implementação do tipo `Vec<T>`, veja [“The
> Rustonomicon”][nomicon].

<a id="iterating-over-the-values-in-a-vector"></a>

### Iterando Pelos Valores em um Vetor

Para acessar cada elemento de um vetor por vez, iteramos por todos os
elementos em vez de usar índices para acessar um de cada vez. A Listagem 8-7 mostra como
usar um laço `for` para obter referências imutáveis a cada elemento em um vetor de
valores `i32` e imprimi-los.

<Listing number="8-7" caption="Imprimindo cada elemento em um vetor iterando sobre os elementos usando um laço `for`">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-07/src/main.rs:here}}
```

</Listing>

Também podemos iterar sobre referências mutáveis para cada elemento em um vetor mutável
a fim de fazer alterações em todos os elementos. O laço `for` na Listagem 8-8
adicionará `50` a cada elemento.

<Listing number="8-8" caption="Iterando sobre referências mutáveis a elementos em um vetor">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-08/src/main.rs:here}}
```

</Listing>

Para alterar o valor ao qual a referência mutável se refere, temos que usar o
operador de desreferenciação `*` para chegar ao valor em `i` antes de podermos usar o
operador `+=`. Falaremos mais sobre o operador de desreferenciação na seção [“Seguindo o
Ponteiro até o Valor com o Operador de Desreferenciação”][deref]<!-- ignore --> do Capítulo 15.

Iterar sobre um vetor, seja de forma imutável ou mutável, é seguro graças às
regras do *borrow checker*. Se tentássemos inserir ou remover itens no corpo do laço `for`
na Listagem 8-7 e na Listagem 8-8, receberíamos um erro do compilador semelhante ao que
tivemos com o código na Listagem 8-6. A referência ao vetor que o laço `for` mantém
impede a modificação simultânea de todo o vetor.

<a id="using-an-enum-to-store-multiple-types"></a>

### Usando um Enum para Armazenar Múltiplos Tipos

Vetores só podem armazenar valores que sejam do mesmo tipo. Isso pode ser
inconveniente; existem definitivamente casos de uso para armazenar uma lista de
itens de tipos diferentes. Felizmente, as variantes de um *enum* são definidas
sob o mesmo tipo de *enum*, então, quando precisamos de um único tipo para representar elementos
de tipos diferentes, podemos definir e usar um *enum*!

Por exemplo, suponha que queiramos obter valores de uma linha em uma planilha na qual
algumas das colunas contêm inteiros, algumas números de ponto flutuante e
outras strings. Podemos definir um *enum* cujas variantes guardarão os diferentes
tipos de valores, e todas as variantes do *enum* serão consideradas do mesmo tipo: o
tipo do próprio *enum*. Depois, podemos criar um vetor para guardar esse *enum* e, assim,
em última análise, guardar tipos diferentes. Demonstramos isso na Listagem 8-9.

<Listing number="8-9" caption="Definindo um enum para armazenar valores de diferentes tipos em um único vetor">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-09/src/main.rs:here}}
```

</Listing>

O Rust precisa saber quais tipos estarão no vetor em tempo de compilação para
saber exatamente quanta memória no *heap* será necessária para armazenar cada elemento.
Também devemos ser explícitos sobre quais tipos são permitidos nesse vetor. Se o Rust
permitisse que um vetor contivesse qualquer tipo, haveria a possibilidade de um ou mais
desses tipos causarem erros com as operações executadas nos elementos do
vetor. Usar um *enum* juntamente com uma expressão `match` significa que o Rust garantirá
em tempo de compilação que todos os casos possíveis sejam tratados, conforme discutido no Capítulo 6.

Se você não souber o conjunto exaustivo de tipos que um programa receberá em tempo de execução
para armazenar em um vetor, a técnica do *enum* não funcionará. Em vez disso, você pode usar um *trait object*,
assunto que cobriremos no Capítulo 18.

Agora que discutimos algumas das maneiras mais comuns de usar vetores, certifique-se
de revisar [a documentação da API][vec-api]<!-- ignore --> para conhecer todos os muitos
métodos úteis definidos em `Vec<T>` pela biblioteca padrão. Por exemplo, além
de `push`, o método `pop` remove e retorna o último elemento.

<a id="dropping-a-vector-drops-its-elements"></a>

### Descartar um Vetor Descarta Seus Elementos

Como qualquer outra `struct`, um vetor é liberado quando sai de escopo, conforme
anotado na Listagem 8-10.

<Listing number="8-10" caption="Mostrando onde o vetor e seus elementos são descartados">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-10/src/main.rs:here}}
```

</Listing>

Quando o vetor é descartado (*dropped*), todo o seu conteúdo também é descartado, o que significa que os
inteiros que ele contém serão liberados. O *borrow checker* garante que qualquer
referência ao conteúdo de um vetor seja usada apenas enquanto o próprio vetor for
válido.

Vamos avançar para o próximo tipo de coleção: `String`!

[data-types]: ch03-02-data-types.html#data-types
[nomicon]: ../nomicon/vec/vec.html
[vec-api]: ../std/vec/struct.Vec.html
[deref]: ch15-02-deref.html#following-the-pointer-to-the-value-with-the-dereference-operator
