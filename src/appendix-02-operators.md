<a id="appendix-b-operators-and-symbols"></a>
## Apêndice B: Operadores e Símbolos

Este apêndice contém um glossário da sintaxe do Rust, incluindo operadores e
outros símbolos que aparecem sozinhos ou no contexto de caminhos, genéricos,
*trait bounds*, *macros*, atributos, comentários, tuplas e colchetes.

<a id="operators"></a>
### Operadores

A Tabela B-1 contém os operadores em Rust, um exemplo de como o operador aparece
no contexto, uma breve explicação e se esse operador pode ser sobrecarregado. Se
um operador puder ser sobrecarregado, o *trait* relevante a ser usado para
sobrecarregá-lo é listado.

<span class="caption">Tabela B-1: Operadores</span>

| Operador                  | Exemplo                                                 | Explicação                                                              | Sobrecarregável? |
| ------------------------- | ------------------------------------------------------- | ----------------------------------------------------------------------- | ---------------- |
| `!`                       | `ident!(...)`, `ident!{...}`, `ident![...]`             | Expansão de *macro*                                                     |                  |
| `!`                       | `!expr`                                                 | Complemento bit a bit ou lógico                                         | `Not`            |
| `!=`                      | `expr != expr`                                          | Comparação de desigualdade                                              | `PartialEq`      |
| `%`                       | `expr % expr`                                           | Resto aritmético                                                        | `Rem`            |
| `%=`                      | `var %= expr`                                           | Resto aritmético e atribuição                                           | `RemAssign`      |
| `&`                       | `&expr`, `&mut expr`                                    | Empréstimo (*borrowing*)                                                |                  |
| `&`                       | `&type`, `&mut type`, `&'a type`, `&'a mut type`        | Tipo de ponteiro emprestado                                             |                  |
| `&`                       | `expr & expr`                                           | E (AND) bit a bit                                                       | `BitAnd`         |
| `&=`                      | `var &= expr`                                           | E (AND) bit a bit e atribuição                                          | `BitAndAssign`   |
| `&&`                      | `expr && expr`                                          | E (AND) lógico com curto-circuito                                       |                  |
| `*`                       | `expr * expr`                                           | Multiplicação aritmética                                                | `Mul`            |
| `*=`                      | `var *= expr`                                           | Multiplicação aritmética e atribuição                                   | `MulAssign`      |
| `*`                       | `*expr`                                                 | Desreferência                                                           | `Deref`          |
| `*`                       | `*const type`, `*mut type`                              | Ponteiro bruto                                                          |                  |
| `+`                       | `trait + trait`, `'a + trait`                           | Restrição de tipo composta                                              |                  |
| `+`                       | `expr + expr`                                           | Adição aritmética                                                       | `Add`            |
| `+=`                      | `var += expr`                                           | Adição aritmética e atribuição                                          | `AddAssign`      |
| `,`                       | `expr, expr`                                            | Separador de argumentos e elementos                                     |                  |
| `-`                       | `- expr`                                                | Negação aritmética                                                      | `Neg`            |
| `-`                       | `expr - expr`                                           | Subtração aritmética                                                    | `Sub`            |
| `-=`                      | `var -= expr`                                           | Subtração aritmética e atribuição                                       | `SubAssign`      |
| `->`                      | `fn(...) -> type`, <code>&vert;...&vert; -> type</code> | Tipo de retorno de função e *closure*                                   |                  |
| `.`                       | `expr.ident`                                            | Acesso a campo                                                          |                  |
| `.`                       | `expr.ident(expr, ...)`                                 | Chamada de método                                                       |                  |
| `.`                       | `expr.0`, `expr.1` e assim por diante                   | Indexação de tupla                                                      |                  |
| `..`                      | `..`, `expr..`, `..expr`, `expr..expr`                  | Literal de intervalo exclusivo à direita                                | `PartialOrd`     |
| `..=`                     | `..=expr`, `expr..=expr`                                | Literal de intervalo inclusivo à direita                                | `PartialOrd`     |
| `..`                      | `..expr`                                                | Sintaxe de atualização de literal de *struct*                           |                  |
| `..`                      | `variant(x, ..)`, `struct_type { x, .. }`               | Vinculação de padrão “e o restante”                                     |                  |
| `...`                     | `expr...expr`                                           | (Obsoleto, use `..=` em vez disso) Em um padrão: padrão de intervalo inclusivo |           |
| `/`                       | `expr / expr`                                           | Divisão aritmética                                                      | `Div`            |
| `/=`                      | `var /= expr`                                           | Divisão aritmética e atribuição                                         | `DivAssign`      |
| `:`                       | `pat: type`, `ident: type`                              | Restrições de tipo                                                      |                  |
| `:`                       | `ident: expr`                                           | Inicializador de campo de *struct*                                      |                  |
| `:`                       | `'a: loop {...}`                                        | Rótulo de laço                                                          |                  |
| `;`                       | `expr;`                                                 | Terminador de instrução e item                                          |                  |
| `;`                       | `[...; len]`                                            | Parte da sintaxe de array de tamanho fixo                               |                  |
| `<<`                      | `expr << expr`                                          | Deslocamento à esquerda                                                 | `Shl`            |
| `<<=`                     | `var <<= expr`                                          | Deslocamento à esquerda e atribuição                                    | `ShlAssign`      |
| `<`                       | `expr < expr`                                           | Comparação “menor que”                                                  | `PartialOrd`     |
| `<=`                      | `expr <= expr`                                          | Comparação “menor ou igual a”                                           | `PartialOrd`     |
| `=`                       | `var = expr`, `ident = type`                            | Atribuição/equivalência                                                 |                  |
| `==`                      | `expr == expr`                                          | Comparação de igualdade                                                 | `PartialEq`      |
| `=>`                      | `pat => expr`                                           | Parte da sintaxe de braço de `match`                                    |                  |
| `>`                       | `expr > expr`                                           | Comparação “maior que”                                                  | `PartialOrd`     |
| `>=`                      | `expr >= expr`                                          | Comparação “maior ou igual a”                                           | `PartialOrd`     |
| `>>`                      | `expr >> expr`                                          | Deslocamento à direita                                                  | `Shr`            |
| `>>=`                     | `var >>= expr`                                          | Deslocamento à direita e atribuição                                     | `ShrAssign`      |
| `@`                       | `ident @ pat`                                           | Vinculação de padrão                                                    |                  |
| `^`                       | `expr ^ expr`                                           | OU exclusivo (XOR) bit a bit                                            | `BitXor`         |
| `^=`                      | `var ^= expr`                                           | OU exclusivo (XOR) bit a bit e atribuição                               | `BitXorAssign`   |
| <code>&vert;</code>       | <code>pat &vert; pat</code>                             | Alternativas de padrão                                                  |                  |
| <code>&vert;</code>       | <code>expr &vert; expr</code>                           | OU (OR) bit a bit                                                       | `BitOr`          |
| <code>&vert;=</code>      | <code>var &vert;= expr</code>                           | OU (OR) bit a bit e atribuição                                          | `BitOrAssign`    |
| <code>&vert;&vert;</code> | <code>expr &vert;&vert; expr</code>                     | OU (OR) lógico com curto-circuito                                       |                  |
| `?`                       | `expr?`                                                 | Propagação de erro                                                      |                  |

<a id="non-operator-symbols"></a>
### Símbolos que Não São Operadores

As tabelas a seguir contêm todos os símbolos que não funcionam como operadores;
ou seja, não se comportam como uma chamada de função ou método.

A Tabela B-2 mostra símbolos que aparecem por conta própria e são válidos em
vários locais.

<span class="caption">Tabela B-2: Sintaxe Autônoma</span>

| Símbolo                                                                     | Explicação                                                                        |
| --------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| `'ident`                                                                    | *Lifetime* nomeado ou rótulo de laço                                              |
| Dígitos imediatamente seguidos por `u8`, `i32`, `f64`, `usize` e assim por diante | Literal numérico de tipo específico                                          |
| `"..."`                                                                     | Literal de string                                                                 |
| `r"..."`, `r#"..."#`, `r##"..."##` e assim por diante                        | Literal de string bruta; caracteres de escape não são processados                 |
| `b"..."`                                                                    | Literal de string de bytes; constrói um array de bytes em vez de uma string       |
| `br"..."`, `br#"..."#`, `br##"..."##` e assim por diante                     | Literal de string bruta de bytes; combinação de string bruta e string de bytes    |
| `'...'`                                                                     | Literal de caractere                                                              |
| `b'...'`                                                                    | Literal de byte ASCII                                                             |
| <code>&vert;...&vert; expr</code>                                           | *Closure*                                                                         |
| `!`                                                                         | Tipo *bottom* sempre vazio para funções divergentes                               |
| `_`                                                                         | Vinculação de padrão “ignorado”; também usado para tornar literais inteiros legíveis |

A Tabela B-3 mostra símbolos que aparecem no contexto de um caminho através da
hierarquia de módulos até um item.

<span class="caption">Tabela B-3: Sintaxe Relacionada a Caminhos</span>

| Símbolo                                 | Explicação                                                                                                           |
| --------------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| `ident::ident`                          | Caminho de *namespace*                                                                                               |
| `::path`                                | Caminho relativo à raiz do *crate* (ou seja, um caminho explicitamente absoluto)                                      |
| `self::path`                            | Caminho relativo ao módulo atual (ou seja, um caminho explicitamente relativo)                                       |
| `super::path`                           | Caminho relativo ao pai do módulo atual                                                                              |
| `type::ident`, `<type as trait>::ident` | Constantes, funções e tipos associados                                                                               |
| `<type>::...`                           | Item associado a um tipo que não pode ser nomeado diretamente (por exemplo, `<&T>::...`, `<[T]>::...` e assim por diante) |
| `trait::method(...)`                    | Desambiguação de chamada de método nomeando o *trait* que o define                                                   |
| `type::method(...)`                     | Desambiguação de chamada de método nomeando o tipo para o qual está definido                                         |
| `<type as trait>::method(...)`          | Desambiguação de chamada de método nomeando o *trait* e o tipo                                                       |

A Tabela B-4 mostra símbolos que aparecem no contexto do uso de parâmetros de
tipo genérico.

<span class="caption">Tabela B-4: Genéricos</span>

| Símbolo                        | Explicação                                                                                                                                      |
| ------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| `path<...>`                    | Especifica parâmetros para um tipo genérico em um tipo (por exemplo, `Vec<u8>`)                                                                 |
| `path::<...>`, `method::<...>` | Especifica parâmetros para um tipo genérico, função ou método em uma expressão; frequentemente chamado de _turbofish_ (por exemplo, `"42".parse::<i32>()`) |
| `fn ident<...> ...`            | Define uma função genérica                                                                                                                      |
| `struct ident<...> ...`        | Define uma *struct* genérica                                                                                                                    |
| `enum ident<...> ...`          | Define um *enum* genérico                                                                                                                       |
| `impl<...> ...`                | Define uma implementação genérica                                                                                                               |
| `for<...> type`                | Limites (*bounds*) de *lifetime* de ordem superior                                                                                              |
| `type<ident=type>`             | Um tipo genérico em que um ou mais tipos associados têm atribuições específicas (por exemplo, `Iterator<Item=T>`)                                 |

A Tabela B-5 mostra símbolos que aparecem no contexto de restringir parâmetros
de tipo genérico com *trait bounds*.

<span class="caption">Tabela B-5: Restrições de Trait Bound</span>

| Símbolo                       | Explicação                                                                                                                                               |
| ----------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `T: U`                        | Parâmetro genérico `T` restrito a tipos que implementam `U`                                                                                              |
| `T: 'a`                       | Tipo genérico `T` deve sobreviver ao *lifetime* `'a` (o que significa que o tipo não pode conter transitivamente referências com *lifetimes* menores que `'a`) |
| `T: 'static`                  | Tipo genérico `T` não contém referências emprestadas além de referências `'static`                                                                       |
| `'b: 'a`                      | *Lifetime* genérico `'b` deve sobreviver ao *lifetime* `'a`                                                                                              |
| `T: ?Sized`                   | Permite que o parâmetro de tipo genérico seja um tipo de tamanho dinâmico                                                                                |
| `'a + trait`, `trait + trait` | Restrição de tipo composta                                                                                                                               |

A Tabela B-6 mostra símbolos que aparecem no contexto de chamar ou definir
*macros* e especificar atributos em um item.

<span class="caption">Tabela B-6: Macros e Atributos</span>

| Símbolo                                    | Explicação               |
| ------------------------------------------ | ------------------------ |
| `#[meta]`                                  | Atributo externo         |
| `#![meta]`                                 | Atributo interno         |
| `$ident`                                   | Substituição de *macro*  |
| `$ident:kind`                              | Metavariável de *macro*  |
| `$(...)...`                                | Repetição de *macro*     |
| `ident!(...)`, `ident!{...}`, `ident![...]` | Invocação de *macro*     |

A Tabela B-7 mostra símbolos que criam comentários.

<span class="caption">Tabela B-7: Comentários</span>

| Símbolo    | Explicação                              |
| ---------- | --------------------------------------- |
| `//`       | Comentário de linha                     |
| `//!`      | Comentário de documentação interno de linha |
| `///`      | Comentário de documentação externo de linha |
| `/*...*/`  | Comentário de bloco                     |
| `/*!...*/` | Comentário de documentação interno de bloco |
| `/**...*/` | Comentário de documentação externo de bloco |

A Tabela B-8 mostra os contextos em que parênteses são usados.

<span class="caption">Tabela B-8: Parênteses</span>

| Símbolo           | Explicação                                                                                        |
| ----------------- | ------------------------------------------------------------------------------------------------- |
| `()`              | Tupla vazia (também conhecida como *unit*), tanto literal quanto tipo                             |
| `(expr)`          | Expressão entre parênteses                                                                        |
| `(expr,)`         | Expressão de tupla com um único elemento                                                          |
| `(type,)`         | Tipo de tupla com um único elemento                                                               |
| `(expr, ...)`     | Expressão de tupla                                                                                |
| `(type, ...)`     | Tipo de tupla                                                                                     |
| `expr(expr, ...)` | Expressão de chamada de função; também usada para inicializar *structs* tupla e variantes de *enum* tupla |

A Tabela B-9 mostra os contextos em que chaves são usadas.

<span class="caption">Tabela B-9: Chaves</span>

| Contexto     | Explicação           |
| ------------ | -------------------- |
| `{...}`      | Expressão de bloco   |
| `Type {...}` | Literal de *struct*  |

A Tabela B-10 mostra os contextos em que colchetes são usados.

<span class="caption">Tabela B-10: Colchetes</span>

| Contexto                                           | Explicação                                                                                                                           |
| -------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `[...]`                                            | Literal de array                                                                                                                     |
| `[expr; len]`                                      | Literal de array contendo `len` cópias de `expr`                                                                                     |
| `[type; len]`                                      | Tipo array contendo `len` instâncias de `type`                                                                                       |
| `expr[expr]`                                       | Indexação de coleção; sobrecarregável (`Index`, `IndexMut`)                                                                          |
| `expr[..]`, `expr[a..]`, `expr[..b]`, `expr[a..b]` | Indexação de coleção simulando fatiamento (*slicing*) de coleção, usando `Range`, `RangeFrom`, `RangeTo` ou `RangeFull` como “índice” |
