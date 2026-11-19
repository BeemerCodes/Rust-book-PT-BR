<a id="generic-types-traits-and-lifetimes"></a>

# Tipos Genéricos, *Traits* e *Lifetimes*

Toda linguagem de programação tem ferramentas para lidar com eficiência com a duplicação de conceitos. Em Rust, uma dessas ferramentas são os *generics*: substitutos abstratos para tipos concretos ou outras propriedades. Podemos expressar o comportamento de *generics*, ou como eles se relacionam com outros *generics*, sem saber o que estará no lugar deles quando o código for compilado e executado.

Funções podem receber parâmetros de algum tipo genérico, em vez de um tipo concreto como `i32` ou `String`, da mesma forma que recebem parâmetros com valores desconhecidos para executar o mesmo código em múltiplos valores concretos. Na verdade, já usamos *generics* no Capítulo 6 com `Option<T>`, no Capítulo 8 com `Vec<T>` e `HashMap<K, V>`, e no Capítulo 9 com `Result<T, E>`. Neste capítulo, você vai explorar como definir seus próprios tipos, funções e métodos com *generics*!

Primeiro, vamos revisar como extrair uma função para reduzir duplicação de código. Em seguida, usaremos a mesma técnica para criar uma função genérica a partir de duas funções que diferem apenas nos tipos dos seus parâmetros. Também vamos explicar como usar tipos genéricos nas definições de *struct* e *enum*.

Depois, você vai aprender a usar *traits* para definir comportamento de forma genérica. Você pode combinar *traits* com tipos genéricos para restringir um tipo genérico a aceitar apenas tipos que tenham um comportamento específico, em vez de qualquer tipo.

Por fim, vamos falar sobre *lifetimes*: uma variedade de *generics* que fornece ao compilador informações sobre como as referências se relacionam entre si. *Lifetimes* nos permitem dar ao compilador informações suficientes sobre valores emprestados (*borrowed*) para que ele possa garantir que as referências serão válidas em mais situações do que poderia sem nossa ajuda.

<a id="removing-duplication-by-extracting-a-function"></a>

## Eliminando Duplicação com Extração de Função

*Generics* nos permitem substituir tipos específicos por um placeholder que representa múltiplos tipos, eliminando duplicação de código. Antes de mergulhar na sintaxe de *generics*, vamos primeiro ver como eliminar duplicação sem usar tipos genéricos, extraindo uma função que substitui valores específicos por um placeholder que representa múltiplos valores. Depois, vamos aplicar a mesma técnica para extrair uma função genérica! Ao aprender a identificar código duplicado que pode ser extraído para uma função, você também vai começar a reconhecer código duplicado que pode se beneficiar de *generics*.

Vamos começar com o programa curto da Listagem 10-1, que encontra o maior número em uma lista.

<Listing number="10-1" file-name="src/main.rs" caption="Encontrando o maior número em uma lista de números">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-01/src/main.rs:here}}
```

</Listing>

Armazenamos uma lista de inteiros na variável `number_list` e colocamos uma referência ao primeiro número da lista em uma variável chamada `largest`. Em seguida, iteramos por todos os números da lista e, se o número atual for maior do que o número armazenado em `largest`, substituímos a referência nessa variável. Porém, se o número atual for menor ou igual ao maior número visto até agora, a variável não muda e o código avança para o próximo número da lista. Depois de considerar todos os números da lista, `largest` deve apontar para o maior número, que neste caso é 100.

Agora fomos encarregados de encontrar o maior número em duas listas de números diferentes. Para isso, podemos duplicar o código da Listagem 10-1 e usar a mesma lógica em dois lugares diferentes do programa, como mostrado na Listagem 10-2.

<Listing number="10-2" file-name="src/main.rs" caption="Código para encontrar o maior número em *duas* listas de números">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-02/src/main.rs}}
```

</Listing>

Embora esse código funcione, duplicar código é tedioso e sujeito a erros. Também precisamos lembrar de atualizar o código em vários lugares quando quisermos alterá-lo.

Para eliminar essa duplicação, vamos criar uma abstração definindo uma função que opera em qualquer lista de inteiros passada como parâmetro. Essa solução torna o código mais claro e nos permite expressar o conceito de encontrar o maior número em uma lista de forma abstrata.

Na Listagem 10-3, extraímos o código que encontra o maior número para uma função chamada `largest`. Em seguida, chamamos essa função para encontrar o maior número nas duas listas da Listagem 10-2. Poderíamos também usar a função em qualquer outra lista de valores `i32` que viermos a ter no futuro.

<Listing number="10-3" file-name="src/main.rs" caption="Código abstraído para encontrar o maior número em duas listas">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-03/src/main.rs:here}}
```

</Listing>

A função `largest` tem um parâmetro chamado `list`, que representa qualquer *slice* concreto de valores `i32` que possamos passar para a função. Como resultado, quando chamamos a função, o código é executado com os valores específicos que passamos.

Em resumo, aqui estão os passos que tomamos para transformar o código da Listagem 10-2 na Listagem 10-3:

1. Identificar o código duplicado.
1. Extrair o código duplicado para o corpo da função e especificar as entradas e os valores de retorno desse código na assinatura da função.
1. Atualizar as duas ocorrências do código duplicado para chamar a função em vez disso.

A seguir, vamos usar esses mesmos passos com *generics* para reduzir a duplicação de código. Da mesma forma que o corpo da função pode operar em um `list` abstrato em vez de valores específicos, *generics* permitem que o código opere em tipos abstratos.

Por exemplo, imagine que temos duas funções: uma que encontra o maior item em um *slice* de valores `i32` e outra que encontra o maior item em um *slice* de valores `char`. Como eliminaríamos essa duplicação? Vamos descobrir!
