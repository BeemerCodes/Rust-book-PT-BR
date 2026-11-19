<a id="appendix-d-useful-development-tools"></a>
## Apêndice D: Ferramentas Úteis de Desenvolvimento

Neste apêndice, falamos sobre algumas ferramentas de desenvolvimento úteis que o
projeto Rust fornece. Veremos formatação automática, maneiras rápidas de aplicar
correções de avisos, um linter e integração com IDEs.

<a id="automatic-formatting-with-rustfmt"></a>
### Formatação Automática com `rustfmt`

A ferramenta `rustfmt` reformata o seu código de acordo com o estilo de código
da comunidade. Muitos projetos colaborativos usam `rustfmt` para evitar
discussões sobre qual estilo usar ao escrever Rust: todos formatam seu código
usando a ferramenta.

As instalações do Rust incluem `rustfmt` por padrão, então você já deve ter os
programas `rustfmt` e `cargo-fmt` em seu sistema. Esses dois comandos são
análogos a `rustc` e `cargo`, pois `rustfmt` permite um controle mais refinado e
`cargo-fmt` entende as convenções de um projeto que usa Cargo. Para formatar
qualquer projeto Cargo, digite o seguinte:

```console
$ cargo fmt
```

Executar esse comando reformata todo o código Rust no *crate* atual. Isso deve
alterar apenas o estilo do código, não a semântica do código. Para mais
informações sobre o `rustfmt`, consulte [sua documentação][rustfmt].

<a id="fix-your-code-with-rustfix"></a>
### Corrija Seu Código com `rustfix`

A ferramenta `rustfix` é incluída nas instalações do Rust e pode corrigir
automaticamente avisos do compilador que têm uma maneira clara de corrigir o
problema que provavelmente é o que você deseja. Você provavelmente já viu avisos
do compilador antes. Por exemplo, considere este código:

<span class="filename">Arquivo: src/main.rs</span>

```rust
fn main() {
    let mut x = 42;
    println!("{x}");
}
```

Aqui, estamos definindo a variável `x` como mutável, mas na verdade nunca a
alteramos. O Rust nos avisa sobre isso:

```console
$ cargo build
   Compiling myprogram v0.1.0 (file:///projects/myprogram)
warning: variable does not need to be mutable
 --> src/main.rs:2:9
  |
2 |     let mut x = 0;
  |         ----^
  |         |
  |         help: remove this `mut`
  |
  = note: `#[warn(unused_mut)]` on by default
```

O aviso sugere que removamos a palavra-chave `mut`. Podemos aplicar essa
sugestão automaticamente usando a ferramenta `rustfix` executando o comando
`cargo fix`:

```console
$ cargo fix
    Checking myprogram v0.1.0 (file:///projects/myprogram)
      Fixing src/main.rs (1 fix)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

Quando olharmos para _src/main.rs_ novamente, veremos que `cargo fix` alterou o
código:

<span class="filename">Arquivo: src/main.rs</span>

```rust
fn main() {
    let x = 42;
    println!("{x}");
}
```

A variável `x` agora é imutável e o aviso não aparece mais.

Você também pode usar o comando `cargo fix` para fazer a transição do seu código
entre diferentes edições do Rust. As edições são abordadas no [Apêndice
E][editions]<!-- ignore -->.

<a id="more-lints-with-clippy"></a>
### Mais Lints com o Clippy

A ferramenta Clippy é uma coleção de lints para analisar seu código de modo que
você possa detectar erros comuns e melhorar seu código Rust. O Clippy está
incluído nas instalações padrão do Rust.

Para executar os lints do Clippy em qualquer projeto Cargo, digite o seguinte:

```console
$ cargo clippy
```

Por exemplo, digamos que você escreva um programa que usa uma aproximação de uma
constante matemática, como pi, como faz este programa:

<Listing file-name="src/main.rs">

```rust
fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

</Listing>

Executar `cargo clippy` neste projeto resulta neste erro:

```text
error: approximate value of `f{32, 64}::consts::PI` found
 --> src/main.rs:2:13
  |
2 |     let x = 3.1415;
  |             ^^^^^^
  |
  = note: `#[deny(clippy::approx_constant)]` on by default
  = help: consider using the constant directly
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#approx_constant
```

Este erro informa que o Rust já possui uma constante `PI` mais precisa definida,
e que o seu programa ficaria mais correto se você usasse a constante em vez da
aproximação. Você então alteraria seu código para usar a constante `PI`.

O código a seguir não resulta em nenhum erro ou aviso do Clippy:

<Listing file-name="src/main.rs">

```rust
fn main() {
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

</Listing>

Para mais informações sobre o Clippy, consulte [sua documentação][clippy].

<a id="ide-integration-using-rust-analyzer"></a>
### Integração com IDE Usando `rust-analyzer`

Para ajudar na integração com IDEs, a comunidade Rust recomenda o uso do
[`rust-analyzer`][rust-analyzer]<!-- ignore -->. Essa ferramenta é um conjunto de
utilitários centrados no compilador que falam o [Language Server
Protocol][lsp]<!-- ignore -->, que é uma especificação para que IDEs e
linguagens de programação se comuniquem entre si. Diferentes clientes podem usar
o `rust-analyzer`, como [o plug-in Rust analyzer para Visual Studio
Code][vscode].

Visite a [página inicial][rust-analyzer]<!-- ignore --> do projeto
`rust-analyzer` para obter instruções de instalação e, em seguida, instale o
suporte ao servidor de linguagem em sua IDE específica. Sua IDE ganhará recursos
como preenchimento automático, ir para a definição e erros embutidos.

[rustfmt]: https://github.com/rust-lang/rustfmt
[editions]: appendix-05-editions.html
[clippy]: https://github.com/rust-lang/rust-clippy
[rust-analyzer]: https://rust-analyzer.github.io
[lsp]: http://langserver.org/
[vscode]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer
