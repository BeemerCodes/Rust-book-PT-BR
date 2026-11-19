<!-- Old headings. Do not remove or links may break. -->

<a id="defining-modules-to-control-scope-and-privacy"></a>

<a id="control-scope-and-privacy-with-modules"></a>
## Controlando Escopo e Privacidade com Módulos

Nesta seção, falaremos sobre módulos e outras partes do sistema de módulos,
especificamente _caminhos_ (*paths*), que permitem nomear itens; a palavra-chave
`use`, que traz um caminho para o escopo; e a palavra-chave `pub`, para tornar
itens públicos. Também discutiremos a palavra-chave `as`, *packages* externos e
o operador glob.

<a id="modules-cheat-sheet"></a>
### Guia Rápido de Módulos

Antes de entrarmos nos detalhes de módulos e caminhos, apresentamos aqui uma
referência rápida sobre como módulos, caminhos, a palavra-chave `use` e a
palavra-chave `pub` funcionam no compilador, e como a maioria dos
desenvolvedores organiza seu código. Veremos exemplos de cada uma dessas regras
ao longo deste capítulo, mas este é um ótimo lugar para consultar como lembrete
de como os módulos funcionam.

- **Comece pela raiz do *crate***: Ao compilar um *crate*, o compilador procura
  primeiro no arquivo raiz do *crate* (geralmente _src/lib.rs_ para um *crate*
  de biblioteca e _src/main.rs_ para um *crate* binário) o código a ser
  compilado.
- **Declarando módulos**: No arquivo raiz do *crate*, você pode declarar novos
  módulos; digamos que você declare um módulo “garden” com `mod garden;`. O
  compilador procurará o código do módulo nestes locais:
  - Inline, dentro de chaves que substituem o ponto e vírgula após `mod garden`
  - No arquivo _src/garden.rs_
  - No arquivo _src/garden/mod.rs_
- **Declarando submódulos**: Em qualquer arquivo que não seja a raiz do *crate*,
  você pode declarar submódulos. Por exemplo, você pode declarar `mod
  vegetables;` em _src/garden.rs_. O compilador procurará o código do submódulo
  dentro do diretório com o nome do módulo pai nestes locais:
  - Inline, logo após `mod vegetables`, dentro de chaves em vez do ponto e
    vírgula
  - No arquivo _src/garden/vegetables.rs_
  - No arquivo _src/garden/vegetables/mod.rs_
- **Caminhos para código em módulos**: Uma vez que um módulo faz parte do seu
  *crate*, você pode se referir ao código nesse módulo de qualquer outro lugar
  do mesmo *crate*, desde que as regras de privacidade permitam, usando o
  caminho para o código. Por exemplo, um tipo `Asparagus` no módulo de vegetais
  do jardim seria encontrado em `crate::garden::vegetables::Asparagus`.
- **Privado vs. público**: O código dentro de um módulo é privado em relação aos
  seus módulos pai por padrão. Para tornar um módulo público, declare-o com `pub
  mod` em vez de `mod`. Para tornar os itens dentro de um módulo público também
  públicos, use `pub` antes de suas declarações.
- **A palavra-chave `use`**: Dentro de um escopo, a palavra-chave `use` cria
  atalhos para itens a fim de reduzir a repetição de caminhos longos. Em
  qualquer escopo que possa se referir a `crate::garden::vegetables::Asparagus`,
  você pode criar um atalho com `use crate::garden::vegetables::Asparagus;` e, a
  partir daí, só precisará escrever `Asparagus` para usar esse tipo no escopo.

Aqui, criamos um *crate* binário chamado `backyard` que ilustra essas regras. O
diretório do *crate*, também chamado _backyard_, contém estes arquivos e
diretórios:

```text
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

O arquivo raiz do *crate* neste caso é _src/main.rs_, e ele contém:

<Listing file-name="src/main.rs">

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/main.rs}}
```

</Listing>

A linha `pub mod garden;` diz ao compilador para incluir o código encontrado em
_src/garden.rs_, que é:

<Listing file-name="src/garden.rs">

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/garden.rs}}
```

</Listing>

Aqui, `pub mod vegetables;` significa que o código em _src/garden/vegetables.rs_
também é incluído. Esse código é:

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/garden/vegetables.rs}}
```

Agora vamos entrar nos detalhes dessas regras e demonstrá-las em ação!

<a id="grouping-related-code-in-modules"></a>
### Agrupando Código Relacionado em Módulos

_Módulos_ permitem organizar o código dentro de um *crate* para facilitar a
leitura e a reutilização. Os módulos também permitem controlar a _privacidade_
dos itens, porque o código dentro de um módulo é privado por padrão. Itens
privados são detalhes internos de implementação que não ficam disponíveis para
uso externo. Podemos optar por tornar módulos e os itens dentro deles públicos,
expondo-os para permitir que código externo os utilize e dependa deles.

Como exemplo, vamos escrever um *crate* de biblioteca que fornece a
funcionalidade de um restaurante. Definiremos as assinaturas das funções, mas
deixaremos seus corpos vazios para focar na organização do código em vez da
implementação de um restaurante.

No setor de restaurantes, algumas partes são chamadas de salão (*front of
house*) e outras de cozinha/bastidores (*back of house*). O _front of house_ é
onde os clientes ficam; isso engloba onde os recepcionistas acomodam os
clientes, os garçons anotam pedidos e recebem pagamentos, e os bartenders
preparam bebidas. O _back of house_ é onde os chefs e cozinheiros trabalham na
cozinha, a equipe limpa a louça e os gerentes fazem o trabalho administrativo.

Para estruturar nosso *crate* dessa forma, podemos organizar suas funções em
módulos aninhados. Crie uma nova biblioteca chamada `restaurant` executando
`cargo new restaurant --lib`. Em seguida, insira o código da Listagem 7-1 em
_src/lib.rs_ para definir alguns módulos e assinaturas de funções; esse código é
a seção *front of house*.

<Listing number="7-1" file-name="src/lib.rs" caption="Um módulo `front_of_house` contendo outros módulos que, por sua vez, contêm funções">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-01/src/lib.rs}}
```

</Listing>

Definimos um módulo com a palavra-chave `mod` seguida do nome do módulo (neste
caso, `front_of_house`). O corpo do módulo vai dentro de chaves. Dentro de
módulos, podemos colocar outros módulos, como neste caso com os módulos
`hosting` e `serving`. Módulos também podem conter definições de outros itens,
como *structs*, *enums*, constantes, *traits* e, como na Listagem 7-1, funções.

Ao usar módulos, podemos agrupar definições relacionadas e nomear o motivo pelo
qual elas estão relacionadas. Programadores que usam esse código podem navegar
por ele com base nos grupos, em vez de precisar ler todas as definições,
tornando mais fácil encontrar o que é relevante para eles. Programadores que
adicionam novas funcionalidades a esse código saberão onde colocá-las para
manter o programa organizado.

Anteriormente, mencionamos que _src/main.rs_ e _src/lib.rs_ são chamados de
_raízes do crate_. O motivo desse nome é que o conteúdo de qualquer um desses
dois arquivos forma um módulo chamado `crate` na raiz da estrutura de módulos do
*crate*, conhecida como _árvore de módulos_.

A Listagem 7-2 mostra a árvore de módulos para a estrutura na Listagem 7-1.

<Listing number="7-2" caption="A árvore de módulos para o código da Listagem 7-1">

```text
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

</Listing>

Essa árvore mostra como alguns módulos ficam aninhados dentro de outros módulos;
por exemplo, `hosting` fica aninhado dentro de `front_of_house`. A árvore também
mostra que alguns módulos são _irmãos_ (*siblings*), o que significa que são
definidos no mesmo módulo; `hosting` e `serving` são irmãos definidos dentro de
`front_of_house`. Se o módulo A está contido dentro do módulo B, dizemos que o
módulo A é _filho_ do módulo B e que o módulo B é _pai_ do módulo A. Note que
toda a árvore de módulos tem como raiz o módulo implícito chamado `crate`.

A árvore de módulos pode lembrar a árvore de diretórios do sistema de arquivos
do seu computador; essa é uma comparação muito boa! Assim como os diretórios em
um sistema de arquivos, você usa módulos para organizar seu código. E assim como
arquivos em um diretório, precisamos de uma maneira de encontrar nossos
módulos.
