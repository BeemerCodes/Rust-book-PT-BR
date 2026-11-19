<a id="patterns-and-matching"></a>

# Padrões e Correspondência (*Pattern Matching*)

Padrões (*patterns*) são uma sintaxe especial em Rust para fazer correspondência
com a estrutura de tipos, tanto complexos quanto simples. Usar padrões em
conjunto com expressões `match` e outras construções dá a você mais controle
sobre o fluxo de controle de um programa. Um padrão consiste em alguma
combinação do seguinte:

- Literais
- Arrays, enums, structs ou tuplas desestruturados
- Variáveis
- Curingas (*wildcards*)
- Marcadores de posição (*placeholders*)

Alguns exemplos de padrões incluem `x`, `(a, 3)` e `Some(Color::Red)`. Nos
contextos em que padrões são válidos, esses componentes descrevem o formato dos
dados. Nosso programa então compara valores com os padrões para determinar se
eles têm o formato correto de dados para continuar executando um determinado
trecho de código.

Para usar um padrão, nós o comparamos com algum valor. Se o padrão corresponder
ao valor, usamos as partes do valor em nosso código. Lembre-se das expressões
`match` no Capítulo 6 que usavam padrões, como o exemplo da máquina de separar
moedas. Se o valor se encaixar no formato do padrão, podemos usar as partes
nomeadas. Se não se encaixar, o código associado ao padrão não será executado.

Este capítulo é uma referência sobre tudo relacionado a padrões. Cobriremos os
lugares válidos para usar padrões, a diferença entre padrões refutáveis e
irrefutáveis e os diferentes tipos de sintaxe de padrões que você pode
encontrar. Ao final do capítulo, você saberá como usar padrões para expressar
muitos conceitos de forma clara.
