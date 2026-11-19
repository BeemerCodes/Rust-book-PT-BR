<a id="variables-and-mutability"></a>
## Variáveis e Mutabilidade

Como mencionado na seção ["Armazenando Valores com Variáveis"][storing-values-with-variables]<!-- ignore -->, por padrão, as variáveis são imutáveis. Esse é um dos vários incentivos que o Rust oferece para que você escreva seu código de modo a aproveitar a segurança e a facilidade de concorrência que o Rust proporciona. No entanto, você ainda tem a opção de tornar suas variáveis mutáveis. Vamos explorar como e por que o Rust incentiva você a preferir a imutabilidade e por que, às vezes, você pode querer abrir mão dela.

Quando uma variável é imutável, uma vez que um valor é vinculado a um nome, você não pode alterar esse valor. Para ilustrar isso, gere um novo projeto chamado _variables_ no seu diretório _projects_ usando `cargo new variables`.

Em seguida, no seu novo diretório _variables_, abra _src/main.rs_ e substitua seu código pelo código a seguir, que ainda não vai compilar:

<span class="filename">Arquivo: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/src/main.rs}}
```

Salve e execute o programa usando `cargo run`. Você deverá receber uma mensagem de erro referente a um erro de imutabilidade, como mostrado nesta saída:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/output.txt}}
```

Este exemplo mostra como o compilador ajuda você a encontrar erros nos seus programas. Erros de compilação podem ser frustrantes, mas na verdade eles apenas significam que seu programa ainda não está fazendo com segurança o que você quer que ele faça; eles _não_ significam que você não é um bom programador! Rustaceans experientes continuam recebendo erros do compilador.

Você recebeu a mensagem de erro `` cannot assign twice to immutable variable `x` `` porque tentou atribuir um segundo valor à variável imutável `x`.

É importante recebermos erros em tempo de compilação quando tentamos alterar um valor designado como imutável, porque exatamente essa situação pode levar a bugs. Se uma parte do nosso código opera sob a premissa de que um valor nunca mudará e outra parte do código altera esse valor, é possível que a primeira parte não faça aquilo para o qual foi projetada. A causa desse tipo de bug pode ser difícil de rastrear depois do ocorrido, especialmente quando a segunda parte do código altera o valor apenas _às vezes_. O compilador Rust garante que, quando você afirma que um valor não vai mudar, ele realmente não vai mudar, então você não precisa ficar acompanhando isso por conta própria. Seu código se torna, assim, mais fácil de raciocinar.

Mas a mutabilidade pode ser muito útil e tornar o código mais conveniente de escrever. Embora as variáveis sejam imutáveis por padrão, você pode torná-las mutáveis adicionando `mut` na frente do nome da variável, como fez no [Capítulo 2][storing-values-with-variables]<!-- ignore -->. Adicionar `mut` também comunica intenção aos futuros leitores do código, indicando que outras partes do código irão alterar o valor dessa variável.

Por exemplo, vamos alterar _src/main.rs_ para o seguinte:

<span class="filename">Arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/src/main.rs}}
```

Quando executamos o programa agora, obtemos isto:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/output.txt}}
```

Podemos alterar o valor vinculado a `x` de `5` para `6` quando `mut` é usado. No fim das contas, decidir se deve usar mutabilidade ou não depende de você e do que você considera mais claro naquela situação específica.

<!-- Old headings. Do not remove or links may break. -->
<a id="constants"></a>

<a id="declaring-constants"></a>
### Declarando Constantes

Assim como variáveis imutáveis, _constantes_ são valores vinculados a um nome que não podem ser alterados, mas existem algumas diferenças entre constantes e variáveis.

Primeiro, você não tem permissão para usar `mut` com constantes. Constantes não são apenas imutáveis por padrão — elas são sempre imutáveis. Você declara constantes usando a palavra-chave `const` em vez de `let`, e o tipo do valor _deve_ ser anotado. Abordaremos tipos e anotações de tipo na próxima seção, ["Tipos de Dados"][data-types]<!-- ignore -->, então não se preocupe com os detalhes agora. Apenas saiba que você sempre precisa anotar o tipo.

Constantes podem ser declaradas em qualquer escopo, incluindo o escopo global, o que as torna úteis para valores que muitas partes do código precisam conhecer.

A última diferença é que constantes só podem ser definidas como uma expressão constante, e não como o resultado de um valor que só poderia ser calculado em tempo de execução.

Aqui está um exemplo de declaração de constante:

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

O nome da constante é `THREE_HOURS_IN_SECONDS`, e seu valor é definido como o resultado da multiplicação de 60 (o número de segundos em um minuto) por 60 (o número de minutos em uma hora) por 3 (o número de horas que queremos contar neste programa). A convenção de nomenclatura do Rust para constantes é usar tudo em maiúsculas com sublinhados entre as palavras. O compilador consegue avaliar um conjunto limitado de operações em tempo de compilação, o que nos permite escrever esse valor de uma forma mais fácil de entender e verificar, em vez de definir a constante diretamente como 10.800. Consulte a [seção da Referência do Rust sobre avaliação de constantes][const-eval] para mais informações sobre quais operações podem ser usadas ao declarar constantes.

Constantes são válidas durante todo o tempo em que um programa é executado, dentro do escopo em que foram declaradas. Essa propriedade torna as constantes úteis para valores no domínio da sua aplicação que várias partes do programa podem precisar conhecer, como o número máximo de pontos que qualquer jogador pode ganhar ou a velocidade da luz.

Nomear valores fixos (*hardcoded*) usados ao longo do seu programa como constantes é útil para transmitir o significado desse valor aos futuros mantenedores do código. Também ajuda ter apenas um lugar no código que você precisaria alterar caso o valor fixo precise ser atualizado no futuro.

<a id="shadowing"></a>
### Sombreamento (*Shadowing*)

Como você viu no tutorial do jogo de adivinhação no [Capítulo 2][comparing-the-guess-to-the-secret-number]<!-- ignore -->, você pode declarar uma nova variável com o mesmo nome de uma variável anterior. Os Rustaceans dizem que a primeira variável é _sombreamento_ (*shadowed*) pela segunda, o que significa que a segunda variável é a que o compilador verá quando você usar o nome da variável. Na prática, a segunda variável ofusca a primeira, assumindo quaisquer usos do nome da variável até que ela própria seja sombreada ou o escopo termine. Podemos sombrear uma variável usando o mesmo nome e repetindo o uso da palavra-chave `let`, da seguinte forma:

<span class="filename">Arquivo: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/src/main.rs}}
```

Este programa primeiro vincula `x` ao valor `5`. Em seguida, cria uma nova variável `x` repetindo `let x =`, pegando o valor original e somando `1`, de modo que o valor de `x` passa a ser `6`. Depois, dentro de um escopo interno criado com as chaves, a terceira instrução `let` também sombreia `x` e cria uma nova variável, multiplicando o valor anterior por `2` para dar a `x` o valor `12`. Quando esse escopo termina, o sombreamento interno acaba e `x` volta a ser `6`. Quando executarmos este programa, ele produzirá a seguinte saída:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/output.txt}}
```

O sombreamento é diferente de marcar uma variável como `mut` porque receberemos um erro em tempo de compilação se tentarmos reatribuir acidentalmente essa variável sem usar a palavra-chave `let`. Ao usar `let`, podemos realizar algumas transformações em um valor, mas manter a variável imutável após a conclusão dessas transformações.

A outra diferença entre `mut` e o sombreamento é que, como estamos efetivamente criando uma nova variável quando usamos a palavra-chave `let` novamente, podemos alterar o tipo do valor reutilizando o mesmo nome. Por exemplo, suponha que nosso programa peça ao usuário para indicar quantos espaços deseja entre um texto digitando caracteres de espaço, e depois queiramos armazenar essa entrada como um número:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-04-shadowing-can-change-types/src/main.rs:here}}
```

A primeira variável `spaces` é de um tipo string, e a segunda variável `spaces` é de um tipo numérico. O sombreamento nos poupa de ter que inventar nomes diferentes, como `spaces_str` e `spaces_num`; em vez disso, podemos reutilizar o nome mais simples `spaces`. No entanto, se tentarmos usar `mut` para isso, como mostrado aqui, receberemos um erro em tempo de compilação:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/src/main.rs:here}}
```

O erro diz que não temos permissão para alterar o tipo de uma variável:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/output.txt}}
```

Agora que exploramos como as variáveis funcionam, vamos ver mais tipos de dados que elas podem ter.

[comparing-the-guess-to-the-secret-number]: ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[data-types]: ch03-02-data-types.html#data-types
[storing-values-with-variables]: ch02-00-guessing-game-tutorial.html#storing-values-with-variables
[const-eval]: ../reference/const_eval.html
