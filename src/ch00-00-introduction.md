<a id="introduction"></a>
# Introdução

> Nota: Esta edição do livro é a mesma que [A Linguagem de Programação Rust][nsprust], disponível em formato impresso e ebook pela [No Starch Press][nsp].

[nsprust]: https://nostarch.com/rust-programming-language-3rd-edition
[nsp]: https://nostarch.com/

Bem-vindo a _A Linguagem de Programação Rust_, um livro introdutório sobre Rust. A linguagem de programação Rust ajuda você a escrever software mais rápido e mais confiável. Ergonomia de alto nível e controle de baixo nível costumam ser antagônicos no design de linguagens de programação; o Rust desafia esse conflito. Por meio do equilíbrio entre grande capacidade técnica e uma excelente experiência de desenvolvimento, o Rust oferece a opção de controlar detalhes de baixo nível (como o uso de memória) sem todo o trabalho tradicionalmente associado a esse controle.

<a id="who-rust-is-for"></a>
## Para Quem o Rust É Feito

O Rust é ideal para muitas pessoas por uma variedade de motivos. Vamos ver alguns dos grupos mais importantes.

<a id="teams-of-developers"></a>
### Times de Desenvolvedores

O Rust está se mostrando uma ferramenta produtiva para colaboração em grandes equipes de desenvolvedores com diferentes níveis de conhecimento em programação de sistemas. Código de baixo nível é propenso a vários bugs sutis que, na maioria das outras linguagens, só podem ser detectados por meio de testes extensivos e revisão cuidadosa de código por desenvolvedores experientes. No Rust, o compilador desempenha o papel de guardião, recusando compilar código com esses bugs elusivos, incluindo bugs de concorrência. Ao trabalhar junto com o compilador, a equipe pode dedicar seu tempo a focar na lógica do programa em vez de caçar bugs.

O Rust também traz ferramentas modernas de desenvolvimento para o mundo da programação de sistemas:

- O Cargo, o gerenciador de dependências e ferramenta de build incluídos, torna a adição, compilação e gerenciamento de dependências simples e consistente em todo o ecossistema Rust.
- A ferramenta de formatação `rustfmt` garante um estilo de codificação consistente entre os desenvolvedores.
- O Rust Language Server proporciona integração com ambientes de desenvolvimento integrado (IDEs) para completar código e exibir mensagens de erro inline.

Usando essas e outras ferramentas do ecossistema Rust, os desenvolvedores conseguem ser produtivos ao escrever código de nível de sistemas.

<a id="students"></a>
### Estudantes

O Rust é para estudantes e para quem tem interesse em aprender conceitos de sistemas. Usando Rust, muitas pessoas aprenderam sobre tópicos como desenvolvimento de sistemas operacionais. A comunidade é muito acolhedora e adora responder às perguntas dos estudantes. Com iniciativas como este livro, as equipes do Rust querem tornar os conceitos de sistemas mais acessíveis a mais pessoas, especialmente as que são novas na programação.

<a id="companies"></a>
### Empresas

Centenas de empresas, grandes e pequenas, usam Rust em produção para uma variedade de tarefas, incluindo ferramentas de linha de comando, serviços web, ferramentas de DevOps, dispositivos embarcados, análise e transcodificação de áudio e vídeo, criptomoedas, bioinformática, mecanismos de busca, aplicações de Internet das Coisas, aprendizado de máquina e até partes importantes do navegador web Firefox.

<a id="open-source-developers"></a>
### Desenvolvedores de Código Aberto

O Rust é para pessoas que querem construir a própria linguagem de programação Rust, sua comunidade, ferramentas de desenvolvimento e bibliotecas. Adoraríamos ter você contribuindo para a linguagem Rust.

<a id="people-who-value-speed-and-stability"></a>
### Pessoas que Valorizam Velocidade e Estabilidade

O Rust é para pessoas que desejam velocidade e estabilidade em uma linguagem. Por velocidade, queremos dizer tanto o quão rapidamente o código Rust pode ser executado quanto a velocidade com que o Rust permite que você escreva programas. As verificações do compilador Rust garantem estabilidade por meio de adições de recursos e refatoração. Isso contrasta com o código legado frágil de linguagens sem essas verificações, que os desenvolvedores frequentemente têm medo de modificar. Ao buscar abstrações de custo zero — recursos de alto nível que compilam para código de baixo nível tão rápido quanto código escrito manualmente —, o Rust se esforça para fazer com que código seguro também seja código rápido.

A linguagem Rust espera atender muitos outros usuários também; os mencionados aqui são apenas alguns dos maiores interessados. No geral, a maior ambição do Rust é eliminar as trocas que os programadores aceitaram por décadas, oferecendo segurança _e_ produtividade, velocidade _e_ ergonomia. Experimente o Rust e veja se suas escolhas funcionam para você.

<a id="who-this-book-is-for"></a>
## Para Quem É Este Livro

Este livro assume que você já escreveu código em outra linguagem de programação, mas não faz suposições sobre qual. Tentamos tornar o material amplamente acessível para pessoas de uma grande variedade de origens de programação. Não passamos muito tempo falando sobre o que é programação _de verdade_ ou como pensar sobre ela. Se você é completamente novo à programação, seria melhor ler um livro que forneça especificamente uma introdução à programação.

<a id="how-to-use-this-book"></a>
## Como Usar Este Livro

Em geral, este livro assume que você o está lendo na ordem, do início ao fim. Capítulos posteriores constroem sobre conceitos de capítulos anteriores, e capítulos anteriores podem não se aprofundar nos detalhes de um tópico específico, mas o revisitarão em um capítulo posterior.

Você encontrará dois tipos de capítulos neste livro: capítulos de conceitos e capítulos de projetos. Em capítulos de conceitos, você aprenderá sobre um aspecto do Rust. Em capítulos de projetos, construiremos pequenos programas juntos, aplicando o que você aprendeu até agora. Os Capítulos 2, 12 e 21 são capítulos de projetos; os demais são capítulos de conceitos.

O **Capítulo 1** explica como instalar o Rust, como escrever um programa "Olá, mundo!" e como usar o Cargo, o gerenciador de *packages* e ferramenta de build do Rust. O **Capítulo 2** é uma introdução prática à escrita de um programa em Rust, onde você vai construir um jogo de adivinhação de números. Aqui, abordamos os conceitos em alto nível, e capítulos posteriores fornecerão detalhes adicionais. Se você quiser colocar a mão na massa imediatamente, o Capítulo 2 é o lugar certo. Se você prefere aprender cada detalhe antes de avançar para o próximo assunto, talvez queira pular o Capítulo 2 e ir direto ao **Capítulo 3**, que cobre recursos do Rust similares aos de outras linguagens de programação; depois, você pode voltar ao Capítulo 2 quando quiser trabalhar em um projeto aplicando os detalhes aprendidos.

No **Capítulo 4**, você aprenderá sobre o sistema de *ownership* do Rust. O **Capítulo 5** discute *structs* e métodos. O **Capítulo 6** cobre *enums*, expressões `match` e os construtos de fluxo de controle `if let` e `let...else`. Você usará *structs* e *enums* para criar tipos personalizados.

No **Capítulo 7**, você aprenderá sobre o sistema de módulos do Rust e as regras de privacidade para organizar seu código e sua interface de programação de aplicativos (API) pública. O **Capítulo 8** discute algumas estruturas de dados de coleção comuns que a biblioteca padrão fornece: vetores, strings e hash maps. O **Capítulo 9** explora a filosofia e as técnicas de tratamento de erros do Rust.

O **Capítulo 10** mergulha em genéricos, *traits* e *lifetimes*, que lhe dão o poder de definir código que se aplica a múltiplos tipos. O **Capítulo 11** é totalmente sobre testes, que mesmo com as garantias de segurança do Rust são necessários para garantir que a lógica do seu programa esteja correta. No **Capítulo 12**, construiremos nossa própria implementação de um subconjunto das funcionalidades da ferramenta de linha de comando `grep`, que busca texto dentro de arquivos. Para isso, usaremos muitos dos conceitos discutidos nos capítulos anteriores.

O **Capítulo 13** explora *closures* e iteradores: recursos do Rust que vêm de linguagens de programação funcional. No **Capítulo 14**, examinaremos o Cargo com mais profundidade e falaremos sobre boas práticas para compartilhar suas bibliotecas com outras pessoas. O **Capítulo 15** discute os *smart pointers* que a biblioteca padrão fornece e os *traits* que habilitam suas funcionalidades.

No **Capítulo 16**, passaremos por diferentes modelos de programação concorrente e falaremos sobre como o Rust ajuda você a programar em múltiplas *threads* sem medo. No **Capítulo 17**, desenvolvemos isso ainda mais, explorando a sintaxe async e await do Rust, juntamente com tarefas, *futures* e *streams*, e o modelo leve de concorrência que eles habilitam.

O **Capítulo 18** analisa como os idiomas do Rust se comparam aos princípios de programação orientada a objetos com os quais você pode estar familiarizado. O **Capítulo 19** é uma referência sobre padrões e *pattern matching*, que são formas poderosas de expressar ideias em programas Rust. O **Capítulo 20** contém uma miscelânea de tópicos avançados de interesse, incluindo Rust inseguro, *macros* e mais sobre *lifetimes*, *traits*, tipos, funções e *closures*.

No **Capítulo 21**, concluiremos um projeto em que implementaremos um servidor web multithread de baixo nível!

Por fim, alguns apêndices contêm informações úteis sobre a linguagem em um formato mais de referência. O **Apêndice A** cobre as palavras-chave do Rust, o **Apêndice B** cobre os operadores e símbolos do Rust, o **Apêndice C** cobre os *traits* deriváveis fornecidos pela biblioteca padrão, o **Apêndice D** cobre algumas ferramentas de desenvolvimento úteis e o **Apêndice E** explica as edições do Rust. No **Apêndice F**, você pode encontrar traduções do livro, e no **Apêndice G** cobriremos como o Rust é feito e o que é o Rust nightly.

Não há maneira errada de ler este livro: se você quiser pular adiante, vá em frente! Talvez você precise voltar a capítulos anteriores se tiver alguma confusão. Mas faça o que funcionar para você.

<span id="ferris"></span>

Uma parte importante do processo de aprender Rust é aprender a ler as mensagens de erro que o compilador exibe: elas vão guiá-lo em direção a um código funcional. Por isso, forneceremos muitos exemplos que não compilam, juntamente com a mensagem de erro que o compilador mostrará em cada situação. Saiba que, se você inserir e executar um exemplo aleatório, ele pode não compilar! Certifique-se de ler o texto ao redor para ver se o exemplo que você está tentando executar é intencionalmente errado. O Ferris também vai ajudá-lo a distinguir código que não deve funcionar:

| Ferris                                                                                                           | Significado                                               |
| ---------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------- |
| <img src="img/ferris/does_not_compile.svg" class="ferris-explain" alt="Ferris com um ponto de interrogação"/>    | Este código não compila!                                  |
| <img src="img/ferris/panics.svg" class="ferris-explain" alt="Ferris jogando as mãos para cima"/>                 | Este código entra em pânico!                              |
| <img src="img/ferris/not_desired_behavior.svg" class="ferris-explain" alt="Ferris com uma garra levantada, dando de ombros"/> | Este código não produz o comportamento desejado. |

Na maioria das situações, vamos guiá-lo até a versão correta de qualquer código que não compile.

<a id="source-code"></a>
## Código-fonte

Os arquivos-fonte a partir dos quais este livro é gerado podem ser encontrados no [GitHub][book].

[book]: https://github.com/rust-lang/book/tree/main/src
