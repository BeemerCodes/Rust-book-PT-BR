# A Linguagem de Programação Rust

![Status da Build](https://github.com/rust-lang/book/workflows/CI/badge.svg)

Este repositório contém o código-fonte do livro "A Linguagem de Programação Rust"
(*The Rust Programming Language*).

[O livro está disponível em formato impresso pela No Starch Press][nostarch].

[nostarch]: https://nostarch.com/rust-programming-language-2nd-edition

Você também pode ler o livro gratuitamente online (em inglês). Consulte o
livro conforme distribuído nos lançamentos mais recentes do Rust
[estável][stable], [beta][beta] ou [nightly][nightly]. Esteja ciente de que
problemas nessas versões já podem ter sido corrigidos neste repositório, pois
aqueles lançamentos são atualizados com menos frequência.

[stable]: https://doc.rust-lang.org/stable/book/
[beta]: https://doc.rust-lang.org/beta/book/
[nightly]: https://doc.rust-lang.org/nightly/book/

Veja a página de [lançamentos (*releases*)][releases] para baixar apenas o código
de todas as listagens que aparecem no livro.

[releases]: https://github.com/rust-lang/book/releases

## Requisitos

Para compilar o livro é necessário o [mdBook][mdBook], de preferência na mesma
versão que `rust-lang/rust` usa [neste arquivo][rust-mdbook]. Para instalá-lo:

[mdBook]: https://github.com/rust-lang/mdBook
[rust-mdbook]: https://github.com/rust-lang/rust/blob/HEAD/src/tools/rustbook/Cargo.toml

```bash
$ cargo install mdbook --locked --version <version_num>
```

## Compilação

Para compilar o livro, execute:

```bash
$ mdbook build
```

O resultado será gerado no subdiretório `book`. Para visualizá-lo, abra o
arquivo no seu navegador web.

_Firefox:_

```bash
$ firefox book/index.html                       # Linux
$ open -a "Firefox" book/index.html             # OS X
$ Start-Process "firefox.exe" .\book\index.html # Windows (PowerShell)
$ start firefox.exe .\book\index.html           # Windows (Cmd)
```

_Chrome:_

```bash
$ google-chrome book/index.html                 # Linux
$ open -a "Google Chrome" book/index.html       # OS X
$ Start-Process "chrome.exe" .\book\index.html  # Windows (PowerShell)
$ start chrome.exe .\book\index.html            # Windows (Cmd)
```

Para executar os testes:

```bash
$ cd packages/trpl
$ mdbook test --library-path packages/trpl/target/debug/deps
```

## Contribuições

Adoraríamos contar com a sua ajuda! Consulte [CONTRIBUTING.md][contrib] para
saber mais sobre os tipos de contribuição que estamos buscando.

[contrib]: https://github.com/rust-lang/book/blob/main/CONTRIBUTING.md

Como o livro é [impresso][nostarch] e queremos manter a versão online o mais
próxima possível da versão impressa, pode levar mais tempo do que você está
acostumado para analisarmos sua *issue* ou *pull request*.

Até o momento, temos feito uma revisão maior para coincidir com as [Edições do
Rust](https://doc.rust-lang.org/edition-guide/). Entre essas revisões maiores,
corrigiremos apenas erros. Se a sua *issue* ou *pull request* não corrigir
estritamente um erro, ela poderá aguardar até a próxima vez em que estivermos
trabalhando em uma revisão grande: espere algo na ordem de meses ou anos.
Agradecemos pela sua paciência!

### Traduções

Adoraríamos receber ajuda para traduzir o livro! Consulte o rótulo
[Translations][Translations] para participar dos esforços que já estão em
andamento. Abra uma nova *issue* para começar a trabalhar em um novo idioma!
Estamos aguardando o [suporte do mdBook][mdbook support] a múltiplos idiomas
antes de fazer o merge de traduções no repositório oficial, mas sinta-se à
vontade para começar!

[Translations]: https://github.com/rust-lang/book/issues?q=is%3Aopen+is%3Aissue+label%3ATranslations
[mdbook support]: https://github.com/rust-lang/mdBook/issues/5

## Verificação Ortográfica

Para verificar erros de ortografia nos arquivos-fonte, você pode usar o script
`spellcheck.sh` disponível no diretório `ci`. Ele precisa de um dicionário de
palavras válidas, fornecido em `ci/dictionary.txt`. Se o script produzir um
falso positivo (por exemplo, se você usou a palavra `BTreeMap` e o script a
considerar inválida), basta adicionar essa palavra em `ci/dictionary.txt`
(mantenha a ordem alfabética para consistência).
