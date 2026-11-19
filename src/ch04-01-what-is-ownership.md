<a id="what-is-ownership"></a>
## O Que É Ownership?

_Ownership_ é um conjunto de regras que governa como um programa Rust gerencia a memória. Todos os programas precisam gerenciar a forma como usam a memória do computador enquanto estão em execução. Algumas linguagens têm coleta de lixo (*garbage collection*) que procura regularmente por memória que não é mais usada enquanto o programa roda; em outras linguagens, o programador precisa alocar e liberar memória explicitamente. O Rust usa uma terceira abordagem: a memória é gerenciada por meio de um sistema de *ownership* com um conjunto de regras que o compilador verifica. Se qualquer uma das regras for violada, o programa não compila. Nenhum dos recursos de *ownership* deixará seu programa mais lento durante a execução.

Como *ownership* é um conceito novo para muitos programadores, leva algum tempo para se acostumar. A boa notícia é que, quanto mais experiente você ficar com Rust e com as regras do sistema de *ownership*, mais fácil será desenvolver naturalmente código seguro e eficiente. Continue firme!

Quando você entender *ownership*, terá uma base sólida para compreender os recursos que tornam o Rust único. Neste capítulo, você aprenderá *ownership* trabalhando em alguns exemplos focados em uma estrutura de dados muito comum: strings.

> <a id="the-stack-and-the-heap"></a>
> ### A Stack e a Heap
>
> Muitas linguagens de programação não exigem que você pense na *stack* e na *heap* com frequência. Mas em uma linguagem de programação de sistemas como Rust, o fato de um valor estar na *stack* ou na *heap* afeta como a linguagem se comporta e por que você precisa tomar certas decisões. Partes de *ownership* serão descritas em relação à *stack* e à *heap* mais adiante neste capítulo, então aqui vai uma breve explicação preparatória.
>
> Tanto a *stack* quanto a *heap* são partes da memória disponíveis para seu código usar em tempo de execução, mas são estruturadas de maneiras diferentes. A *stack* armazena valores na ordem em que os recebe e remove os valores na ordem oposta. Isso é conhecido como _último a entrar, primeiro a sair (LIFO — last in, first out)_. Pense em uma pilha de pratos: quando você adiciona mais pratos, coloca-os no topo da pilha, e quando precisa de um prato, tira um do topo. Adicionar ou remover pratos do meio ou da base não funcionaria tão bem! Adicionar dados é chamado de _empilhar na stack_ (*pushing onto the stack*), e remover dados é chamado de _desempilhar da stack_ (*popping off the stack*). Todos os dados armazenados na *stack* devem ter um tamanho conhecido e fixo. Dados com tamanho desconhecido em tempo de compilação ou cujo tamanho pode mudar devem ser armazenados na *heap*.
>
> A *heap* é menos organizada: quando você coloca dados na *heap*, solicita uma certa quantidade de espaço. O alocador de memória encontra um espaço vazio na *heap* que seja grande o suficiente, marca-o como em uso e retorna um _ponteiro_, que é o endereço desse local. Esse processo é chamado de _alocar na heap_ e às vezes é abreviado simplesmente como _alocar_ (empilhar valores na *stack* não é considerado alocação). Como o ponteiro para a *heap* tem um tamanho conhecido e fixo, você pode armazenar o ponteiro na *stack*, mas quando quiser os dados reais, precisará seguir o ponteiro. Pense em sentar-se em um restaurante. Quando você entra, informa o número de pessoas no seu grupo, e o recepcionista encontra uma mesa vazia que comporta todos e leva vocês até lá. Se alguém do seu grupo chegar atrasado, pode perguntar onde vocês estão sentados para encontrá-los.
>
> Empilhar na *stack* é mais rápido do que alocar na *heap* porque o alocador nunca precisa procurar um lugar para armazenar novos dados; esse local é sempre o topo da *stack*. Comparativamente, alocar espaço na *heap* exige mais trabalho porque o alocador precisa primeiro encontrar um espaço grande o suficiente para guardar os dados e depois fazer o controle interno para preparar a próxima alocação.
>
> Acessar dados na *heap* geralmente é mais lento do que acessar dados na *stack* porque você precisa seguir um ponteiro para chegar lá. Processadores modernos são mais rápidos quando precisam pular menos pela memória. Continuando a analogia, considere um garçom em um restaurante anotando pedidos de várias mesas. É mais eficiente anotar todos os pedidos de uma mesa antes de passar para a próxima. Anotar um pedido da mesa A, depois um da mesa B, depois outro da A novamente e outro da B novamente seria um processo muito mais lento. Da mesma forma, um processador geralmente faz melhor seu trabalho se operar sobre dados que estão próximos de outros dados (como na *stack*) em vez de mais distantes (como podem estar na *heap*).
>
> Quando seu código chama uma função, os valores passados para a função (incluindo, potencialmente, ponteiros para dados na *heap*) e as variáveis locais da função são empilhados na *stack*. Quando a função termina, esses valores são desempilhados da *stack*.
>
> Acompanhar quais partes do código estão usando quais dados na *heap*, minimizar a quantidade de dados duplicados na *heap* e limpar dados não utilizados na *heap* para que não falte espaço são todos problemas que *ownership* resolve. Depois que você entender *ownership*, não precisará pensar na *stack* e na *heap* com tanta frequência. Mas saber que o principal objetivo de *ownership* é gerenciar dados na *heap* ajuda a explicar por que ele funciona da maneira que funciona.

<a id="ownership-rules"></a>
### Regras de Ownership

Primeiro, vamos dar uma olhada nas regras de *ownership*. Tenha essas regras em mente enquanto trabalhamos nos exemplos que as ilustram:

- Cada valor em Rust tem um _dono_ (*owner*).
- Só pode haver um dono por vez.
- Quando o dono sai de escopo, o valor será descartado (*dropped*).

<a id="variable-scope"></a>
### Escopo de Variáveis

Agora que já passamos pela sintaxe básica do Rust, não incluiremos todo o código `fn main() {` nos exemplos, portanto, se estiver acompanhando, lembre-se de colocar os exemplos a seguir dentro de uma função `main` manualmente. Como resultado, nossos exemplos ficarão mais concisos, permitindo focar nos detalhes reais em vez de código repetitivo.

Como primeiro exemplo de *ownership*, vamos observar o escopo de algumas variáveis. Um _escopo_ é o intervalo dentro de um programa no qual um item é válido. Considere a seguinte variável:

```rust
let s = "hello";
```

A variável `s` refere-se a um literal de string, cujo valor está gravado diretamente no texto do nosso programa. A variável é válida desde o ponto em que é declarada até o final do escopo atual. A Listagem 4-1 mostra um programa com comentários indicando onde a variável `s` seria válida.

<Listing number="4-1" caption="Uma variável e o escopo no qual ela é válida">

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-01/src/main.rs:here}}
```

</Listing>

Em outras palavras, há dois momentos importantes aqui:

- Quando `s` entra _em_ escopo, ela é válida.
- Ela permanece válida até sair _de_ escopo.

Neste ponto, a relação entre escopos e quando as variáveis são válidas é semelhante à de outras linguagens de programação. Agora vamos construir sobre esse entendimento introduzindo o tipo `String`.

<a id="the-string-type"></a>
### O Tipo `String`

Para ilustrar as regras de *ownership*, precisamos de um tipo de dados mais complexo do que os que abordamos na seção ["Tipos de Dados"][data-types]<!-- ignore --> do Capítulo 3. Os tipos vistos anteriormente têm tamanho conhecido, podem ser armazenados na *stack* e desempilhados quando seu escopo termina, e podem ser copiados rápida e trivialmente para criar uma nova instância independente se outra parte do código precisar usar o mesmo valor em um escopo diferente. Mas queremos observar dados armazenados na *heap* e explorar como o Rust sabe quando limpar esses dados, e o tipo `String` é um ótimo exemplo.

Vamos nos concentrar nas partes de `String` relacionadas a *ownership*. Esses aspectos também se aplicam a outros tipos de dados complexos, sejam eles fornecidos pela biblioteca padrão ou criados por você. Discutiremos aspectos de `String` não relacionados a *ownership* no [Capítulo 8][ch8]<!-- ignore -->.

Já vimos literais de string, nos quais o valor da string é fixo no código do programa. Literais de string são convenientes, mas não servem para todas as situações em que queremos usar texto. Um motivo é que eles são imutáveis. Outro é que nem todo valor de string pode ser conhecido quando escrevemos nosso código: por exemplo, e se quisermos receber entrada do usuário e armazená-la? É para essas situações que o Rust tem o tipo `String`. Esse tipo gerencia dados alocados na *heap* e, portanto, é capaz de armazenar uma quantidade de texto desconhecida em tempo de compilação. Você pode criar uma `String` a partir de um literal de string usando a função `from`, assim:

```rust
let s = String::from("hello");
```

O operador de dois pontos duplos `::` nos permite colocar essa função `from` específica dentro do namespace do tipo `String`, em vez de usar algum nome como `string_from`. Discutiremos mais sobre essa sintaxe na seção ["Métodos"][methods]<!-- ignore --> do Capítulo 5, e quando falarmos sobre namespaces com módulos em ["Caminhos para Referenciar um Item na Árvore de Módulos"][paths-module-tree]<!-- ignore --> no Capítulo 7.

Esse tipo de string _pode_ ser mutado:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-01-can-mutate-string/src/main.rs:here}}
```

Então, qual é a diferença aqui? Por que `String` pode ser mutada, mas literais não? A diferença está em como esses dois tipos lidam com a memória.

<a id="memory-and-allocation"></a>
### Memória e Alocação

No caso de um literal de string, conhecemos o conteúdo em tempo de compilação, então o texto é gravado diretamente no executável final. É por isso que literais de string são rápidos e eficientes. Mas essas propriedades vêm apenas da imutabilidade do literal de string. Infelizmente, não podemos colocar um bloco de memória no binário para cada pedaço de texto cujo tamanho é desconhecido em tempo de compilação e cujo tamanho pode mudar enquanto o programa roda.

Com o tipo `String`, para suportar um texto mutável e expansível, precisamos alocar uma quantidade de memória na *heap*, desconhecida em tempo de compilação, para guardar o conteúdo. Isso significa:

- A memória precisa ser solicitada ao alocador de memória em tempo de execução.
- Precisamos de uma forma de devolver essa memória ao alocador quando terminarmos de usar nossa `String`.

A primeira parte é feita por nós: quando chamamos `String::from`, sua implementação solicita a memória de que precisa. Isso é praticamente universal em linguagens de programação.

No entanto, a segunda parte é diferente. Em linguagens com um _coletor de lixo (GC — garbage collector)_, o GC acompanha e limpa a memória que não está mais sendo usada, e não precisamos pensar nisso. Na maioria das linguagens sem GC, é nossa responsabilidade identificar quando a memória não está mais em uso e chamar código para liberá-la explicitamente, assim como fizemos para solicitá-la. Fazer isso corretamente tem sido historicamente um problema difícil de programação. Se esquecermos, desperdiçaremos memória. Se fizermos cedo demais, teremos uma variável inválida. Se fizermos duas vezes, isso também é um bug. Precisamos parear exatamente um `allocate` com exatamente um `free`.

O Rust segue um caminho diferente: a memória é devolvida automaticamente assim que a variável que é dona dela sai de escopo. Aqui está uma versão do nosso exemplo de escopo da Listagem 4-1 usando uma `String` em vez de um literal de string:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-02-string-scope/src/main.rs:here}}
```

Há um ponto natural no qual podemos devolver ao alocador a memória de que nossa `String` precisa: quando `s` sai de escopo. Quando uma variável sai de escopo, o Rust chama uma função especial para nós. Essa função se chama `drop`, e é onde o autor de `String` pode colocar o código para devolver a memória. O Rust chama `drop` automaticamente na chave de fechamento.

> Nota: Em C++, esse padrão de desalocar recursos ao final do tempo de vida de um item às vezes é chamado de _Resource Acquisition Is Initialization (RAII)_. A função `drop` em Rust será familiar se você já usou padrões RAII.

Esse padrão tem um impacto profundo na maneira como código Rust é escrito. Pode parecer simples agora, mas o comportamento do código pode ser inesperado em situações mais complicadas quando queremos que múltiplas variáveis usem os dados que alocamos na *heap*. Vamos explorar algumas dessas situações agora.

<!-- Old headings. Do not remove or links may break. -->

<a id="ways-variables-and-data-interact-move"></a>

<a id="variables-and-data-interacting-with-move"></a>
#### Variáveis e Dados Interagindo com Move

Múltiplas variáveis podem interagir com os mesmos dados de maneiras diferentes em Rust. A Listagem 4-2 mostra um exemplo usando um inteiro.

<Listing number="4-2" caption="Atribuindo o valor inteiro da variável `x` para `y`">

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-02/src/main.rs:here}}
```

</Listing>

Provavelmente conseguimos adivinhar o que isso faz: "Vincule o valor `5` a `x`; depois, faça uma cópia do valor em `x` e vincule-a a `y`." Agora temos duas variáveis, `x` e `y`, e ambas são iguais a `5`. É de fato isso que está acontecendo, porque inteiros são valores simples com tamanho conhecido e fixo, e esses dois valores `5` são empilhados na *stack*.

Agora vamos ver a versão com `String`:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-03-string-move/src/main.rs:here}}
```

Isso parece muito semelhante, então poderíamos assumir que funcionaria da mesma maneira: isto é, a segunda linha faria uma cópia do valor em `s1` e o vincularia a `s2`. Mas não é bem isso que acontece.

Dê uma olhada na Figura 4-1 para ver o que está acontecendo com `String` por baixo dos panos. Uma `String` é composta de três partes, mostradas à esquerda: um ponteiro para a memória que contém o conteúdo da string, um tamanho (*length*) e uma capacidade (*capacity*). Esse grupo de dados é armazenado na *stack*. À direita está a memória na *heap* que guarda o conteúdo.

<img alt="Duas tabelas: a primeira tabela contém a representação de s1 na stack, consistindo de seu tamanho (5), capacidade (5) e um ponteiro para o primeiro valor na segunda tabela. A segunda tabela contém a representação dos dados da string na heap, byte a byte." src="img/trpl04-01.svg" class="center"
style="width: 50%;" />

<span class="caption">Figura 4-1: A representação em memória de uma `String` contendo o valor `"hello"` vinculada a `s1`</span>

O tamanho (*length*) é quanta memória, em bytes, o conteúdo da `String` está usando no momento. A capacidade (*capacity*) é a quantidade total de memória, em bytes, que a `String` recebeu do alocador. A diferença entre tamanho e capacidade importa, mas não neste contexto, então por enquanto tudo bem ignorar a capacidade.

Quando atribuímos `s1` a `s2`, os dados da `String` são copiados, o que significa que copiamos o ponteiro, o tamanho e a capacidade que estão na *stack*. Não copiamos os dados na *heap* aos quais o ponteiro se refere. Em outras palavras, a representação dos dados na memória fica como na Figura 4-2.

<img alt="Três tabelas: as tabelas s1 e s2 representando essas strings na stack, respectivamente, e ambas apontando para os mesmos dados de string na heap."
src="img/trpl04-02.svg" class="center" style="width: 50%;" />

<span class="caption">Figura 4-2: A representação em memória da variável `s2` que tem uma cópia do ponteiro, tamanho e capacidade de `s1`</span>

A representação _não_ fica como na Figura 4-3, que é como a memória ficaria se o Rust também copiasse os dados na *heap*. Se o Rust fizesse isso, a operação `s2 = s1` poderia ser muito cara em termos de desempenho em tempo de execução se os dados na *heap* fossem grandes.

<img alt="Quatro tabelas: duas tabelas representando os dados na stack para s1 e s2, e cada uma aponta para sua própria cópia dos dados da string na heap."
src="img/trpl04-03.svg" class="center" style="width: 50%;" />

<span class="caption">Figura 4-3: Outra possibilidade para o que `s2 = s1` poderia fazer se o Rust copiasse também os dados da heap</span>

Anteriormente, dissemos que quando uma variável sai de escopo, o Rust chama automaticamente a função `drop` e limpa a memória na *heap* para essa variável. Mas a Figura 4-2 mostra ambos os ponteiros de dados apontando para o mesmo local. Isso é um problema: quando `s2` e `s1` saírem de escopo, ambas tentarão liberar a mesma memória. Isso é conhecido como erro de _double free_ (liberação dupla) e é um dos bugs de segurança de memória que mencionamos anteriormente. Liberar memória duas vezes pode levar à corrupção de memória, o que potencialmente pode levar a vulnerabilidades de segurança.

Para garantir a segurança de memória, após a linha `let s2 = s1;`, o Rust considera `s1` como não mais válida. Portanto, o Rust não precisa liberar nada quando `s1` sair de escopo. Veja o que acontece quando você tenta usar `s1` depois que `s2` é criada; não vai funcionar:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-04-cant-use-after-move/src/main.rs:here}}
```

Você receberá um erro como este porque o Rust impede que você use a referência invalidada:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-04-cant-use-after-move/output.txt}}
```

Se você já ouviu os termos _cópia rasa_ (*shallow copy*) e _cópia profunda_ (*deep copy*) trabalhando com outras linguagens, o conceito de copiar o ponteiro, o tamanho e a capacidade sem copiar os dados provavelmente soa como fazer uma cópia rasa. Mas como o Rust também invalida a primeira variável, em vez de ser chamado de cópia rasa, isso é conhecido como _move_ (movimentação). Neste exemplo, diríamos que `s1` foi _movida_ para `s2`. Portanto, o que realmente acontece é mostrado na Figura 4-4.

<img alt="Três tabelas: as tabelas s1 e s2 representando essas strings na stack, respectivamente, e ambas apontando para os mesmos dados de string na heap. A tabela s1 está esmaecida porque s1 não é mais válida; apenas s2 pode ser usada para acessar os dados na heap." src="img/trpl04-04.svg" class="center" style="width:
50%;" />

<span class="caption">Figura 4-4: A representação em memória após `s1` ter sido invalidada</span>

Isso resolve nosso problema! Com apenas `s2` válida, quando ela sair de escopo, somente ela liberará a memória, e pronto.

Além disso, há uma decisão de design implícita nisso: o Rust nunca criará automaticamente cópias "profundas" dos seus dados. Portanto, pode-se assumir que qualquer cópia _automática_ é barata em termos de desempenho em tempo de execução.

<a id="scope-and-assignment"></a>
#### Escopo e Atribuição

O inverso disso também é verdadeiro para a relação entre escopo, *ownership* e memória liberada por meio da função `drop`. Quando você atribui um valor completamente novo a uma variável existente, o Rust chamará `drop` e liberará a memória do valor original imediatamente. Considere este código, por exemplo:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-04b-replacement-drop/src/main.rs:here}}
```

Inicialmente declaramos uma variável `s` e a vinculamos a uma `String` com o valor `"hello"`. Em seguida, criamos imediatamente uma nova `String` com o valor `"ahoy"` e a atribuímos a `s`. Neste ponto, nada mais está se referindo ao valor original na *heap*. A Figura 4-5 ilustra os dados na *stack* e na *heap* agora:

<img alt="Uma tabela representando o valor da string na stack, apontando para o segundo pedaço de dados da string (ahoy) na heap, com os dados da string original (hello) esmaecidos porque não podem mais ser acessados."
src="img/trpl04-05.svg" class="center" style="width: 50%;" />

<span class="caption">Figura 4-5: A representação em memória após o valor inicial ter sido substituído por completo</span>

A string original, portanto, sai imediatamente de escopo. O Rust executará a função `drop` nela e sua memória será liberada na hora. Quando imprimirmos o valor no final, será `"ahoy, world!"`.

<!-- Old headings. Do not remove or links may break. -->

<a id="ways-variables-and-data-interact-clone"></a>

<a id="variables-and-data-interacting-with-clone"></a>
#### Variáveis e Dados Interagindo com Clone

Se nós _realmente_ quisermos fazer uma cópia profunda dos dados na *heap* da `String`, e não apenas dos dados na *stack*, podemos usar um método comum chamado `clone`. Discutiremos a sintaxe de métodos no Capítulo 5, mas como métodos são um recurso comum em muitas linguagens de programação, você provavelmente já os viu antes.

Aqui está um exemplo do método `clone` em ação:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-05-clone/src/main.rs:here}}
```

Isso funciona perfeitamente e produz explicitamente o comportamento mostrado na Figura 4-3, em que os dados na *heap* _são_ copiados.

Quando você vê uma chamada a `clone`, sabe que algum código arbitrário está sendo executado e que esse código pode ser custoso. É um indicador visual de que algo diferente está acontecendo.

<a id="stack-only-data-copy"></a>
#### Dados Apenas na Stack: Copy

Há outro detalhe sobre o qual ainda não falamos. Este código usando inteiros — parte do qual foi mostrado na Listagem 4-2 — funciona e é válido:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-06-copy/src/main.rs:here}}
```

Mas este código parece contradizer o que acabamos de aprender: não temos uma chamada a `clone`, mas `x` continua válida e não foi movida para `y`.

O motivo é que tipos como inteiros, que têm tamanho conhecido em tempo de compilação, são armazenados inteiramente na *stack*, então cópias dos valores reais são rápidas de fazer. Isso significa que não há razão para querermos impedir que `x` continue válida depois de criarmos a variável `y`. Em outras palavras, não há diferença entre cópia profunda e cópia rasa aqui, então chamar `clone` não faria nada diferente da cópia rasa usual, e podemos omiti-lo.

O Rust tem uma anotação especial chamada *trait* `Copy` que podemos colocar em tipos armazenados na *stack*, como os inteiros (falaremos mais sobre *traits* no [Capítulo 10][traits]<!-- ignore -->). Se um tipo implementa o *trait* `Copy`, variáveis que o usam não são movidas, mas sim copiadas trivialmente, mantendo-as válidas após a atribuição a outra variável.

O Rust não nos permite anotar um tipo com `Copy` se o tipo, ou qualquer uma de suas partes, tiver implementado o *trait* `Drop`. Se o tipo precisa que algo especial aconteça quando o valor sai de escopo e adicionarmos a anotação `Copy` a esse tipo, receberemos um erro em tempo de compilação. Para saber como adicionar a anotação `Copy` ao seu tipo para implementar o *trait*, veja ["Traits Deriváveis"][derivable-traits]<!-- ignore --> no Apêndice C.

Então, quais tipos implementam o *trait* `Copy`? Você pode consultar a documentação do tipo específico para ter certeza, mas, como regra geral, qualquer grupo de valores escalares simples pode implementar `Copy`, e nada que exija alocação ou seja alguma forma de recurso pode implementar `Copy`. Aqui estão alguns dos tipos que implementam `Copy`:

- Todos os tipos inteiros, como `u32`.
- O tipo booleano, `bool`, com valores `true` e `false`.
- Todos os tipos de ponto flutuante, como `f64`.
- O tipo caractere, `char`.
- Tuplas, se contiverem apenas tipos que também implementam `Copy`. Por exemplo, `(i32, i32)` implementa `Copy`, mas `(i32, String)` não.

<a id="ownership-and-functions"></a>
### Ownership e Funções

A mecânica de passar um valor para uma função é semelhante à de atribuir um valor a uma variável. Passar uma variável para uma função irá movê-la ou copiá-la, exatamente como a atribuição faz. A Listagem 4-3 traz um exemplo com algumas anotações mostrando onde as variáveis entram e saem de escopo.

<Listing number="4-3" file-name="src/main.rs" caption="Funções com ownership e escopo anotados">

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-03/src/main.rs}}
```

</Listing>

Se tentássemos usar `s` após a chamada a `takes_ownership`, o Rust lançaria um erro em tempo de compilação. Essas verificações estáticas nos protegem contra erros. Tente adicionar código a `main` que use `s` e `x` para ver onde você pode usá-los e onde as regras de *ownership* impedem que você faça isso.

<a id="return-values-and-scope"></a>
### Valores de Retorno e Escopo

Retornar valores também pode transferir *ownership*. A Listagem 4-4 mostra um exemplo de uma função que retorna algum valor, com anotações semelhantes às da Listagem 4-3.

<Listing number="4-4" file-name="src/main.rs" caption="Transferindo ownership de valores de retorno">

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-04/src/main.rs}}
```

</Listing>

O *ownership* de uma variável segue sempre o mesmo padrão: atribuir um valor a outra variável o move. Quando uma variável que inclui dados na *heap* sai de escopo, o valor será limpo por `drop`, a menos que o *ownership* dos dados tenha sido movido para outra variável.

Embora isso funcione, tomar *ownership* e depois devolver *ownership* em cada função é um pouco tedioso. E se quisermos deixar uma função usar um valor, mas sem tomar *ownership*? É bem chato que qualquer coisa que passemos precise ser devolvida se quisermos usá-la novamente, além de quaisquer dados resultantes do corpo da função que também possamos querer retornar.

O Rust nos permite retornar múltiplos valores usando uma tupla, como mostrado na Listagem 4-5.

<Listing number="4-5" file-name="src/main.rs" caption="Devolvendo o ownership de parâmetros">

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-05/src/main.rs}}
```

</Listing>

Mas isso é cerimônia demais e muito trabalho para um conceito que deveria ser comum. Felizmente para nós, o Rust tem um recurso para usar um valor sem transferir *ownership*: referências.

[data-types]: ch03-02-data-types.html#data-types
[ch8]: ch08-02-strings.html
[traits]: ch10-02-traits.html
[derivable-traits]: appendix-03-derivable-traits.html
[methods]: ch05-03-method-syntax.html#methods
[paths-module-tree]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
