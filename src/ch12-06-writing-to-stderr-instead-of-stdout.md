<!-- Old headings. Do not remove or links may break. -->

<a id="writing-error-messages-to-standard-error-instead-of-standard-output"></a>

## Redirecionando Erros para a Saída de Erro Padrão

No momento, estamos escrevendo toda a nossa saída no terminal usando a *macro* `println!`. Na maioria dos terminais, há dois tipos de saída: _saída padrão_ (`stdout`) para informações gerais e _saída de erro padrão_ (`stderr`) para mensagens de erro. Essa distinção permite que os usuários escolham redirecionar a saída bem-sucedida de um programa para um arquivo, mas ainda vejam as mensagens de erro na tela.

A *macro* `println!` só consegue imprimir para a saída padrão, então precisamos usar algo diferente para imprimir na saída de erro padrão.

<a id="checking-where-errors-are-written"></a>
### Verificando Onde os Erros São Escritos

Primeiro, vamos observar como o conteúdo impresso pelo `minigrep` está sendo escrito atualmente na saída padrão, incluindo as mensagens de erro que queremos escrever na saída de erro padrão. Faremos isso redirecionando o fluxo de saída padrão para um arquivo enquanto causamos intencionalmente um erro. Não redirecionaremos o fluxo de saída de erro padrão, então qualquer conteúdo enviado para o erro padrão continuará sendo exibido na tela.

Programas de linha de comando devem enviar mensagens de erro para o fluxo de erro padrão para que possamos ainda ver as mensagens de erro na tela mesmo que redirecionemos o fluxo de saída padrão para um arquivo. Nosso programa não está se comportando corretamente: vamos ver agora que ele salva a saída da mensagem de erro em um arquivo!

Para demonstrar esse comportamento, executaremos o programa com `>` e o caminho do arquivo, _output.txt_, para o qual queremos redirecionar o fluxo de saída padrão. Não passaremos nenhum argumento, o que deve causar um erro:

```console
$ cargo run > output.txt
```

A sintaxe `>` diz ao shell para escrever o conteúdo da saída padrão em _output.txt_ em vez de na tela. Não vimos a mensagem de erro que esperávamos impressa na tela, então isso significa que deve ter acabado no arquivo. Veja o que _output.txt_ contém:

```text
Problem parsing arguments: not enough arguments
```

Pois é, nossa mensagem de erro está sendo impressa na saída padrão. É muito mais útil para mensagens de erro como essa serem impressas no erro padrão para que apenas os dados de uma execução bem-sucedida acabem no arquivo. Vamos mudar isso.

<a id="printing-errors-to-standard-error"></a>
### Imprimindo Erros na Saída de Erro Padrão

Usaremos o código da Listagem 12-24 para mudar como as mensagens de erro são impressas. Por causa da refatoração que fizemos anteriormente neste capítulo, todo o código que imprime mensagens de erro está em uma única função, `main`. A biblioteca padrão fornece a *macro* `eprintln!` que imprime no fluxo de saída de erro padrão, então vamos mudar os dois lugares onde estávamos chamando `println!` para imprimir erros para usar `eprintln!` em vez disso.

<Listing number="12-24" file-name="src/main.rs" caption="Escrevendo mensagens de erro na saída de erro padrão em vez da saída padrão usando `eprintln!`">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-24/src/main.rs:here}}
```

</Listing>

Vamos agora executar o programa novamente da mesma forma, sem nenhum argumento e redirecionando a saída padrão com `>`:

```console
$ cargo run > output.txt
Problem parsing arguments: not enough arguments
```

Agora vemos o erro na tela e _output.txt_ não contém nada, que é o comportamento esperado de programas de linha de comando.

Vamos executar o programa novamente com argumentos que não causam erro, mas ainda redirecionando a saída padrão para um arquivo, assim:

```console
$ cargo run -- to poem.txt > output.txt
```

Não veremos nenhuma saída no terminal, e _output.txt_ conterá nossos resultados:

<span class="filename">Arquivo: output.txt</span>

```text
Are you nobody, too?
How dreary to be somebody!
```

Isso demonstra que agora estamos usando a saída padrão para saída bem-sucedida e a saída de erro padrão para saída de erros, como é adequado.

<a id="summary"></a>
## Resumo

Este capítulo recapitulou alguns dos principais conceitos que você aprendeu até agora e abordou como realizar operações comuns de E/S no Rust. Usando argumentos de linha de comando, arquivos, variáveis de ambiente e a *macro* `eprintln!` para imprimir erros, você agora está preparado para escrever aplicações de linha de comando. Combinado com os conceitos dos capítulos anteriores, seu código será bem organizado, armazenará dados de forma eficaz nas estruturas de dados apropriadas, tratará erros de forma elegante e será bem testado.

A seguir, exploraremos alguns recursos do Rust que foram influenciados por linguagens funcionais: *closures* e iteradores.
