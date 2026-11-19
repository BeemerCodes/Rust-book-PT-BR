<a id="appendix-a-keywords"></a>
## Apêndice A: Palavras-chave

As listas a seguir contêm palavras-chave reservadas para uso atual ou futuro pela
linguagem Rust. Como tal, elas não podem ser usadas como identificadores (exceto
como identificadores brutos, como discutimos na seção [“Identificadores
Brutos”][raw-identifiers]<!-- ignore -->). _Identificadores_ são nomes de
funções, variáveis, parâmetros, campos de *structs*, módulos, *crates*,
constantes, *macros*, valores estáticos, atributos, tipos, *traits* ou
*lifetimes*.

[raw-identifiers]: #raw-identifiers

<a id="keywords-currently-in-use"></a>
### Palavras-chave Atualmente em Uso

A seguir está uma lista das palavras-chave atualmente em uso, com suas
funcionalidades descritas.

- **`as`**: Realiza conversão primitiva (*casting*), desambigua o *trait*
  específico que contém um item ou renomeia itens em instruções `use`.
- **`async`**: Retorna um `Future` em vez de bloquear a *thread* atual.
- **`await`**: Suspende a execução até que o resultado de um `Future` esteja
  pronto.
- **`break`**: Sai de um laço (*loop*) imediatamente.
- **`const`**: Define itens constantes ou ponteiros brutos constantes.
- **`continue`**: Continua para a próxima iteração do laço.
- **`crate`**: Em um caminho de módulo, refere-se à raiz do *crate*.
- **`dyn`**: Despacho dinâmico para um *trait object*.
- **`else`**: Alternativa para as estruturas de controle de fluxo `if` e
  `if let`.
- **`enum`**: Define uma enumeração (*enum*).
- **`extern`**: Vincula uma função ou variável externa.
- **`false`**: Literal booleano falso.
- **`fn`**: Define uma função ou o tipo de ponteiro de função.
- **`for`**: Itera sobre itens de um iterador, implementa um *trait* ou
  especifica um *lifetime* de ordem superior.
- **`if`**: Ramifica com base no resultado de uma expressão condicional.
- **`impl`**: Implementa funcionalidade inerente ou de um *trait*.
- **`in`**: Parte da sintaxe do laço `for`.
- **`let`**: Vincula uma variável.
- **`loop`**: Executa um laço incondicionalmente.
- **`match`**: Faz correspondência de um valor com padrões (*pattern matching*).
- **`mod`**: Define um módulo.
- **`move`**: Faz uma *closure* assumir o *ownership* de todas as suas capturas.
- **`mut`**: Indica mutabilidade em referências, ponteiros brutos ou vinculações
  de padrão.
- **`pub`**: Indica visibilidade pública em campos de *structs*, blocos `impl`
  ou módulos.
- **`ref`**: Vincula por referência.
- **`return`**: Retorna de uma função.
- **`Self`**: Um alias de tipo para o tipo que estamos definindo ou
  implementando.
- **`self`**: O sujeito de um método ou o módulo atual.
- **`static`**: Variável global ou *lifetime* que dura toda a execução do
  programa.
- **`struct`**: Define uma estrutura (*struct*).
- **`super`**: Módulo pai do módulo atual.
- **`trait`**: Define um *trait*.
- **`true`**: Literal booleano verdadeiro.
- **`type`**: Define um alias de tipo ou tipo associado.
- **`union`**: Define uma [union][union]<!-- ignore -->; é uma palavra-chave
  apenas quando usada em uma declaração de *union*.
- **`unsafe`**: Indica código, funções, *traits* ou implementações inseguras
  (*unsafe*).
- **`use`**: Traz símbolos para o escopo.
- **`where`**: Indica cláusulas que restringem um tipo.
- **`while`**: Executa um laço condicionalmente com base no resultado de uma
  expressão.

[union]: ../reference/items/unions.html

<a id="keywords-reserved-for-future-use"></a>
### Palavras-chave Reservadas para Uso Futuro

As palavras-chave a seguir ainda não possuem nenhuma funcionalidade, mas estão
reservadas pelo Rust para possível uso futuro:

- `abstract`
- `become`
- `box`
- `do`
- `final`
- `gen`
- `macro`
- `override`
- `priv`
- `try`
- `typeof`
- `unsized`
- `virtual`
- `yield`

<a id="raw-identifiers"></a>
### Identificadores Brutos

_Identificadores brutos_ (*raw identifiers*) são a sintaxe que permite usar
palavras-chave onde elas normalmente não seriam permitidas. Você usa um
identificador bruto prefixando uma palavra-chave com `r#`.

Por exemplo, `match` é uma palavra-chave. Se você tentar compilar a função a
seguir que usa `match` como seu nome:

<span class="filename">Arquivo: src/main.rs</span>

```rust,ignore,does_not_compile
fn match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}
```

você receberá este erro:

```text
error: expected identifier, found keyword `match`
 --> src/main.rs:4:4
  |
4 | fn match(needle: &str, haystack: &str) -> bool {
  |    ^^^^^ expected identifier, found keyword
```

O erro mostra que você não pode usar a palavra-chave `match` como identificador
da função. Para usar `match` como nome de função, você precisa usar a sintaxe de
identificador bruto, desta forma:

<span class="filename">Arquivo: src/main.rs</span>

```rust
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

fn main() {
    assert!(r#match("foo", "foobar"));
}
```

Este código compilará sem nenhum erro. Observe o prefixo `r#` no nome da função
em sua definição, bem como onde a função é chamada em `main`.

Identificadores brutos permitem que você use qualquer palavra que escolher como
identificador, mesmo que essa palavra seja uma palavra-chave reservada. Isso nos
dá mais liberdade para escolher nomes de identificadores, além de nos permitir
integrar com programas escritos em uma linguagem onde essas palavras não são
palavras-chave. Além disso, identificadores brutos permitem que você use
bibliotecas escritas em uma edição do Rust diferente da que o seu *crate* usa.
Por exemplo, `try` não é uma palavra-chave na edição 2015, mas é nas edições
2018, 2021 e 2024. Se você depender de uma biblioteca escrita usando a edição
2015 e que tenha uma função `try`, precisará usar a sintaxe de identificador
bruto, `r#try` neste caso, para chamar essa função a partir do seu código em
edições posteriores. Consulte o [Apêndice E][appendix-e]<!-- ignore --> para
mais informações sobre edições.

[appendix-e]: appendix-05-editions.html
