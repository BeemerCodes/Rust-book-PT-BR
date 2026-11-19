<a id="storing-utf-8-encoded-text-with-strings"></a>

## Armazenando Texto Codificado em UTF-8 com Strings

Conversamos sobre strings no Capítulo 4, mas vamos analisá-las mais a fundo agora.
Novos *Rustaceans* geralmente encontram dificuldades com strings por uma combinação de três
razões: a propensão do Rust de expor possíveis erros, o fato de strings serem uma estrutura de
dados mais complexa do que muitos programadores imaginam, e o UTF-8. Esses fatores combinam-se
de uma forma que pode parecer difícil quando você vem de outras linguagens de programação.

Discutimos strings no contexto de coleções porque strings são
implementadas como uma coleção de bytes, além de alguns métodos para fornecer funcionalidades
úteis quando esses bytes são interpretados como texto. Nesta seção, falaremos sobre
as operações em `String` comuns a todos os tipos de coleções, como
criação, atualização e leitura. Também discutiremos como `String`
difere de outras coleções, especificamente como a indexação em uma `String` é
complicada pelas diferenças entre a forma como humanos e computadores interpretam
dados de `String`.

<!-- Old headings. Do not remove or links may break. -->

<a id="what-is-a-string"></a>

<a id="defining-strings"></a>

### Definindo Strings

Primeiro, vamos definir o que queremos dizer com o termo _string_. O Rust possui apenas um tipo
de string na linguagem principal (*core*), que é o *slice* de string `str`, geralmente visto
em sua forma emprestada, `&str`. No Capítulo 4, falamos sobre *slices* de string,
que são referências a dados de string codificados em UTF-8 armazenados em outro lugar. Literais de
string, por exemplo, são armazenados no binário do programa e são, portanto, *slices* de string.

O tipo `String`, fornecido pela biblioteca padrão do Rust em vez de ser
embutido no núcleo da linguagem, é um tipo de string crescível, mutável, com posse (*owned*)
e codificado em UTF-8. Quando os *Rustaceans* se referem a “strings” em Rust, geralmente
podem estar se referindo tanto a `String` quanto ao *slice* de string `&str`, e não apenas a um
deles. Embora esta seção seja em grande parte sobre `String`, ambos os tipos são
amplamente usados na biblioteca padrão do Rust, e tanto `String` quanto *slices* de string
são codificados em UTF-8.

<a id="creating-a-new-string"></a>

### Criando uma Nova String

Muitas das operações disponíveis em `Vec<T>` também estão disponíveis em `String`,
pois `String` é na verdade implementada como um invólucro (*wrapper*) em torno de um vetor
de bytes, trazendo garantias, restrições e capacidades adicionais. Um exemplo
de função que opera da mesma forma em `Vec<T>` e em `String` é a função `new`
para criar uma instância, mostrada na Listagem 8-11.

<Listing number="8-11" caption="Criando uma nova `String` vazia">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-11/src/main.rs:here}}
```

</Listing>

Essa linha cria uma nova string vazia chamada `s`, na qual podemos então carregar
dados. Frequentemente, temos dados iniciais com os quais desejamos inicializar a
string. Para isso, usamos o método `to_string`, que está disponível em qualquer tipo
que implemente a *trait* `Display`, como é o caso dos literais de string. A Listagem 8-12 mostra
dois exemplos.

<Listing number="8-12" caption="Usando o método `to_string` para criar uma `String` a partir de um literal de string">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-12/src/main.rs:here}}
```

</Listing>

Esse código cria uma string contendo `initial contents`.

Também podemos usar a função `String::from` para criar uma `String` a partir de um literal
de string. O código na Listagem 8-13 é equivalente ao código na Listagem 8-12
que usa `to_string`.

<Listing number="8-13" caption="Usando a função `String::from` para criar uma `String` a partir de um literal de string">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-13/src/main.rs:here}}
```

</Listing>

Como as strings são usadas para tantas coisas, podemos usar diversas APIs genéricas
diferentes para strings, o que nos dá muitas opções. Algumas delas podem parecer
redundantes, mas todas têm seu espaço! Neste caso, `String::from` e
`to_string` fazem a mesma coisa, portanto escolher entre elas é uma questão de estilo e
legibilidade.

Lembre-se de que strings são codificadas em UTF-8, portanto podemos incluir nelas quaisquer dados
devidamente codificados, como mostrado na Listagem 8-14.

<Listing number="8-14" caption="Armazenando saudações em diferentes idiomas em strings">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:here}}
```

</Listing>

Todos esses são valores de `String` válidos.

<a id="updating-a-string"></a>

### Atualizando uma String

Uma `String` pode crescer de tamanho e seu conteúdo pode mudar, assim como o conteúdo
de um `Vec<T>`, caso você adicione mais dados a ela. Além disso, você pode convenientemente
usar o operador `+` ou a macro `format!` para concatenar valores de `String`.

<!-- Old headings. Do not remove or links may break. -->

<a id="appending-to-a-string-with-push_str-and-push"></a>

<a id="appending-with-push_str-or-push"></a>

#### Anexando com `push_str` ou `push`

Podemos fazer uma `String` crescer usando o método `push_str` para anexar um *slice* de string,
como mostrado na Listagem 8-15.

<Listing number="8-15" caption="Anexando um slice de string a uma `String` usando o método `push_str`">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-15/src/main.rs:here}}
```

</Listing>

Depois dessas duas linhas, `s` conterá `foobar`. O método `push_str` recebe um
*slice* de string porque não queremos necessariamente assumir a posse (*ownership*) do
parâmetro. Por exemplo, no código da Listagem 8-16, queremos poder usar
`s2` após anexar seu conteúdo a `s1`.

<Listing number="8-16" caption="Usando um slice de string após anexar seu conteúdo a uma `String`">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-16/src/main.rs:here}}
```

</Listing>

Se o método `push_str` tomasse posse de `s2`, não poderíamos imprimir
seu valor na última linha. No entanto, esse código funciona como esperado!

O método `push` recebe um único caractere como parâmetro e o adiciona à
`String`. A Listagem 8-17 adiciona a letra _l_ a uma `String` usando o método
`push`.

<Listing number="8-17" caption="Adicionando um caractere a um valor de `String` usando `push`">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-17/src/main.rs:here}}
```

</Listing>

Como resultado, `s` conterá `lol`.

<!-- Old headings. Do not remove or links may break. -->

<a id="concatenation-with-the--operator-or-the-format-macro"></a>

<a id="concatenating-with--or-format"></a>

#### Concatenando com `+` ou `format!`

Frequentemente, você vai querer combinar duas strings existentes. Uma forma de fazer isso é usar
o operador `+`, como mostrado na Listagem 8-18.

<Listing number="8-18" caption="Usando o operador `+` para combinar dois valores de `String` em um novo valor de `String`">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-18/src/main.rs:here}}
```

</Listing>

A string `s3` conterá `Hello, world!`. O motivo de `s1` não ser mais
válida após a adição, e o motivo de termos usado uma referência a `s2`, tem a ver
com a assinatura do método chamado quando usamos o operador `+`.
O operador `+` usa o método `add`, cuja assinatura é parecida com esta:

```rust,ignore
fn add(self, s: &str) -> String {
```

Na biblioteca padrão, você verá `add` definido com tipos genéricos e tipos associados.
Aqui, substituímos por tipos concretos, que é o que acontece quando
chamamos esse método com valores de `String`. Discutiremos genéricos no Capítulo 10.
Essa assinatura nos dá as pistas necessárias para entender os detalhes mais sutis
do operador `+`.

Primeiro, `s2` tem um `&`, o que significa que estamos adicionando uma *referência* da segunda
string à primeira string. Isso decorre do parâmetro `s` na função `add`: só podemos adicionar um
*slice* de string a uma `String`; não podemos somar dois valores de `String`. Mas espere—o tipo
de `&s2` é `&String`, e não `&str`, como especificado no segundo parâmetro de `add`. Sendo assim,
por que a Listagem 8-18 compila?

A razão de podermos usar `&s2` na chamada a `add` é que o compilador
consegue coagir o argumento `&String` para `&str`. Ao chamar o método `add`,
o Rust usa uma coerção de desreferenciação (*deref coercion*), que aqui converte `&s2` em `&s2[..]`.
Discutiremos a coerção de desreferenciação mais a fundo no Capítulo 15. Como `add` não assume
a posse do parâmetro `s`, `s2` continuará sendo uma `String` válida após essa operação.

Em segundo lugar, podemos observar na assinatura que `add` assume a posse de `self`,
porque `self` *não* tem um `&`. Isso significa que `s1` na Listagem 8-18 será
movida para dentro da chamada `add` e não será mais válida depois disso. Portanto, embora
`let s3 = s1 + &s2;` pareça copiar ambas as strings e criar uma nova,
essa instrução na verdade toma posse de `s1`, anexa uma cópia do conteúdo
de `s2` e, em seguida, devolve a posse do resultado. Em outras palavras, parece
que ela faz muitas cópias, mas não faz; a implementação é mais eficiente do que copiar.

Se precisarmos concatenar várias strings, o comportamento do operador `+` torna-se deselegante:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-01-concat-multiple-strings/src/main.rs:here}}
```

Nesse ponto, `s` será `tic-tac-toe`. Com todos esses caracteres `+` e `"`,
fica difícil entender o que está acontecendo. Para combinar strings de formas mais complexas,
podemos usar a macro `format!`:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-02-format/src/main.rs:here}}
```

Esse código também define `s` como `tic-tac-toe`. A macro `format!` funciona como
`println!`, mas em vez de imprimir a saída na tela, ela retorna uma
`String` com o conteúdo. A versão do código que usa `format!` é muito
mais fácil de ler, e o código gerado pela macro `format!` usa referências
para não tomar posse de nenhum de seus parâmetros.

<a id="indexing-into-strings"></a>

### Indexação em Strings

Em muitas outras linguagens de programação, acessar caracteres individuais de uma
string referenciando-os por índice é uma operação válida e muito comum. No entanto,
se você tentar acessar partes de uma `String` usando a sintaxe de indexação em Rust,
você receberá um erro. Considere o código inválido na Listagem 8-19.

<Listing number="8-19" caption="Tentando usar a sintaxe de indexação em uma `String`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-19/src/main.rs:here}}
```

</Listing>

Esse código resultará no seguinte erro:

```console
{{#include ../listings/ch08-common-collections/listing-08-19/output.txt}}
```

O erro diz tudo: strings em Rust não suportam indexação. Mas por que não? Para
responder a essa pergunta, precisamos discutir como o Rust armazena strings na memória.

<a id="internal-representation"></a>

#### Representação Interna

Uma `String` é um invólucro sobre um `Vec<u8>`. Vamos examinar algumas de nossas strings de exemplo
corretamente codificadas em UTF-8 da Listagem 8-14. Primeiro, esta:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:spanish}}
```

Nesse caso, `len` será `4`, o que significa que o vetor que armazena a string
`"Hola"` tem 4 bytes de comprimento. Cada uma dessas letras ocupa 1 byte quando codificada em
UTF-8. A linha seguinte, no entanto, pode surpreender você (observe que essa string
começa com a letra cirílica maiúscula _Ze_, não com o número 3):

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:russian}}
```

Se lhe perguntassem qual é o comprimento da string, você poderia responder 12. Na verdade, a
resposta do Rust é 24: essa é a quantidade de bytes necessária para codificar “Здравствуйте” em
UTF-8, já que cada valor escalar Unicode nessa string ocupa 2 bytes de
armazenamento. Consequentemente, um índice nos bytes da string nem sempre corresponderá
a um valor escalar Unicode válido. Para demonstrar, considere este código Rust inválido:

```rust,ignore,does_not_compile
let hello = "Здравствуйте";
let answer = &hello[0];
```

Você já sabe que `answer` não será `З`, a primeira letra. Quando codificado
em UTF-8, o primeiro byte de `З` é `208` e o segundo é `151`, logo pareceria
que `answer` deveria, na verdade, ser `208`, mas `208` não é um caractere válido
por si só. Retornar `208` provavelmente não é o que um usuário gostaria se pedisse
a primeira letra dessa string; no entanto, esse é o único dado que o Rust
possui no índice de byte 0. Os usuários geralmente não querem o valor em bytes retornado, mesmo
que a string contenha apenas caracteres latinos: se `&"hi"[0]` fosse um código válido que
retornasse o valor do byte, retornaria `104`, não `h`.

A resposta, então, é que para evitar retornar um valor inesperado e causar
erros que poderiam não ser descobertos de imediato, o Rust nem sequer compila esse código,
evitando mal-entendidos logo no início do desenvolvimento.

<!-- Old headings. Do not remove or links may break. -->

<a id="bytes-and-scalar-values-and-grapheme-clusters-oh-my"></a>

<a id="bytes-scalar-values-and-grapheme-clusters"></a>

#### Bytes, Valores Escalares e Agrupamentos de Grafemas

Outro ponto sobre UTF-8 é que existem, na verdade, três maneiras relevantes de
enxergar strings da perspectiva do Rust: como bytes, valores escalares e agrupamentos de grafemas
(*grapheme clusters* — a coisa mais próxima do que chamaríamos de _letras_).

Se olharmos para a palavra em híndi “नमस्ते” escrita na escrita devanágari, ela é
armazenada como um vetor de valores `u8` parecido com este:

```text
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
```

São 18 bytes, e é assim que os computadores armazenam esses dados em última instância. Se os considerarmos
como valores escalares Unicode, que é o que o tipo `char` do Rust representa, esses
bytes se parecem com isto:

```text
['न', 'म', 'स', '्', 'त', 'े']
```

Temos seis valores `char` aqui, mas o quarto e o sexto não são letras:
são diacríticos que não fazem sentido sozinhos. Por fim, se os considerarmos
como agrupamentos de grafemas, obteríamos o que uma pessoa chamaria de quatro letras
que formam a palavra em híndi:

```text
["न", "म", "स्", "ते"]
```

O Rust oferece diferentes formas de interpretar os dados brutos de strings armazenados
pelos computadores, para que cada programa possa escolher a interpretação que precisa, independentemente
do idioma humano em que os dados estejam expressos.

Uma última razão pela qual o Rust não permite indexar uma `String` para obter um
caractere é que se espera que operações de indexação sempre levem tempo constante
(O(1)). Mas não é possível garantir esse desempenho com uma `String`,
pois o Rust teria que percorrer o conteúdo desde o início até o
índice para determinar quantos caracteres válidos existem até ali.

<a id="slicing-strings"></a>

### Fatiando Strings

Indexar uma string geralmente não é uma boa ideia porque não é óbvio qual deveria ser o
tipo de retorno da operação de indexação: um byte, um caractere, um agrupamento de grafemas
ou um *slice* de string. Portanto, se você realmente precisa usar
índices para criar *slices* de string, o Rust pede que você seja mais específico.

Em vez de indexar usando `[]` com um único número, você pode usar `[]` com um
intervalo (*range*) para criar um *slice* de string contendo bytes específicos:

```rust
let hello = "Здравствуйте";

let s = &hello[0..4];
```

Aqui, `s` será um `&str` contendo os primeiros 4 bytes da string.
Mencionamos antes que cada um desses caracteres ocupa 2 bytes, o que significa que
`s` será `Зд`.

Se tentássemos fatiar apenas parte dos bytes de um caractere, com algo como
`&hello[0..1]`, o Rust causaria um pânico (*panic*) em tempo de execução da mesma forma que faria se um
índice inválido fosse acessado em um vetor:

```console
{{#include ../listings/ch08-common-collections/output-only-01-not-char-boundary/output.txt}}
```

Tenha muito cuidado ao criar *slices* de string com intervalos, pois fazer
isso pode derrubar o seu programa.

<!-- Old headings. Do not remove or links may break. -->

<a id="methods-for-iterating-over-strings"></a>

<a id="iterating-over-strings"></a>

### Iterando Sobre Strings

A melhor maneira de operar em partes de strings é ser explícito quanto a querer
caracteres ou bytes. Para valores escalares Unicode individuais, use o
método `chars`. Chamar `chars` em “Зд” separa e retorna dois valores do
tipo `char`, e você pode iterar sobre o resultado para acessar cada elemento:

```rust
for c in "Зд".chars() {
    println!("{c}");
}
```

Esse código imprimirá o seguinte:

```text
З
д
```

Como alternativa, o método `bytes` retorna cada byte bruto, o que pode ser
adequado para o seu domínio:

```rust
for b in "Зд".bytes() {
    println!("{b}");
}
```

Esse código imprimirá os 4 bytes que compõem essa string:

```text
208
151
208
180
```

Mas lembre-se sempre de que valores escalares Unicode válidos podem ser formados por mais
de 1 byte.

Obter agrupamentos de grafemas a partir de strings, como na escrita devanágari, é
algo complexo, por isso essa funcionalidade não é fornecida pela biblioteca padrão. *Crates*
estão disponíveis no [crates.io](https://crates.io/)<!-- ignore --> se essa for a
funcionalidade de que você precisa.

<!-- Old headings. Do not remove or links may break. -->

<a id="strings-are-not-so-simple"></a>

<a id="handling-the-complexities-of-strings"></a>

### Lidando com as Complexidades das Strings

Em resumo, strings são complexas. Diferentes linguagens de programação fazem
diferentes escolhas sobre como apresentar essa complexidade ao desenvolvedor. O Rust
optou por fazer do tratamento correto de dados de `String` o comportamento padrão
de todos os programas Rust, o que exige que os desenvolvedores pensem mais sobre
o tratamento de dados UTF-8 logo no início. Esse compromisso expõe mais da complexidade
das strings do que é evidente em outras linguagens, mas evita que você
tenha que lidar com erros envolvendo caracteres não ASCII mais adiante no seu
ciclo de desenvolvimento.

A boa notícia é que a biblioteca padrão oferece muitas funcionalidades construídas
em torno dos tipos `String` e `&str` para ajudar a lidar com essas situações complexas
corretamente. Não deixe de conferir a documentação para métodos muito úteis como
`contains`, para buscas dentro de uma string, e `replace`, para substituir partes de uma
string por outra string.

Vamos passar para algo um pouco menos complexo: os hash maps!
