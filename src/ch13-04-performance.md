<!-- Old headings. Do not remove or links may break. -->

<a id="comparing-performance-loops-vs-iterators"></a>

<a id="performance-in-loops-vs-iterators"></a>
## Desempenho: Laços vs. Iteradores

Para decidir se deve usar laços ou iteradores, você precisa saber qual implementação é mais rápida: a versão da função `search` com um laço `for` explícito ou a versão com iteradores.

Rodamos um *benchmark* carregando todo o conteúdo de _As Aventuras de Sherlock Holmes_, de Sir Arthur Conan Doyle, em uma `String` e buscando a palavra _the_ no texto. Aqui estão os resultados do *benchmark* na versão de `search` usando o laço `for` e na versão usando iteradores:

```text
test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)
```

As duas implementações têm desempenho semelhante! Não vamos explicar o código do *benchmark* aqui, porque o objetivo não é provar que as duas versões são equivalentes, mas ter uma noção geral de como elas se comparam em termos de desempenho.

Para um *benchmark* mais abrangente, você deveria testar usando textos de vários tamanhos em `contents`, palavras diferentes e de comprimentos variados em `query`, além de todo tipo de variação. O ponto principal é este: embora os iteradores sejam uma abstração de alto nível, eles são compilados basicamente para o mesmo código que você escreveria à mão em nível mais baixo. Iteradores são uma das *abstrações de custo zero* (*zero-cost abstractions*) do Rust, o que significa que usar a abstração não impõe nenhuma sobrecarga adicional em tempo de execução. Isso é análogo à forma como Bjarne Stroustrup, o criador e implementador original do C++, define *zero-overhead* em sua palestra de abertura no ETAPS 2012, “Foundations of C++”:

> Em geral, implementações de C++ obedecem ao princípio de sobrecarga zero (*zero-overhead*): aquilo que você não usa, você não paga por isso. E mais ainda: aquilo que você usa, você não conseguiria escrever à mão de forma melhor.

Em muitos casos, código Rust usando iteradores compila para exatamente o mesmo *assembly* que você escreveria manualmente. Otimizações como desenrolamento de laços (*loop unrolling*) e eliminação de verificações de limites em acessos a arrays são aplicadas, tornando o código resultante extremamente eficiente. Agora que você sabe disso, pode usar iteradores e *closures* sem medo! Eles fazem o código parecer de nível mais alto, sem impor penalidade de desempenho em tempo de execução por isso.

<a id="summary"></a>
## Resumo

*Closures* e iteradores são recursos do Rust inspirados em ideias de linguagens de programação funcional. Eles contribuem para a capacidade do Rust de expressar ideias de alto nível com clareza mantendo desempenho de baixo nível. As implementações de *closures* e iteradores são feitas de tal forma que o desempenho em tempo de execução não é afetado. Isso faz parte do objetivo do Rust de oferecer abstrações de custo zero.

Agora que melhoramos a expressividade do nosso projeto de I/O, vamos ver mais alguns recursos do `cargo` que vão nos ajudar a compartilhar o projeto com o mundo.
