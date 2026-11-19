<a id="paths-for-referring-to-an-item-in-the-module-tree"></a>
## Caminhos para Referenciar um Item na Árvore de Módulos

Para mostrar ao Rust onde encontrar um item em uma árvore de módulos, usamos um
caminho da mesma forma que usamos um caminho ao navegar em um sistema de
arquivos. Para chamar uma função, precisamos saber o seu caminho.

Um caminho pode assumir duas formas:

- Um _caminho absoluto_ é o caminho completo começando a partir da raiz de um
  *crate*; para código de um *crate* externo, o caminho absoluto começa com o
  nome do *crate*, e para código do *crate* atual, começa com o literal `crate`.
- Um _caminho relativo_ começa a partir do módulo atual e usa `self`, `super` ou
  um identificador no módulo atual.

Tanto caminhos absolutos quanto relativos são seguidos por um ou mais
identificadores separados por dois pontos duplos (`::`).

Voltando à Listagem 7-1, digamos que queremos chamar a função `add_to_waitlist`.
Isso é o mesmo que perguntar: qual é o caminho da função `add_to_waitlist`? A
Listagem 7-3 contém a Listagem 7-1 com alguns dos módulos e funções removidos.

Mostraremos duas maneiras de chamar a função `add_to_waitlist` a partir de uma
nova função, `eat_at_restaurant`, definida na raiz do *crate*. Esses caminhos
estão corretos, mas ainda há outro problema que impedirá este exemplo de
compilar como está. Explicaremos o porquê em breve.

A função `eat_at_restaurant` faz parte da API pública do nosso *crate* de
biblioteca, então a marcamos com a palavra-chave `pub`. Na seção [“Expondo
Caminhos com a Palavra-chave `pub`”][pub]<!-- ignore -->, entraremos em mais
detalhes sobre `pub`.

<Listing number="7-3" file-name="src/lib.rs" caption="Chamando a função `add_to_waitlist` usando caminhos absoluto e relativo">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-03/src/lib.rs}}
```

</Listing>

Na primeira vez em que chamamos a função `add_to_waitlist` em
`eat_at_restaurant`, usamos um caminho absoluto. A função `add_to_waitlist` está
definida no mesmo *crate* que `eat_at_restaurant`, o que significa que podemos
usar a palavra-chave `crate` para iniciar um caminho absoluto. Em seguida,
incluímos cada um dos módulos sucessivos até chegarmos a `add_to_waitlist`. Você
pode imaginar um sistema de arquivos com a mesma estrutura: especificaríamos o
caminho `/front_of_house/hosting/add_to_waitlist` para executar o programa
`add_to_waitlist`; usar o nome `crate` para começar a partir da raiz do *crate*
é como usar `/` para começar a partir da raiz do sistema de arquivos no seu
shell.

Na segunda vez em que chamamos `add_to_waitlist` em `eat_at_restaurant`, usamos
um caminho relativo. O caminho começa com `front_of_house`, o nome do módulo
definido no mesmo nível da árvore de módulos que `eat_at_restaurant`. Aqui, o
equivalente no sistema de arquivos seria usar o caminho
`front_of_house/hosting/add_to_waitlist`. Começar com o nome de um módulo
significa que o caminho é relativo.

Escolher entre usar um caminho relativo ou absoluto é uma decisão que você
tomará com base no seu projeto e depende de ser mais provável mover o código de
definição do item separadamente ou junto com o código que usa o item. Por
exemplo, se movêssemos o módulo `front_of_house` e a função `eat_at_restaurant`
para um módulo chamado `customer_experience`, precisaríamos atualizar o caminho
absoluto para `add_to_waitlist`, mas o caminho relativo continuaria válido. No
entanto, se movêssemos a função `eat_at_restaurant` separadamente para um módulo
chamado `dining`, o caminho absoluto na chamada de `add_to_waitlist`
permaneceria o mesmo, mas o caminho relativo precisaria ser atualizado. Nossa
preferência geral é especificar caminhos absolutos, porque é mais provável que
queiramos mover definições de código e chamadas de itens de forma independente
umas das outras.

Vamos tentar compilar a Listagem 7-3 e descobrir por que ela ainda não compila!
Os erros que obtemos são mostrados na Listagem 7-4.

<Listing number="7-4" caption="Erros de compilação ao compilar o código da Listagem 7-3">

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-03/output.txt}}
```

</Listing>

As mensagens de erro dizem que o módulo `hosting` é privado. Em outras palavras,
temos os caminhos corretos para o módulo `hosting` e para a função
`add_to_waitlist`, mas o Rust não nos deixa usá-los porque não temos acesso às
seções privadas. No Rust, todos os itens (funções, métodos, *structs*, *enums*,
módulos e constantes) são privados em relação aos módulos pai por padrão. Se
você quiser tornar um item como uma função ou *struct* privado, basta colocá-lo
em um módulo.

Itens em um módulo pai não podem usar os itens privados dentro de módulos
filhos, mas itens em módulos filhos podem usar os itens em seus módulos
ancestrais. Isso acontece porque os módulos filhos encapsulam e escondem seus
detalhes de implementação, mas conseguem ver o contexto no qual estão definidos.
Continuando com nossa metáfora, pense nas regras de privacidade como o
escritório interno de um restaurante: o que acontece lá dentro é privado para
os clientes, mas os gerentes do escritório podem ver e fazer tudo no restaurante
que administram.

O Rust escolheu fazer o sistema de módulos funcionar dessa maneira para que
esconder detalhes internos de implementação seja o padrão. Assim, você sabe
quais partes do código interno pode alterar sem quebrar o código externo. No
entanto, o Rust oferece a opção de expor partes internas do código de módulos
filhos para módulos ancestrais externos usando a palavra-chave `pub` para tornar
um item público.

<a id="exposing-paths-with-the-pub-keyword"></a>
### Expondo Caminhos com a Palavra-chave `pub`

Vamos voltar ao erro da Listagem 7-4 que nos disse que o módulo `hosting` é
privado. Queremos que a função `eat_at_restaurant` no módulo pai tenha acesso à
função `add_to_waitlist` no módulo filho, então marcamos o módulo `hosting` com
a palavra-chave `pub`, conforme mostrado na Listagem 7-5.

<Listing number="7-5" file-name="src/lib.rs" caption="Declarando o módulo `hosting` como `pub` para usá-lo a partir de `eat_at_restaurant`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-05/src/lib.rs:here}}
```

</Listing>

Infelizmente, o código da Listagem 7-5 ainda resulta em erros de compilação,
conforme mostrado na Listagem 7-6.

<Listing number="7-6" caption="Erros de compilação ao compilar o código da Listagem 7-5">

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-05/output.txt}}
```

</Listing>

O que aconteceu? Adicionar a palavra-chave `pub` na frente de `mod hosting`
torna o módulo público. Com essa alteração, se pudermos acessar
`front_of_house`, poderemos acessar `hosting`. Mas o _conteúdo_ de `hosting`
continua privado; tornar o módulo público não torna seu conteúdo público. A
palavra-chave `pub` em um módulo permite apenas que o código em seus módulos
ancestrais se refira a ele, não que acesse seu código interno. Como módulos são
contêineres, não há muito que possamos fazer apenas tornando o módulo público;
precisamos ir além e escolher tornar público um ou mais dos itens dentro do
módulo também.

Os erros na Listagem 7-6 dizem que a função `add_to_waitlist` é privada. As
regras de privacidade aplicam-se a *structs*, *enums*, funções e métodos, bem
como a módulos.

Vamos também tornar a função `add_to_waitlist` pública adicionando a
palavra-chave `pub` antes de sua definição, como na Listagem 7-7.

<Listing number="7-7" file-name="src/lib.rs" caption="Adicionar a palavra-chave `pub` a `mod hosting` e `fn add_to_waitlist` nos permite chamar a função a partir de `eat_at_restaurant`.">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-07/src/lib.rs:here}}
```

</Listing>

Agora o código vai compilar! Para entender por que adicionar a palavra-chave
`pub` nos permite usar esses caminhos em `eat_at_restaurant` respeitando as
regras de privacidade, vamos analisar os caminhos absoluto e relativo.

No caminho absoluto, começamos com `crate`, a raiz da árvore de módulos do nosso
*crate*. O módulo `front_of_house` é definido na raiz do *crate*. Embora
`front_of_house` não seja público, como a função `eat_at_restaurant` está
definida no mesmo módulo que `front_of_house` (ou seja, `eat_at_restaurant` e
`front_of_house` são irmãos), podemos nos referir a `front_of_house` a partir de
`eat_at_restaurant`. Em seguida vem o módulo `hosting` marcado com `pub`. Podemos
acessar o módulo pai de `hosting`, então podemos acessar `hosting`. Por fim, a
função `add_to_waitlist` está marcada com `pub` e podemos acessar seu módulo
pai, então essa chamada de função funciona!

No caminho relativo, a lógica é a mesma do caminho absoluto, exceto pelo
primeiro passo: em vez de começar da raiz do *crate*, o caminho começa em
`front_of_house`. O módulo `front_of_house` está definido no mesmo módulo que
`eat_at_restaurant`, então o caminho relativo que começa no módulo em que
`eat_at_restaurant` está definido funciona. Depois, como `hosting` e
`add_to_waitlist` estão marcados com `pub`, o restante do caminho funciona e
essa chamada de função é válida!

Se você planeja compartilhar seu *crate* de biblioteca para que outros projetos
possam usar seu código, sua API pública é o seu contrato com os usuários do seu
*crate*, determinando como eles podem interagir com seu código. Existem muitas
considerações sobre como gerenciar mudanças na sua API pública para facilitar
que outras pessoas dependam do seu *crate*. Essas considerações estão além do
escopo deste livro; se você tiver interesse nesse assunto, consulte [as
Diretrizes de API do Rust][api-guidelines].

> <a id="best-practices-for-packages-with-a-binary-and-a-library"></a>
> #### Boas Práticas para Packages com um Binário e uma Biblioteca
>
> Mencionamos que um *package* pode conter tanto uma raiz de *crate* binário
> _src/main.rs_ quanto uma raiz de *crate* de biblioteca _src/lib.rs_, e ambos
> os *crates* terão o nome do *package* por padrão. Normalmente, *packages* que
> seguem esse padrão de conter tanto uma biblioteca quanto um *crate* binário
> terão apenas código suficiente no *crate* binário para iniciar um executável
> que chama o código definido no *crate* de biblioteca. Isso permite que outros
> projetos aproveitem a maior parte das funcionalidades que o *package* oferece,
> pois o código do *crate* de biblioteca pode ser compartilhado.
>
> A árvore de módulos deve ser definida em _src/lib.rs_. Então, quaisquer itens
> públicos podem ser usados no *crate* binário iniciando os caminhos com o nome
> do *package*. O *crate* binário se torna um usuário do *crate* de biblioteca
> exatamente como um *crate* completamente externo o usaria: ele só pode usar a
> API pública. Isso ajuda você a projetar uma boa API; você não é apenas o
> autor, mas também um cliente!
>
> No [Capítulo 12][ch12]<!-- ignore -->, demonstraremos essa prática de
> organização com um programa de linha de comando que conterá tanto um *crate*
> binário quanto um *crate* de biblioteca.

<a id="starting-relative-paths-with-super"></a>
### Iniciando Caminhos Relativos com `super`

Podemos construir caminhos relativos que começam no módulo pai, em vez do módulo
atual ou da raiz do *crate*, usando `super` no início do caminho. Isso é como
iniciar um caminho do sistema de arquivos com a sintaxe `..`, que significa ir
para o diretório pai. Usar `super` nos permite referenciar um item que sabemos
estar no módulo pai, o que pode facilitar a reorganização da árvore de módulos
quando o módulo está intimamente relacionado ao pai, mas o pai pode ser movido
para outro lugar na árvore de módulos algum dia.

Considere o código na Listagem 7-8, que modela a situação em que um chef corrige
um pedido incorreto e o leva pessoalmente ao cliente. A função
`fix_incorrect_order` definida no módulo `back_of_house` chama a função
`deliver_order` definida no módulo pai especificando o caminho para
`deliver_order` começando com `super`.

<Listing number="7-8" file-name="src/lib.rs" caption="Chamando uma função usando um caminho relativo que começa com `super`">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-08/src/lib.rs}}
```

</Listing>

A função `fix_incorrect_order` está no módulo `back_of_house`, então podemos
usar `super` para ir ao módulo pai de `back_of_house`, que neste caso é `crate`,
a raiz. A partir daí, procuramos por `deliver_order` e a encontramos. Sucesso!
Acreditamos que o módulo `back_of_house` e a função `deliver_order`
provavelmente manterão a mesma relação entre si e serão movidos juntos caso
decidamos reorganizar a árvore de módulos do *crate*. Portanto, usamos `super`
para termos menos lugares para atualizar o código no futuro se ele for movido
para um módulo diferente.

<a id="making-structs-and-enums-public"></a>
### Tornando Structs e Enums Públicos

Também podemos usar `pub` para designar *structs* e *enums* como públicos, mas
há alguns detalhes extras no uso de `pub` com *structs* e *enums*. Se usarmos
`pub` antes da definição de uma *struct*, tornamos a *struct* pública, mas os
campos dela continuarão privados. Podemos tornar cada campo público ou não,
caso a caso. Na Listagem 7-9, definimos uma *struct* pública
`back_of_house::Breakfast` com um campo `toast` público, mas um campo
`seasonal_fruit` privado. Isso modela o caso em um restaurante em que o cliente
pode escolher o tipo de pão que acompanha a refeição, mas o chef decide qual
fruta acompanha o prato com base no que está na estação e em estoque. A fruta
disponível muda rapidamente, então os clientes não podem escolhê-la nem mesmo
ver qual fruta receberão.

<Listing number="7-9" file-name="src/lib.rs" caption="Uma struct com alguns campos públicos e alguns campos privados">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-09/src/lib.rs}}
```

</Listing>

Como o campo `toast` na *struct* `back_of_house::Breakfast` é público, em
`eat_at_restaurant` podemos ler e escrever no campo `toast` usando a notação de
ponto. Note que não podemos usar o campo `seasonal_fruit` em
`eat_at_restaurant`, porque `seasonal_fruit` é privado. Tente descomentar a
linha que modifica o valor do campo `seasonal_fruit` para ver qual erro você
obtém!

Além disso, observe que, como `back_of_house::Breakfast` tem um campo privado, a
*struct* precisa fornecer uma função associada pública que construa uma
instância de `Breakfast` (aqui a chamamos de `summer`). Se `Breakfast` não
tivesse essa função, não conseguiríamos criar uma instância de `Breakfast` em
`eat_at_restaurant`, porque não poderíamos definir o valor do campo privado
`seasonal_fruit` em `eat_at_restaurant`.

Por outro lado, se tornarmos um *enum* público, todas as suas variantes se
tornam públicas. Precisamos apenas do `pub` antes da palavra-chave `enum`, como
mostrado na Listagem 7-10.

<Listing number="7-10" file-name="src/lib.rs" caption="Designar um enum como público torna todas as suas variantes públicas.">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-10/src/lib.rs}}
```

</Listing>

Como tornamos o *enum* `Appetizer` público, podemos usar as variantes `Soup` e
`Salad` em `eat_at_restaurant`.

*Enums* não são muito úteis a menos que suas variantes sejam públicas; seria
incômodo ter que anotar todas as variantes de um *enum* com `pub` em todos os
casos, então o padrão para variantes de *enum* é serem públicas. *Structs*
frequentemente são úteis sem que seus campos sejam públicos, então os campos de
*structs* seguem a regra geral de tudo ser privado por padrão, a menos que seja
anotado com `pub`.

Há mais uma situação envolvendo `pub` que ainda não abordamos, e ela envolve
nosso último recurso do sistema de módulos: a palavra-chave `use`. Abordaremos
`use` sozinha primeiro e, depois, mostraremos como combinar `pub` e `use`.

[pub]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#exposing-paths-with-the-pub-keyword
[api-guidelines]: https://rust-lang.github.io/api-guidelines/
[ch12]: ch12-00-an-io-project.html
