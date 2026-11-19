<a id="reference-cycles-can-leak-memory"></a>
## Ciclos de Referência Podem Vazar Memória

As garantias de segurança de memória do Rust tornam difícil, mas não impossível,
criar acidentalmente memória que nunca é limpa (conhecido como *vazamento de
memória* ou *memory leak*). Prevenir vazamentos de memória completamente não é
uma das garantias do Rust, o que significa que vazamentos de memória são seguros
em termos de memória em Rust. Podemos ver que o Rust permite vazamentos de
memória usando `Rc<T>` e `RefCell<T>`: é possível criar referências em que itens
se referenciam mutuamente em um ciclo. Isso cria vazamentos de memória porque a
contagem de referências de cada item no ciclo nunca chegará a 0, e os valores
nunca serão liberados.

<a id="creating-a-reference-cycle"></a>
### Criando um Ciclo de Referência

Vamos ver como um ciclo de referência pode acontecer e como evitá-lo, começando
com a definição do *enum* `List` e um método `tail` na Listagem 15-25.

<Listing number="15-25" file-name="src/main.rs" caption="Uma definição de *cons list* que armazena um `RefCell<T>` para podermos modificar o que uma variante `Cons` está referenciando">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-25/src/main.rs:here}}
```

</Listing>

Estamos usando outra variação da definição de `List` da Listagem 15-5. O segundo
elemento na variante `Cons` agora é `RefCell<Rc<List>>`, o que significa que, em
vez de ter a capacidade de modificar o valor `i32` como fizemos na Listagem
15-24, queremos modificar o valor `List` para o qual uma variante `Cons` está
apontando. Também estamos adicionando um método `tail` para facilitar o acesso
ao segundo item se tivermos uma variante `Cons`.

Na Listagem 15-26, estamos adicionando uma função `main` que usa as definições
da Listagem 15-25. Esse código cria uma lista em `a` e uma lista em `b` que
aponta para a lista em `a`. Em seguida, modifica a lista em `a` para apontar
para `b`, criando um ciclo de referência. Há instruções `println!` ao longo do
caminho para mostrar quais são as contagens de referências em vários pontos
desse processo.

<Listing number="15-26" file-name="src/main.rs" caption="Criando um ciclo de referência de dois valores `List` apontando um para o outro">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-26/src/main.rs:here}}
```

</Listing>

Criamos uma instância de `Rc<List>` armazenando um valor `List` na variável `a`
com uma lista inicial de `5, Nil`. Em seguida, criamos uma instância de
`Rc<List>` armazenando outro valor `List` na variável `b` que contém o valor
`10` e aponta para a lista em `a`.

Modificamos `a` para que aponte para `b` em vez de `Nil`, criando um ciclo.
Fazemos isso usando o método `tail` para obter uma referência ao
`RefCell<Rc<List>>` em `a`, que colocamos na variável `link`. Em seguida, usamos
o método `borrow_mut` no `RefCell<Rc<List>>` para alterar o valor interno de um
`Rc<List>` que armazena um valor `Nil` para o `Rc<List>` em `b`.

Quando executamos esse código, mantendo o último `println!` comentado por
enquanto, obtemos esta saída:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-26/output.txt}}
```

A contagem de referências das instâncias de `Rc<List>` tanto em `a` quanto em
`b` é 2 depois que mudamos a lista em `a` para apontar para `b`. No final de
`main`, o Rust libera a variável `b`, o que diminui a contagem de referências da
instância `Rc<List>` de `b` de 2 para 1. A memória que `Rc<List>` tem no *heap*
não será liberada nesse ponto porque sua contagem de referências é 1, não 0.
Então, o Rust libera `a`, o que diminui a contagem de referências da instância
`Rc<List>` de `a` de 2 para 1 também. A memória dessa instância também não pode
ser liberada, porque a outra instância de `Rc<List>` ainda se refere a ela. A
memória alocada para a lista permanecerá sem ser coletada para sempre. Para
visualizar esse ciclo de referência, criamos o diagrama na Figura 15-4.

<img alt="Um retângulo rotulado como 'a' que aponta para um retângulo contendo o inteiro 5. Um retângulo rotulado como 'b' que aponta para um retângulo contendo o inteiro 10. O retângulo contendo 5 aponta para o retângulo contendo 10, e o retângulo contendo 10 aponta de volta para o retângulo contendo 5, criando um ciclo." src="img/trpl15-04.svg" class="center" />

<span class="caption">Figura 15-4: Um ciclo de referência das listas `a` e `b`
apontando uma para a outra</span>

Se você descomentar o último `println!` e executar o programa, o Rust tentará
imprimir esse ciclo com `a` apontando para `b` apontando para `a` e assim por
diante até estourar a *stack* (*stack overflow*).

Em comparação com um programa do mundo real, as consequências de criar um ciclo
de referência neste exemplo não são muito graves: logo depois que criamos o
ciclo de referência, o programa termina. No entanto, se um programa mais complexo
alocasse muita memória em um ciclo e a mantivesse por muito tempo, o programa
usaria mais memória do que precisava e poderia sobrecarregar o sistema, fazendo
com que ficasse sem memória disponível.

Criar ciclos de referência não é algo fácil de fazer, mas também não é
impossível. Se você tiver valores `RefCell<T>` que contêm valores `Rc<T>` ou
combinações aninhadas semelhantes de tipos com mutabilidade interior e contagem
de referências, precisará garantir que não crie ciclos; você não pode depender
do Rust para capturá-los. Criar um ciclo de referência seria um bug lógico no
seu programa que você deve minimizar usando testes automatizados, revisões de
código e outras práticas de desenvolvimento de software.

Outra solução para evitar ciclos de referência é reorganizar suas estruturas de
dados de modo que algumas referências expressem propriedade (*ownership*) e
outras não. Como resultado, você pode ter ciclos compostos por algumas relações
de propriedade e algumas relações de não propriedade, e apenas as relações de
propriedade afetam se um valor pode ou não ser liberado. Na Listagem 15-25,
sempre queremos que variantes `Cons` sejam donas de sua lista, portanto
reorganizar a estrutura de dados não é possível. Vamos olhar um exemplo usando
grafos compostos por nós pais e nós filhos para ver quando relações de não
propriedade são uma forma apropriada de evitar ciclos de referência.

<!-- Old headings. Do not remove or links may break. -->

<a id="preventing-reference-cycles-turning-an-rct-into-a-weakt"></a>

<a id="preventing-reference-cycles-using-weakt"></a>
### Prevenindo Ciclos de Referência Usando `Weak<T>`

Até agora, demonstramos que chamar `Rc::clone` aumenta o `strong_count` de uma
instância de `Rc<T>`, e uma instância de `Rc<T>` só é limpa se seu
`strong_count` for 0. Você também pode criar uma referência fraca (*weak
reference*) para o valor dentro de uma instância de `Rc<T>` chamando
`Rc::downgrade` e passando uma referência ao `Rc<T>`. *Referências fortes*
(*strong references*) são como você pode compartilhar a propriedade de uma
instância de `Rc<T>`. *Referências fracas* (*weak references*) não expressam uma
relação de propriedade, e sua contagem não afeta quando uma instância de `Rc<T>`
é limpa. Elas não causarão um ciclo de referência, porque qualquer ciclo
envolvendo algumas referências fracas será quebrado assim que a contagem de
referências fortes dos valores envolvidos for 0.

Quando você chama `Rc::downgrade`, obtém um *smart pointer* do tipo `Weak<T>`.
Em vez de aumentar o `strong_count` na instância de `Rc<T>` em 1, chamar
`Rc::downgrade` aumenta o `weak_count` em 1. O tipo `Rc<T>` usa `weak_count`
para manter o controle de quantas referências `Weak<T>` existem, de forma
semelhante a `strong_count`. A diferença é que `weak_count` não precisa ser 0
para que a instância de `Rc<T>` seja limpa.

Como o valor que `Weak<T>` referencia pode já ter sido liberado, para fazer
qualquer coisa com o valor para o qual um `Weak<T>` está apontando você precisa
garantir que o valor ainda existe. Faça isso chamando o método `upgrade` em uma
instância de `Weak<T>`, que retornará um `Option<Rc<T>>`. Você obterá um
resultado `Some` se o valor `Rc<T>` ainda não tiver sido liberado e um resultado
`None` se o valor `Rc<T>` já tiver sido liberado. Como `upgrade` retorna um
`Option<Rc<T>>`, o Rust garantirá que o caso `Some` e o caso `None` sejam
tratados, e não haverá um ponteiro inválido.

Como exemplo, em vez de usar uma lista cujos itens conhecem apenas o próximo
item, criaremos uma árvore cujos itens conhecem seus itens filhos *e* seus itens
pais.

<!-- Old headings. Do not remove or links may break. -->

<a id="creating-a-tree-data-structure-a-node-with-child-nodes"></a>

<a id="creating-a-tree-data-structure"></a>
#### Criando uma Estrutura de Dados de Árvore

Para começar, construiremos uma árvore com nós que conhecem seus nós filhos.
Criaremos uma *struct* chamada `Node` que armazena seu próprio valor `i32`, bem
como referências para seus valores `Node` filhos:

<span class="filename">Arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-27/src/main.rs:here}}
```

Queremos que um `Node` seja dono de seus filhos, e queremos compartilhar essa
propriedade com variáveis para podermos acessar cada `Node` na árvore
diretamente. Para fazer isso, definimos os itens de `Vec<T>` como valores do
tipo `Rc<Node>`. Também queremos modificar quais nós são filhos de outro nó,
então temos um `RefCell<T>` em `children` envolvendo o `Vec<Rc<Node>>`.

Em seguida, usaremos nossa definição de *struct* e criaremos uma instância de
`Node` chamada `leaf` com o valor `3` e sem filhos, e outra instância chamada
`branch` com o valor `5` e `leaf` como um de seus filhos, como mostrado na
Listagem 15-27.

<Listing number="15-27" file-name="src/main.rs" caption="Criando um nó `leaf` sem filhos e um nó `branch` com `leaf` como um de seus filhos">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-27/src/main.rs:there}}
```

</Listing>

Clonamos o `Rc<Node>` em `leaf` e armazenamos isso em `branch`, o que significa
que o `Node` em `leaf` agora tem dois proprietários: `leaf` e `branch`. Podemos
ir de `branch` para `leaf` por meio de `branch.children`, mas não há como ir de
`leaf` para `branch`. O motivo é que `leaf` não tem referência para `branch` e
não sabe que eles estão relacionados. Queremos que `leaf` saiba que `branch` é
seu pai. Faremos isso a seguir.

<a id="adding-a-reference-from-a-child-to-its-parent"></a>
#### Adicionando uma Referência de um Filho para seu Pai

Para tornar o nó filho ciente de seu pai, precisamos adicionar um campo `parent`
à nossa definição da *struct* `Node`. O problema está em decidir qual deve ser o
tipo de `parent`. Sabemos que não pode conter um `Rc<T>`, porque isso criaria um
ciclo de referência com `leaf.parent` apontando para `branch` e `branch.children`
apontando para `leaf`, o que faria com que seus valores `strong_count` nunca
fossem 0.

Pensando nas relações de outra forma, um nó pai deve ser dono de seus filhos: se
um nó pai for liberado, seus nós filhos também devem ser liberados. No entanto,
um filho não deve ser dono de seu pai: se liberarmos um nó filho, o pai ainda
deve existir. Esse é um caso para referências fracas!

Portanto, em vez de `Rc<T>`, faremos o tipo de `parent` usar `Weak<T>`,
especificamente um `RefCell<Weak<Node>>`. Agora nossa definição da *struct*
`Node` fica assim:

<span class="filename">Arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-28/src/main.rs:here}}
```

Um nó poderá se referir ao seu nó pai, mas não é dono de seu pai. Na Listagem
15-28, atualizamos `main` para usar essa nova definição, de modo que o nó `leaf`
tenha uma maneira de se referir ao seu pai, `branch`.

<Listing number="15-28" file-name="src/main.rs" caption="Um nó `leaf` com uma referência fraca para seu nó pai, `branch`">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-28/src/main.rs:there}}
```

</Listing>

Criar o nó `leaf` é semelhante à Listagem 15-27, com exceção do campo `parent`:
`leaf` começa sem um pai, então criamos uma nova instância vazia de referência
`Weak<Node>`.

Nesse ponto, quando tentamos obter uma referência ao pai de `leaf` usando o
método `upgrade`, obtemos um valor `None`. Vemos isso na saída da primeira
instrução `println!`:

```text
leaf parent = None
```

Quando criamos o nó `branch`, ele também terá uma nova referência `Weak<Node>`
no campo `parent` porque `branch` não tem um nó pai. Ainda temos `leaf` como um
dos filhos de `branch`. Assim que temos a instância de `Node` em `branch`,
podemos modificar `leaf` para dar a ele uma referência `Weak<Node>` para seu
pai. Usamos o método `borrow_mut` no `RefCell<Weak<Node>>` no campo `parent` de
`leaf` e, em seguida, usamos a função `Rc::downgrade` para criar uma referência
`Weak<Node>` para `branch` a partir do `Rc<Node>` em `branch`.

Quando imprimimos o pai de `leaf` novamente, desta vez obteremos uma variante
`Some` contendo `branch`: agora `leaf` pode acessar seu pai! Quando imprimimos
`leaf`, também evitamos o ciclo que eventualmente terminava em um *stack
overflow* como tivemos na Listagem 15-26; as referências `Weak<Node>` são
impressas como `(Weak)`:

```text
leaf parent = Some(Node { value: 5, parent: RefCell { value: (Weak) },
children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) },
children: RefCell { value: [] } }] } })
```

A ausência de saída infinita indica que esse código não criou um ciclo de
referência. Também podemos perceber isso observando os valores que obtemos ao
chamar `Rc::strong_count` e `Rc::weak_count`.

<a id="visualizing-changes-to-strong_count-and-weak_count"></a>
#### Visualizando Mudanças em `strong_count` e `weak_count`

Vamos ver como os valores `strong_count` e `weak_count` das instâncias de
`Rc<Node>` mudam criando um novo escopo interno e movendo a criação de `branch`
para dentro desse escopo. Ao fazer isso, podemos ver o que acontece quando
`branch` é criado e depois liberado ao sair de escopo. As modificações são
mostradas na Listagem 15-29.

<Listing number="15-29" file-name="src/main.rs" caption="Criando `branch` em um escopo interno e examinando as contagens de referências fortes e fracas">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-29/src/main.rs:here}}
```

</Listing>

Depois que `leaf` é criado, seu `Rc<Node>` tem uma contagem forte de 1 e uma
contagem fraca de 0. No escopo interno, criamos `branch` e o associamos a
`leaf`; nesse momento, quando imprimimos as contagens, o `Rc<Node>` em `branch`
terá uma contagem forte de 1 e uma contagem fraca de 1 (por causa de
`leaf.parent` apontando para `branch` com um `Weak<Node>`). Quando imprimimos as
contagens em `leaf`, veremos que ele terá uma contagem forte de 2 porque `branch`
agora tem um clone do `Rc<Node>` de `leaf` armazenado em `branch.children`, mas
ainda terá uma contagem fraca de 0.

Quando o escopo interno termina, `branch` sai de escopo e a contagem forte do
`Rc<Node>` diminui para 0, portanto seu `Node` é liberado. A contagem fraca de 1
de `leaf.parent` não tem influência sobre o `Node` ser liberado ou não, portanto
não temos nenhum vazamento de memória!

Se tentarmos acessar o pai de `leaf` após o fim do escopo, obteremos `None`
novamente. No final do programa, o `Rc<Node>` em `leaf` tem uma contagem forte
de 1 e uma contagem fraca de 0 porque a variável `leaf` é agora a única
referência ao `Rc<Node>` novamente.

Toda a lógica que gerencia as contagens e a liberação de valores está embutida
em `Rc<T>` e `Weak<T>` e em suas implementações da *trait* `Drop`. Ao
especificar que a relação de um filho para seu pai deve ser uma referência
`Weak<T>` na definição de `Node`, você consegue ter nós pais apontando para nós
filhos e vice-versa sem criar um ciclo de referência e vazamentos de memória.

<a id="summary"></a>
## Resumo

Este capítulo abordou como usar *smart pointers* para oferecer garantias e
contrapartidas diferentes daquelas que o Rust faz por padrão com referências
regulares. O tipo `Box<T>` tem um tamanho conhecido e aponta para dados alocados
no *heap*. O tipo `Rc<T>` mantém o controle do número de referências a dados no
*heap* para que os dados possam ter múltiplos proprietários. O tipo `RefCell<T>`
com sua mutabilidade interior nos dá um tipo que podemos usar quando precisamos
de um tipo imutável, mas precisamos alterar um valor interno desse tipo; ele
também aplica as regras de *borrowing* em tempo de execução em vez de em tempo
de compilação.

Também foram discutidas as *traits* `Deref` e `Drop`, que habilitam grande parte
da funcionalidade dos *smart pointers*. Exploramos ciclos de referência que
podem causar vazamentos de memória e como evitá-los usando `Weak<T>`.

Se este capítulo despertou seu interesse e você quiser implementar seus próprios
*smart pointers*, confira ["The Rustonomicon"][nomicon] para obter mais
informações úteis.

A seguir, falaremos sobre concorrência em Rust. Você até aprenderá sobre alguns
novos *smart pointers*.

[nomicon]: ../nomicon/index.html
