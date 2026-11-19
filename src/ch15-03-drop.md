<a id="running-code-on-cleanup-with-the-drop-trait"></a>
## Executando Código na Limpeza com a *Trait* `Drop`

A segunda *trait* importante para o padrão de *smart pointer* é `Drop`, que
permite que você personalize o que acontece quando um valor está prestes a sair
de escopo. Você pode fornecer uma implementação da *trait* `Drop` em qualquer
tipo, e esse código pode ser usado para liberar recursos como arquivos ou conexões
de rede.

Estamos apresentando `Drop` no contexto de *smart pointers* porque a
funcionalidade da *trait* `Drop` é quase sempre usada na implementação de um
*smart pointer*. Por exemplo, quando um `Box<T>` é liberado (*dropped*), ele
desalocará o espaço no *heap* para o qual o *box* aponta.

Em algumas linguagens, para certos tipos, o programador deve chamar código para
liberar memória ou recursos toda vez que terminar de usar uma instância desses
tipos. Exemplos incluem identificadores de arquivo, *sockets* e *locks*. Se o
programador esquecer, o sistema pode ficar sobrecarregado e travar. Em Rust, você
pode especificar que um determinado trecho de código seja executado sempre que um
valor sai de escopo, e o compilador inserirá esse código automaticamente. Como
resultado, você não precisa se preocupar em colocar código de limpeza em todo
lugar no programa onde uma instância de um determinado tipo termina de ser
usada — você ainda não vazará recursos!

Você especifica o código a ser executado quando um valor sai de escopo
implementando a *trait* `Drop`. A *trait* `Drop` exige que você implemente um
método chamado `drop` que recebe uma referência mutável a `self`. Para ver quando
o Rust chama `drop`, vamos implementar `drop` com instruções `println!` por
enquanto.

A Listagem 15-14 mostra uma *struct* `CustomSmartPointer` cuja única
funcionalidade personalizada é imprimir `Dropping CustomSmartPointer!` quando a
instância sai de escopo, para mostrar quando o Rust executa o método `drop`.

<Listing number="15-14" file-name="src/main.rs" caption="Uma *struct* `CustomSmartPointer` que implementa a *trait* `Drop` onde colocaríamos nosso código de limpeza">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-14/src/main.rs}}
```

</Listing>

A *trait* `Drop` está incluída no prelúdio, portanto não precisamos trazê-la
para o escopo. Implementamos a *trait* `Drop` em `CustomSmartPointer` e
fornecemos uma implementação para o método `drop` que chama `println!`. O corpo
do método `drop` é onde você colocaria qualquer lógica que quisesse executar
quando uma instância do seu tipo sair de escopo. Estamos imprimindo algum texto
aqui para demonstrar visualmente quando o Rust chamará `drop`.

Em `main`, criamos duas instâncias de `CustomSmartPointer` e depois imprimimos
`CustomSmartPointers created`. No final de `main`, nossas instâncias de
`CustomSmartPointer` sairão de escopo, e o Rust chamará o código que colocamos
no método `drop`, imprimindo nossa mensagem final. Observe que não precisamos
chamar o método `drop` explicitamente.

Quando executarmos este programa, veremos a seguinte saída:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-14/output.txt}}
```

O Rust chamou `drop` automaticamente para nós quando nossas instâncias saíram
de escopo, executando o código que especificamos. As variáveis são liberadas em
ordem inversa à de sua criação, portanto `d` foi liberado antes de `c`. O
objetivo deste exemplo é fornecer um guia visual de como o método `drop` funciona;
normalmente você especificaria o código de limpeza que seu tipo precisa executar
em vez de uma mensagem de impressão.

<!-- Old headings. Do not remove or links may break. -->

<a id="dropping-a-value-early-with-std-mem-drop"></a>

<a id="dropping-a-value-early-with-stdmemdrop"></a>
### Liberando um Valor Antes do Fim do Escopo com `std::mem::drop`

Infelizmente, não é simples desabilitar a funcionalidade automática de `drop`.
Desabilitar `drop` geralmente não é necessário; o objetivo principal da *trait*
`Drop` é justamente ser gerenciada automaticamente. Ocasionalmente, porém, você
pode querer limpar um valor antecipadamente. Um exemplo é quando se usa *smart
pointers* que gerenciam *locks*: você pode querer forçar o método `drop` que
libera o *lock* para que outros códigos no mesmo escopo possam adquirir o *lock*.
O Rust não permite que você chame o método `drop` da *trait* `Drop` manualmente;
em vez disso, você deve chamar a função `std::mem::drop` fornecida pela
biblioteca padrão se quiser forçar que um valor seja liberado antes do final de
seu escopo.

Tentar chamar o método `drop` da *trait* `Drop` manualmente, modificando a
função `main` da Listagem 15-14, não funcionará, como mostrado na Listagem 15-15.

<Listing number="15-15" file-name="src/main.rs" caption="Tentando chamar o método `drop` da *trait* `Drop` manualmente para limpar antecipadamente">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-15/src/main.rs:here}}
```

</Listing>

Quando tentarmos compilar esse código, obteremos este erro:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-15/output.txt}}
```

Essa mensagem de erro afirma que não temos permissão para chamar `drop`
explicitamente. A mensagem de erro usa o termo *destrutor*, que é o termo geral
de programação para uma função que limpa uma instância. Um *destrutor* é análogo
a um *construtor*, que cria uma instância. A função `drop` em Rust é um destrutor
específico.

O Rust não nos permite chamar `drop` explicitamente porque o Rust ainda chamaria
automaticamente `drop` no valor ao final de `main`. Isso causaria um erro de
*double free* porque o Rust tentaria limpar o mesmo valor duas vezes.

Não podemos desabilitar a inserção automática de `drop` quando um valor sai de
escopo, e não podemos chamar o método `drop` explicitamente. Portanto, se
precisamos forçar que um valor seja limpo antecipadamente, usamos a função
`std::mem::drop`.

A função `std::mem::drop` é diferente do método `drop` na *trait* `Drop`. A
chamamos passando como argumento o valor que queremos forçar a ser liberado. A
função está no prelúdio, portanto podemos modificar `main` na Listagem 15-15 para
chamar a função `drop`, como mostrado na Listagem 15-16.

<Listing number="15-16" file-name="src/main.rs" caption="Chamando `std::mem::drop` para liberar explicitamente um valor antes que ele saia de escopo">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-16/src/main.rs:here}}
```

</Listing>

Executar esse código imprimirá o seguinte:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-16/output.txt}}
```

O texto `` Dropping CustomSmartPointer with data `some data`! `` é impresso
entre o texto `CustomSmartPointer created` e `CustomSmartPointer dropped before
the end of main`, mostrando que o código do método `drop` é chamado para liberar
`c` naquele ponto.

Você pode usar o código especificado em uma implementação da *trait* `Drop` de
várias maneiras para tornar a limpeza conveniente e segura: por exemplo, poderia
usá-lo para criar seu próprio alocador de memória! Com a *trait* `Drop` e o
sistema de *ownership* do Rust, você não precisa se lembrar de fazer a limpeza,
porque o Rust faz isso automaticamente.

Você também não precisa se preocupar com problemas decorrentes de limpar
acidentalmente valores ainda em uso: o sistema de *ownership* que garante que as
referências são sempre válidas também garante que `drop` seja chamado apenas uma
vez quando o valor não está mais sendo utilizado.

Agora que examinamos `Box<T>` e algumas características dos *smart pointers*,
vamos ver alguns outros *smart pointers* definidos na biblioteca padrão.
