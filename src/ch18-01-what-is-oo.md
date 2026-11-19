<a id="characteristics-of-object-oriented-languages"></a>

## Características de Linguagens Orientadas a Objetos

Não há consenso na comunidade de programação sobre quais recursos uma linguagem
precisa ter para ser considerada orientada a objetos. Rust é influenciada por
muitos paradigmas de programação, incluindo POO; por exemplo, exploramos os
recursos vindos da programação funcional no Capítulo 13. Pode-se argumentar que
linguagens orientadas a objetos compartilham certas características em comum —
especificamente, objetos, encapsulamento e herança. Vamos analisar o que cada
uma dessas características significa e se Rust oferece suporte a ela.

<a id="objects-contain-data-and-behavior"></a>

### Objetos Contêm Dados e Comportamento

O livro *Design Patterns: Elements of Reusable Object-Oriented Software*, de
Erich Gamma, Richard Helm, Ralph Johnson e John Vlissides (Addison-Wesley,
1994), conhecido coloquialmente como o livro da *Gang of Four* (Gangue dos
Quatro), é um catálogo de padrões de projeto orientados a objetos. Ele define
POO da seguinte maneira:

> Programas orientados a objetos são compostos por objetos. Um **objeto**
> empacota tanto dados quanto os procedimentos que operam sobre esses dados.
> Os procedimentos são tipicamente chamados de **métodos** ou **operações**.

Usando essa definição, Rust é orientada a objetos: structs e enums possuem
dados, e blocos `impl` fornecem métodos em structs e enums. Mesmo que structs e
enums com métodos não sejam *chamados* de objetos, eles fornecem a mesma
funcionalidade, de acordo com a definição de objetos da *Gang of Four*.

<a id="encapsulation-that-hides-implementation-details"></a>

### Encapsulamento que Oculta Detalhes de Implementação

Outro aspecto comumente associado à POO é a ideia de *encapsulamento*, o que
significa que os detalhes de implementação de um objeto não ficam acessíveis ao
código que utiliza esse objeto. Portanto, a única maneira de interagir com um
objeto é por meio de sua API pública; o código que usa o objeto não deve poder
acessar os internos do objeto e alterar dados ou comportamentos diretamente.
Isso permite que a pessoa programadora altere e refatore os internos de um
objeto sem precisar alterar o código que o utiliza.

Discutimos como controlar o encapsulamento no Capítulo 7: podemos usar a
palavra-chave `pub` para decidir quais módulos, tipos, funções e métodos em
nosso código devem ser públicos e, por padrão, todo o resto é privado. Por
exemplo, podemos definir uma struct `AveragedCollection` que tem um campo
contendo um vetor de valores `i32`. A struct também pode ter um campo que
contém a média dos valores no vetor, o que significa que a média não precisa ser
calculada sob demanda sempre que alguém precisar dela. Em outras palavras,
`AveragedCollection` armazenará em cache a média calculada para nós. A Listagem
18-1 traz a definição da struct `AveragedCollection`.

<Listing number="18-1" file-name="src/lib.rs" caption="Uma struct `AveragedCollection` que mantém uma lista de inteiros e a média dos itens na coleção">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-01/src/lib.rs}}
```

</Listing>

A struct é marcada como `pub` para que outro código possa usá-la, mas os campos
dentro dela permanecem privados. Isso é importante nesse caso porque queremos
garantir que, sempre que um valor for adicionado ou removido da lista, a média
também seja atualizada. Fazemos isso implementando os métodos `add`, `remove` e
`average` na struct, conforme mostrado na Listagem 18-2.

<Listing number="18-2" file-name="src/lib.rs" caption="Implementações dos métodos públicos `add`, `remove` e `average` em `AveragedCollection`">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-02/src/lib.rs:here}}
```

</Listing>

Os métodos públicos `add`, `remove` e `average` são as únicas maneiras de
acessar ou modificar dados em uma instância de `AveragedCollection`. Quando um
item é adicionado a `list` usando o método `add` ou removido usando o método
`remove`, a implementação de cada um chama o método privado `update_average`,
que cuida de atualizar o campo `average` também.

Deixamos os campos `list` e `average` privados para que não haja como um código
externo adicionar ou remover itens diretamente do campo `list`; caso contrário,
o campo `average` poderia ficar dessincronizado quando `list` mudasse. O método
`average` retorna o valor no campo `average`, permitindo que código externo leia
`average`, mas não o modifique.

Como encapsulamos os detalhes de implementação da struct `AveragedCollection`,
podemos alterar facilmente certos aspectos no futuro, como a estrutura de dados.
Por exemplo, poderíamos usar um `HashSet<i32>` em vez de um `Vec<i32>` para o
campo `list`. Desde que as assinaturas dos métodos públicos `add`, `remove` e
`average` permanecessem iguais, o código que usa `AveragedCollection` não
precisaria mudar. Se tivéssemos tornado `list` público, esse não seria
necessariamente o caso: `HashSet<i32>` e `Vec<i32>` têm métodos diferentes para
adicionar e remover itens, então o código externo provavelmente teria que mudar
se estivesse modificando `list` diretamente.

Se o encapsulamento for um aspecto obrigatório para que uma linguagem seja
considerada orientada a objetos, então Rust atende a esse requisito. A opção de
usar `pub` ou não para diferentes partes do código permite o encapsulamento dos
detalhes de implementação.

<a id="inheritance-as-a-type-system-and-as-code-sharing"></a>

### Herança como Sistema de Tipos e como Compartilhamento de Código

*Herança* é um mecanismo pelo qual um objeto pode herdar elementos da definição
de outro objeto, obtendo assim os dados e o comportamento do objeto pai sem que
você precise defini-los novamente.

Se uma linguagem precisar ter herança para ser orientada a objetos, então Rust
não é uma linguagem desse tipo. Não há maneira de definir uma struct que herde
os campos e implementações de métodos de uma struct pai sem usar uma macro.

No entanto, se você está acostumado a ter herança na sua caixa de ferramentas de
programação, pode usar outras soluções em Rust, dependendo do motivo pelo qual
você recorreria à herança em primeiro lugar.

Você escolheria a herança por dois motivos principais. Um deles é o reuso de
código: você pode implementar um comportamento específico para um tipo, e a
herança permite reutilizar essa implementação em um tipo diferente. Você pode
fazer isso de forma limitada em código Rust usando implementações padrão de
métodos em traits, o que você viu na Listagem 10-14 quando adicionamos uma
implementação padrão do método `summarize` no trait `Summary`. Qualquer tipo que
implemente o trait `Summary` terá o método `summarize` disponível sem precisar
de nenhum código adicional. Isso é semelhante a uma classe pai ter a
implementação de um método e uma classe filha que herda dela também ter a
implementação do método. Também podemos sobrescrever a implementação padrão do
método `summarize` quando implementamos o trait `Summary`, o que é semelhante a
uma classe filha sobrescrever a implementação de um método herdado de uma
classe pai.

O outro motivo para usar herança está relacionado ao sistema de tipos: permitir
que um tipo filho seja usado nos mesmos lugares que o tipo pai. Isso também é
chamado de *polimorfismo*, o que significa que você pode substituir múltiplos
objetos uns pelos outros em tempo de execução se eles compartilharem certas
características.

<a id="polymorphism"></a>

> ### Polimorfismo
>
> Para muitas pessoas, polimorfismo é sinônimo de herança. Mas, na verdade, é
> um conceito mais geral que se refere a código que pode trabalhar com dados de
> múltiplos tipos. No caso da herança, esses tipos geralmente são subclasses.
>
> Em vez disso, Rust usa genéricos para abstrair sobre diferentes tipos
> possíveis e *trait bounds* para impor restrições sobre o que esses tipos
> devem fornecer. Isso às vezes é chamado de *polimorfismo paramétrico
> limitado* (*bounded parametric polymorphism*).

Rust escolheu um conjunto diferente de *trade-offs* ao não oferecer herança. A
herança frequentemente corre o risco de compartilhar mais código do que o
necessário. Subclasses nem sempre deveriam compartilhar todas as características
da classe pai, mas acabam fazendo isso com a herança. Isso pode tornar o
design de um programa menos flexível. Também introduz a possibilidade de chamar
métodos em subclasses que não fazem sentido ou que causam erros porque os
métodos não se aplicam à subclasse. Além disso, algumas linguagens permitem
apenas *herança simples* (o que significa que uma subclasse só pode herdar de
uma classe), restringindo ainda mais a flexibilidade do design de um programa.

Por esses motivos, Rust adota a abordagem diferente de usar objetos trait
(*trait objects*) em vez de herança para alcançar polimorfismo em tempo de
execução. Vamos ver como objetos trait funcionam.
