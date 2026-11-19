<a id="appendix-c-derivable-traits"></a>
## Apêndice C: Traits Deriváveis

Em vários pontos do livro, discutimos o atributo `derive`, que você pode aplicar
à definição de uma *struct* ou de um *enum*. O atributo `derive` gera código que
implementará um *trait* com sua própria implementação padrão no tipo que você
anotou com a sintaxe `derive`.

Neste apêndice, fornecemos uma referência de todos os *traits* da biblioteca
padrão que você pode usar com `derive`. Cada seção aborda:

- Quais operadores e métodos a derivação desse *trait* habilitará
- O que a implementação do *trait* fornecida por `derive` faz
- O que implementar o *trait* significa sobre o tipo
- As condições em que você tem ou não permissão para implementar o *trait*
- Exemplos de operações que exigem o *trait*

Se você quiser um comportamento diferente do fornecido pelo atributo `derive`,
consulte a [documentação da biblioteca padrão](../std/index.html)<!-- ignore -->
para cada *trait* para obter detalhes sobre como implementá-los manualmente.

Os *traits* listados aqui são os únicos definidos pela biblioteca padrão que
podem ser implementados em seus tipos usando `derive`. Outros *traits* definidos
na biblioteca padrão não têm um comportamento padrão sensato, então cabe a você
implementá-los da maneira que fizer sentido para o que está tentando alcançar.

Um exemplo de *trait* que não pode ser derivado é `Display`, que lida com a
formatação para usuários finais. Você deve sempre considerar a maneira
apropriada de exibir um tipo para um usuário final. Quais partes do tipo um
usuário final deve ter permissão para ver? Quais partes ele acharia relevantes?
Qual formato dos dados seria mais relevante para ele? O compilador Rust não tem
essa percepção, portanto não pode fornecer um comportamento padrão apropriado
para você.

A lista de *traits* deriváveis fornecida neste apêndice não é exaustiva:
bibliotecas podem implementar `derive` para seus próprios *traits*, tornando a
lista de *traits* com os quais você pode usar `derive` verdadeiramente aberta.
Implementar `derive` envolve o uso de uma *macro* procedural, o que é abordado
na seção [“Macros `derive` Personalizadas”][custom-derive-macros]<!-- ignore -->
no Capítulo 20.

<a id="debug-for-programmer-output"></a>
### `Debug` para Saída de Programadores

O *trait* `Debug` habilita a formatação de depuração em strings de formatação,
o que você indica adicionando `:?` dentro dos marcadores `{}`.

O *trait* `Debug` permite imprimir instâncias de um tipo para fins de depuração,
de modo que você e outros programadores que usam o seu tipo possam inspecionar
uma instância em um ponto específico da execução de um programa.

O *trait* `Debug` é necessário, por exemplo, no uso da *macro* `assert_eq!`.
Essa *macro* imprime os valores das instâncias passadas como argumentos se a
asserção de igualdade falhar, para que os programadores possam ver por que as
duas instâncias não eram iguais.

<a id="partialeq-and-eq-for-equality-comparisons"></a>
### `PartialEq` e `Eq` para Comparações de Igualdade

O *trait* `PartialEq` permite comparar instâncias de um tipo para verificar
igualdade e habilita o uso dos operadores `==` e `!=`.

Derivar `PartialEq` implementa o método `eq`. Quando `PartialEq` é derivado em
*structs*, duas instâncias são iguais somente se _todos_ os campos forem iguais,
e as instâncias não são iguais se _quaisquer_ campos forem diferentes. Quando
derivado em *enums*, cada variante é igual a si mesma e diferente das outras
variantes.

O *trait* `PartialEq` é necessário, por exemplo, com o uso da *macro*
`assert_eq!`, que precisa ser capaz de comparar duas instâncias de um tipo para
verificar igualdade.

O *trait* `Eq` não possui métodos. Seu propósito é sinalizar que, para todo
valor do tipo anotado, o valor é igual a si mesmo. O *trait* `Eq` só pode ser
aplicado a tipos que também implementam `PartialEq`, embora nem todos os tipos
que implementam `PartialEq` possam implementar `Eq`. Um exemplo disso são os
tipos de número de ponto flutuante: a implementação de números de ponto
flutuante define que duas instâncias do valor “não é um número” (`NaN`) não são
iguais entre si.

Um exemplo de quando `Eq` é necessário é para chaves em um `HashMap<K, V>`, de
modo que o `HashMap<K, V>` possa saber se duas chaves são iguais.

<a id="partialord-and-ord-for-ordering-comparisons"></a>
### `PartialOrd` e `Ord` para Comparações de Ordenação

O *trait* `PartialOrd` permite comparar instâncias de um tipo para fins de
ordenação. Um tipo que implementa `PartialOrd` pode ser usado com os operadores
`<`, `>`, `<=` e `>=`. Você só pode aplicar o *trait* `PartialOrd` a tipos que
também implementam `PartialEq`.

Derivar `PartialOrd` implementa o método `partial_cmp`, que retorna um
`Option<Ordering>` que será `None` quando os valores fornecidos não produzirem
uma ordenação. Um exemplo de valor que não produz uma ordenação, embora a
maioria dos valores desse tipo possa ser comparada, é o valor de ponto flutuante
`NaN`. Chamar `partial_cmp` com qualquer número de ponto flutuante e o valor de
ponto flutuante `NaN` retornará `None`.

Quando derivado em *structs*, `PartialOrd` compara duas instâncias comparando o
valor de cada campo na ordem em que os campos aparecem na definição da *struct*.
Quando derivado em *enums*, variantes do *enum* declaradas anteriormente na
definição do *enum* são consideradas menores do que as variantes listadas
depois.

O *trait* `PartialOrd` é necessário, por exemplo, para o método `gen_range` do
*crate* `rand`, que gera um valor aleatório no intervalo especificado por uma
expressão de intervalo.

O *trait* `Ord` permite saber que, para quaisquer dois valores do tipo anotado,
existirá uma ordenação válida. O *trait* `Ord` implementa o método `cmp`, que
retorna um `Ordering` em vez de um `Option<Ordering>`, porque uma ordenação
válida sempre será possível. Você só pode aplicar o *trait* `Ord` a tipos que
também implementam `PartialOrd` e `Eq` (e `Eq` exige `PartialEq`). Quando
derivado em *structs* e *enums*, `cmp` se comporta da mesma maneira que a
implementação derivada para `partial_cmp` com `PartialOrd`.

Um exemplo de quando `Ord` é necessário é ao armazenar valores em um
`BTreeSet<T>`, uma estrutura de dados que armazena dados com base na ordem de
classificação dos valores.

<a id="clone-and-copy-for-duplicating-values"></a>
### `Clone` e `Copy` para Duplicar Valores

O *trait* `Clone` permite criar explicitamente uma cópia profunda de um valor, e
o processo de duplicação pode envolver a execução de código arbitrário e a cópia
de dados da *heap*. Consulte a seção [“Variáveis e Dados Interagindo com
Clone”][variables-and-data-interacting-with-clone]<!-- ignore --> no Capítulo 4
para mais informações sobre `Clone`.

Derivar `Clone` implementa o método `clone`, que, quando implementado para o
tipo inteiro, chama `clone` em cada uma das partes do tipo. Isso significa que
todos os campos ou valores no tipo também devem implementar `Clone` para derivar
`Clone`.

Um exemplo de quando `Clone` é necessário é ao chamar o método `to_vec` em uma
*slice*. A *slice* não possui o *ownership* das instâncias de tipo que contém,
mas o vetor retornado por `to_vec` precisará ter *ownership* de suas instâncias,
então `to_vec` chama `clone` em cada item. Assim, o tipo armazenado na *slice*
deve implementar `Clone`.

O *trait* `Copy` permite duplicar um valor copiando apenas os bits armazenados
na *stack*; nenhum código arbitrário é necessário. Consulte a seção [“Dados
Apenas na Stack: Copy”][stack-only-data-copy]<!-- ignore --> no Capítulo 4 para
mais informações sobre `Copy`.

O *trait* `Copy` não define nenhum método para evitar que programadores
sobrecarreguem esses métodos e violem a suposição de que nenhum código
arbitrário está sendo executado. Dessa forma, todos os programadores podem
presumir que copiar um valor será muito rápido.

Você pode derivar `Copy` em qualquer tipo cujas partes implementem todas `Copy`.
Um tipo que implementa `Copy` também deve implementar `Clone`, porque um tipo que
implementa `Copy` tem uma implementação trivial de `Clone` que realiza a mesma
tarefa que `Copy`.

O *trait* `Copy` raramente é obrigatório; tipos que implementam `Copy` possuem
otimizações disponíveis, o que significa que você não precisa chamar `clone`,
tornando o código mais conciso.

Tudo o que é possível com `Copy` você também pode fazer com `Clone`, mas o
código pode ficar mais lento ou ter que usar `clone` em alguns lugares.

<a id="hash-for-mapping-a-value-to-a-value-of-fixed-size"></a>
### `Hash` para Mapear um Valor para um Valor de Tamanho Fixo

O *trait* `Hash` permite pegar uma instância de um tipo de tamanho arbitrário e
mapear essa instância para um valor de tamanho fixo usando uma função de hash.
Derivar `Hash` implementa o método `hash`. A implementação derivada do método
`hash` combina o resultado da chamada de `hash` em cada uma das partes do tipo,
o que significa que todos os campos ou valores também devem implementar `Hash`
para derivar `Hash`.

Um exemplo de quando `Hash` é necessário é ao armazenar chaves em um
`HashMap<K, V>` para armazenar dados de maneira eficiente.

<a id="default-for-default-values"></a>
### `Default` para Valores Padrão

O *trait* `Default` permite criar um valor padrão para um tipo. Derivar
`Default` implementa a função `default`. A implementação derivada da função
`default` chama a função `default` em cada parte do tipo, o que significa que
todos os campos ou valores no tipo também devem implementar `Default` para
derivar `Default`.

A função `Default::default` é comumente usada em combinação com a sintaxe de
atualização de *struct* discutida na seção [“Criando Instâncias a partir de
Outras Instâncias com a Sintaxe de Atualização de
Struct”][creating-instances-from-other-instances-with-struct-update-syntax]<!--
ignore --> no Capítulo 5. Você pode personalizar alguns campos de uma *struct* e
então definir e usar um valor padrão para o restante dos campos usando
`..Default::default()`.

O *trait* `Default` é necessário quando você usa o método `unwrap_or_default`
em instâncias de `Option<T>`, por exemplo. Se o `Option<T>` for `None`, o método
`unwrap_or_default` retornará o resultado de `Default::default` para o tipo `T`
armazenado no `Option<T>`.

[creating-instances-from-other-instances-with-struct-update-syntax]: ch05-01-defining-structs.html#creating-instances-from-other-instances-with-struct-update-syntax
[stack-only-data-copy]: ch04-01-what-is-ownership.html#stack-only-data-copy
[variables-and-data-interacting-with-clone]: ch04-01-what-is-ownership.html#variables-and-data-interacting-with-clone
[custom-derive-macros]: ch20-05-macros.html#custom-derive-macros
