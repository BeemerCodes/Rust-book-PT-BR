<!-- Old headings. Do not remove or links may break. -->

<a id="installing-binaries-from-cratesio-with-cargo-install"></a>

<a id="installing-binaries-with-cargo-install"></a>
## Instalando Binários com `cargo install`

O comando `cargo install` permite que você instale e use *crates* binários
localmente. Isso não tem a intenção de substituir pacotes do sistema; é uma forma
conveniente para desenvolvedores Rust instalarem ferramentas que outras pessoas compartilharam no
[crates.io](https://crates.io/)<!-- ignore -->. Note que você só pode instalar
*packages* que tenham alvos binários (*binary targets*). Um _alvo binário_ é o programa executável
criado quando o *crate* possui um arquivo _src/main.rs_ ou outro arquivo especificado
como binário, ao contrário de um alvo de biblioteca, que não é executável por conta própria, mas
é adequado para ser incluído em outros programas. Geralmente, os *crates* trazem
informações no arquivo README indicando se são uma biblioteca, se têm um
alvo binário ou ambos.

Todos os binários instalados com `cargo install` são armazenados na pasta _bin_ da raiz
de instalação. Se você instalou o Rust usando _rustup.rs_ e não fez nenhuma
configuração personalizada, esse diretório será *$HOME/.cargo/bin*. Certifique-se de que
esse diretório esteja no seu `$PATH` para poder executar os programas instalados
com `cargo install`.

Por exemplo, no Capítulo 12 mencionamos que existe uma implementação em Rust da
ferramenta `grep` chamada `ripgrep` para pesquisar arquivos. Para instalar o `ripgrep`,
podemos executar o seguinte:

<!-- manual-regeneration
cargo install something you don't have, copy relevant output below
-->

```console
$ cargo install ripgrep
    Updating crates.io index
  Downloaded ripgrep v14.1.1
  Downloaded 1 crate (213.6 KB) in 0.40s
  Installing ripgrep v14.1.1
--snip--
   Compiling grep v0.3.2
    Finished `release` profile [optimized + debuginfo] target(s) in 6.73s
  Installing ~/.cargo/bin/rg
   Installed package `ripgrep v14.1.1` (executable `rg`)
```

A penúltima linha da saída mostra a localização e o nome do
binário instalado, que no caso do `ripgrep` é `rg`. Desde que o
diretório de instalação esteja no seu `$PATH`, como mencionado anteriormente, você pode
então executar `rg --help` e começar a usar uma ferramenta mais rápida e mais Rustácea para pesquisar arquivos!
