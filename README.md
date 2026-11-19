# A Linguagem de Programação Rust (PT-BR)

Tradução completa para Português Brasileiro do livro oficial [*The Rust Programming Language*](https://github.com/rust-lang/book) (Rust Edição 2024), incluindo todos os capítulos, apêndices e comentários dos exemplos de código em `listings/`.

## Ler online

A versão compilada está publicada no GitHub Pages:

https://beemercodes.github.io/Rust-book-PT-BR/

## Como usar localmente

Você precisa do [Rust](https://www.rust-lang.org/tools/install) (com `cargo`) e do [mdBook](https://github.com/rust-lang/mdBook) na versão `0.5.1`:

```bash
cargo install mdbook --locked --version 0.5.1
```

Clone o repositório e abra o livro localmente com recarregamento automático:

```bash
git clone https://github.com/BeemerCodes/Rust-book-PT-BR.git
cd Rust-book-PT-BR
mdbook serve --open
```

Se quiser apenas gerar os arquivos HTML estáticos na pasta `book/`:

```bash
mdbook build
```

## Testes e validação

Para rodar os testes dos exemplos de código do livro:

```bash
(cd packages/trpl && cargo build)
mdbook test --library-path packages/trpl/target/debug/deps
```

Para rodar os validadores de caminhos, links de referência e testes do pre-processor:

```bash
cargo run --bin lfp src
bash ci/validate.sh
cargo test --manifest-path packages/mdbook-trpl/Cargo.toml
```

## Contribuições

Encontrou algum erro de tradução, trecho estranho ou link quebrado? Abra uma issue ou envie um pull request.

Ao editar arquivos em `src/`:

- Não renomeie os arquivos `.md` nem altere caminhos em `{{#include}}` / `{{#rustdoc_include}}`.
- Mantenha as âncoras `<a id="slug-em-ingles"></a>` acima dos títulos traduzidos para não quebrar links entre capítulos.
- Em tags `<Listing ... caption="...">`, altere apenas o texto de `caption`.
- Em `listings/`, traduza apenas comentários (`//` e `/* */`), sem alterar a lógica do código ou marcadores `ANCHOR`.

## Licença

Este repositório é baseado no [rust-lang/book](https://github.com/rust-lang/book) e segue o mesmo licenciamento dual: MIT ou Apache-2.0.
