<a id="hello-cargo"></a>
## Olá, Cargo!

O Cargo é o sistema de build e gerenciador de *packages* do Rust. A maioria dos Rustaceans usa essa ferramenta para gerenciar seus projetos Rust, pois o Cargo cuida de muitas tarefas para você, como compilar seu código, baixar as bibliotecas das quais seu código depende e compilar essas bibliotecas. (Chamamos as bibliotecas de que seu código precisa de _dependências_.)

Os programas Rust mais simples, como o que escrevemos até agora, não têm nenhuma dependência. Se tivéssemos construído o projeto "Hello, world!" com o Cargo, ele usaria apenas a parte do Cargo que lida com a compilação do seu código. À medida que você escreve programas Rust mais complexos, vai adicionar dependências e, se você iniciar um projeto usando o Cargo, adicionar dependências será muito mais fácil.

Como a grande maioria dos projetos Rust usa o Cargo, o restante deste livro assume que você também está usando o Cargo. O Cargo vem instalado com o Rust se você usou os instaladores oficiais discutidos na seção ["Instalação"][installation]<!-- ignore -->. Se você instalou o Rust por algum outro meio, verifique se o Cargo está instalado digitando o seguinte no seu terminal:

```console
$ cargo --version
```

Se você ver um número de versão, você o tem! Se ver um erro, como `command not found`, consulte a documentação do seu método de instalação para determinar como instalar o Cargo separadamente.

<a id="creating-a-project-with-cargo"></a>
### Criando um Projeto com Cargo

Vamos criar um novo projeto usando o Cargo e ver como ele difere do nosso projeto original "Hello, world!". Volte para o seu diretório _projects_ (ou onde quer que você tenha decidido armazenar seu código). Em seguida, em qualquer sistema operacional, execute o seguinte:

```console
$ cargo new hello_cargo
$ cd hello_cargo
```

O primeiro comando cria um novo diretório e projeto chamado _hello_cargo_. Nomeamos nosso projeto _hello_cargo_ e o Cargo cria seus arquivos em um diretório com o mesmo nome.

Acesse o diretório _hello_cargo_ e liste os arquivos. Você verá que o Cargo gerou dois arquivos e um diretório para nós: um arquivo _Cargo.toml_ e um diretório _src_ com um arquivo _main.rs_ dentro.

Ele também inicializou um novo repositório Git juntamente com um arquivo _.gitignore_. Os arquivos Git não serão gerados se você executar `cargo new` dentro de um repositório Git existente; você pode substituir esse comportamento usando `cargo new --vcs=git`.

> Nota: Git é um sistema de controle de versão comum. Você pode mudar o `cargo new` para usar um sistema de controle de versão diferente ou nenhum sistema de controle de versão usando a flag `--vcs`. Execute `cargo new --help` para ver as opções disponíveis.

Abra _Cargo.toml_ no editor de texto de sua preferência. Ele deve ser parecido com o código da Listagem 1-2.

<Listing number="1-2" file-name="Cargo.toml" caption="Conteúdo do *Cargo.toml* gerado pelo `cargo new`">

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]
```

</Listing>

Este arquivo está no formato [_TOML_][toml]<!-- ignore --> (_Tom's Obvious, Minimal Language_), que é o formato de configuração do Cargo.

A primeira linha, `[package]`, é um cabeçalho de seção que indica que as declarações a seguir estão configurando um *package*. À medida que adicionamos mais informações a este arquivo, adicionaremos outras seções.

As próximas três linhas definem as informações de configuração que o Cargo precisa para compilar seu programa: o nome, a versão e a edição do Rust a ser usada. Falaremos sobre a chave `edition` no [Apêndice E][appendix-e]<!-- ignore -->.

A última linha, `[dependencies]`, é o início de uma seção para você listar quaisquer dependências do seu projeto. No Rust, *packages* de código são chamados de _crates_. Não precisaremos de outros *crates* para este projeto, mas precisaremos no primeiro projeto do Capítulo 2, então usaremos esta seção de dependências lá.

Agora abra _src/main.rs_ e dê uma olhada:

<span class="filename">Arquivo: src/main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

O Cargo gerou um programa "Hello, world!" para você, exatamente como o que escrevemos na Listagem 1-1! Até agora, as diferenças entre nosso projeto e o projeto que o Cargo gerou são que o Cargo colocou o código no diretório _src_ e temos um arquivo de configuração _Cargo.toml_ no diretório raiz.

O Cargo espera que seus arquivos-fonte fiquem dentro do diretório _src_. O diretório raiz do projeto é apenas para arquivos README, informações de licença, arquivos de configuração e qualquer outra coisa não relacionada ao seu código. Usar o Cargo ajuda a organizar seus projetos. Há um lugar para tudo, e tudo está no seu lugar.

Se você iniciou um projeto que não usa o Cargo, como fizemos com o projeto "Hello, world!", pode convertê-lo para um projeto que usa o Cargo. Mova o código do projeto para o diretório _src_ e crie um arquivo _Cargo.toml_ apropriado. Uma maneira fácil de obter esse arquivo _Cargo.toml_ é executar `cargo init`, que o criará automaticamente.

<a id="building-and-running-a-cargo-project"></a>
### Compilando e Executando um Projeto Cargo

Agora vamos ver o que é diferente quando compilamos e executamos o programa "Hello, world!" com o Cargo! No seu diretório _hello_cargo_, compile o projeto digitando o seguinte comando:

```console
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

Este comando cria um arquivo executável em _target/debug/hello_cargo_ (ou _target\debug\hello_cargo.exe_ no Windows) em vez de no seu diretório atual. Como o build padrão é um build de debug, o Cargo coloca o binário em um diretório chamado _debug_. Você pode executar o executável com este comando:

```console
$ ./target/debug/hello_cargo # ou .\target\debug\hello_cargo.exe no Windows
Hello, world!
```

Se tudo correr bem, `Hello, world!` deve ser impresso no terminal. Executar `cargo build` pela primeira vez também faz com que o Cargo crie um novo arquivo no nível raiz: _Cargo.lock_. Este arquivo acompanha as versões exatas das dependências do seu projeto. Este projeto não tem dependências, então o arquivo está um pouco esparso. Você nunca precisará alterar este arquivo manualmente; o Cargo gerencia seu conteúdo para você.

Acabamos de compilar um projeto com `cargo build` e executá-lo com `./target/debug/hello_cargo`, mas também podemos usar `cargo run` para compilar o código e então executar o executável resultante em um único comando:

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Usar `cargo run` é mais conveniente do que ter que lembrar de executar `cargo build` e depois usar o caminho completo para o binário, então a maioria dos desenvolvedores usa `cargo run`.

Observe que desta vez não vimos a saída indicando que o Cargo estava compilando o `hello_cargo`. O Cargo percebeu que os arquivos não mudaram, então não recompilou, apenas executou o binário. Se você tivesse modificado o código-fonte, o Cargo teria recompilado o projeto antes de executá-lo, e você teria visto esta saída:

```console
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

O Cargo também fornece um comando chamado `cargo check`. Este comando verifica rapidamente seu código para garantir que ele compila, mas não produz um executável:

```console
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

Por que você não quereria um executável? Muitas vezes, `cargo check` é muito mais rápido que `cargo build` porque pula a etapa de produção de um executável. Se você está verificando continuamente seu trabalho enquanto escreve o código, usar `cargo check` vai acelerar o processo de saber se seu projeto ainda compila! Assim sendo, muitos Rustaceans executam `cargo check` periodicamente enquanto escrevem o programa para garantir que ele compila. Depois, executam `cargo build` quando estão prontos para usar o executável.

Vamos recapitular o que aprendemos até agora sobre o Cargo:

- Podemos criar um projeto usando `cargo new`.
- Podemos compilar um projeto usando `cargo build`.
- Podemos compilar e executar um projeto em uma etapa usando `cargo run`.
- Podemos compilar um projeto sem produzir um binário para verificar erros usando `cargo check`.
- Em vez de salvar o resultado do build no mesmo diretório que nosso código, o Cargo o armazena no diretório _target/debug_.

Uma vantagem adicional de usar o Cargo é que os comandos são os mesmos independentemente do sistema operacional em que você está trabalhando. Portanto, neste momento, não forneceremos mais instruções específicas para Linux e macOS versus Windows.

<a id="building-for-release"></a>
### Compilando para Release

Quando seu projeto estiver finalmente pronto para release, você pode usar `cargo build --release` para compilá-lo com otimizações. Este comando criará um executável em _target/release_ em vez de _target/debug_. As otimizações fazem seu código Rust executar mais rapidamente, mas ativá-las aumenta o tempo que leva para o programa compilar. É por isso que há dois perfis diferentes: um para desenvolvimento, quando você quer recompilar rapidamente e com frequência, e outro para compilar o programa final que você dará ao usuário, que não será recompilado repetidamente e que executará o mais rápido possível. Se você estiver medindo o tempo de execução do seu código, certifique-se de executar `cargo build --release` e fazer o benchmark com o executável em _target/release_.

<!-- Old headings. Do not remove or links may break. -->
<a id="cargo-as-convention"></a>

<a id="leveraging-cargos-conventions"></a>
### Aproveitando as Convenções do Cargo

Com projetos simples, o Cargo não oferece muito valor em relação ao uso direto de `rustc`, mas vai provar seu valor à medida que seus programas se tornarem mais complexos. Uma vez que os programas crescem para múltiplos arquivos ou precisam de uma dependência, é muito mais fácil deixar o Cargo coordenar o build.

Mesmo que o projeto `hello_cargo` seja simples, ele agora usa muitas das ferramentas reais que você usará no restante de sua carreira com Rust. De fato, para trabalhar em quaisquer projetos existentes, você pode usar os seguintes comandos para verificar o código usando Git, mudar para o diretório do projeto e compilar:

```console
$ git clone example.org/someproject
$ cd someproject
$ cargo build
```

Para mais informações sobre o Cargo, confira [sua documentação][cargo].

<a id="summary"></a>
## Resumo

Você já está em um ótimo começo na sua jornada com Rust! Neste capítulo, você aprendeu como:

- Instalar a versão estável mais recente do Rust usando `rustup`.
- Atualizar para uma versão mais recente do Rust.
- Abrir a documentação instalada localmente.
- Escrever e executar um programa "Hello, world!" usando `rustc` diretamente.
- Criar e executar um novo projeto usando as convenções do Cargo.

Este é um ótimo momento para construir um programa mais substancial para se acostumar a ler e escrever código Rust. Então, no Capítulo 2, vamos construir um programa de jogo de adivinhação. Se preferir começar aprendendo como os conceitos comuns de programação funcionam no Rust, veja o Capítulo 3 e depois volte ao Capítulo 2.

[installation]: ch01-01-installation.html#installation
[toml]: https://toml.io
[appendix-e]: appendix-05-editions.html
[cargo]: https://doc.rust-lang.org/cargo/
