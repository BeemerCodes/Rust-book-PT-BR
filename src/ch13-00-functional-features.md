<a id="functional-language-features-iterators-and-closures"></a>

# Recursos de Linguagens Funcionais: Iterators e Closures

O design do Rust foi inspirado em muitas linguagens e técnicas existentes, e uma
influência significativa é a _programação funcional_. Programar em estilo
funcional costuma incluir usar funções como valores: passá-las como argumentos,
retorná-las de outras funções, atribuí-las a variáveis para execução posterior,
e assim por diante.

Neste capítulo, não vamos debater o que a programação funcional é ou deixa de
ser. Em vez disso, vamos discutir alguns recursos do Rust que são parecidos com
recursos de muitas linguagens normalmente chamadas de funcionais.

Mais especificamente, vamos abordar:

- _Closures_, uma construção parecida com uma função que você pode armazenar em
  uma variável
- _Iterators_, uma forma de processar uma série de elementos
- Como usar closures e iterators para melhorar o projeto de E/S do Capítulo 12
- O desempenho de closures e iterators (alerta de spoiler: eles são mais rápidos
  do que você imagina!)

Já vimos outros recursos do Rust, como pattern matching e enums, que também
são influenciados pelo estilo funcional. Como dominar closures e iterators é
parte importante de escrever código Rust rápido e idiomático, vamos dedicar este
capítulo inteiro a eles.
