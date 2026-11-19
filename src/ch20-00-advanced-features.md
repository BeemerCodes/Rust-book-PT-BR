<a id="advanced-features"></a>
# Recursos Avançados

A esta altura, você já aprendeu as partes mais usadas da linguagem de
programação Rust. Antes de fazermos mais um projeto, no Capítulo 21, vamos
explorar alguns aspectos da linguagem com os quais você pode se deparar de vez
em quando, mas que talvez não use todos os dias. Você pode usar este capítulo
como referência quando encontrar algo desconhecido. Os recursos abordados aqui
são úteis em situações bastante específicas. Embora você talvez não recorra a
eles com frequência, queremos garantir que você tenha uma boa noção de tudo o
que o Rust tem a oferecer.

Neste capítulo, vamos abordar:

- Rust *unsafe*: como abrir mão de algumas garantias do Rust e assumir a
  responsabilidade de mantê-las manualmente
- *Traits* avançadas: tipos associados, parâmetros de tipo padrão, sintaxe
  totalmente qualificada, *supertraits* e o padrão *newtype* em relação a *traits*
- Tipos avançados: mais sobre o padrão *newtype*, sinônimos de tipo (*type
  aliases*), o tipo *never* e tipos de tamanho dinâmico
- Funções e *closures* avançadas: ponteiros de função e retorno de *closures*
- *Macros*: maneiras de definir código que gera mais código em tempo de
  compilação

É uma grande variedade de recursos do Rust, com algo para todo mundo! Vamos
mergulhar nisso!
