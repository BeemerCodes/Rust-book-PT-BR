<a id="refcellt-and-the-interior-mutability-pattern"></a>
## `RefCell<T>` e o Padrão de Mutabilidade Interior

*Mutabilidade interior* (*interior mutability*) é um padrão de projeto em Rust
que permite mutar dados mesmo quando há referências imutáveis para esses dados;
normalmente, essa ação não é permitida pelas regras de *borrowing*. Para mutar
dados, o padrão usa código `unsafe` dentro de uma estrutura de dados para
contornar as regras usuais do Rust que regem mutação e *borrowing*. Código
`unsafe` indica ao compilador que estamos verificando as regras manualmente em
vez de depender do compilador para verificá-las por nós; discutiremos código
`unsafe` com mais detalhes no Capítulo 20.

Podemos usar tipos que aplicam o padrão de mutabilidade interior apenas quando
conseguimos garantir que as regras de *borrowing* serão seguidas em tempo de
execução, mesmo que o compilador não possa garantir isso. O código `unsafe`
envolvido é então encapsulado em uma API segura, e o tipo externo continua
imutável.

Vamos explorar esse conceito observando o tipo `RefCell<T>`, que segue o padrão
de mutabilidade interior.

<!-- Old headings. Do not remove or links may break. -->

<a id="enforcing-borrowing-rules-at-runtime-with-refcellt"></a>

<a id="enforcing-borrowing-rules-at-runtime"></a>
### Aplicando Regras de *Borrowing* em Tempo de Execução

Diferentemente de `Rc<T>`, o tipo `RefCell<T>` representa propriedade única
(*single ownership*) sobre os dados que armazena. Então, o que torna
`RefCell<T>` diferente de um tipo como `Box<T>`? Relembre as regras de
*borrowing* que você aprendeu no Capítulo 4:

- A qualquer momento, você pode ter *ou* uma referência mutável *ou* qualquer
  número de referências imutáveis (mas não ambos).
- Referências devem ser sempre válidas.

Com referências e `Box<T>`, as invariantes das regras de *borrowing* são
aplicadas em tempo de compilação. Com `RefCell<T>`, essas invariantes são
aplicadas *em tempo de execução*. Com referências, se você quebrar essas regras,
receberá um erro de compilação. Com `RefCell<T>`, se você quebrar essas regras,
seu programa entrará em *panic* e encerrará.

As vantagens de verificar as regras de *borrowing* em tempo de compilação são
que os erros são capturados mais cedo no processo de desenvolvimento e não há
impacto no desempenho em tempo de execução, pois toda a análise é concluída
antecipadamente. Por esses motivos, verificar as regras de *borrowing* em tempo
de compilação é a melhor escolha na maioria dos casos, e é por isso que esse é o
padrão do Rust.

A vantagem de verificar as regras de *borrowing* em tempo de execução é que
certos cenários seguros em termos de memória passam a ser permitidos, quando
teriam sido rejeitados pelas verificações em tempo de compilação. A análise
estática, como a do compilador Rust, é inerentemente conservadora. Algumas
propriedades de código são impossíveis de detectar analisando o código: o
exemplo mais famoso é o Problema da Parada (*Halting Problem*), que está além
do escopo deste livro, mas é um tópico interessante para pesquisar.

Como algumas análises são impossíveis, se o compilador Rust não puder ter
certeza de que o código cumpre as regras de *ownership*, ele pode rejeitar um
programa correto; dessa forma, ele é conservador. Se o Rust aceitasse um
programa incorreto, os usuários não poderiam confiar nas garantias que o Rust
oferece. No entanto, se o Rust rejeitar um programa correto, o programador terá
um inconveniente, mas nada catastrófico pode ocorrer. O tipo `RefCell<T>` é útil
quando você tem certeza de que seu código segue as regras de *borrowing*, mas o
compilador não consegue entender e garantir isso.

De forma semelhante a `Rc<T>`, `RefCell<T>` é apenas para uso em cenários de
*thread* única e gerará um erro de compilação se você tentar usá-lo em um
contexto *multithreaded*. Falaremos sobre como obter a funcionalidade de
`RefCell<T>` em um programa *multithreaded* no Capítulo 16.

Aqui está um resumo dos motivos para escolher `Box<T>`, `Rc<T>` ou `RefCell<T>`:

- `Rc<T>` permite múltiplos proprietários dos mesmos dados; `Box<T>` e
  `RefCell<T>` têm proprietários únicos.
- `Box<T>` permite *borrows* imutáveis ou mutáveis verificados em tempo de
  compilação; `Rc<T>` permite apenas *borrows* imutáveis verificados em tempo de
  compilação; `RefCell<T>` permite *borrows* imutáveis ou mutáveis verificados
  em tempo de execução.
- Como `RefCell<T>` permite *borrows* mutáveis verificados em tempo de execução,
  você pode mutar o valor dentro de `RefCell<T>` mesmo quando o `RefCell<T>` é
  imutável.

Mutar o valor dentro de um valor imutável é o padrão de mutabilidade interior.
Vamos ver uma situação em que a mutabilidade interior é útil e examinar como ela
é possível.

<!-- Old headings. Do not remove or links may break. -->

<a id="interior-mutability-a-mutable-borrow-to-an-immutable-value"></a>

<a id="using-interior-mutability"></a>
### Usando Mutabilidade Interior

Uma consequência das regras de *borrowing* é que, quando você tem um valor
imutável, não pode fazer um *borrow* mutável dele. Por exemplo, este código não
compila:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/no-listing-01-cant-borrow-immutable-as-mutable/src/main.rs}}
```

Se você tentasse compilar esse código, obteria o seguinte erro:

```console
{{#include ../listings/ch15-smart-pointers/no-listing-01-cant-borrow-immutable-as-mutable/output.txt}}
```

No entanto, há situações em que seria útil que um valor se mutasse dentro de
seus métodos, mas parecesse imutável para outros códigos. Códigos fora dos
métodos do valor não poderiam mutar o valor. Usar `RefCell<T>` é uma maneira de
obter mutabilidade interior, mas `RefCell<T>` não contorna as regras de
*borrowing* completamente: o *borrow checker* no compilador permite essa
mutabilidade interior, e as regras de *borrowing* são verificadas em tempo de
execução. Se você violar as regras, obterá um `panic!` em vez de um erro de
compilação.

Vamos trabalhar em um exemplo prático em que podemos usar `RefCell<T>` para
mutar um valor imutável e ver por que isso é útil.

<!-- Old headings. Do not remove or links may break. -->

<a id="a-use-case-for-interior-mutability-mock-objects"></a>

<a id="testing-with-mock-objects"></a>
#### Testando com Objetos *Mock*

Às vezes, durante os testes automatizados, um programador usa um tipo no lugar
de outro para observar um comportamento específico e verificar se ele está
implementado corretamente. Esse tipo substituto é chamado de *test double*.
Pense nisso no sentido de um dublê no cinema, em que uma pessoa entra e
substitui um ator para fazer uma cena particularmente complicada. *Test doubles*
substituem outros tipos quando estamos executando testes. *Objetos mock* (*mock
objects*) são tipos específicos de *test doubles* que registram o que acontece
durante um teste para que você possa verificar se as ações corretas ocorreram.

O Rust não tem objetos no mesmo sentido que outras linguagens, e não tem
funcionalidade de objetos *mock* embutida na biblioteca padrão como algumas
outras linguagens têm. No entanto, você certamente pode criar uma *struct* que
servirá aos mesmos propósitos de um objeto *mock*.

Aqui está o cenário que vamos testar: criaremos uma biblioteca que acompanha um
valor em relação a um valor máximo e envia mensagens com base em quão próximo o
valor atual está do valor máximo. Essa biblioteca poderia ser usada para
acompanhar a cota de chamadas de API que um usuário tem permissão para fazer,
por exemplo.

Nossa biblioteca fornecerá apenas a funcionalidade de acompanhar quão próximo um
valor está do máximo e quais mensagens devem ser enviadas em quais momentos.
Espera-se que as aplicações que usam nossa biblioteca forneçam o mecanismo para
enviar as mensagens: a aplicação pode mostrar a mensagem diretamente ao usuário,
enviar um e-mail, enviar uma mensagem de texto ou fazer outra coisa. A
biblioteca não precisa saber desse detalhe. Tudo o que ela precisa é de algo que
implemente uma *trait* que forneceremos, chamada `Messenger`. A Listagem 15-20
mostra o código da biblioteca.

<Listing number="15-20" file-name="src/lib.rs" caption="Uma biblioteca para acompanhar quão próximo um valor está de um valor máximo e avisar quando o valor atinge certos níveis">

```rust,noplayground
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-20/src/lib.rs}}
```

</Listing>

Uma parte importante desse código é que a *trait* `Messenger` tem um método
chamado `send` que recebe uma referência imutável a `self` e o texto da
mensagem. Essa *trait* é a interface que nosso objeto *mock* precisa implementar
para que o *mock* possa ser usado da mesma forma que um objeto real. A outra
parte importante é que queremos testar o comportamento do método `set_value` em
`LimitTracker`. Podemos mudar o que passamos para o parâmetro `value`, mas
`set_value` não retorna nada para fazermos asserções. Queremos poder afirmar
que, se criarmos um `LimitTracker` com algo que implementa a *trait* `Messenger`
e um determinado valor para `max`, o mensageiro é instruído a enviar as
mensagens apropriadas quando passamos números diferentes para `value`.

Precisamos de um objeto *mock* que, em vez de enviar um e-mail ou mensagem de
texto quando chamamos `send`, apenas mantenha o registro das mensagens que foi
instruído a enviar. Podemos criar uma nova instância do objeto *mock*, criar um
`LimitTracker` que usa o objeto *mock*, chamar o método `set_value` em
`LimitTracker` e então verificar se o objeto *mock* contém as mensagens que
esperamos. A Listagem 15-21 mostra uma tentativa de implementar um objeto *mock*
para fazer exatamente isso, mas o *borrow checker* não permitirá.

<Listing number="15-21" file-name="src/lib.rs" caption="Uma tentativa de implementar um `MockMessenger` que não é permitida pelo *borrow checker*">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-21/src/lib.rs:here}}
```

</Listing>

Esse código de teste define uma *struct* `MockMessenger` que tem um campo
`sent_messages` com um `Vec` de valores `String` para manter o registro das
mensagens que foi instruído a enviar. Também definimos uma função associada
`new` para facilitar a criação de novos valores `MockMessenger` que começam com
uma lista vazia de mensagens. Em seguida, implementamos a *trait* `Messenger`
para `MockMessenger` para podermos passar um `MockMessenger` a um
`LimitTracker`. Na definição do método `send`, pegamos a mensagem passada como
parâmetro e a armazenamos na lista `sent_messages` de `MockMessenger`.

No teste, estamos testando o que acontece quando o `LimitTracker` é instruído a
definir `value` para algo maior que 75 por cento do valor `max`. Primeiro,
criamos um novo `MockMessenger`, que começará com uma lista vazia de mensagens.
Em seguida, criamos um novo `LimitTracker` e passamos a ele uma referência ao
novo `MockMessenger` e um valor `max` de `100`. Chamamos o método `set_value` no
`LimitTracker` com um valor de `80`, que é maior que 75 por cento de 100.
Depois, verificamos se a lista de mensagens que o `MockMessenger` está
acompanhando agora tem uma mensagem.

No entanto, há um problema com esse teste, como mostrado aqui:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-21/output.txt}}
```

Não podemos modificar o `MockMessenger` para registrar as mensagens, porque o
método `send` recebe uma referência imutável a `self`. Também não podemos
aceitar a sugestão da mensagem de erro de usar `&mut self` tanto no método
`impl` quanto na definição da *trait*. Não queremos alterar a *trait*
`Messenger` apenas para fins de teste. Em vez disso, precisamos encontrar uma
maneira de fazer nosso código de teste funcionar corretamente com nosso design
existente.

Essa é uma situação em que a mutabilidade interior pode ajudar! Armazenaremos
`sent_messages` dentro de um `RefCell<T>`, e então o método `send` poderá
modificar `sent_messages` para armazenar as mensagens que vimos. A Listagem
15-22 mostra como isso fica.

<Listing number="15-22" file-name="src/lib.rs" caption="Usando `RefCell<T>` para mutar um valor interno enquanto o valor externo é considerado imutável">

```rust,noplayground
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-22/src/lib.rs:here}}
```

</Listing>

O campo `sent_messages` agora é do tipo `RefCell<Vec<String>>` em vez de
`Vec<String>`. Na função `new`, criamos uma nova instância de
`RefCell<Vec<String>>` envolvendo o vetor vazio.

Na implementação do método `send`, o primeiro parâmetro continua sendo um
*borrow* imutável de `self`, o que corresponde à definição da *trait*. Chamamos
`borrow_mut` no `RefCell<Vec<String>>` em `self.sent_messages` para obter uma
referência mutável ao valor dentro do `RefCell<Vec<String>>`, que é o vetor.
Então, podemos chamar `push` na referência mutável ao vetor para registrar as
mensagens enviadas durante o teste.

A última mudança que precisamos fazer está na asserção: para ver quantos itens
estão no vetor interno, chamamos `borrow` no `RefCell<Vec<String>>` para obter
uma referência imutável ao vetor.

Agora que você viu como usar `RefCell<T>`, vamos entender como ele funciona!

<!-- Old headings. Do not remove or links may break. -->

<a id="keeping-track-of-borrows-at-runtime-with-refcellt"></a>

<a id="tracking-borrows-at-runtime"></a>
#### Rastreando *Borrows* em Tempo de Execução

Ao criar referências imutáveis e mutáveis, usamos a sintaxe `&` e `&mut`,
respectivamente. Com `RefCell<T>`, usamos os métodos `borrow` e `borrow_mut`,
que fazem parte da API segura pertencente a `RefCell<T>`. O método `borrow`
retorna o tipo de *smart pointer* `Ref<T>`, e `borrow_mut` retorna o tipo de
*smart pointer* `RefMut<T>`. Ambos os tipos implementam `Deref`, portanto
podemos tratá-los como referências regulares.

O `RefCell<T>` mantém o controle de quantos *smart pointers* `Ref<T>` e
`RefMut<T>` estão ativos no momento. Cada vez que chamamos `borrow`, o
`RefCell<T>` aumenta sua contagem de quantos *borrows* imutáveis estão ativos.
Quando um valor `Ref<T>` sai de escopo, a contagem de *borrows* imutáveis
diminui em 1. Assim como nas regras de *borrowing* em tempo de compilação, o
`RefCell<T>` nos permite ter muitos *borrows* imutáveis ou um *borrow* mutável
em qualquer momento.

Se tentarmos violar essas regras, em vez de receber um erro de compilação como
aconteceria com referências, a implementação de `RefCell<T>` entrará em *panic*
em tempo de execução. A Listagem 15-23 mostra uma modificação da implementação de
`send` na Listagem 15-22. Estamos tentando deliberadamente criar dois *borrows*
mutáveis ativos no mesmo escopo para ilustrar que `RefCell<T>` nos impede de
fazer isso em tempo de execução.

<Listing number="15-23" file-name="src/lib.rs" caption="Criando duas referências mutáveis no mesmo escopo para ver que `RefCell<T>` entrará em *panic*">

```rust,ignore,panics
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-23/src/lib.rs:here}}
```

</Listing>

Criamos uma variável `one_borrow` para o *smart pointer* `RefMut<T>` retornado
por `borrow_mut`. Em seguida, criamos outro *borrow* mutável da mesma forma na
variável `two_borrow`. Isso cria duas referências mutáveis no mesmo escopo, o
que não é permitido. Quando executamos os testes da nossa biblioteca, o código
da Listagem 15-23 compilará sem erros, mas o teste falhará:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-23/output.txt}}
```

Observe que o código entrou em *panic* com a mensagem `already borrowed:
BorrowMutError`. É assim que `RefCell<T>` lida com violações das regras de
*borrowing* em tempo de execução.

Escolher capturar erros de *borrowing* em tempo de execução em vez de em tempo
de compilação, como fizemos aqui, significa que você pode acabar encontrando
erros no seu código mais tarde no processo de desenvolvimento: possivelmente
apenas quando o código já estiver em produção. Além disso, seu código terá uma
pequena penalidade de desempenho em tempo de execução como resultado de rastrear
os *borrows* em tempo de execução em vez de em tempo de compilação. No entanto,
usar `RefCell<T>` torna possível escrever um objeto *mock* que pode se modificar
para registrar as mensagens que viu enquanto é usado em um contexto em que
apenas valores imutáveis são permitidos. Você pode usar `RefCell<T>` apesar de
suas contrapartidas para obter mais funcionalidade do que referências regulares
fornecem.

<!-- Old headings. Do not remove or links may break. -->

<a id="having-multiple-owners-of-mutable-data-by-combining-rc-t-and-ref-cell-t"></a>
<a id="allowing-multiple-owners-of-mutable-data-with-rct-and-refcellt"></a>

<a id="allowing-multiple-owners-of-mutable-data"></a>
### Permitindo Múltiplos Proprietários de Dados Mutáveis

Uma forma comum de usar `RefCell<T>` é em combinação com `Rc<T>`. Lembre-se de
que `Rc<T>` permite que você tenha múltiplos proprietários de alguns dados, mas
só dá acesso imutável a esses dados. Se você tiver um `Rc<T>` que armazena um
`RefCell<T>`, poderá obter um valor que tem múltiplos proprietários *e* que você
pode mutar!

Por exemplo, lembre-se do exemplo da *cons list* na Listagem 15-18, em que
usamos `Rc<T>` para permitir que múltiplas listas compartilhassem a propriedade
de outra lista. Como `Rc<T>` armazena apenas valores imutáveis, não podemos
alterar nenhum dos valores na lista depois de criá-los. Vamos adicionar
`RefCell<T>` por sua capacidade de alterar os valores nas listas. A Listagem
15-24 mostra que, usando um `RefCell<T>` na definição de `Cons`, podemos
modificar o valor armazenado em todas as listas.

<Listing number="15-24" file-name="src/main.rs" caption="Usando `Rc<RefCell<i32>>` para criar uma `List` que podemos mutar">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-24/src/main.rs}}
```

</Listing>

Criamos um valor que é uma instância de `Rc<RefCell<i32>>` e o armazenamos em
uma variável chamada `value` para podermos acessá-lo diretamente mais tarde. Em
seguida, criamos uma `List` em `a` com uma variante `Cons` que armazena `value`.
Precisamos clonar `value` para que tanto `a` quanto `value` tenham propriedade
do valor interno `5`, em vez de transferir a propriedade de `value` para `a` ou
fazer `a` realizar *borrowing* de `value`.

Envolvemos a lista `a` em um `Rc<T>` para que, quando criarmos as listas `b` e
`c`, ambas possam se referir a `a`, que é o que fizemos na Listagem 15-18.

Depois de criarmos as listas em `a`, `b` e `c`, queremos adicionar 10 ao valor
em `value`. Fazemos isso chamando `borrow_mut` em `value`, o que usa o recurso
de desreferência automática que discutimos em ["Onde Está o Operador
`->`?"][wheres-the---operator]<!-- ignore --> no Capítulo 5 para desreferenciar
o `Rc<T>` até o valor interno `RefCell<T>`. O método `borrow_mut` retorna um
*smart pointer* `RefMut<T>`, e usamos o operador de desreferência nele para
alterar o valor interno.

Quando imprimimos `a`, `b` e `c`, podemos ver que todos eles têm o valor
modificado `15` em vez de `5`:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-24/output.txt}}
```

Essa técnica é bem legal! Usando `RefCell<T>`, temos um valor `List`
externamente imutável. Mas podemos usar os métodos em `RefCell<T>` que fornecem
acesso à sua mutabilidade interior para podermos modificar nossos dados quando
precisarmos. As verificações em tempo de execução das regras de *borrowing* nos
protegem contra corridas de dados, e às vezes vale a pena trocar um pouco de
velocidade por essa flexibilidade em nossas estruturas de dados. Observe que
`RefCell<T>` não funciona para código *multithreaded*! `Mutex<T>` é a versão
*thread-safe* de `RefCell<T>`, e discutiremos `Mutex<T>` no Capítulo 16.

[wheres-the---operator]: ch05-03-method-syntax.html#wheres-the---operator
