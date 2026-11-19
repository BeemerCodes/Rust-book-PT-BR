<a id="programming-a-guessing-game"></a>
# Programando um Jogo de Adivinhação

Vamos mergulhar no Rust trabalhando juntos em um projeto prático! Este capítulo apresenta alguns conceitos comuns do Rust mostrando como usá-los em um programa real. Você aprenderá sobre `let`, `match`, métodos, funções associadas, *crates* externos e muito mais! Nos capítulos seguintes, exploraremos essas ideias com mais detalhes. Neste capítulo, você vai praticar os fundamentos.

Vamos implementar um problema clássico de programação para iniciantes: um jogo de adivinhação. Veja como funciona: o programa vai gerar um número inteiro aleatório entre 1 e 100. Em seguida, pedirá ao jogador que insira um palpite. Após inserir o palpite, o programa indicará se o palpite está baixo demais ou alto demais. Se o palpite estiver correto, o jogo imprimirá uma mensagem de parabéns e encerrará.

<a id="setting-up-a-new-project"></a>
## Criando um Novo Projeto

Para criar um novo projeto, vá ao diretório _projects_ que você criou no Capítulo 1 e crie um novo projeto usando o Cargo, assim:

```console
$ cargo new guessing_game
$ cd guessing_game
```

O primeiro comando, `cargo new`, recebe o nome do projeto (`guessing_game`) como primeiro argumento. O segundo comando muda para o diretório do novo projeto.

Veja o arquivo _Cargo.toml_ gerado:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial
rm -rf no-listing-01-cargo-new
cargo new no-listing-01-cargo-new --name guessing_game
cd no-listing-01-cargo-new
cargo run > output.txt 2>&1
cd ../../..
-->

<span class="filename">Arquivo: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/Cargo.toml}}
```

Como você viu no Capítulo 1, `cargo new` gera um programa "Hello, world!" para você. Confira o arquivo _src/main.rs_:

<span class="filename">Arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/src/main.rs}}
```

Agora vamos compilar este programa "Hello, world!" e executá-lo em um único passo usando o comando `cargo run`:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/output.txt}}
```

O comando `run` é útil quando você precisa iterar rapidamente em um projeto, como faremos neste jogo, testando cada iteração rapidamente antes de avançar para a próxima.

Reabra o arquivo _src/main.rs_. Você vai escrever todo o código neste arquivo.

<a id="processing-a-guess"></a>
## Processando um Palpite

A primeira parte do programa do jogo de adivinhação pedirá entrada do usuário, processará essa entrada e verificará se a entrada está no formato esperado. Para começar, vamos permitir que o jogador insira um palpite. Insira o código da Listagem 2-1 no _src/main.rs_.

<Listing number="2-1" file-name="src/main.rs" caption="Código que obtém um palpite do usuário e o imprime">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:all}}
```

</Listing>

Este código contém muita informação, então vamos percorrê-lo linha por linha. Para obter a entrada do usuário e depois imprimir o resultado como saída, precisamos trazer a biblioteca de entrada/saída `io` para o escopo. A biblioteca `io` vem da biblioteca padrão, conhecida como `std`:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:io}}
```

Por padrão, o Rust tem um conjunto de itens definidos na biblioteca padrão que traz para o escopo de cada programa. Esse conjunto é chamado de _prelúdio_ (*prelude*), e você pode ver tudo nele [na documentação da biblioteca padrão][prelude].

Se um tipo que você quer usar não está no prelúdio, você precisa trazê-lo explicitamente para o escopo com uma declaração `use`. Usar a biblioteca `std::io` oferece vários recursos úteis, incluindo a capacidade de aceitar entrada do usuário.

Como você viu no Capítulo 1, a função `main` é o ponto de entrada do programa:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:main}}
```

A sintaxe `fn` declara uma nova função; os parênteses `()` indicam que não há parâmetros; e a chave `{` inicia o corpo da função.

Como você também aprendeu no Capítulo 1, `println!` é uma *macro* que imprime uma string na tela:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print}}
```

Este código imprime um aviso dizendo qual é o jogo e solicitando entrada do usuário.

<a id="storing-values-with-variables"></a>
### Armazenando Valores com Variáveis

Em seguida, vamos criar uma _variável_ para armazenar a entrada do usuário, assim:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:string}}
```

Agora o programa está ficando interessante! Há muita coisa acontecendo nessa pequena linha. Usamos a instrução `let` para criar a variável. Aqui está outro exemplo:

```rust,ignore
let apples = 5;
```

Esta linha cria uma nova variável chamada `apples` e a vincula ao valor `5`. No Rust, variáveis são imutáveis por padrão, o que significa que, uma vez que damos à variável um valor, esse valor não vai mudar. Vamos discutir esse conceito em detalhes na seção ["Variáveis e Mutabilidade"][variables-and-mutability]<!-- ignore --> no Capítulo 3. Para tornar uma variável mutável, adicionamos `mut` antes do nome da variável:

```rust,ignore
let apples = 5; // imutável
let mut bananas = 5; // mutável
```

> Nota: A sintaxe `//` inicia um comentário que continua até o final da linha. O Rust ignora tudo nos comentários. Vamos discutir comentários com mais detalhes no [Capítulo 3][comments]<!-- ignore -->.

Voltando ao programa do jogo de adivinhação, agora você sabe que `let mut guess` vai introduzir uma variável mutável chamada `guess`. O sinal de igual (`=`) diz ao Rust que queremos vincular algo à variável agora. À direita do sinal de igual está o valor ao qual `guess` está vinculado, que é o resultado da chamada a `String::new`, uma função que retorna uma nova instância de `String`. [`String`][string]<!-- ignore --> é um tipo de string fornecido pela biblioteca padrão que é um texto crescente codificado em UTF-8.

A sintaxe `::` na linha `::new` indica que `new` é uma função associada do tipo `String`. Uma _função associada_ é uma função implementada em um tipo, neste caso `String`. Esta função `new` cria uma nova string vazia. Você encontrará uma função `new` em muitos tipos porque é um nome comum para uma função que cria um novo valor de algum tipo.

No total, a linha `let mut guess = String::new();` criou uma variável mutável que está atualmente vinculada a uma nova instância vazia de `String`. Ufa!

<a id="receiving-user-input"></a>
### Recebendo Entrada do Usuário

Lembre-se de que incluímos a funcionalidade de entrada/saída da biblioteca padrão com `use std::io;` na primeira linha do programa. Agora vamos chamar a função `stdin` do módulo `io`, que nos permitirá lidar com a entrada do usuário:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:read}}
```

Se não tivéssemos importado o módulo `io` com `use std::io;` no início do programa, ainda poderíamos usar a função escrevendo esta chamada como `std::io::stdin`. A função `stdin` retorna uma instância de [`std::io::Stdin`][iostdin]<!-- ignore -->, que é um tipo que representa um identificador para a entrada padrão do seu terminal.

Em seguida, a linha `.read_line(&mut guess)` chama o método [`read_line`][read_line]<!-- ignore --> no identificador da entrada padrão para obter entrada do usuário. Também estamos passando `&mut guess` como argumento para `read_line` para dizer em qual string armazenar a entrada do usuário. O trabalho completo de `read_line` é pegar o que quer que o usuário digite na entrada padrão e acrescentar isso a uma string (sem sobrescrever seu conteúdo), então passamos essa string como argumento. O argumento da string precisa ser mutável para que o método possa alterar o conteúdo da string.

O `&` indica que este argumento é uma _referência_, o que lhe dá uma maneira de permitir que múltiplas partes do seu código acessem um único dado sem precisar copiar esse dado para a memória várias vezes. Referências são um recurso complexo, e uma das principais vantagens do Rust é o quanto é seguro e fácil usar referências. Você não precisa saber muito desses detalhes para terminar este programa. Por enquanto, tudo que você precisa saber é que, assim como variáveis, referências são imutáveis por padrão. Portanto, você precisa escrever `&mut guess` em vez de `&guess` para torná-la mutável. (O Capítulo 4 explicará referências mais detalhadamente.)

<!-- Old headings. Do not remove or links may break. -->

<a id="handling-potential-failure-with-the-result-type"></a>

<a id="handling-potential-failure-with-result"></a>
### Lidando com Falhas Potenciais com `Result`

Ainda estamos trabalhando nesta linha de código. Estamos agora discutindo uma terceira linha de texto, mas observe que ela ainda faz parte de uma única linha lógica de código. A próxima parte é este método:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:expect}}
```

Poderíamos ter escrito este código assim:

```rust,ignore
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

No entanto, uma linha longa é difícil de ler, então é melhor dividi-la. Muitas vezes é aconselhável introduzir uma nova linha e outros espaços em branco para ajudar a quebrar linhas longas quando você chama um método com a sintaxe `.nome_do_método()`. Agora vamos discutir o que esta linha faz.

Como mencionado anteriormente, `read_line` coloca o que o usuário digita na string que passamos para ele, mas também retorna um valor `Result`. [`Result`][result]<!-- ignore --> é uma [_enumeração_][enums]<!-- ignore -->, frequentemente chamada de _enum_, que é um tipo que pode estar em um de vários estados possíveis. Chamamos cada estado possível de _variante*.

O [Capítulo 6][enums]<!-- ignore --> cobrirá *enums* com mais detalhes. O propósito desses tipos `Result` é codificar informações de tratamento de erros.

As variantes de `Result` são `Ok` e `Err`. A variante `Ok` indica que a operação foi bem-sucedida e contém o valor gerado com sucesso. A variante `Err` significa que a operação falhou e contém informações sobre como ou por que a operação falhou.

Valores do tipo `Result`, assim como valores de qualquer tipo, têm métodos definidos neles. Uma instância de `Result` tem um [método `expect`][expect]<!-- ignore --> que você pode chamar. Se esta instância de `Result` for um valor `Err`, `expect` vai fazer o programa travar e exibir a mensagem que você passou como argumento para `expect`. Se o método `read_line` retornar um `Err`, provavelmente será o resultado de um erro vindo do sistema operacional subjacente. Se esta instância de `Result` for um valor `Ok`, `expect` vai pegar o valor de retorno que `Ok` está contendo e retornar apenas esse valor para você para que você possa usá-lo. Neste caso, esse valor é o número de bytes na entrada do usuário.

Se você não chamar `expect`, o programa vai compilar, mas você receberá um aviso:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-02-without-expect/output.txt}}
```

O Rust avisa que você não usou o valor `Result` retornado por `read_line`, indicando que o programa não tratou um possível erro.

A maneira correta de suprimir o aviso é realmente escrever código de tratamento de erros, mas no nosso caso queremos apenas travar este programa quando um problema ocorrer, então podemos usar `expect`. Você aprenderá sobre como se recuperar de erros no [Capítulo 9][recover]<!-- ignore -->.

<a id="printing-values-with-println-placeholders"></a>
### Imprimindo Valores com Espaços Reservados `println!`

Além da chave de fechamento, há apenas mais uma linha para discutir no código até agora:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print_guess}}
```

Esta linha imprime a string que agora contém a entrada do usuário. O conjunto de chaves `{}` é um espaço reservado (*placeholder*): pense em `{}` como pequenas pinças de caranguejo que seguram um valor no lugar. Ao imprimir o valor de uma variável, o nome da variável pode ir dentro das chaves. Ao imprimir o resultado de avaliar uma expressão, coloque chaves vazias na string de formato, depois siga a string de formato com uma lista separada por vírgulas de expressões para imprimir em cada espaço reservado de chaves vazias na mesma ordem. Imprimir uma variável e o resultado de uma expressão em uma chamada a `println!` ficaria assim:

```rust
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);
```

Este código imprimiria `x = 5 and y + 2 = 12`.

<a id="testing-the-first-part"></a>
### Testando a Primeira Parte

Vamos testar a primeira parte do jogo de adivinhação. Execute-o usando `cargo run`:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-01/
cargo clean
cargo run
input 6 -->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.44s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
6
You guessed: 6
```

Neste ponto, a primeira parte do jogo está feita: estamos obtendo entrada do teclado e depois a imprimindo.

<a id="generating-a-secret-number"></a>
## Gerando um Número Secreto

Em seguida, precisamos gerar um número secreto que o usuário tentará adivinhar. O número secreto deve ser diferente a cada vez para que o jogo seja divertido de jogar mais de uma vez. Usaremos um número aleatório entre 1 e 100 para que o jogo não seja muito difícil. O Rust ainda não inclui funcionalidade de números aleatórios em sua biblioteca padrão. No entanto, a equipe do Rust fornece um [*crate* `rand`][randcrate] com essa funcionalidade.

<!-- Old headings. Do not remove or links may break. -->
<a id="using-a-crate-to-get-more-functionality"></a>

<a id="increasing-functionality-with-a-crate"></a>
### Aumentando a Funcionalidade com um Crate

Lembre-se de que um *crate* é uma coleção de arquivos de código-fonte Rust. O projeto que estamos construindo é um *crate* binário, que é um executável. O *crate* `rand` é um *crate* de biblioteca, que contém código destinado a ser usado em outros programas e não pode ser executado por si só.

A coordenação de *crates* externos pelo Cargo é onde o Cargo realmente brilha. Antes de podermos escrever código que usa `rand`, precisamos modificar o arquivo _Cargo.toml_ para incluir o *crate* `rand` como dependência. Abra esse arquivo agora e adicione a seguinte linha ao fundo, abaixo do cabeçalho da seção `[dependencies]` que o Cargo criou para você. Certifique-se de especificar `rand` exatamente como temos aqui, com esse número de versão, ou os exemplos de código neste tutorial podem não funcionar:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:

* ch01-01-installation.md
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
* ch14-03-cargo-workspaces.md
-->

<span class="filename">Arquivo: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:8:}}
```

No arquivo _Cargo.toml_, tudo que segue um cabeçalho faz parte dessa seção, que continua até outra seção começar. Em `[dependencies]`, você diz ao Cargo de quais *crates* externos seu projeto depende e quais versões desses *crates* você exige. Neste caso, especificamos o *crate* `rand` com o especificador de versão semântica `0.10.1`. O Cargo entende [Versionamento Semântico][semver]<!-- ignore --> (às vezes chamado de _SemVer_), que é um padrão para escrever números de versão. O especificador `0.10.1` é na verdade uma abreviação de `^0.10.1`, o que significa qualquer versão que seja pelo menos 0.10.1 mas abaixo de 0.11.0.

O Cargo considera essas versões como tendo APIs públicas compatíveis com a versão 0.10.1, e essa especificação garante que você obtenha a versão de patch mais recente que ainda compilará com o código neste capítulo. Qualquer versão 0.11.0 ou maior não tem garantia de ter a mesma API que os exemplos a seguir usam.

Agora, sem alterar nenhum código, vamos compilar o projeto, como mostrado na Listagem 2-2.

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
rm Cargo.lock
cargo clean
cargo build -->

<Listing number="2-2" caption="A saída de executar `cargo build` após adicionar o *crate* `rand` como dependência">

```console
$ cargo build
    Updating crates.io index
     Locking 8 packages to latest Rust 1.96.0 compatible versions
  Downloaded rand_core v0.10.1
  Downloaded chacha20 v0.10.1
  Downloaded rand v0.10.1
  Downloaded 3 crates (162.9KiB) in 0.59s
   Compiling libc v0.2.186
   Compiling rand_core v0.10.1
   Compiling getrandom v0.4.3
   Compiling cfg-if v1.0.4
   Compiling chacha20 v0.10.1
   Compiling rand v0.10.1
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.03s
```

</Listing>

Você pode ver números de versão diferentes (mas todos serão compatíveis com o código, graças ao SemVer!) e linhas diferentes (dependendo do sistema operacional), e as linhas podem estar em uma ordem diferente.

Quando incluímos uma dependência externa, o Cargo busca as versões mais recentes de tudo que essa dependência precisa do _registro_, que é uma cópia dos dados do [Crates.io][cratesio]. O Crates.io é onde as pessoas no ecossistema Rust postam seus projetos Rust de código aberto para outros usarem.

Após atualizar o registro, o Cargo verifica a seção `[dependencies]` e baixa quaisquer *crates* listados que ainda não foram baixados. Neste caso, embora tenhamos listado apenas `rand` como dependência, o Cargo também pegou outros *crates* dos quais `rand` depende para funcionar. Após baixar os *crates*, o Rust os compila e depois compila o projeto com as dependências disponíveis.

Se você executar `cargo build` novamente imediatamente sem fazer nenhuma alteração, não verá nenhuma saída além da linha `Finished`. O Cargo sabe que já baixou e compilou as dependências, e você não mudou nada sobre elas no seu arquivo _Cargo.toml_. O Cargo também sabe que você não mudou nada sobre seu código, então ele também não recompila isso. Sem nada para fazer, ele simplesmente sai.

Se você abrir o arquivo _src/main.rs_, fizer uma mudança trivial, salvar e compilar novamente, verá apenas duas linhas de saída:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
touch src/main.rs
cargo build -->

```console
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
```

Essas linhas mostram que o Cargo só atualiza o build com sua pequena mudança no arquivo _src/main.rs_. Suas dependências não mudaram, então o Cargo sabe que pode reutilizar o que já baixou e compilou para elas.

<!-- Old headings. Do not remove or links may break. -->
<a id="ensuring-reproducible-builds-with-the-cargo-lock-file"></a>

<a id="ensuring-reproducible-builds"></a>
#### Garantindo Builds Reproduzíveis

O Cargo tem um mecanismo que garante que você possa reconstruir o mesmo artefato toda vez que você ou qualquer outra pessoa compilar seu código: o Cargo usará apenas as versões das dependências que você especificou até que você indique o contrário. Por exemplo, digamos que na próxima semana saia a versão 0.10.2 do *crate* `rand`, e essa versão contém uma correção importante de bug, mas também contém uma regressão que quebrará seu código. Para lidar com isso, o Rust cria o arquivo _Cargo.lock_ na primeira vez que você executa `cargo build`, então agora temos isso no diretório _guessing_game_.

Quando você compila um projeto pela primeira vez, o Cargo descobre todas as versões das dependências que se encaixam nos critérios e então as escreve no arquivo _Cargo.lock_. Quando você compilar seu projeto no futuro, o Cargo verá que o arquivo _Cargo.lock_ existe e usará as versões especificadas lá em vez de fazer todo o trabalho de descobrir versões novamente. Isso permite que você tenha um build reproduzível automaticamente. Em outras palavras, seu projeto permanecerá em 0.10.1 até que você faça uma atualização explícita, graças ao arquivo _Cargo.lock_. Como o arquivo _Cargo.lock_ é importante para builds reproduzíveis, ele frequentemente é incluído no controle de versão junto com o restante do código do seu projeto.

<a id="updating-a-crate-to-get-a-new-version"></a>
#### Atualizando um Crate para Obter uma Nova Versão

Quando você _quiser_ atualizar um *crate*, o Cargo fornece o comando `update`, que vai ignorar o arquivo _Cargo.lock_ e descobrir todas as versões mais recentes que se encaixam nas suas especificações em _Cargo.toml_. O Cargo então escreverá essas versões no arquivo _Cargo.lock_. Caso contrário, por padrão, o Cargo só procurará versões maiores que 0.10.1 e menores que 0.11.0. Se o *crate* `rand` lançou as duas novas versões 0.10.2 e 0.999.0, você veria o seguinte se executasse `cargo update`:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
cargo update
assuming there is a new version of rand; otherwise use another update
as a guide to creating the hypothetical output shown here -->

```console
$ cargo update
    Updating crates.io index
     Locking 1 package to latest Rust 1.96.0 compatible version
    Updating rand v0.10.1 -> v0.10.2 (available: v0.999.0)
```

O Cargo ignora a versão 0.999.0. Neste momento, você também notaria uma mudança no seu arquivo _Cargo.lock_ observando que a versão do *crate* `rand` que você está usando agora é 0.10.2. Para usar a versão 0.999.0 do `rand` ou qualquer versão da série 0.999._x_, você teria que atualizar o arquivo _Cargo.toml_ para ficar assim (não faça essa mudança agora, porque os exemplos a seguir assumem que você está usando `rand` 0.10):

```toml
[dependencies]
rand = "0.999.0"
```

Da próxima vez que você executar `cargo build`, o Cargo atualizará o registro de *crates* disponíveis e reavaliará seus requisitos de `rand` de acordo com a nova versão que você especificou.

Há muito mais a dizer sobre o [Cargo][doccargo]<!-- ignore --> e [seu ecossistema][doccratesio]<!-- ignore -->, que discutiremos no Capítulo 14, mas por enquanto é tudo que você precisa saber. O Cargo torna muito fácil reutilizar bibliotecas, então os Rustaceans conseguem escrever projetos menores que são montados a partir de vários *packages*.

<a id="generating-a-random-number"></a>
### Gerando um Número Aleatório

Vamos começar a usar `rand` para gerar um número para adivinhar. O próximo passo é atualizar _src/main.rs_, como mostrado na Listagem 2-3.

<Listing number="2-3" file-name="src/main.rs" caption="Adicionando código para gerar um número aleatório">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:all}}
```

</Listing>

Primeiro, adicionamos a linha `use rand::prelude::*;`. O módulo `prelude` contém as partes mais usadas do *crate* `rand`, e `use` torna esses itens disponíveis no escopo do nosso programa.

Em seguida, estamos adicionando duas linhas no meio. Na primeira linha, chamamos a função `rand::rng` que nos dá o gerador de números aleatórios específico que vamos usar: um que é local para a *thread* de execução atual e é inicializado pelo sistema operacional. Em seguida, chamamos o método `random_range` no gerador de números aleatórios. Este método é definido pelo *trait* `RngExt` que faz parte do módulo `rand::prelude` que trouxemos para o escopo com a instrução `use rand::prelude::*;`. O método `random_range` recebe uma expressão de intervalo como argumento e gera um número aleatório dentro do intervalo. O tipo de expressão de intervalo que estamos usando aqui tem a forma `início..=fim` e é inclusiva nos limites inferior e superior, então precisamos especificar `1..=100` para solicitar um número entre 1 e 100.

> Nota: Você não vai simplesmente saber o que trazer para o escopo e quais métodos e funções chamar de um *crate*, então cada *crate* tem documentação com instruções para usá-lo. Outro recurso interessante do Cargo é que executar o comando `cargo doc --open` vai construir a documentação fornecida por todas as suas dependências localmente e abri-la no seu navegador. Se você estiver interessado em outras funcionalidades do *crate* `rand`, por exemplo, execute `cargo doc --open` e clique em `rand` na barra lateral à esquerda.

A segunda nova linha imprime o número secreto. Isso é útil enquanto estamos desenvolvendo o programa para podermos testá-lo, mas vamos deletá-la da versão final. Não é muito divertido se o programa imprime a resposta assim que começa!

Tente executar o programa algumas vezes:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-03/
cargo run
4
cargo run
5
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 7
Please input your guess.
4
You guessed: 4

$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 83
Please input your guess.
5
You guessed: 5
```

Você deverá obter números aleatórios diferentes, e todos devem ser números entre 1 e 100. Se você receber avisos, eles são seguros para ignorar. Se você receber erros, verifique se você tem `rand = "0.10.1"` no seu *Cargo.toml*, pois versões futuras do `rand` podem ter uma API diferente, mas qualquer versão na série `0.10` deve funcionar com o código neste capítulo.

<a id="comparing-the-guess-to-the-secret-number"></a>
## Comparando o Palpite com o Número Secreto

Agora que temos a entrada do usuário e um número aleatório, podemos compará-los. Essa etapa é mostrada na Listagem 2-4. Observe que este código ainda não vai compilar, como explicaremos.

<Listing number="2-4" file-name="src/main.rs" caption="Lidando com os possíveis valores de retorno ao comparar dois números">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-04/src/main.rs:here}}
```

</Listing>

Primeiro, adicionamos outra instrução `use`, trazendo um tipo chamado `std::cmp::Ordering` para o escopo da biblioteca padrão. O tipo `Ordering` é outro *enum* e tem as variantes `Less`, `Greater` e `Equal`. Esses são os três resultados possíveis quando você compara dois valores.

Em seguida, adicionamos cinco novas linhas no fundo que usam o tipo `Ordering`. O método `cmp` compara dois valores e pode ser chamado em qualquer coisa que possa ser comparada. Ele recebe uma referência ao que quer que você queira comparar: aqui, está comparando `guess` com `secret_number`. Depois, retorna uma variante do *enum* `Ordering` que trouxemos para o escopo com a instrução `use`. Usamos uma expressão [`match`][match]<!-- ignore --> para decidir o que fazer a seguir com base em qual variante de `Ordering` foi retornada pela chamada a `cmp` com os valores em `guess` e `secret_number`.

Uma expressão `match` é composta de _braços_ (*arms*). Um braço consiste em um _padrão_ (*pattern*) para corresponder e o código que deve ser executado se o valor fornecido a `match` se encaixar no padrão desse braço. O Rust pega o valor fornecido a `match` e percorre o padrão de cada braço em sequência. Os padrões e o construto `match` são recursos poderosos do Rust: eles permitem que você expresse uma variedade de situações que seu código pode encontrar e garantem que você lide com todas elas. Esses recursos serão cobertos em detalhes no Capítulo 6 e Capítulo 19, respectivamente.

Vamos percorrer um exemplo com a expressão `match` que usamos aqui. Digamos que o usuário chutou 50 e o número secreto gerado aleatoriamente desta vez é 38.

Quando o código compara 50 com 38, o método `cmp` retornará `Ordering::Greater` porque 50 é maior que 38. A expressão `match` obtém o valor `Ordering::Greater` e começa a verificar o padrão de cada braço. Ela olha para o padrão do primeiro braço, `Ordering::Less`, e percebe que o valor `Ordering::Greater` não corresponde a `Ordering::Less`, então ignora o código naquele braço e passa para o próximo. O padrão do próximo braço é `Ordering::Greater`, que _corresponde_ a `Ordering::Greater`! O código associado naquele braço será executado e imprimirá `Too big!` na tela. A expressão `match` termina após o primeiro *match* bem-sucedido, então não verificará o último braço neste cenário.

No entanto, o código da Listagem 2-4 ainda não vai compilar. Vamos tentar:

<!--
The error numbers in this output should be that of the code **WITHOUT** the
anchor or snip comments
-->

```console
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-04/output.txt}}
```

O núcleo do erro afirma que há _tipos incompatíveis_ (*mismatched types*). O Rust tem um sistema de tipos forte e estático. No entanto, ele também tem inferência de tipos. Quando escrevemos `let mut guess = String::new()`, o Rust conseguiu inferir que `guess` deveria ser uma `String` e não nos fez escrever o tipo. O `secret_number`, por outro lado, é um tipo numérico. Alguns tipos numéricos do Rust podem ter um valor entre 1 e 100: `i32`, um número de 32 bits; `u32`, um número de 32 bits sem sinal; `i64`, um número de 64 bits; entre outros. A menos que especificado de outra forma, o Rust usa `i32` como padrão, que é o tipo de `secret_number`, a não ser que você adicione informação de tipo em outro lugar que faria o Rust inferir um tipo numérico diferente. O motivo do erro é que o Rust não consegue comparar uma string e um tipo numérico.

Em última análise, queremos converter a `String` que o programa lê como entrada em um tipo numérico para que possamos compará-la numericamente com o número secreto. Fazemos isso adicionando esta linha ao corpo da função `main`:

<span class="filename">Arquivo: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/src/main.rs:here}}
```

A linha é:

```rust,ignore
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

Criamos uma variável chamada `guess`. Mas espera, o programa já não tem uma variável chamada `guess`? Tem, mas o Rust nos permite _sombrear_ (*shadow*) o valor anterior de `guess` com um novo. O *shadowing* nos permite reutilizar o nome da variável `guess` em vez de nos forçar a criar duas variáveis únicas, como `guess_str` e `guess`, por exemplo. Vamos cobrir isso com mais detalhes no [Capítulo 3][shadowing]<!-- ignore -->, mas por enquanto, saiba que esse recurso é frequentemente usado quando você quer converter um valor de um tipo para outro tipo.

Vinculamos esta nova variável à expressão `guess.trim().parse()`. O `guess` na expressão refere-se à variável `guess` original que continha a entrada como string. O método `trim` em uma instância de `String` vai eliminar qualquer espaço em branco no início e no final, o que devemos fazer antes de converter a string para `u32`, que só pode conter dados numéricos. O usuário deve pressionar <kbd>enter</kbd> para satisfazer `read_line` e inserir seu palpite, o que adiciona um caractere de nova linha à string. Por exemplo, se o usuário digitar <kbd>5</kbd> e pressionar <kbd>enter</kbd>, `guess` fica assim: `5\n`. O `\n` representa "nova linha." (No Windows, pressionar <kbd>enter</kbd> resulta em um retorno de carro e uma nova linha, `\r\n`.) O método `trim` elimina `\n` ou `\r\n`, resultando em apenas `5`.

O [método `parse` em strings][parse]<!-- ignore --> converte uma string para outro tipo. Aqui, o usamos para converter de uma string para um número. Precisamos dizer ao Rust o tipo exato de número que queremos usando `let guess: u32`. Os dois pontos (`:`) após `guess` dizem ao Rust que vamos anotar o tipo da variável. O Rust tem alguns tipos de número embutidos; o `u32` visto aqui é um inteiro sem sinal de 32 bits. É uma boa escolha padrão para um pequeno número positivo. Você aprenderá sobre outros tipos de número no [Capítulo 3][integers]<!-- ignore -->.

Além disso, a anotação `u32` neste programa de exemplo e a comparação com `secret_number` significa que o Rust vai inferir que `secret_number` também deve ser um `u32`. Então, agora a comparação será entre dois valores do mesmo tipo!

O método `parse` só funcionará em caracteres que possam ser logicamente convertidos em números e, portanto, pode facilmente causar erros. Se, por exemplo, a string contivesse `A👍%`, não haveria como convertê-la para um número. Porque pode falhar, o método `parse` retorna um tipo `Result`, assim como o método `read_line` (discutido anteriormente em ["Lidando com Falhas Potenciais com `Result`"](#handling-potential-failure-with-result)<!-- ignore -->). Vamos tratar este `Result` da mesma forma, usando o método `expect` novamente. Se `parse` retornar uma variante `Err` de `Result` porque não conseguiu criar um número a partir da string, a chamada a `expect` vai travar o jogo e imprimir a mensagem que fornecemos. Se `parse` conseguir converter com sucesso a string em um número, ela retornará a variante `Ok` de `Result`, e `expect` retornará o número que queremos do valor `Ok`.

Vamos executar o programa agora:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/
touch src/main.rs
cargo run
  76
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 58
Please input your guess.
  76
You guessed: 76
Too big!
```

Ótimo! Mesmo que espaços tenham sido adicionados antes do palpite, o programa ainda descobriu que o usuário chutou 76. Execute o programa algumas vezes para verificar o comportamento diferente com diferentes tipos de entrada: adivinhe o número corretamente, adivinhe um número muito alto e adivinhe um número muito baixo.

Já temos a maior parte do jogo funcionando, mas o usuário só pode fazer um palpite. Vamos mudar isso adicionando um *loop*!

<a id="allowing-multiple-guesses-with-looping"></a>
## Permitindo Múltiplos Palpites com Loop

A palavra-chave `loop` cria um *loop* infinito. Vamos adicionar um *loop* para dar aos usuários mais chances de adivinhar o número:

<span class="filename">Arquivo: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-04-looping/src/main.rs:here}}
```

Como você pode ver, movemos tudo do prompt de entrada do palpite em diante para dentro de um *loop*. Certifique-se de indentar as linhas dentro do *loop* mais quatro espaços cada e execute o programa novamente. O programa agora pedirá outro palpite para sempre, o que na verdade introduz um novo problema. Parece que o usuário não pode sair!

O usuário sempre pode interromper o programa usando o atalho de teclado <kbd>ctrl</kbd>-<kbd>C</kbd>. Mas há outra maneira de escapar desse monstro insaciável, como mencionado na discussão sobre `parse` em ["Comparando o Palpite com o Número Secreto"](#comparing-the-guess-to-the-secret-number)<!-- ignore -->: se o usuário inserir uma resposta que não seja um número, o programa vai travar. Podemos aproveitar isso para permitir que o usuário saia, como mostrado aqui:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-04-looping/
touch src/main.rs
cargo run
(too small guess)
(too big guess)
(correct guess)
quit
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 59
Please input your guess.
45
You guessed: 45
Too small!
Please input your guess.
60
You guessed: 60
Too big!
Please input your guess.
59
You guessed: 59
You win!
Please input your guess.
quit

thread 'main' (6694925) panicked at src/main.rs:28:47:
Please type a number!: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Digitar `quit` vai encerrar o jogo, mas como você vai notar, digitar qualquer outra entrada que não seja um número também vai. Isso é subótimo, pelo menos; queremos que o jogo também pare quando o número correto for adivinhado.

<a id="quitting-after-a-correct-guess"></a>
### Saindo Após um Palpite Correto

Vamos programar o jogo para sair quando o usuário vencer adicionando uma instrução `break`:

<span class="filename">Arquivo: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-05-quitting/src/main.rs:here}}
```

Adicionar a linha `break` depois de `You win!` faz o programa sair do *loop* quando o usuário adivinhar o número secreto corretamente. Sair do *loop* também significa sair do programa, porque o *loop* é a última parte de `main`.

<a id="handling-invalid-input"></a>
### Lidando com Entradas Inválidas

Para refinar ainda mais o comportamento do jogo, em vez de travar o programa quando o usuário insere um valor que não é número, vamos fazer o jogo ignorar um valor que não é número para que o usuário possa continuar adivinhando. Podemos fazer isso alterando a linha onde `guess` é convertido de `String` para `u32`, como mostrado na Listagem 2-5.

<Listing number="2-5" file-name="src/main.rs" caption="Ignorando um palpite que não é número e pedindo outro palpite em vez de travar o programa">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:here}}
```

</Listing>

Mudamos de uma chamada `expect` para uma expressão `match` para passarmos de travar com um erro para lidar com o erro. Lembre-se de que `parse` retorna um tipo `Result` e `Result` é um *enum* que tem as variantes `Ok` e `Err`. Estamos usando uma expressão `match` aqui, como fizemos com o resultado `Ordering` do método `cmp`.

Se `parse` conseguir converter com sucesso a string em um número, ele retornará um valor `Ok` que contém o número resultante. Esse valor `Ok` corresponderá ao padrão do primeiro braço, e a expressão `match` simplesmente retornará o valor `num` que `parse` produziu e colocou dentro do valor `Ok`. Esse número vai acabar exatamente onde queremos na nova variável `guess` que estamos criando.

Se `parse` _não_ conseguir converter a string em um número, ele retornará um valor `Err` que contém mais informações sobre o erro. O valor `Err` não corresponde ao padrão `Ok(num)` no primeiro braço do `match`, mas corresponde ao padrão `Err(_)` no segundo braço. O sublinhado `_` é um valor curinga; neste exemplo, estamos dizendo que queremos corresponder a todos os valores `Err`, não importa quais informações eles tenham dentro. Então, o programa executará o código do segundo braço, `continue`, que diz ao programa para ir para a próxima iteração do `loop` e pedir outro palpite. Assim, efetivamente, o programa ignora todos os erros que `parse` possa encontrar!

Agora tudo no programa deve funcionar como esperado. Vamos tentar:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-05/
cargo run
(too small guess)
(too big guess)
foo
(correct guess)
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 61
Please input your guess.
10
You guessed: 10
Too small!
Please input your guess.
99
You guessed: 99
Too big!
Please input your guess.
foo
Please input your guess.
61
You guessed: 61
You win!
```

Incrível! Com um último pequeno ajuste, terminaremos o jogo de adivinhação. Lembre-se de que o programa ainda está imprimindo o número secreto. Isso funcionou bem para testes, mas arruína o jogo. Vamos deletar o `println!` que exibe o número secreto. A Listagem 2-6 mostra o código final.

<Listing number="2-6" file-name="src/main.rs" caption="Código completo do jogo de adivinhação">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-06/src/main.rs}}
```

</Listing>

Neste ponto, você construiu com sucesso o jogo de adivinhação. Parabéns!

<a id="summary"></a>
## Resumo

Este projeto foi uma forma prática de apresentar a você muitos novos conceitos do Rust: `let`, `match`, funções, o uso de *crates* externos e muito mais. Nos próximos capítulos, você aprenderá sobre esses conceitos com mais detalhes. O Capítulo 3 cobre conceitos que a maioria das linguagens de programação tem, como variáveis, tipos de dados e funções, e mostra como usá-los no Rust. O Capítulo 4 explora *ownership*, um recurso que torna o Rust diferente das outras linguagens. O Capítulo 5 discute *structs* e sintaxe de métodos, e o Capítulo 6 explica como os *enums* funcionam.

[prelude]: ../std/prelude/index.html
[variables-and-mutability]: ch03-01-variables-and-mutability.html#variables-and-mutability
[comments]: ch03-04-comments.html
[string]: ../std/string/struct.String.html
[iostdin]: ../std/io/struct.Stdin.html
[read_line]: ../std/io/struct.Stdin.html#method.read_line
[result]: ../std/result/enum.Result.html
[enums]: ch06-00-enums.html
[expect]: ../std/result/enum.Result.html#method.expect
[recover]: ch09-02-recoverable-errors-with-result.html
[randcrate]: https://crates.io/crates/rand
[semver]: http://semver.org
[cratesio]: https://crates.io/
[doccargo]: https://doc.rust-lang.org/cargo/
[doccratesio]: https://doc.rust-lang.org/cargo/reference/publishing.html
[match]: ch06-02-match.html
[shadowing]: ch03-01-variables-and-mutability.html#shadowing
[parse]: ../std/primitive.str.html#method.parse
[integers]: ch03-02-data-types.html#integer-types
