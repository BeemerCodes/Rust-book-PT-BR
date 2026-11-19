<a id="an-io-project-building-a-command-line-program"></a>
# Um Projeto de I/O: Construindo um Programa de Linha de Comando

Este capítulo é uma recapitulação de muitas das habilidades que você aprendeu até aqui e uma exploração de mais alguns recursos da biblioteca padrão. Vamos construir uma ferramenta de linha de comando que interage com entrada e saída (I/O) de arquivos e de linha de comando para praticar alguns dos conceitos de Rust que você já domina.

A velocidade, a segurança, a geração de um único binário e o suporte multiplataforma do Rust o tornam uma linguagem ideal para criar ferramentas de linha de comando. Por isso, para o nosso projeto, faremos nossa própria versão da clássica ferramenta de busca em linha de comando `grep` (**g**lobally search a **r**egular **e**xpression and **p**rint — busca global por expressão regular e impressão). No caso de uso mais simples, o `grep` procura uma string específica em um arquivo específico. Para isso, o `grep` recebe como argumentos um caminho de arquivo e uma string. Em seguida, ele lê o arquivo, encontra as linhas desse arquivo que contêm o argumento da string e imprime essas linhas.

Ao longo do caminho, mostraremos como fazer nossa ferramenta de linha de comando usar recursos do terminal que muitas outras ferramentas desse tipo utilizam. Vamos ler o valor de uma variável de ambiente para permitir que o usuário configure o comportamento da nossa ferramenta. Também imprimiremos mensagens de erro no fluxo de erro padrão do console (`stderr`) em vez da saída padrão (`stdout`), para que, por exemplo, o usuário possa redirecionar a saída bem-sucedida para um arquivo enquanto continua vendo mensagens de erro na tela.

Um membro da comunidade Rust, Andrew Gallant, já criou uma versão completa e muito rápida do `grep`, chamada `ripgrep`. Em comparação, nossa versão será bem simples, mas este capítulo lhe dará o conhecimento básico necessário para entender um projeto do mundo real como o `ripgrep`.

Nosso projeto `grep` combinará vários conceitos que você já aprendeu até agora:

- Organização de código ([Capítulo 7][ch7]<!-- ignore -->)
- Uso de vetores e strings ([Capítulo 8][ch8]<!-- ignore -->)
- Tratamento de erros ([Capítulo 9][ch9]<!-- ignore -->)
- Uso de *traits* e *lifetimes* quando apropriado ([Capítulo 10][ch10]<!-- ignore -->)
- Escrita de testes ([Capítulo 11][ch11]<!-- ignore -->)

Também apresentaremos brevemente *closures*, iteradores e *trait objects*, que o [Capítulo 13][ch13]<!-- ignore --> e o [Capítulo 18][ch18]<!-- ignore --> abordarão em detalhes.

[ch7]: ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
[ch8]: ch08-00-common-collections.html
[ch9]: ch09-00-error-handling.html
[ch10]: ch10-00-generics.html
[ch11]: ch11-00-testing.html
[ch13]: ch13-00-functional-features.html
[ch18]: ch18-00-oop.html
