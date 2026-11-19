<a id="packages-and-crates"></a>
## Packages e Crates

As primeiras partes do sistema de módulos que abordaremos são *packages* e
*crates*.

Um _crate_ é a menor quantidade de código que o compilador Rust considera de
cada vez. Mesmo se você executar `rustc` em vez de `cargo` e passar um único
arquivo de código-fonte (como fizemos lá atrás em [“Fundamentos de um Programa
Rust”][basics]<!-- ignore --> no Capítulo 1), o compilador considera esse
arquivo um *crate*. *Crates* podem conter módulos, e os módulos podem ser
definidos em outros arquivos que são compilados junto com o *crate*, como
veremos nas próximas seções.

Um *crate* pode assumir uma de duas formas: um *crate* binário ou um *crate* de
biblioteca. _Crates binários_ são programas que você pode compilar para gerar um
executável que pode ser rodado, como um programa de linha de comando ou um
servidor. Cada um precisa ter uma função chamada `main` que define o que
acontece quando o executável é executado. Todos os *crates* que criamos até
agora foram *crates* binários.

_Crates de biblioteca_ não têm uma função `main` e não compilam para um
executável. Em vez disso, eles definem funcionalidades pensadas para serem
compartilhadas entre vários projetos. Por exemplo, o *crate* `rand` que usamos
no [Capítulo 2][rand]<!-- ignore --> fornece funcionalidades para gerar números
aleatórios. Na maioria das vezes, quando Rustaceans dizem “*crate*”, eles estão
se referindo a um *crate* de biblioteca e usam “*crate*” de forma intercambiável
com o conceito geral de programação de uma “biblioteca”.

A _raiz do crate_ (*crate root*) é um arquivo-fonte a partir do qual o
compilador Rust começa e que compõe o módulo raiz do seu *crate* (explicaremos
os módulos em profundidade em [“Controlando Escopo e Privacidade com
Módulos”][modules]<!-- ignore -->).

Um _package_ é um pacote com um ou mais *crates* que fornece um conjunto de
funcionalidades. Um *package* contém um arquivo _Cargo.toml_ que descreve como
compilar esses *crates*. O Cargo é, na verdade, um *package* que contém o
*crate* binário da ferramenta de linha de comando que você tem usado para
compilar seu código. O *package* do Cargo também contém um *crate* de biblioteca
do qual o *crate* binário depende. Outros projetos podem depender do *crate* de
biblioteca do Cargo para usar a mesma lógica que a ferramenta de linha de
comando do Cargo usa.

Um *package* pode conter quantos *crates* binários você quiser, mas no máximo um
único *crate* de biblioteca. Um *package* precisa conter pelo menos um *crate*,
seja ele de biblioteca ou binário.

Vamos acompanhar o que acontece quando criamos um *package*. Primeiro, digitamos
o comando `cargo new my-project`:

```console
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

Depois de executar `cargo new my-project`, usamos `ls` para ver o que o Cargo
criou. No diretório _my-project_, há um arquivo _Cargo.toml_, nos dando um
*package*. Também há um diretório _src_ que contém _main.rs_. Abra _Cargo.toml_
no seu editor de texto e note que não há menção a _src/main.rs_. O Cargo segue a
convenção de que _src/main.rs_ é a raiz de um *crate* binário com o mesmo nome
do *package*. Da mesma forma, o Cargo sabe que, se o diretório do *package*
contiver _src/lib.rs_, o *package* contém um *crate* de biblioteca com o mesmo
nome do *package*, e _src/lib.rs_ é a raiz desse *crate*. O Cargo passa os
arquivos raiz do *crate* para o `rustc` compilar a biblioteca ou o binário.

Aqui, temos um *package* que contém apenas _src/main.rs_, o que significa que
ele contém apenas um *crate* binário chamado `my-project`. Se um *package*
contiver _src/main.rs_ e _src/lib.rs_, ele tem dois *crates*: um binário e uma
biblioteca, ambos com o mesmo nome do *package*. Um *package* pode ter vários
*crates* binários colocando arquivos no diretório _src/bin_: cada arquivo será
um *crate* binário separado.

[basics]: ch01-02-hello-world.html#rust-program-basics
[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
[rand]: ch02-00-guessing-game-tutorial.html#generating-a-random-number
