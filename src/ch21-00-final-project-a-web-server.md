<a id="final-project-building-a-multithreaded-web-server"></a>
# Projeto Final: Construindo um Servidor Web Multithreaded

Foi uma longa jornada, mas chegamos ao final do livro. Neste capítulo,
construiremos mais um projeto juntos para demonstrar alguns dos conceitos que
abordamos nos capítulos finais, além de recapitular algumas lições anteriores.

Para o nosso projeto final, faremos um servidor web que diz “Hello!” e se
parece com a Figura 21-1 em um navegador web.

Aqui está o nosso plano para construir o servidor web:

1. Aprender um pouco sobre TCP e HTTP.
2. Escutar conexões TCP em um socket.
3. Analisar (fazer o parsing de) um pequeno número de requisições HTTP.
4. Criar uma resposta HTTP adequada.
5. Melhorar a vazão (*throughput*) do nosso servidor com um *thread pool*.

<img alt="Captura de tela de um navegador web acessando o endereço 127.0.0.1:8080 exibindo uma página web com o conteúdo de texto “Hello! Hi from Rust”" src="img/trpl21-01.png" class="center" style="width: 50%;" />

<span class="caption">Figura 21-1: Nosso projeto final compartilhado</span>

Antes de começarmos, devemos mencionar dois detalhes. Primeiro, o método que
usaremos não será a melhor maneira de construir um servidor web com Rust.
Membros da comunidade publicaram várias *crates* prontas para produção
disponíveis em [crates.io](https://crates.io/) que fornecem implementações de
servidor web e *thread pool* mais completas do que a que construiremos. No
entanto, nossa intenção neste capítulo é ajudar você a aprender, e não pegar o
caminho mais fácil. Como Rust é uma linguagem de programação de sistemas,
podemos escolher o nível de abstração com o qual queremos trabalhar e descer a
um nível mais baixo do que é possível ou prático em outras linguagens.

Segundo, não usaremos `async` e `await` aqui. Construir um *thread pool* já é um
desafio grande o suficiente por si só, sem acrescentar a construção de um
runtime assíncrono! Contudo, observaremos como `async` e `await` podem ser
aplicáveis a alguns dos mesmos problemas que veremos neste capítulo. Em última
análise, como observamos no Capítulo 17, muitos runtimes assíncronos usam
*thread pools* para gerenciar seu trabalho.

Portanto, escreveremos manualmente o servidor HTTP básico e o *thread pool* para
que você possa aprender as ideias e técnicas gerais por trás das *crates* que
poderá usar no futuro.
