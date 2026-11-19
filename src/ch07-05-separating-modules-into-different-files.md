<a id="separating-modules-into-different-files"></a>
## Separando Módulos em Arquivos Diferentes

Até agora, todos os exemplos neste capítulo definiram vários módulos em um único
arquivo. Quando os módulos ficam grandes, você pode querer mover suas definições
para um arquivo separado para facilitar a navegação pelo código.

Por exemplo, vamos começar a partir do código da Listagem 7-17, que tinha vários
módulos do restaurante. Vamos extrair módulos para arquivos em vez de ter todos
os módulos definidos no arquivo raiz do *crate*. Neste caso, o arquivo raiz do
*crate* é _src/lib.rs_, mas esse procedimento também funciona com *crates*
binários cujo arquivo raiz do *crate* é _src/main.rs_.

Primeiro, vamos extrair o módulo `front_of_house` para seu próprio arquivo.
Remova o código dentro das chaves do módulo `front_of_house`, deixando apenas a
declaração `mod front_of_house;`, de modo que _src/lib.rs_ contenha o código
mostrado na Listagem 7-21. Note que isso não vai compilar até criarmos o arquivo
_src/front_of_house.rs_ na Listagem 7-22.

<Listing number="7-21" file-name="src/lib.rs" caption="Declarando o módulo `front_of_house` cujo corpo ficará em *src/front_of_house.rs*">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/lib.rs}}
```

</Listing>

Em seguida, coloque o código que estava dentro das chaves em um novo arquivo
chamado _src/front_of_house.rs_, conforme mostrado na Listagem 7-22. O
compilador sabe que deve procurar nesse arquivo porque encontrou a declaração do
módulo na raiz do *crate* com o nome `front_of_house`.

<Listing number="7-22" file-name="src/front_of_house.rs" caption="Definições dentro do módulo `front_of_house` em *src/front_of_house.rs*">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/front_of_house.rs}}
```

</Listing>

Note que você só precisa carregar um arquivo usando uma declaração `mod` _uma
única vez_ na sua árvore de módulos. Assim que o compilador sabe que o arquivo
faz parte do projeto (e sabe onde na árvore de módulos o código reside por causa
de onde você colocou a declaração `mod`), outros arquivos do seu projeto devem
se referir ao código do arquivo carregado usando um caminho para onde ele foi
declarado, conforme abordado na seção [“Caminhos para Referenciar um Item na
Árvore de Módulos”][paths]<!-- ignore -->. Em outras palavras, `mod` _não_ é uma
operação de “include” que você pode ter visto em outras linguagens de
programação.

Em seguida, vamos extrair o módulo `hosting` para seu próprio arquivo. O
processo é um pouco diferente porque `hosting` é um módulo filho de
`front_of_house`, não do módulo raiz. Colocaremos o arquivo de `hosting` em um
novo diretório que terá o nome dos seus ancestrais na árvore de módulos, neste
caso _src/front_of_house_.

Para começar a mover `hosting`, alteramos _src/front_of_house.rs_ para conter
apenas a declaração do módulo `hosting`:

<Listing file-name="src/front_of_house.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house.rs}}
```

</Listing>

Depois, criamos um diretório _src/front_of_house_ e um arquivo _hosting.rs_ para
conter as definições feitas no módulo `hosting`:

<Listing file-name="src/front_of_house/hosting.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house/hosting.rs}}
```

</Listing>

Se em vez disso colocássemos _hosting.rs_ no diretório _src_, o compilador
esperaria que o código de _hosting.rs_ estivesse em um módulo `hosting`
declarado na raiz do *crate* e não declarado como filho do módulo
`front_of_house`. As regras do compilador sobre quais arquivos verificar para o
código de cada módulo fazem com que os diretórios e arquivos correspondam mais
de perto à árvore de módulos.

> <a id="alternate-file-paths"></a>
> ### Caminhos de Arquivo Alternativos
>
> Até agora abordamos os caminhos de arquivo mais idiomáticos que o compilador
> Rust usa, mas o Rust também oferece suporte a um estilo mais antigo de caminho
> de arquivo. Para um módulo chamado `front_of_house` declarado na raiz do
> *crate*, o compilador procurará o código do módulo em:
>
> - _src/front_of_house.rs_ (o que abordamos)
> - _src/front_of_house/mod.rs_ (estilo mais antigo, caminho ainda suportado)
>
> Para um módulo chamado `hosting` que é submódulo de `front_of_house`, o
> compilador procurará o código do módulo em:
>
> - _src/front_of_house/hosting.rs_ (o que abordamos)
> - _src/front_of_house/hosting/mod.rs_ (estilo mais antigo, caminho ainda
>   suportado)
>
> Se você usar ambos os estilos para o mesmo módulo, receberá um erro de
> compilação. Usar uma mistura de ambos os estilos para módulos diferentes no
> mesmo projeto é permitido, mas pode ser confuso para quem navega pelo seu
> projeto.
>
> A principal desvantagem do estilo que usa arquivos chamados _mod.rs_ é que seu
> projeto pode acabar com muitos arquivos chamados _mod.rs_, o que pode ficar
> confuso quando você os tem abertos no seu editor ao mesmo tempo.

Movemos o código de cada módulo para um arquivo separado, e a árvore de módulos
continua a mesma. As chamadas de função em `eat_at_restaurant` funcionarão sem
nenhuma modificação, mesmo com as definições vivendo em arquivos diferentes.
Essa técnica permite mover módulos para novos arquivos à medida que eles crescem
em tamanho.

Note que a declaração `pub use crate::front_of_house::hosting` em _src/lib.rs_
também não mudou, nem o `use` tem qualquer impacto sobre quais arquivos são
compilados como parte do *crate*. A palavra-chave `mod` declara módulos, e o
Rust procura em um arquivo com o mesmo nome do módulo o código que vai dentro
desse módulo.

<a id="summary"></a>
## Resumo

O Rust permite dividir um *package* em vários *crates* e um *crate* em módulos
para que você possa se referir a itens definidos em um módulo a partir de outro
módulo. Você pode fazer isso especificando caminhos absolutos ou relativos.
Esses caminhos podem ser trazidos para o escopo com uma declaração `use` para
que você possa usar um caminho mais curto para múltiplos usos do item naquele
escopo. O código de um módulo é privado por padrão, mas você pode tornar
definições públicas adicionando a palavra-chave `pub`.

No próximo capítulo, veremos algumas estruturas de dados de coleções na
biblioteca padrão que você pode usar no seu código bem organizado.

[paths]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
