<a id="writing-automated-tests"></a>

# Escrevendo Testes Automatizados

Em seu ensaio de 1972, "The Humble Programmer" (O Programador Humilde), Edsger W. Dijkstra disse que "testar um programa pode ser uma forma muito eficaz de mostrar a presença de bugs, mas é completamente inadequado para mostrar a ausência deles." Isso não significa que não devamos tentar testar o máximo que pudermos!

A _corretude_ dos nossos programas é o grau em que o nosso código faz o que pretendemos. Rust foi projetado com um alto grau de preocupação com a corretude dos programas, mas corretude é algo complexo e não é fácil de provar. O sistema de tipos do Rust carrega uma grande parte desse peso, mas o sistema de tipos não consegue pegar tudo. Por isso, Rust inclui suporte para escrever testes automatizados de software.

Digamos que escrevemos uma função `add_two` que soma 2 a qualquer número passado para ela. A assinatura dessa função aceita um inteiro como parâmetro e retorna um inteiro como resultado. Quando implementamos e compilamos essa função, Rust realiza toda a verificação de tipos e a verificação de *borrowing* que você aprendeu até agora para garantir que, por exemplo, não estamos passando um valor `String` ou uma referência inválida para essa função. Mas Rust _não consegue_ verificar se essa função fará exatamente o que pretendemos, que é retornar o parâmetro mais 2 e não, digamos, o parâmetro mais 10 ou o parâmetro menos 50! É aí que entram os testes.

Podemos escrever testes que afirmam, por exemplo, que quando passamos `3` para a função `add_two`, o valor retornado é `5`. Podemos executar esses testes sempre que fizermos alterações no nosso código para garantir que qualquer comportamento correto existente não tenha mudado.

Testar é uma habilidade complexa: embora não possamos cobrir em um único capítulo todos os detalhes de como escrever bons testes, neste capítulo discutiremos a mecânica das facilidades de teste do Rust. Falaremos sobre as anotações e as *macros* disponíveis para você ao escrever seus testes, o comportamento padrão e as opções fornecidas para executar seus testes, e como organizar testes em testes unitários e testes de integração.
