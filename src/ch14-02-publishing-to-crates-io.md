<a id="publishing-a-crate-to-cratesio"></a>
## Publicando um Crate no Crates.io

Já usamos *packages* do [crates.io](https://crates.io/)<!-- ignore --> como
dependências do nosso projeto, mas você também pode compartilhar seu código com outras pessoas
publicando seus próprios *packages*. O registro de *crates* em
[crates.io](https://crates.io/)<!-- ignore --> distribui o código-fonte dos
seus *packages*, portanto ele hospeda principalmente código de código aberto (*open source*).

O Rust e o Cargo têm recursos que tornam seu *package* publicado mais fácil para as pessoas
encontrarem e usarem. Falaremos sobre alguns desses recursos a seguir e depois explicaremos
como publicar um *package*.

<a id="making-useful-documentation-comments"></a>
### Criando Comentários de Documentação Úteis

Documentar seus *packages* com precisão ajudará outros usuários a saber como e quando
usá-los, então vale a pena investir tempo escrevendo documentação. No Capítulo
3, discutimos como comentar código Rust usando duas barras, `//`. O Rust também possui
um tipo específico de comentário para documentação, conhecido convenientemente como
_comentário de documentação_, que gera documentação em HTML. O HTML
exibe o conteúdo dos comentários de documentação para itens da API pública destinados
a programadores interessados em saber como _usar_ seu *crate*, em vez de como
seu *crate* é _implementado_.

Comentários de documentação usam três barras, `///`, em vez de duas e suportam
notação Markdown para formatar o texto. Coloque comentários de documentação logo
antes do item que eles estão documentando. A Listagem 14-1 mostra comentários de documentação
para uma função `add_one` em um *crate* chamado `my_crate`.

<Listing number="14-1" file-name="src/lib.rs" caption="Um comentário de documentação para uma função">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-01/src/lib.rs}}
```

</Listing>

Aqui, fornecemos uma descrição do que a função `add_one` faz, iniciamos uma
seção com o título `Examples` e, em seguida, fornecemos um código que demonstra
como usar a função `add_one`. Podemos gerar a documentação HTML a partir
desse comentário de documentação executando `cargo doc`. Esse comando executa a
ferramenta `rustdoc` distribuída com o Rust e coloca a documentação HTML gerada
no diretório _target/doc_.

Por conveniência, executar `cargo doc --open` compila o HTML da
documentação do seu *crate* atual (bem como a documentação de todas as
dependências do seu *crate*) e abre o resultado no navegador. Navegue até a
função `add_one` e você verá como o texto nos comentários de documentação é
renderizado, conforme mostrado na Figura 14-1.

<img alt="Documentação HTML renderizada para a função `add_one` de `my_crate`" src="img/trpl14-01.png" class="center" />

<span class="caption">Figura 14-1: A documentação HTML para a função `add_one`</span>

<a id="commonly-used-sections"></a>
#### Seções Comumente Usadas

Usamos o título Markdown `# Examples` na Listagem 14-1 para criar uma seção
no HTML com o título “Examples”. Aqui estão algumas outras seções que autores de *crates*
costumam usar em sua documentação:

- **Panics**: São os cenários nos quais a função documentada
  pode entrar em pânico (*panic*). Quem chama a função e não quer que seu programa entre em pânico
  deve garantir que não a chamará nessas situações.
- **Errors**: Se a função retorna um `Result`, descrever os tipos de
  erros que podem ocorrer e quais condições podem causar o retorno desses erros
  pode ser útil para quem chama a função, permitindo escrever código para tratar os
  diferentes tipos de erros de maneiras diferentes.
- **Safety**: Se a função for `unsafe` para chamar (discutimos código inseguro no
  Capítulo 20), deve haver uma seção explicando por que a função é `unsafe`
  e detalhando as invariantes que a função espera que quem a chama mantenha.

A maioria dos comentários de documentação não precisa de todas essas seções, mas esta é uma
boa lista de verificação para lembrar você dos aspectos do seu código que os usuários terão
interesse em conhecer.

<a id="documentation-comments-as-tests"></a>
#### Comentários de Documentação como Testes

Adicionar blocos de código de exemplo nos seus comentários de documentação ajuda a demonstrar
como usar sua biblioteca e traz um bônus adicional: executar `cargo test`
executará os exemplos de código na sua documentação como testes! Nada é melhor do que
uma documentação com exemplos. Mas nada é pior do que exemplos que não funcionam
porque o código mudou desde que a documentação foi escrita. Se executarmos
`cargo test` com a documentação da função `add_one` da Listagem
14-1, veremos uma seção nos resultados dos testes semelhante a esta:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-01/
cargo test
copy just the doc-tests section below
-->

```text
   Doc-tests my_crate

running 1 test
test src/lib.rs - add_one (line 5) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.27s
```

Agora, se alterarmos a função ou o exemplo de modo que o `assert_eq!`
no exemplo entre em pânico, e executarmos `cargo test` novamente, veremos que os testes de documentação (*doc tests*)
detectam que o exemplo e o código estão fora de sincronia!

<!-- Old headings. Do not remove or links may break. -->

<a id="commenting-contained-items"></a>

<a id="contained-item-comments"></a>
#### Comentários de Itens Contidos

O estilo de comentário de documentação `//!` adiciona documentação ao item que *contém*
os comentários, em vez dos itens *seguintes* aos comentários. Normalmente
usamos esses comentários dentro do arquivo raiz do *crate* (_src/lib.rs_ por convenção)
ou dentro de um módulo para documentar o *crate* ou o módulo como um todo.

Por exemplo, para adicionar documentação que descreve o propósito do *crate* `my_crate`
que contém a função `add_one`, adicionamos comentários de documentação que
começam com `//!` no início do arquivo _src/lib.rs_, conforme mostrado na Listagem
14-2.

<Listing number="14-2" file-name="src/lib.rs" caption="A documentação para o crate `my_crate` como um todo">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-02/src/lib.rs:here}}
```

</Listing>

Note que não há nenhum código após a última linha que começa com `//!`. Como
começamos os comentários com `//!` em vez de `///`, estamos documentando o item
que contém esse comentário, e não um item que o sucede. Neste
caso, esse item é o arquivo _src/lib.rs_, que é a raiz do *crate*. Esses
comentários descrevem o *crate* inteiro.

Quando executarmos `cargo doc --open`, esses comentários aparecerão na página inicial
da documentação do `my_crate`, acima da lista de itens públicos do
*crate*, conforme mostrado na Figura 14-2.

Comentários de documentação dentro de itens são especialmente úteis para descrever *crates* e
módulos. Use-os para explicar o propósito geral do contêiner e
ajudar seus usuários a entender a organização do *crate*.

<img alt="Documentação HTML renderizada com um comentário para o crate como um todo" src="img/trpl14-02.png" class="center" />

<span class="caption">Figura 14-2: A documentação renderizada para `my_crate`,
incluindo o comentário que descreve o *crate* como um todo</span>

<!-- Old headings. Do not remove or links may break. -->

<a id="exporting-a-convenient-public-api-with-pub-use"></a>

<a id="exporting-a-convenient-public-api"></a>
### Exportando uma API Pública Conveniente

A estrutura da sua API pública é uma consideração importante ao publicar um
*crate*. As pessoas que usam seu *crate* estão menos familiarizadas com a estrutura do que você
e podem ter dificuldade em encontrar as partes que desejam usar se o seu *crate*
tiver uma grande hierarquia de módulos.

No Capítulo 7, vimos como tornar itens públicos usando a palavra-chave `pub` e
como trazer itens para o escopo com a palavra-chave `use`. No entanto, a estrutura
que faz sentido para você enquanto desenvolve um *crate* pode não ser muito
conveniente para seus usuários. Você pode querer organizar suas *structs* em uma
hierarquia com vários níveis, mas então quem quiser usar um tipo
definido lá no fundo da hierarquia pode ter dificuldade em descobrir que esse tipo
existe. Também pode ser chato ter que digitar `use
my_crate::some_module::another_module::UsefulType;` em vez de `use
my_crate::UsefulType;`.

A boa notícia é que, se a estrutura _não_ for conveniente para outras pessoas usarem
a partir de outra biblioteca, você não precisa reorganizar sua estrutura interna:
em vez disso, você pode reexportar itens para criar uma estrutura pública diferente
da sua estrutura privada usando `pub use`. *Reexportar* pega um item público
em um local e o torna público em outro local, como se ele tivesse sido
definido nesse outro local.

Por exemplo, digamos que criamos uma biblioteca chamada `art` para modelar conceitos artísticos.
Dentro dessa biblioteca há dois módulos: um módulo `kinds` contendo dois *enums*
chamados `PrimaryColor` e `SecondaryColor` e um módulo `utils` contendo uma
função chamada `mix`, conforme mostrado na Listagem 14-3.

<Listing number="14-3" file-name="src/lib.rs" caption="Uma biblioteca `art` com itens organizados nos módulos `kinds` e `utils`">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-03/src/lib.rs:here}}
```

</Listing>

A Figura 14-3 mostra como ficaria a página inicial da documentação desse *crate*
gerada por `cargo doc`.

<img alt="Documentação renderizada para o crate `art` que lista os módulos `kinds` e `utils`" src="img/trpl14-03.png" class="center" />

<span class="caption">Figura 14-3: A página inicial da documentação de `art`
que lista os módulos `kinds` e `utils`</span>

Note que os tipos `PrimaryColor` e `SecondaryColor` não estão listados na
página inicial, nem a função `mix`. Precisamos clicar em `kinds` e `utils` para
vê-los.

Outro *crate* que dependa dessa biblioteca precisaria de instruções `use` que
tragam os itens de `art` para o escopo, especificando a estrutura de módulos definida
atualmente. A Listagem 14-4 mostra um exemplo de um *crate* que usa os itens
`PrimaryColor` e `mix` do *crate* `art`.

<Listing number="14-4" file-name="src/main.rs" caption="Um crate usando os itens do crate `art` com sua estrutura interna exportada">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-04/src/main.rs}}
```

</Listing>

O autor do código na Listagem 14-4, que usa o *crate* `art`, teve que
descobrir que `PrimaryColor` está no módulo `kinds` e `mix` está no
módulo `utils`. A estrutura de módulos do *crate* `art` é mais relevante para
desenvolvedores que trabalham no *crate* `art` do que para quem o utiliza. A estrutura
interna não contém nenhuma informação útil para alguém que está tentando
entender como usar o *crate* `art`; pelo contrário, causa confusão porque
desenvolvedores que o usam precisam descobrir onde procurar e devem especificar os
nomes dos módulos nas instruções `use`.

Para remover a organização interna da API pública, podemos modificar o código do
*crate* `art` na Listagem 14-3 para adicionar instruções `pub use` a fim de reexportar os
itens no nível superior, conforme mostrado na Listagem 14-5.

<Listing number="14-5" file-name="src/lib.rs" caption="Adicionando instruções `pub use` para reexportar itens">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-05/src/lib.rs:here}}
```

</Listing>

A documentação da API que `cargo doc` gera para esse *crate* agora listará
e criará links para as reexportações na página inicial, conforme mostrado na Figura 14-4, tornando os
tipos `PrimaryColor` e `SecondaryColor` e a função `mix` mais fáceis de encontrar.

<img alt="Documentação renderizada para o crate `art` com as reexportações na página inicial" src="img/trpl14-04.png" class="center" />

<span class="caption">Figura 14-4: A página inicial da documentação de `art`
que lista as reexportações</span>

Os usuários do *crate* `art` ainda podem ver e usar a estrutura interna da Listagem
14-3 conforme demonstrado na Listagem 14-4, ou podem usar a estrutura mais conveniente
da Listagem 14-5, conforme mostrado na Listagem 14-6.

<Listing number="14-6" file-name="src/main.rs" caption="Um programa usando os itens reexportados do crate `art`">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-06/src/main.rs:here}}
```

</Listing>

Nos casos em que há muitos módulos aninhados, reexportar os tipos no nível
superior com `pub use` pode fazer uma diferença significativa na experiência de
quem usa o *crate*. Outro uso comum de `pub use` é reexportar
definições de uma dependência no *crate* atual para tornar as definições desse *crate*
parte da API pública do seu *crate*.

Criar uma estrutura de API pública útil é mais uma arte do que uma ciência, e você
pode iterar para encontrar a API que funciona melhor para seus usuários. Escolher `pub use`
dá flexibilidade na forma como você estrutura seu *crate* internamente e desacopla
essa estrutura interna daquilo que você apresenta aos seus usuários. Dê uma olhada no
código de alguns *crates* que você instalou para ver se a estrutura interna deles difere
da API pública.

<a id="setting-up-a-cratesio-account"></a>
### Configurando uma Conta no Crates.io

Antes de poder publicar qualquer *crate*, você precisa criar uma conta no
[crates.io](https://crates.io/)<!-- ignore --> e obter um token de API. Para fazer isso,
visite a página inicial em [crates.io](https://crates.io/)<!-- ignore --> e faça
login usando uma conta do GitHub. (A conta do GitHub é atualmente um requisito, mas
o site pode suportar outras formas de criar uma conta no futuro.) Depois de
fazer login, visite as configurações da sua conta em
[https://crates.io/me/](https://crates.io/me/)<!-- ignore --> e obtenha sua
chave de API. Em seguida, execute o comando `cargo login` e cole sua chave de API quando solicitado, assim:

```console
$ cargo login
abcdefghijklmnopqrstuvwxyz012345
```

Esse comando informará seu token de API ao Cargo e o armazenará localmente em
_~/.cargo/credentials.toml_. Note que esse token é um segredo: não o compartilhe
com ninguém. Se você compartilhá-lo com alguém por qualquer motivo, deverá
revogá-lo e gerar um novo token no [crates.io](https://crates.io/)<!-- ignore
-->.

<a id="adding-metadata-to-a-new-crate"></a>
### Adicionando Metadados a um Novo Crate

Digamos que você tenha um *crate* que deseja publicar. Antes de publicar, você precisará
adicionar alguns metadados na seção `[package]` do arquivo _Cargo.toml_ do
*crate*.

Seu *crate* precisará de um nome único. Enquanto estiver trabalhando em um *crate* localmente,
você pode dar a ele o nome que quiser. No entanto, os nomes de *crates* no
[crates.io](https://crates.io/)<!-- ignore --> são alocados por ordem de chegada.
Uma vez que um nome de *crate* é ocupado, ninguém mais pode publicar um *crate*
com esse nome. Antes de tentar publicar um *crate*, pesquise pelo nome que você
deseja usar. Se o nome já tiver sido usado, você precisará encontrar outro nome e
editar o campo `name` no arquivo _Cargo.toml_ sob a seção `[package]` para
usar o novo nome na publicação, desta forma:

<span class="filename">Arquivo: Cargo.toml</span>

```toml
[package]
name = "guessing_game"
```

Mesmo que você tenha escolhido um nome único, ao executar `cargo publish` para publicar
o *crate* neste ponto, você receberá um aviso e depois um erro:

<!-- manual-regeneration
Create a new package with an unregistered name, making no further modifications
  to the generated package, so it is missing the description and license fields.
cargo publish
copy just the relevant lines below
-->

```console
$ cargo publish
    Updating crates.io index
warning: manifest has no description, license, license-file, documentation, homepage or repository.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
--snip--
error: failed to publish to registry at https://crates.io

Caused by:
  the remote server responded with an error (status 400 Bad Request): missing or empty metadata fields: description, license. Please see https://doc.rust-lang.org/cargo/reference/manifest.html for more information on configuring these fields
```

Isso resulta em um erro porque faltam algumas informações cruciais: uma
descrição e uma licença são obrigatórias para que as pessoas saibam o que seu *crate*
faz e sob quais termos podem usá-lo. No _Cargo.toml_, adicione uma descrição
de apenas uma ou duas frases, pois ela aparecerá junto ao seu *crate* nos resultados de
pesquisa. Para o campo `license`, você precisa fornecer um _valor identificador de
licença_. O [Software Package Data Exchange (SPDX) da Linux Foundation][spdx]
lista os identificadores que você pode usar para esse valor. Por exemplo, para especificar que
você licenciou seu *crate* usando a Licença MIT, adicione o identificador `MIT`:

<span class="filename">Arquivo: Cargo.toml</span>

```toml
[package]
name = "guessing_game"
license = "MIT"
```

Se você quiser usar uma licença que não aparece no SPDX, precisará colocar
o texto dessa licença em um arquivo, incluir o arquivo no seu projeto e então
usar `license-file` para especificar o nome desse arquivo em vez de usar a
chave `license`.

Orientações sobre qual licença é apropriada para o seu projeto estão além do escopo
deste livro. Muitas pessoas na comunidade Rust licenciam seus projetos da
mesma forma que o Rust, usando uma licença dupla de `MIT OR Apache-2.0`. Essa prática
demonstra que você também pode especificar múltiplos identificadores de licença separados
por `OR` para ter múltiplas licenças para o seu projeto.

Com um nome único, a versão, sua descrição e uma licença adicionados, o
arquivo _Cargo.toml_ de um projeto pronto para publicação pode ficar assim:

<span class="filename">Arquivo: Cargo.toml</span>

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2024"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"

[dependencies]
```

[A documentação do Cargo](https://doc.rust-lang.org/cargo/) descreve outros
metadados que você pode especificar para garantir que outras pessoas possam descobrir e usar seu *crate*
com mais facilidade.

<a id="publishing-to-cratesio"></a>
### Publicando no Crates.io

Agora que você criou uma conta, salvou seu token de API, escolheu um nome para
seu *crate* e especificou os metadados obrigatórios, você está pronto para publicar!
Publicar um *crate* envia uma versão específica para o
[crates.io](https://crates.io/)<!-- ignore --> para que outras pessoas possam usá-la.

Tenha cuidado, pois uma publicação é _permanente_. A versão nunca pode ser
sobrescrita, e o código não pode ser excluído, exceto em certas circunstâncias.
Um dos principais objetivos do Crates.io é atuar como um arquivo permanente de código para que
as compilações de todos os projetos que dependem de *crates* do
[crates.io](https://crates.io/)<!-- ignore --> continuem funcionando. Permitir
exclusões de versões tornaria impossível cumprir esse objetivo. No entanto, não há
limite para o número de versões de um *crate* que você pode publicar.

Execute o comando `cargo publish` novamente. Agora ele deve funcionar:

<!-- manual-regeneration
go to some valid crate, publish a new version
cargo publish
copy just the relevant lines below
-->

```console
$ cargo publish
    Updating crates.io index
   Packaging guessing_game v0.1.0 (file:///projects/guessing_game)
    Packaged 6 files, 1.2KiB (895.0B compressed)
   Verifying guessing_game v0.1.0 (file:///projects/guessing_game)
   Compiling guessing_game v0.1.0
(file:///projects/guessing_game/target/package/guessing_game-0.1.0)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
   Uploading guessing_game v0.1.0 (file:///projects/guessing_game)
    Uploaded guessing_game v0.1.0 to registry `crates-io`
note: waiting for `guessing_game v0.1.0` to be available at registry
`crates-io`.
You may press ctrl-c to skip waiting; the crate should be available shortly.
   Published guessing_game v0.1.0 at registry `crates-io`
```

Parabéns! Agora você compartilhou seu código com a comunidade Rust, e
qualquer pessoa pode facilmente adicionar seu *crate* como dependência do próprio projeto.

<a id="publishing-a-new-version-of-an-existing-crate"></a>
### Publicando uma Nova Versão de um Crate Existente

Quando você fizer alterações no seu *crate* e estiver pronto para lançar uma nova versão,
basta alterar o valor de `version` especificado no seu arquivo _Cargo.toml_ e
republicar. Use as [regras de Versionamento Semântico][semver] para decidir qual é o
próximo número de versão apropriado, com base nos tipos de alterações que você fez.
Em seguida, execute `cargo publish` para enviar a nova versão.

<!-- Old headings. Do not remove or links may break. -->

<a id="removing-versions-from-cratesio-with-cargo-yank"></a>
<a id="deprecating-versions-from-cratesio-with-cargo-yank"></a>

<a id="deprecating-versions-from-cratesio"></a>
### Depreciando Versões no Crates.io

Embora você não possa remover versões anteriores de um *crate*, pode impedir que
projetos futuros as adicionem como uma nova dependência. Isso é útil quando uma
versão de um *crate* está quebrada por algum motivo. Nessas situações, o Cargo
oferece suporte para retirar (*yank*) uma versão do *crate*.

_Retirar_ (*yanking*) uma versão impede que novos projetos dependam dessa versão,
enquanto permite que todos os projetos existentes que dependem dela continuem funcionando. Essencialmente, um
*yank* significa que todos os projetos com um _Cargo.lock_ não quebrarão, e quaisquer arquivos
_Cargo.lock_ gerados futuramente não usarão a versão retirada.

Para retirar uma versão de um *crate*, no diretório do *crate* que você
publicou anteriormente, execute `cargo yank` e especifique qual versão deseja
retirar. Por exemplo, se publicamos um *crate* chamado `guessing_game` versão
1.0.1 e queremos retirá-la, executaríamos o seguinte no diretório do projeto
`guessing_game`:

<!-- manual-regeneration:
cargo yank carol-test --version 2.1.0
cargo yank carol-test --version 2.1.0 --undo
-->

```console
$ cargo yank --vers 1.0.1
    Updating crates.io index
        Yank guessing_game@1.0.1
```

Adicionando `--undo` ao comando, você também pode desfazer um *yank* e permitir que projetos
voltem a depender de uma versão:

```console
$ cargo yank --vers 1.0.1 --undo
    Updating crates.io index
      Unyank guessing_game@1.0.1
```

Um *yank* _não_ exclui nenhum código. Ele não pode, por exemplo, excluir segredos
enviados acidentalmente. Se isso acontecer, você deve redefinir esses segredos imediatamente.

[spdx]: https://spdx.org/licenses/
[semver]: https://semver.org/
