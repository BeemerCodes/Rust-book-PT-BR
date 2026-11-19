<a id="unsafe-rust"></a>
## Rust *Unsafe*

Todo o código que discutimos até agora teve as garantias de segurança de memória
do Rust impostas em tempo de compilação. No entanto, o Rust tem uma segunda
linguagem escondida dentro dele que não impõe essas garantias de segurança de
memória: ela é chamada de *Rust unsafe* e funciona exatamente como o Rust comum,
mas nos dá superpoderes extras.

O Rust *unsafe* existe porque, por natureza, a análise estática é conservadora.
Quando o compilador tenta determinar se um código mantém as garantias ou não, é
melhor que ele rejeite alguns programas válidos do que aceite alguns programas
inválidos. Embora o código *possa* estar correto, se o compilador Rust não
tiver informações suficientes para ter certeza, ele rejeitará o código. Nesses
casos, você pode usar código *unsafe* para dizer ao compilador: “Confie em mim,
eu sei o que estou fazendo.” Esteja avisado, porém, de que você usa Rust
*unsafe* por sua conta e risco: se usar código *unsafe* incorretamente, podem
ocorrer problemas devido à insegurança de memória, como a desreferenciação de
ponteiros nulos.

Outro motivo pelo qual o Rust tem um alter ego *unsafe* é que o hardware
subjacente do computador é inerentemente inseguro. Se o Rust não deixasse você
realizar operações inseguras, você não conseguiria fazer certas tarefas. O Rust
precisa permitir que você faça programação de sistemas de baixo nível, como
interagir diretamente com o sistema operacional ou até escrever seu próprio
sistema operacional. Trabalhar com programação de sistemas de baixo nível é um
dos objetivos da linguagem. Vamos explorar o que podemos fazer com Rust *unsafe*
e como fazê-lo.

<!-- Old headings. Do not remove or links may break. -->

<a id="unsafe-superpowers"></a>
<a id="performing-unsafe-superpowers"></a>

### Executando Superpoderes *Unsafe*

Para alternar para o Rust *unsafe*, use a palavra-chave `unsafe` e inicie um
novo bloco que contém o código *unsafe*. Você pode realizar cinco ações em Rust
*unsafe* que não pode em Rust seguro (*safe Rust*), as quais chamamos de
*superpoderes unsafe*. Esses superpoderes incluem a capacidade de:

1. Desreferenciar um ponteiro bruto (*raw pointer*).
1. Chamar uma função ou método *unsafe*.
1. Acessar ou modificar uma variável estática mutável.
1. Implementar uma *trait* *unsafe*.
1. Acessar campos de `union`s.

É importante entender que `unsafe` não desativa o *borrow checker* nem
desabilita nenhuma das outras verificações de segurança do Rust: se você usar
uma referência dentro de código *unsafe*, ela ainda será verificada. A
palavra-chave `unsafe` apenas dá acesso a esses cinco recursos que então não
são verificados pelo compilador quanto à segurança de memória. Você ainda terá
algum grau de segurança dentro de um bloco `unsafe`.

Além disso, `unsafe` não significa que o código dentro do bloco seja
necessariamente perigoso ou que definitivamente terá problemas de segurança de
memória: a intenção é que, como programador, você garanta que o código dentro
de um bloco `unsafe` acessará a memória de forma válida.

Pessoas são falíveis e erros acontecem, mas ao exigir que essas cinco
operações inseguras estejam dentro de blocos anotados com `unsafe`, você saberá
que quaisquer erros relacionados à segurança de memória devem estar dentro de
um bloco `unsafe`. Mantenha os blocos `unsafe` pequenos; você agradecerá mais
tarde quando estiver investigando bugs de memória.

Para isolar código *unsafe* o máximo possível, o ideal é encapsular esse código
em uma abstração segura e fornecer uma API segura, o que discutiremos mais
adiante no capítulo ao examinarmos funções e métodos *unsafe*. Partes da
biblioteca padrão são implementadas como abstrações seguras sobre código
*unsafe* que foi auditado. Envolver código *unsafe* em uma abstração segura
evita que o uso de `unsafe` vaze para todos os lugares onde você ou seus
usuários possam querer usar a funcionalidade implementada com código *unsafe*,
porque usar uma abstração segura é seguro.

Vamos analisar cada um dos cinco superpoderes *unsafe* por vez. Também veremos
algumas abstrações que fornecem uma interface segura para código *unsafe*.

<a id="dereferencing-a-raw-pointer"></a>
### Desreferenciando um Ponteiro Bruto

No Capítulo 4, na seção [“Referências Pendentes”][dangling-references]<!-- ignore
-->, mencionamos que o compilador garante que as referências sejam sempre
válidas. O Rust *unsafe* tem dois novos tipos chamados *ponteiros brutos*
(*raw pointers*) que são semelhantes às referências. Assim como as referências,
ponteiros brutos podem ser imutáveis ou mutáveis e são escritos como `*const T`
e `*mut T`, respectivamente. O asterisco não é o operador de desreferenciação;
ele faz parte do nome do tipo. No contexto de ponteiros brutos, *imutável*
significa que o ponteiro não pode receber uma atribuição diretamente após ser
desreferenciado.

Diferentemente das referências e dos *smart pointers*, os ponteiros brutos:

- Podem ignorar as regras de *borrowing* tendo tanto ponteiros imutáveis quanto
  mutáveis, ou múltiplos ponteiros mutáveis, apontando para o mesmo local
- Não têm garantia de apontar para memória válida
- Podem ser nulos
- Não implementam nenhuma limpeza automática

Ao abrir mão de fazer o Rust impor essas garantias, você pode abdicar da
segurança garantida em troca de maior desempenho ou da capacidade de fazer
interface com outra linguagem ou hardware onde as garantias do Rust não se
aplicam.

A Listagem 20-1 mostra como criar um ponteiro bruto imutável e um mutável.

<Listing number="20-1" caption="Criando ponteiros brutos com os operadores de *borrow* bruto">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-01/src/main.rs:here}}
```

</Listing>

Note que não incluímos a palavra-chave `unsafe` neste código. Podemos criar
ponteiros brutos em código seguro; só não podemos desreferenciar ponteiros
brutos fora de um bloco `unsafe`, como você verá daqui a pouco.

Criamos ponteiros brutos usando os operadores de *borrow* bruto: `&raw const num`
cria um ponteiro bruto imutável `*const i32`, e `&raw mut num` cria um ponteiro
bruto mutável `*mut i32`. Como os criamos diretamente a partir de uma variável
local, sabemos que esses ponteiros brutos em particular são válidos, mas não
podemos fazer essa suposição sobre qualquer ponteiro bruto.

Para demonstrar isso, a seguir criaremos um ponteiro bruto de cuja validade não
podemos ter tanta certeza, usando a palavra-chave `as` para converter um valor
em vez de usar o operador de *borrow* bruto. A Listagem 20-2 mostra como criar
um ponteiro bruto para um local arbitrário na memória. Tentar usar memória
arbitrária é indefinido: pode haver dados naquele endereço ou não, o compilador
pode otimizar o código de modo que não haja acesso à memória, ou o programa pode
terminar com uma falha de segmentação (*segmentation fault*). Normalmente, não há
um bom motivo para escrever código assim, especialmente em casos em que você
pode usar um operador de *borrow* bruto em seu lugar, mas é possível.

<Listing number="20-2" caption="Criando um ponteiro bruto para um endereço de memória arbitrário">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-02/src/main.rs:here}}
```

</Listing>

Lembre-se de que podemos criar ponteiros brutos em código seguro, mas não
podemos desreferenciar ponteiros brutos e ler os dados para os quais apontam. Na
Listagem 20-3, usamos o operador de desreferenciação `*` em um ponteiro bruto, o
que requer um bloco `unsafe`.

<Listing number="20-3" caption="Desreferenciando ponteiros brutos dentro de um bloco `unsafe`">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-03/src/main.rs:here}}
```

</Listing>

Criar um ponteiro não faz mal algum; é apenas quando tentamos acessar o valor
para o qual ele aponta que podemos acabar lidando com um valor inválido.

Note também que, nas Listagens 20-1 e 20-3, criamos ponteiros brutos `*const i32`
e `*mut i32` que apontavam ambos para o mesmo local de memória, onde `num` está
armazenado. Se, em vez disso, tentássemos criar uma referência imutável e uma
mutável para `num`, o código não teria compilado porque as regras de *ownership*
do Rust não permitem uma referência mutável ao mesmo tempo que quaisquer
referências imutáveis. Com ponteiros brutos, podemos criar um ponteiro mutável e
um ponteiro imutável para o mesmo local e alterar dados por meio do ponteiro
mutável, potencialmente criando uma corrida de dados (*data race*). Tenha
cuidado!

Com todos esses perigos, por que você usaria ponteiros brutos? Um grande caso de
uso é ao fazer interface com código C, como você verá na próxima seção. Outro
caso é ao construir abstrações seguras que o *borrow checker* não entende.
Apresentaremos funções *unsafe* e depois veremos um exemplo de uma abstração
segura que usa código *unsafe*.

<a id="calling-an-unsafe-function-or-method"></a>
### Chamando uma Função ou Método *Unsafe*

O segundo tipo de operação que você pode executar em um bloco `unsafe` é chamar
funções *unsafe*. Funções e métodos *unsafe* têm exatamente a mesma aparência que
funções e métodos comuns, mas têm um `unsafe` extra antes do restante da
definição. A palavra-chave `unsafe` neste contexto indica que a função tem
requisitos que precisamos cumprir quando a chamamos, porque o Rust não pode
garantir que atendemos a esses requisitos. Ao chamar uma função *unsafe* dentro
de um bloco `unsafe`, estamos dizendo que lemos a documentação dessa função e
assumimos a responsabilidade de cumprir os contratos da função.

Aqui está uma função *unsafe* chamada `dangerous` que não faz nada no seu corpo:

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-01-unsafe-fn/src/main.rs:here}}
```

Devemos chamar a função `dangerous` dentro de um bloco `unsafe` separado. Se
tentarmos chamar `dangerous` sem o bloco `unsafe`, receberemos um erro:

```console
{{#include ../listings/ch20-advanced-features/output-only-01-missing-unsafe/output.txt}}
```

Com o bloco `unsafe`, estamos afirmando ao Rust que lemos a documentação da
função, entendemos como usá-la corretamente e verificamos que estamos cumprindo
o contrato da função.

Para executar operações *unsafe* no corpo de uma função `unsafe`, você ainda
precisa usar um bloco `unsafe`, assim como dentro de uma função comum, e o
compilador o avisará se você esquecer. Isso nos ajuda a manter os blocos
`unsafe` os menores possíveis, pois operações *unsafe* podem não ser necessárias
em todo o corpo da função.

<a id="creating-a-safe-abstraction-over-unsafe-code"></a>
#### Criando uma Abstração Segura sobre Código *Unsafe*

Só porque uma função contém código *unsafe* não significa que precisamos marcar
a função inteira como *unsafe*. Na verdade, envolver código *unsafe* em uma
função segura é uma abstração comum. Como exemplo, vamos estudar a função
`split_at_mut` da biblioteca padrão, que requer um pouco de código *unsafe*.
Vamos explorar como poderíamos implementá-la. Esse método seguro é definido em
*slices* mutáveis: ele recebe um *slice* e o transforma em dois, dividindo o
*slice* no índice passado como argumento. A Listagem 20-4 mostra como usar
`split_at_mut`.

<Listing number="20-4" caption="Usando a função segura `split_at_mut`">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-04/src/main.rs:here}}
```

</Listing>

Não conseguimos implementar essa função usando apenas Rust seguro. Uma
tentativa poderia se parecer com a Listagem 20-5, que não compila. Para
simplificar, implementaremos `split_at_mut` como uma função em vez de um método,
e apenas para *slices* de valores `i32` em vez de um tipo genérico `T`.

<Listing number="20-5" caption="Uma tentativa de implementação de `split_at_mut` usando apenas Rust seguro">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-05/src/main.rs:here}}
```

</Listing>

Essa função primeiro obtém o comprimento total do *slice*. Em seguida, ela
verifica por asserção (*assert*) que o índice passado como parâmetro está dentro
do *slice*, checando se é menor ou igual ao comprimento. A asserção significa
que, se passarmos um índice maior que o comprimento para dividir o *slice*, a
função entrará em pânico antes de tentar usar esse índice.

Então, retornamos dois *slices* mutáveis em uma tupla: um do início do *slice*
original até o índice `mid` e outro de `mid` até o final do *slice*.

Quando tentamos compilar o código na Listagem 20-5, obtemos um erro:

```console
{{#include ../listings/ch20-advanced-features/listing-20-05/output.txt}}
```

O *borrow checker* do Rust não consegue entender que estamos pegando emprestadas
partes diferentes do *slice*; ele apenas sabe que estamos fazendo *borrowing* do
mesmo *slice* duas vezes. Fazer *borrowing* de partes diferentes de um *slice* é
fundamentalmente aceitável porque os dois *slices* não se sobrepõem, mas o Rust
não é inteligente o bastante para saber disso. Quando sabemos que o código está
certo, mas o Rust não sabe, é hora de recorrer a código *unsafe*.

A Listagem 20-6 mostra como usar um bloco `unsafe`, um ponteiro bruto e algumas
chamadas a funções *unsafe* para fazer a implementação de `split_at_mut`
funcionar.

<Listing number="20-6" caption="Usando código *unsafe* na implementação da função `split_at_mut`">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-06/src/main.rs:here}}
```

</Listing>

Lembre-se da seção [“O Tipo *Slice*”][the-slice-type]<!-- ignore --> no
Capítulo 4 que um *slice* é um ponteiro para alguns dados e o comprimento do
*slice*. Usamos o método `len` para obter o comprimento de um *slice* e o método
`as_mut_ptr` para acessar o ponteiro bruto de um *slice*. Neste caso, como temos
um *slice* mutável para valores `i32`, `as_mut_ptr` retorna um ponteiro bruto
com o tipo `*mut i32`, que armazenamos na variável `ptr`.

Mantemos a asserção de que o índice `mid` está dentro do *slice*. Em seguida,
chegamos ao código *unsafe*: a função `slice::from_raw_parts_mut` recebe um
ponteiro bruto e um comprimento, e cria um *slice*. Usamos essa função para
criar um *slice* que começa em `ptr` e tem `mid` itens de comprimento. Depois,
chamamos o método `add` em `ptr` com `mid` como argumento para obter um ponteiro
bruto que começa em `mid`, e criamos um *slice* usando esse ponteiro e o número
restante de itens após `mid` como comprimento.

A função `slice::from_raw_parts_mut` é *unsafe* porque recebe um ponteiro bruto
e precisa confiar que esse ponteiro é válido. O método `add` em ponteiros
brutos também é *unsafe* porque precisa confiar que o local deslocado também é
um ponteiro válido. Portanto, tivemos que colocar um bloco `unsafe` ao redor
das nossas chamadas a `slice::from_raw_parts_mut` e `add` para podermos
chamá-las. Ao analisar o código e adicionar a asserção de que `mid` deve ser
menor ou igual a `len`, podemos garantir que todos os ponteiros brutos usados
dentro do bloco `unsafe` serão ponteiros válidos para dados dentro do *slice*.
Esse é um uso aceitável e apropriado de `unsafe`.

Note que não precisamos marcar a função resultante `split_at_mut` como `unsafe`,
e podemos chamar essa função a partir de Rust seguro. Criamos uma abstração
segura para o código *unsafe* com uma implementação da função que usa código
`unsafe` de maneira segura, pois ela cria apenas ponteiros válidos a partir dos
dados aos quais essa função tem acesso.

Em contraste, o uso de `slice::from_raw_parts_mut` na Listagem 20-7
provavelmente travaria quando o *slice* fosse usado. Este código pega um local
arbitrário de memória e cria um *slice* com 10.000 itens de comprimento.

<Listing number="20-7" caption="Criando um *slice* a partir de um local arbitrário de memória">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-07/src/main.rs:here}}
```

</Listing>

Não possuímos o *ownership* da memória nesse local arbitrário, e não há garantia
de que o *slice* criado por este código contenha valores `i32` válidos. Tentar
usar `values` como se fosse um *slice* válido resulta em comportamento indefinido.

<a id="using-extern-functions-to-call-external-code"></a>
#### Usando Funções `extern` para Chamar Código Externo

Às vezes, seu código Rust pode precisar interagir com código escrito em outra
linguagem. Para isso, o Rust tem a palavra-chave `extern` que facilita a criação
e o uso de uma *Foreign Function Interface (FFI)* (Interface de Funções
Estrangeiras), que é uma forma de uma linguagem de programação definir funções e
permitir que uma linguagem de programação diferente (estrangeira) chame essas
funções.

A Listagem 20-8 demonstra como configurar uma integração com a função `abs` da
biblioteca padrão do C. Funções declaradas dentro de blocos `extern` são
geralmente inseguras de chamar a partir de código Rust, portanto blocos `extern`
também devem ser marcados como `unsafe`. O motivo é que outras linguagens não
impõem as regras e garantias do Rust, e o Rust não consegue verificá-las, então
a responsabilidade recai sobre o programador para garantir a segurança.

<Listing number="20-8" file-name="src/main.rs" caption="Declarando e chamando uma função `extern` definida em outra linguagem">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-08/src/main.rs}}
```

</Listing>

Dentro do bloco `unsafe extern "C"`, listamos os nomes e as assinaturas das
funções externas de outra linguagem que queremos chamar. A parte `"C"` define
qual *interface binária de aplicação (ABI — application binary interface)* a
função externa usa: a ABI define como chamar a função em nível de assembly. A
ABI `"C"` é a mais comum e segue a ABI da linguagem de programação C. Informações
sobre todas as ABIs que o Rust suporta estão disponíveis na [Referência do
Rust][ABI].

Todo item declarado dentro de um bloco `unsafe extern` é implicitamente
*unsafe*. No entanto, algumas funções FFI *são* seguras de chamar. Por exemplo,
a função `abs` da biblioteca padrão do C não possui considerações de segurança
de memória, e sabemos que ela pode ser chamada com qualquer `i32`. Em casos
como esse, podemos usar a palavra-chave `safe` para dizer que essa função
específica é segura para chamar, mesmo estando dentro de um bloco `unsafe
extern`. Assim que fazemos essa mudança, chamá-la não requer mais um bloco
`unsafe`, como mostra a Listagem 20-9.

<Listing number="20-9" file-name="src/main.rs" caption="Marcando explicitamente uma função como `safe` dentro de um bloco `unsafe extern` e chamando-a com segurança">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-09/src/main.rs}}
```

</Listing>

Marcar uma função como `safe` não a torna inerentemente segura! Em vez disso, é
como uma promessa que você está fazendo ao Rust de que ela é segura. Ainda é
sua responsabilidade garantir que essa promessa seja cumprida!

<a id="calling-rust-functions-from-other-languages"></a>
#### Chamando Funções Rust a Partir de Outras Linguagens

Também podemos usar `extern` para criar uma interface que permita que outras
linguagens chamem funções Rust. Em vez de criar um bloco `extern` inteiro,
adicionamos a palavra-chave `extern` e especificamos a ABI a ser usada logo
antes da palavra-chave `fn` da função relevante. Também precisamos adicionar uma
anotação `#[unsafe(no_mangle)]` para instruir o compilador Rust a não alterar
(*mangle*) o nome dessa função. *Mangling* é quando um compilador altera o nome
que demos a uma função para um nome diferente que contém mais informações para
outras partes do processo de compilação consumirem, mas é menos legível para
humanos. Todo compilador de linguagem de programação faz o *mangling* de nomes
de maneira ligeiramente diferente; portanto, para que uma função Rust possa ser
nomeada por outras linguagens, precisamos desativar o *name mangling* do
compilador Rust. Isso é *unsafe* porque pode haver colisões de nomes entre
bibliotecas sem o *mangling* integrado, então é nossa responsabilidade garantir
que o nome escolhido seja seguro para exportar sem *mangling*.

No exemplo a seguir, tornamos a função `call_from_c` acessível a partir de
código C, depois que ela for compilada para uma biblioteca compartilhada e
vinculada a partir do C:

```
#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
```

Esse uso de `extern` requer `unsafe` apenas no atributo, não no bloco `extern`.

<a id="accessing-or-modifying-a-mutable-static-variable"></a>
### Acessando ou Modificando uma Variável Estática Mutável

Neste livro, ainda não falamos sobre variáveis globais, às quais o Rust dá
suporte, mas que podem ser problemáticas com as regras de *ownership* do Rust.
Se duas *threads* estiverem acessando a mesma variável global mutável, isso pode
causar uma corrida de dados.

Em Rust, variáveis globais são chamadas de variáveis *estáticas* (*static*). A
Listagem 20-10 mostra um exemplo de declaração e uso de uma variável estática
com um *slice* de string como valor.

<Listing number="20-10" file-name="src/main.rs" caption="Definindo e usando uma variável estática imutável">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-10/src/main.rs}}
```

</Listing>

Variáveis estáticas são semelhantes a constantes, sobre as quais discutimos na
seção [“Declarando Constantes”][constants]<!-- ignore --> no Capítulo 3. Os
nomes das variáveis estáticas seguem `SCREAMING_SNAKE_CASE` por convenção.
Variáveis estáticas só podem armazenar referências com o *lifetime* `'static`, o
que significa que o compilador Rust consegue deduzir o *lifetime* e não somos
obrigados a anotá-lo explicitamente. Acessar uma variável estática imutável é
seguro.

Uma diferença sutil entre constantes e variáveis estáticas imutáveis é que os
valores em uma variável estática têm um endereço fixo na memória. Usar o valor
sempre acessará os mesmos dados. Constantes, por outro lado, têm permissão para
duplicar seus dados sempre que são usadas. Outra diferença é que variáveis
estáticas podem ser mutáveis. Acessar e modificar variáveis estáticas mutáveis é
*unsafe*. A Listagem 20-11 mostra como declarar, acessar e modificar uma
variável estática mutável chamada `COUNTER`.

<Listing number="20-11" file-name="src/main.rs" caption="Ler ou escrever em uma variável estática mutável é *unsafe*.">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-11/src/main.rs}}
```

</Listing>

Assim como com variáveis comuns, especificamos a mutabilidade usando a
palavra-chave `mut`. Qualquer código que leia ou escreva em `COUNTER` precisa
estar dentro de um bloco `unsafe`. O código na Listagem 20-11 compila e imprime
`COUNTER: 3` como esperaríamos porque é *single-threaded*. Ter múltiplas
*threads* acessando `COUNTER` provavelmente resultaria em corridas de dados,
sendo portanto comportamento indefinido. Logo, precisamos marcar a função
inteira como `unsafe` e documentar a limitação de segurança para que qualquer
pessoa que chame a função saiba o que pode e o que não pode fazer com segurança.

Sempre que escrevemos uma função *unsafe*, é idiomático escrever um comentário
começando com `SAFETY` explicando o que o chamador precisa fazer para chamar a
função com segurança. Da mesma forma, sempre que realizamos uma operação
*unsafe*, é idiomático escrever um comentário começando com `SAFETY` para
explicar como as regras de segurança são cumpridas.

Além disso, o compilador negará por padrão qualquer tentativa de criar
referências para uma variável estática mutável por meio de um *lint* do
compilador. Você deve optar explicitamente por desativar as proteções desse
*lint* adicionando uma anotação `#[allow(static_mut_refs)]` ou acessar a
variável estática mutável por meio de um ponteiro bruto criado com um dos
operadores de *borrow* bruto. Isso inclui casos em que a referência é criada de
forma invisível, como quando ela é usada no `println!` desta listagem de código.
Exigir que referências a variáveis estáticas mutáveis sejam criadas via
ponteiros brutos ajuda a tornar mais óbvios os requisitos de segurança para
usá-las.

Com dados mutáveis que são globalmente acessíveis, é difícil garantir que não
haja corridas de dados, razão pela qual o Rust considera variáveis estáticas
mutáveis como *unsafe*. Sempre que possível, é preferível usar as técnicas de
concorrência e os *smart pointers* *thread-safe* que discutimos no Capítulo 16,
para que o compilador verifique se o acesso aos dados por diferentes *threads* é
feito com segurança.

<a id="implementing-an-unsafe-trait"></a>
### Implementando uma *Trait* *Unsafe*

Podemos usar `unsafe` para implementar uma *trait* *unsafe*. Uma *trait* é
*unsafe* quando pelo menos um de seus métodos tem alguma invariante que o
compilador não consegue verificar. Declaramos que uma *trait* é `unsafe`
adicionando a palavra-chave `unsafe` antes de `trait` e marcando a implementação
da *trait* como `unsafe` também, como mostrado na Listagem 20-12.

<Listing number="20-12" caption="Definindo e implementando uma *trait* *unsafe*">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-12/src/main.rs:here}}
```

</Listing>

Ao usar `unsafe impl`, estamos prometendo que cumpriremos as invariantes que o
compilador não consegue verificar.

Como exemplo, lembre-se das *traits* marcadoras `Send` e `Sync` que discutimos na
seção [“Concorrência Extensível com `Send` e `Sync`”][send-and-sync]<!-- ignore
--> no Capítulo 16: o compilador implementa essas *traits* automaticamente se
nossos tipos forem compostos inteiramente de outros tipos que implementam `Send`
e `Sync`. Se implementarmos um tipo que contenha um tipo que não implementa
`Send` ou `Sync`, como ponteiros brutos, e quisermos marcar esse tipo como
`Send` ou `Sync`, precisamos usar `unsafe`. O Rust não consegue verificar que
nosso tipo mantém as garantias de que pode ser enviado com segurança entre
*threads* ou acessado a partir de múltiplas *threads*; portanto, precisamos fazer
essas verificações manualmente e indicar isso com `unsafe`.

<a id="accessing-fields-of-a-union"></a>
### Acessando Campos de uma *Union*

A ação final que funciona apenas com `unsafe` é acessar campos de uma *union*.
Uma *union* é semelhante a uma `struct`, mas apenas um campo declarado é usado
em uma instância específica por vez. *Unions* são usadas principalmente para
fazer interface com *unions* em código C. Acessar campos de uma *union* é
*unsafe* porque o Rust não pode garantir o tipo do dado armazenado atualmente na
instância da *union*. Você pode aprender mais sobre *unions* na [Referência do
Rust][unions].

<a id="using-miri-to-check-unsafe-code"></a>
### Usando o Miri para Verificar Código *Unsafe*

Ao escrever código *unsafe*, você pode querer verificar se o que escreveu é
realmente seguro e correto. Uma das melhores maneiras de fazer isso é usar o
Miri, uma ferramenta oficial do Rust para detectar comportamento indefinido.
Enquanto o *borrow checker* é uma ferramenta *estática* que funciona em tempo de
compilação, o Miri é uma ferramenta *dinâmica* que funciona em tempo de
execução. Ele verifica seu código executando seu programa, ou sua suíte de
testes, e detectando quando você viola as regras que ele entende sobre como o
Rust deve funcionar.

Usar o Miri requer uma versão *nightly* do Rust (sobre a qual falamos mais no
[Apêndice G: Como o Rust é Feito e “Rust Nightly”][nightly]<!-- ignore -->). Você
pode instalar tanto uma versão *nightly* do Rust quanto a ferramenta Miri
digitando `rustup +nightly component add miri`. Isso não altera qual versão do
Rust seu projeto usa; apenas adiciona a ferramenta ao seu sistema para que você
possa usá-la quando quiser. Você pode executar o Miri em um projeto digitando
`cargo +nightly miri run` ou `cargo +nightly miri test`.

Para ver um exemplo de quão útil isso pode ser, considere o que acontece quando
o executamos contra a Listagem 20-7.

```console
{{#include ../listings/ch20-advanced-features/listing-20-07/output.txt}}
```

O Miri nos avisa corretamente que estamos convertendo um inteiro em um ponteiro,
o que pode ser um problema, mas o Miri não consegue determinar se existe um
problema porque não sabe como o ponteiro se originou. Em seguida, o Miri retorna
um erro onde a Listagem 20-7 tem comportamento indefinido porque temos um
ponteiro pendente (*dangling pointer*). Graças ao Miri, agora sabemos que há um
risco de comportamento indefinido e podemos pensar em como tornar o código
seguro. Em alguns casos, o Miri pode até fazer recomendações sobre como corrigir
erros.

O Miri não pega tudo o que você pode errar ao escrever código *unsafe*. O Miri é
uma ferramenta de análise dinâmica, então ele só detecta problemas em código que
é efetivamente executado. Isso significa que você precisará usá-lo em conjunto
com boas técnicas de teste para aumentar sua confiança sobre o código *unsafe*
que escreveu. O Miri também não cobre todas as formas possíveis pelas quais seu
código pode ser *unsound* (inconsistente em segurança).

Em outras palavras: se o Miri *detecta* um problema, você sabe que há um bug,
mas o fato de o Miri *não* detectar um bug não significa que não exista um
problema. Ainda assim, ele consegue detectar muita coisa. Experimente executá-lo
nos outros exemplos de código *unsafe* deste capítulo e veja o que ele diz!

Você pode saber mais sobre o Miri no [repositório do projeto no GitHub][miri].

<!-- Old headings. Do not remove or links may break. -->

<a id="when-to-use-unsafe-code"></a>
<a id="using-unsafe-code-correctly"></a>

### Usando Código *Unsafe* Corretamente

Usar `unsafe` para aproveitar um dos cinco superpoderes recém-discutidos não é
errado nem malvisto, mas é mais complicado acertar código `unsafe` porque o
compilador não pode ajudar a manter a segurança de memória. Quando você tiver um
motivo para usar código `unsafe`, pode fazê-lo, e ter a anotação explícita
`unsafe` facilita rastrear a origem de problemas quando eles ocorrem. Sempre
que escrever código *unsafe*, você pode usar o Miri para ajudar a ter mais
confiança de que o código escrito respeita as regras do Rust.

Para uma exploração muito mais profunda sobre como trabalhar eficazmente com
Rust *unsafe*, leia o guia oficial do Rust para `unsafe`, [The
Rustonomicon][nomicon].

[dangling-references]: ch04-02-references-and-borrowing.html#dangling-references
[ABI]: ../reference/items/external-blocks.html#abi
[constants]: ch03-01-variables-and-mutability.html#declaring-constants
[send-and-sync]: ch16-04-extensible-concurrency-sync-and-send.html
[the-slice-type]: ch04-03-slices.html#the-slice-type
[unions]: ../reference/items/unions.html
[miri]: https://github.com/rust-lang/miri
[editions]: appendix-05-editions.html
[nightly]: appendix-07-nightly-rust.html
[nomicon]: https://doc.rust-lang.org/nomicon/
