<a id="processing-a-series-of-items-with-iterators"></a>
## Processando uma Série de Itens com Iteradores

O padrão de iterador permite realizar alguma tarefa sobre uma sequência de itens, um por vez. Um iterador é responsável pela lógica de iterar sobre cada item e determinar quando a sequência chegou ao fim. Quando você usa iteradores, não precisa reimplementar essa lógica manualmente.

Em Rust, os iteradores são *preguiçosos* (*lazy*), o que significa que eles não têm efeito nenhum até que você chame métodos que consumam o iterador para utilizá-lo. Por exemplo, o código da Listagem 13-10 cria um iterador sobre os itens do vetor `v1` chamando o método `iter` definido em `Vec<T>`. Esse código por si só não faz nada de útil.

<Listing number="13-10" file-name="src/main.rs" caption="Criando um iterador">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-10/src/main.rs:here}}
```

</Listing>

O iterador é armazenado na variável `v1_iter`. Depois de criar um iterador, podemos usá-lo de várias maneiras. Na Listagem 3-5, iteramos sobre um array usando um laço `for` para executar algum código em cada um de seus itens. Por baixo dos panos, isso criava e consumia um iterador implicitamente, mas até agora não tínhamos entrado nos detalhes de como isso funciona.

No exemplo da Listagem 13-11, separamos a criação do iterador do seu uso no laço `for`. Quando o laço `for` é executado usando o iterador em `v1_iter`, cada elemento do iterador é usado em uma iteração do laço, imprimindo cada valor.

<Listing number="13-11" file-name="src/main.rs" caption="Usando um iterador em um laço `for`">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-11/src/main.rs:here}}
```

</Listing>

Em linguagens que não oferecem iteradores na biblioteca padrão, você provavelmente escreveria essa mesma funcionalidade iniciando uma variável no índice 0, usando essa variável para indexar o vetor e obter um valor, e incrementando a variável em um laço até atingir o número total de itens do vetor.

Os iteradores cuidam de toda essa lógica para você, eliminando código repetitivo onde é fácil errar. Eles também dão mais flexibilidade para usar a mesma lógica com vários tipos diferentes de sequências, não apenas estruturas de dados indexáveis como vetores. Vamos ver como os iteradores fazem isso.

<a id="the-iterator-trait-and-the-next-method"></a>
### O Trait `Iterator` e o Método `next`

Todos os iteradores implementam um *trait* chamado `Iterator`, definido na biblioteca padrão. A definição do *trait* é assim:

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```

Note que essa definição usa uma sintaxe nova: `type Item` e `Self::Item`, que definem um tipo associado a esse *trait*. Falaremos em detalhes sobre tipos associados no Capítulo 20. Por enquanto, tudo o que você precisa saber é que esse código diz que implementar o *trait* `Iterator` exige que você também defina um tipo `Item`, e esse tipo `Item` é usado no tipo de retorno do método `next`. Em outras palavras, `Item` será o tipo retornado pelo iterador.

O *trait* `Iterator` exige que quem o implementa defina apenas um método: o método `next`, que retorna um item do iterador por vez, envolvido em `Some`, e, quando a iteração termina, retorna `None`.

Podemos chamar o método `next` diretamente em iteradores; a Listagem 13-12 demonstra quais valores são retornados por chamadas repetidas a `next` no iterador criado a partir do vetor.

<Listing number="13-12" file-name="src/lib.rs" caption="Chamando o método `next` em um iterador">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-12/src/lib.rs:here}}
```

</Listing>

Note que precisamos tornar `v1_iter` mutável: chamar o método `next` em um iterador altera o estado interno que ele usa para saber em que ponto da sequência está. Em outras palavras, esse código *consome* o iterador. Cada chamada a `next` consome um item do iterador. Não precisamos tornar `v1_iter` mutável quando usamos um laço `for` porque o laço assumiu a *ownership* de `v1_iter` e o tornou mutável por baixo dos panos.

Note também que os valores obtidos pelas chamadas a `next` são referências imutáveis para os valores no vetor. O método `iter` produz um iterador sobre referências imutáveis. Se quisermos criar um iterador que assuma a *ownership* de `v1` e retorne valores próprios, podemos chamar `into_iter` em vez de `iter`. Da mesma forma, se quisermos iterar sobre referências mutáveis, podemos chamar `iter_mut` em vez de `iter`.

<a id="methods-that-consume-the-iterator"></a>
### Métodos que Consomem o Iterador

O *trait* `Iterator` possui vários métodos diferentes com implementações padrão fornecidas pela biblioteca padrão; você pode conhecê-los consultando a documentação da API da biblioteca padrão para o *trait* `Iterator`. Alguns desses métodos chamam `next` em sua definição, e é por isso que você precisa implementar `next` ao implementar o *trait* `Iterator`.

Métodos que chamam `next` são chamados de *adaptadores de consumo* (*consuming adapters*), porque chamá-los esgota o iterador. Um exemplo é o método `sum`, que assume a *ownership* do iterador e percorre os itens chamando `next` repetidamente, consumindo o iterador. Conforme itera, ele soma cada item a um total acumulado e retorna esse total quando a iteração termina. A Listagem 13-13 traz um teste que ilustra o uso do método `sum`.

<Listing number="13-13" file-name="src/lib.rs" caption="Chamando o método `sum` para obter o total de todos os itens no iterador">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-13/src/lib.rs:here}}
```

</Listing>

Não podemos usar `v1_iter` depois da chamada a `sum`, porque `sum` assume a *ownership* do iterador sobre o qual é chamado.

<a id="methods-that-produce-other-iterators"></a>
### Métodos que Produzem Outros Iteradores

*Adaptadores de iterador* (*iterator adapters*) são métodos definidos no *trait* `Iterator` que não consomem o iterador. Em vez disso, eles produzem iteradores diferentes alterando algum aspecto do iterador original.

A Listagem 13-14 mostra um exemplo chamando o adaptador de iterador `map`, que recebe uma *closure* para chamar em cada item conforme eles são percorridos. O método `map` retorna um novo iterador que produz os itens modificados. Aqui, a *closure* cria um novo iterador em que cada item do vetor será incrementado em 1.

<Listing number="13-14" file-name="src/main.rs" caption="Chamando o adaptador de iterador `map` para criar um novo iterador">

```rust,not_desired_behavior
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-14/src/main.rs:here}}
```

</Listing>

No entanto, esse código gera um aviso (*warning*):

```console
{{#include ../listings/ch13-functional-features/listing-13-14/output.txt}}
```

O código da Listagem 13-14 não faz nada; a *closure* que especificamos nunca chega a ser chamada. O aviso nos lembra o motivo: adaptadores de iterador são preguiçosos, e precisamos consumir o iterador aqui.

Para corrigir esse aviso e consumir o iterador, usaremos o método `collect`, que usamos com `env::args` na Listagem 12-1. Esse método consome o iterador e reúne os valores resultantes em um tipo de dados de coleção.

Na Listagem 13-15, coletamos em um vetor os resultados da iteração sobre o iterador retornado pela chamada a `map`. Esse vetor terminará contendo cada item do vetor original incrementado em 1.

<Listing number="13-15" file-name="src/main.rs" caption="Chamando o método `map` para criar um novo iterador e depois chamando `collect` para consumir o novo iterador e criar um vetor">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-15/src/main.rs:here}}
```

</Listing>

Como `map` recebe uma *closure*, podemos especificar qualquer operação que quisermos executar em cada item. Esse é um ótimo exemplo de como *closures* permitem personalizar comportamentos enquanto reutilizamos o mecanismo de iteração fornecido pelo *trait* `Iterator`.

Você pode encadear várias chamadas a adaptadores de iterador para realizar ações complexas de maneira legível. Mas, como todos os iteradores são preguiçosos, você precisa chamar um dos métodos adaptadores de consumo para obter resultados das chamadas aos adaptadores de iterador.

<!-- Old headings. Do not remove or links may break. -->

<a id="using-closures-that-capture-their-environment"></a>

<a id="closures-that-capture-their-environment"></a>
### Closures que Capturam o Ambiente

Muitos adaptadores de iterador recebem *closures* como argumentos e, comumente, essas *closures* capturam valores do ambiente.

Para este exemplo, vamos usar o método `filter`, que recebe uma *closure*. A *closure* recebe um item do iterador e retorna um `bool`. Se a *closure* retornar `true`, o valor será incluído no iterador produzido por `filter`. Se retornar `false`, o valor não será incluído.

Na Listagem 13-16, usamos `filter` com uma *closure* que captura a variável `shoe_size` do ambiente para iterar sobre uma coleção de instâncias da *struct* `Shoe`. A função retornará apenas os calçados do tamanho especificado.

<Listing number="13-16" file-name="src/lib.rs" caption="Usando o método `filter` com uma *closure* que captura `shoe_size`">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-16/src/lib.rs}}
```

</Listing>

A função `shoes_in_size` assume a *ownership* de um vetor de calçados e de um tamanho como parâmetros, retornando um vetor contendo apenas os calçados do tamanho especificado.

No corpo de `shoes_in_size`, chamamos `into_iter` para criar um iterador que assume a *ownership* do vetor. Em seguida, chamamos `filter` para adaptar esse iterador em um novo iterador que contém apenas os elementos para os quais a *closure* retorna `true`.

A *closure* captura o parâmetro `shoe_size` do ambiente e compara o valor com o tamanho de cada calçado, mantendo apenas os do tamanho desejado. Por fim, chamar `collect` reúne os valores retornados pelo iterador adaptado em um vetor que é retornado pela função.

O teste mostra que, quando chamamos `shoes_in_size`, recebemos de volta apenas os calçados que têm o mesmo tamanho que o valor especificado.
