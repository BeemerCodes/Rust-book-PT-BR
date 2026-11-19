<a id="references-and-borrowing"></a>
## Referências e Borrowing

O problema com o código da tupla na Listagem 4-5 é que precisamos devolver a `String` para a função chamadora para que ainda possamos usar a `String` após a chamada a `calculate_length`, porque a `String` foi movida para dentro de `calculate_length`. Em vez disso, podemos fornecer uma referência ao valor `String`. Uma referência é como um ponteiro, pois é um endereço que podemos seguir para acessar os dados armazenados naquele endereço; esses dados pertencem a alguma outra variável. Diferente de um ponteiro, uma referência tem a garantia de apontar para um valor válido de um tipo específico durante toda a vida dessa referência.

Veja como você definiria e usaria uma função `calculate_length` que tem uma referência a um objeto como parâmetro em vez de tomar *ownership* do valor:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-07-reference/src/main.rs:all}}
```

</Listing>

Primeiro, observe que todo o código de tupla na declaração de variável e no valor de retorno da função desapareceu. Segundo, observe que passamos `&s1` para `calculate_length` e, na sua definição, recebemos `&String` em vez de `String`. Esses caracteres `&` (e comercial) representam referências e permitem que você se refira a algum valor sem tomar *ownership* dele. A Figura 4-6 ilustra esse conceito.

<img alt="Três tabelas: a tabela para s contém apenas um ponteiro para a tabela de s1. A tabela de s1 contém os dados na stack para s1 e aponta para os dados da string na heap." src="img/trpl04-06.svg" class="center" />

<span class="caption">Figura 4-6: Um diagrama de `&String` `s` apontando para `String` `s1`</span>

> Nota: O oposto de referenciar usando `&` é _desreferenciar_ (*dereferencing*), o que é feito com o operador de desreferência `*`. Veremos alguns usos do operador de desreferência no Capítulo 8 e discutiremos detalhes sobre desreferência no Capítulo 15.

Vamos olhar mais de perto a chamada de função aqui:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-07-reference/src/main.rs:here}}
```

A sintaxe `&s1` nos permite criar uma referência que _aponta_ para o valor de `s1`, mas não é dona dele. Como a referência não tem *ownership*, o valor para o qual ela aponta não será descartado (*dropped*) quando a referência deixar de ser usada.

Da mesma forma, a assinatura da função usa `&` para indicar que o tipo do parâmetro `s` é uma referência. Vamos adicionar algumas anotações explicativas:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-08-reference-with-annotations/src/main.rs:here}}
```

O escopo no qual a variável `s` é válida é o mesmo escopo de qualquer parâmetro de função, mas o valor apontado pela referência não é descartado quando `s` deixa de ser usada, porque `s` não tem *ownership*. Quando funções recebem referências como parâmetros em vez dos valores reais, não precisamos retornar os valores para devolver *ownership*, porque nunca tivemos *ownership*.

Chamamos a ação de criar uma referência de _empréstimo_ (*borrowing*). Assim como na vida real, se uma pessoa é dona de algo, você pode pegar emprestado dela. Quando terminar, precisa devolver. Você não é o dono.

Então, o que acontece se tentarmos modificar algo que estamos pegando emprestado? Experimente o código da Listagem 4-6. Alerta de spoiler: não funciona!

<Listing number="4-6" file-name="src/main.rs" caption="Tentando modificar um valor emprestado">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-06/src/main.rs}}
```

</Listing>

Aqui está o erro:

```console
{{#include ../listings/ch04-understanding-ownership/listing-04-06/output.txt}}
```

Assim como as variáveis são imutáveis por padrão, as referências também são. Não temos permissão para modificar algo ao qual temos apenas uma referência imutável.

<a id="mutable-references"></a>
### Referências Mutáveis

Podemos corrigir o código da Listagem 4-6 para nos permitir modificar um valor emprestado com apenas alguns pequenos ajustes que usam, em vez disso, uma _referência mutável_:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-09-fixes-listing-04-06/src/main.rs}}
```

</Listing>

Primeiro, mudamos `s` para ser `mut`. Em seguida, criamos uma referência mutável com `&mut s` onde chamamos a função `change` e atualizamos a assinatura da função para aceitar uma referência mutável com `some_string: &mut String`. Isso deixa bem claro que a função `change` vai mutar o valor que ela pega emprestado.

Referências mutáveis têm uma grande restrição: se você tiver uma referência mutável para um valor, não poderá ter nenhuma outra referência para esse valor. Este código que tenta criar duas referências mutáveis para `s` falhará:

<Listing file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-10-multiple-mut-not-allowed/src/main.rs:here}}
```

</Listing>

Aqui está o erro:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-10-multiple-mut-not-allowed/output.txt}}
```

Este erro diz que o código é inválido porque não podemos pegar `s` emprestado como mutável mais de uma vez ao mesmo tempo. O primeiro empréstimo mutável está em `r1` e precisa durar até ser usado no `println!`, mas entre a criação dessa referência mutável e seu uso, tentamos criar outra referência mutável em `r2` que pega emprestado os mesmos dados que `r1`.

A restrição que impede múltiplas referências mutáveis para os mesmos dados ao mesmo tempo permite mutação, mas de maneira muito controlada. É algo com que novos Rustaceans costumam ter dificuldade porque a maioria das linguagens permite mutar quando você quiser. O benefício dessa restrição é que o Rust consegue evitar condições de corrida de dados (*data races*) em tempo de compilação. Um _data race_ é semelhante a uma condição de corrida e acontece quando estes três comportamentos ocorrem:

- Dois ou mais ponteiros acessam os mesmos dados ao mesmo tempo.
- Pelo menos um dos ponteiros está sendo usado para escrever nos dados.
- Não há nenhum mecanismo sendo usado para sincronizar o acesso aos dados.

*Data races* causam comportamento indefinido e podem ser difíceis de diagnosticar e corrigir quando você tenta rastreá-los em tempo de execução; o Rust evita esse problema recusando-se a compilar código com *data races*!

Como sempre, podemos usar chaves para criar um novo escopo, permitindo múltiplas referências mutáveis, apenas não _simultâneas_:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-11-muts-in-separate-scopes/src/main.rs:here}}
```

O Rust aplica uma regra semelhante para combinar referências mutáveis e imutáveis. Este código resulta em erro:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-12-immutable-and-mutable-not-allowed/src/main.rs:here}}
```

Aqui está o erro:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-12-immutable-and-mutable-not-allowed/output.txt}}
```

Ufa! Também _não_ podemos ter uma referência mutável enquanto temos uma imutável para o mesmo valor.

Usuários de uma referência imutável não esperam que o valor mude repentinamente sem aviso! No entanto, múltiplas referências imutáveis são permitidas porque ninguém que esteja apenas lendo os dados tem a capacidade de afetar a leitura dos dados por outra pessoa.

Note que o escopo de uma referência começa onde ela é introduzida e continua até a última vez em que essa referência é usada. Por exemplo, este código compilará porque o último uso das referências imutáveis está no `println!`, antes de a referência mutável ser introduzida:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-13-reference-scope-ends/src/main.rs:here}}
```

Os escopos das referências imutáveis `r1` e `r2` terminam após o `println!` onde são usadas pela última vez, o que ocorre antes de a referência mutável `r3` ser criada. Esses escopos não se sobrepõem, então este código é permitido: o compilador consegue perceber que a referência não está mais sendo usada em um ponto antes do final do escopo.

Mesmo que erros de *borrowing* possam ser frustrantes às vezes, lembre-se de que é o compilador Rust apontando um possível bug cedo (em tempo de compilação em vez de tempo de execução) e mostrando exatamente onde está o problema. Assim, você não precisa investigar por que seus dados não são o que você pensava que fossem.

<a id="dangling-references"></a>
### Referências Pendentes (*Dangling References*)

Em linguagens com ponteiros, é fácil criar erroneamente um _ponteiro pendente_ (*dangling pointer*) — um ponteiro que referencia um local na memória que pode ter sido entregue a outra parte do programa — liberando alguma memória enquanto mantém um ponteiro para essa memória. Em Rust, por contraste, o compilador garante que referências nunca serão referências pendentes: se você tiver uma referência para algum dado, o compilador garantirá que o dado não sairá de escopo antes da referência para ele.

Vamos tentar criar uma referência pendente para ver como o Rust as impede com um erro em tempo de compilação:

<Listing file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-14-dangling-reference/src/main.rs}}
```

</Listing>

Aqui está o erro:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-14-dangling-reference/output.txt}}
```

Esta mensagem de erro se refere a um recurso que ainda não abordamos: *lifetimes*. Discutiremos *lifetimes* em detalhes no Capítulo 10. Mas, se você desconsiderar as partes sobre *lifetimes*, a mensagem contém a chave do porquê esse código é um problema:

```text
this function's return type contains a borrowed value, but there is no value
for it to be borrowed from
```

Vamos olhar mais de perto o que exatamente está acontecendo em cada etapa do nosso código `dangle`:

<Listing file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-15-dangling-reference-annotated/src/main.rs:here}}
```

</Listing>

Como `s` é criada dentro de `dangle`, quando o código de `dangle` terminar, `s` será desalocada. Mas tentamos retornar uma referência para ela. Isso significa que essa referência apontaria para uma `String` inválida. Isso não é nada bom! O Rust não nos permite fazer isso.

A solução aqui é retornar a `String` diretamente:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-16-no-dangle/src/main.rs:here}}
```

Isso funciona sem problemas. O *ownership* é movido para fora, e nada é desalocado.

<a id="the-rules-of-references"></a>
### As Regras das Referências

Vamos recapitular o que discutimos sobre referências:

- Em qualquer momento, você pode ter _ou_ uma referência mutável _ou_ qualquer número de referências imutáveis.
- Referências devem ser sempre válidas.

A seguir, veremos um tipo diferente de referência: *slices*.
