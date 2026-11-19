<a id="building-a-single-threaded-web-server"></a>
## Construindo um Servidor Web Single-Threaded

Começaremos colocando um servidor web *single-threaded* (de uma única
*thread*) para funcionar. Antes de começar, vamos dar uma rápida olhada nos
protocolos envolvidos na construção de servidores web. Os detalhes desses
protocolos estão além do escopo deste livro, mas uma breve visão geral fornecerá
as informações de que você precisa.

Os dois principais protocolos envolvidos em servidores web são o _Hypertext
Transfer Protocol_ _(HTTP)_ e o _Transmission Control Protocol_ _(TCP)_. Ambos
os protocolos são protocolos de _requisição-resposta_ (*request-response*), o
que significa que um _cliente_ inicia requisições e um _servidor_ escuta as
requisições e fornece uma resposta ao cliente. O conteúdo dessas requisições e
respostas é definido pelos protocolos.

O TCP é o protocolo de nível mais baixo que descreve os detalhes de como a
informação chega de um servidor a outro, mas não especifica o que é essa
informação. O HTTP se baseia no TCP definindo o conteúdo das requisições e
respostas. Tecnicamente, é possível usar HTTP com outros protocolos, mas na
grande maioria dos casos, o HTTP envia seus dados sobre TCP. Trabalharemos com
os bytes brutos das requisições e respostas TCP e HTTP.

<a id="listening-to-the-tcp-connection"></a>
### Escutando a Conexão TCP

Nosso servidor web precisa escutar uma conexão TCP, então essa é a primeira
parte em que trabalharemos. A biblioteca padrão oferece um módulo `std::net` que
nos permite fazer isso. Vamos criar um novo projeto da maneira habitual:

```console
$ cargo new hello
     Created binary (application) `hello` project
$ cd hello
```

Agora insira o código da Listagem 21-1 em _src/main.rs_ para começar. Este
código escutará no endereço local `127.0.0.1:7878` por *streams* TCP de entrada.
Quando receber um *stream* de entrada, ele imprimirá `Connection established!`.

<Listing number="21-1" file-name="src/main.rs" caption="Escutando *streams* de entrada e imprimindo uma mensagem quando recebemos um *stream*">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-01/src/main.rs}}
```

</Listing>

Usando `TcpListener`, podemos escutar conexões TCP no endereço
`127.0.0.1:7878`. No endereço, a parte antes dos dois-pontos é um endereço IP
que representa o seu computador (isso é igual em todos os computadores e não
representa o computador dos autores especificamente), e `7878` é a porta.
Escolhemos essa porta por dois motivos: normalmente o HTTP não é aceito nessa
porta, portanto é improvável que nosso servidor entre em conflito com qualquer
outro servidor web que possa estar rodando na sua máquina, e 7878 é _rust_
digitado no teclado de um telefone.

A função `bind` neste cenário funciona como a função `new`, pois retornará uma
nova instância de `TcpListener`. A função se chama `bind` porque, em redes,
conectar-se a uma porta para escutá-la é conhecido como “fazer o *bind* em uma
porta” (vincular-se a uma porta).

A função `bind` retorna um `Result<T, E>`, o que indica que é possível que o
*bind* falhe — por exemplo, se executarmos duas instâncias do nosso programa e,
assim, tivermos dois programas escutando a mesma porta. Como estamos escrevendo
um servidor básico apenas para fins de aprendizado, não nos preocuparemos em
tratar esses tipos de erro; em vez disso, usamos `unwrap` para interromper o
programa se ocorrerem erros.

O método `incoming` em `TcpListener` retorna um iterador que nos fornece uma
sequência de *streams* (mais especificamente, *streams* do tipo `TcpStream`). Um
único _stream_ representa uma conexão aberta entre o cliente e o servidor.
_Conexão_ é o nome dado ao processo completo de requisição e resposta no qual um
cliente se conecta ao servidor, o servidor gera uma resposta e o servidor fecha
a conexão. Dessa forma, leremos do `TcpStream` para ver o que o cliente enviou
e então escreveremos nossa resposta no *stream* para enviar dados de volta ao
cliente. No geral, esse loop `for` processará cada conexão por vez e produzirá
uma série de *streams* para tratarmos.

Por enquanto, nosso tratamento do *stream* consiste em chamar `unwrap` para
encerrar nosso programa caso o *stream* tenha algum erro; se não houver erros,
o programa imprime uma mensagem. Adicionaremos mais funcionalidades para o caso
de sucesso na próxima listagem. O motivo pelo qual podemos receber erros do
método `incoming` quando um cliente se conecta ao servidor é que, na verdade,
não estamos iterando sobre conexões. Em vez disso, estamos iterando sobre
_tentativas de conexão_. A conexão pode não ser bem-sucedida por vários motivos,
muitos deles específicos do sistema operacional. Por exemplo, muitos sistemas
operacionais têm um limite para o número de conexões abertas simultâneas que
podem suportar; novas tentativas de conexão além desse número produzirão um erro
até que algumas das conexões abertas sejam fechadas.

Vamos tentar rodar este código! Execute `cargo run` no terminal e depois carregue
_127.0.0.1:7878_ em um navegador web. O navegador deve exibir uma mensagem de
erro como “Connection reset” (Conexão reiniciada) porque o servidor ainda não
está enviando nenhum dado de volta. Mas, quando você olhar para o seu terminal,
deverá ver várias mensagens que foram impressas quando o navegador se conectou
ao servidor!

```text
     Running `target/debug/hello`
Connection established!
Connection established!
Connection established!
```

Às vezes você verá várias mensagens impressas para uma única requisição do
navegador; o motivo pode ser que o navegador está fazendo uma requisição para a
página e também uma requisição para outros recursos, como o ícone _favicon.ico_
que aparece na aba do navegador.

Também pode ser que o navegador esteja tentando se conectar ao servidor várias
vezes porque o servidor não está respondendo com nenhum dado. Quando `stream`
sai de escopo e é descartado (*dropped*) ao final do loop, a conexão é fechada
como parte da implementação de `drop`. Às vezes os navegadores lidam com
conexões fechadas tentando novamente, porque o problema pode ser temporário.

Os navegadores também às vezes abrem várias conexões com o servidor sem enviar
nenhuma requisição para que, caso *venham* a enviar requisições depois, essas
requisições possam acontecer mais rapidamente. Quando isso ocorre, nosso
servidor verá cada conexão, independentemente de haver ou não requisições nessa
conexão. Muitas versões de navegadores baseados no Chrome fazem isso, por
exemplo; você pode desabilitar essa otimização usando o modo de navegação
privada ou usando um navegador diferente.

O fator importante é que conseguimos obter com sucesso um manipulador (*handle*)
para uma conexão TCP!

Lembre-se de interromper o programa pressionando <kbd>ctrl</kbd>-<kbd>C</kbd>
quando terminar de rodar uma determinada versão do código. Depois, reinicie o
programa executando o comando `cargo run` após fazer cada conjunto de alterações
no código para garantir que você está rodando o código mais recente.

<a id="reading-the-request"></a>
### Lendo a Requisição

Vamos implementar a funcionalidade para ler a requisição do navegador! Para
separar as responsabilidades de primeiro obter uma conexão e depois tomar alguma
ação com a conexão, iniciaremos uma nova função para processar conexões. Nesta
nova função `handle_connection`, leremos os dados do *stream* TCP e os
imprimiremos para que possamos ver os dados que estão sendo enviados pelo
navegador. Altere o código para ficar como na Listagem 21-2.

<Listing number="21-2" file-name="src/main.rs" caption="Lendo do `TcpStream` e imprimindo os dados">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-02/src/main.rs}}
```

</Listing>

Trazemos `std::io::BufReader` e `std::io::prelude` para o escopo para ter acesso
a *traits* e tipos que nos permitem ler e escrever no *stream*. No loop `for` da
função `main`, em vez de imprimir uma mensagem dizendo que fizemos uma conexão,
agora chamamos a nova função `handle_connection` e passamos `stream` para ela.

Na função `handle_connection`, criamos uma nova instância de `BufReader` que
envolve uma referência para `stream`. O `BufReader` adiciona *buffering*
gerenciando as chamadas aos métodos da *trait* `std::io::Read` para nós.

Criamos uma variável chamada `http_request` para coletar as linhas da requisição
que o navegador envia para o nosso servidor. Indicamos que queremos coletar
essas linhas em um vetor adicionando a anotação de tipo `Vec<_>`.

`BufReader` implementa a *trait* `std::io::BufRead`, que fornece o método
`lines`. O método `lines` retorna um iterador de `Result<String,
std::io::Error>` dividindo o *stream* de dados sempre que encontra um byte de
quebra de linha. Para obter cada `String`, fazemos `map` e `unwrap` em cada
`Result`. O `Result` pode ser um erro se os dados não forem UTF-8 válidos ou se
houver um problema ao ler do *stream*. Novamente, um programa de produção
deveria tratar esses erros de forma mais elegante, mas estamos escolhendo
interromper o programa em caso de erro pela simplicidade.

O navegador sinaliza o fim de uma requisição HTTP enviando dois caracteres de
quebra de linha seguidos; portanto, para obter uma requisição do *stream*,
pegamos linhas até encontrarmos uma linha que seja uma string vazia. Depois de
coletarmos as linhas no vetor, imprimimos todas usando a formatação de depuração
bonita (*pretty debug formatting*) para podermos dar uma olhada nas instruções
que o navegador web está enviando ao nosso servidor.

Vamos testar este código! Inicie o programa e faça uma requisição em um
navegador web novamente. Note que ainda receberemos uma página de erro no
navegador, mas a saída do nosso programa no terminal agora será parecida com
esta:

<!-- manual-regeneration
cd listings/ch21-web-server/listing-21-02
cargo run
make a request to 127.0.0.1:7878
Can't automate because the output depends on making requests
-->

```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/hello`
Request: [
    "GET / HTTP/1.1",
    "Host: 127.0.0.1:7878",
    "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:99.0) Gecko/20100101 Firefox/99.0",
    "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
    "Accept-Language: en-US,en;q=0.5",
    "Accept-Encoding: gzip, deflate, br",
    "DNT: 1",
    "Connection: keep-alive",
    "Upgrade-Insecure-Requests: 1",
    "Sec-Fetch-Dest: document",
    "Sec-Fetch-Mode: navigate",
    "Sec-Fetch-Site: none",
    "Sec-Fetch-User: ?1",
    "Cache-Control: max-age=0",
]
```

Dependendo do seu navegador, você pode obter uma saída ligeiramente diferente.
Agora que estamos imprimindo os dados da requisição, podemos ver por que
recebemos várias conexões a partir de uma única requisição do navegador olhando
para o caminho após `GET` na primeira linha da requisição. Se todas as conexões
repetidas estiverem requisitando _/_, sabemos que o navegador está tentando
buscar _/_ repetidamente porque não está recebendo uma resposta do nosso
programa.

Vamos destrinchar esses dados da requisição para entender o que o navegador está
pedindo ao nosso programa.

<!-- Old headings. Do not remove or links may break. -->

<a id="a-closer-look-at-an-http-request"></a>
<a id="looking-closer-at-an-http-request"></a>
<a id="looking-more-closely-at-an-http-request"></a>

### Olhando Mais de Perto uma Requisição HTTP

O HTTP é um protocolo baseado em texto, e uma requisição segue este formato:

```text
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```

A primeira linha é a _linha de requisição_ (*request line*) que guarda
informações sobre o que o cliente está requisitando. A primeira parte da linha
de requisição indica o método sendo usado, como `GET` ou `POST`, que descreve
como o cliente está fazendo essa requisição. Nosso cliente usou uma requisição
`GET`, o que significa que ele está pedindo informações.

A próxima parte da linha de requisição é _/_, que indica o _uniform resource
identifier_ _(URI)_ que o cliente está requisitando: um URI é quase, mas não
exatamente, a mesma coisa que um _uniform resource locator_ _(URL)_. A diferença
entre URIs e URLs não é importante para os nossos propósitos neste capítulo, mas
a especificação HTTP usa o termo _URI_, então podemos simplesmente substituir
mentalmente _URI_ por _URL_ aqui.

A última parte é a versão do HTTP que o cliente usa e, em seguida, a linha de
requisição termina em uma sequência CRLF. (_CRLF_ significa _carriage return_ e
_line feed_, retorno de carro e alimentação de linha — termos dos tempos das
máquinas de escrever!) A sequência CRLF também pode ser escrita como `\r\n`,
onde `\r` é um retorno de carro e `\n` é uma quebra de linha. A _sequência
CRLF_ separa a linha de requisição do restante dos dados da requisição. Observe
que, quando o CRLF é impresso, vemos uma nova linha começar em vez de `\r\n`.

Olhando para os dados da linha de requisição que recebemos ao rodar nosso
programa até agora, vemos que `GET` é o método, _/_ é o URI da requisição e
`HTTP/1.1` é a versão.

Após a linha de requisição, as linhas restantes a partir de `Host:` são
cabeçalhos (*headers*). Requisições `GET` não têm corpo (*body*).

Tente fazer uma requisição a partir de outro navegador ou pedir um endereço
diferente, como _127.0.0.1:7878/test_, para ver como os dados da requisição
mudam.

Agora que sabemos o que o navegador está pedindo, vamos enviar alguns dados de
volta!

<a id="writing-a-response"></a>
### Escrevendo uma Resposta

Vamos implementar o envio de dados em resposta à requisição de um cliente. As
respostas têm o seguinte formato:

```text
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```

A primeira linha é uma _linha de status_ (*status line*) que contém a versão do
HTTP usada na resposta, um código de status numérico que resume o resultado da
requisição e uma frase de motivo (*reason phrase*) que fornece uma descrição em
texto do código de status. Após a sequência CRLF vêm quaisquer cabeçalhos, outra
sequência CRLF e o corpo da resposta.

Aqui está um exemplo de resposta que usa a versão 1.1 do HTTP e tem um código
de status 200, uma frase de motivo OK, nenhum cabeçalho e nenhum corpo:

```text
HTTP/1.1 200 OK\r\n\r\n
```

O código de status 200 é a resposta padrão de sucesso. O texto acima é uma
pequena resposta HTTP bem-sucedida. Vamos escrever isso no *stream* como nossa
resposta para uma requisição bem-sucedida! Na função `handle_connection`, remova
o `println!` que estava imprimindo os dados da requisição e substitua-o pelo
código da Listagem 21-3.

<Listing number="21-3" file-name="src/main.rs" caption="Escrevendo uma pequena resposta HTTP bem-sucedida no *stream*">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-03/src/main.rs:here}}
```

</Listing>

A primeira linha nova define a variável `response` que guarda os dados da
mensagem de sucesso. Em seguida, chamamos `as_bytes` em nossa `response` para
converter os dados da string em bytes. O método `write_all` em `stream` recebe
um `&[u8]` e envia esses bytes diretamente pela conexão. Como a operação
`write_all` pode falhar, usamos `unwrap` em qualquer resultado de erro como
antes. Novamente, em uma aplicação real, você adicionaria tratamento de erros
aqui.

Com essas alterações, vamos rodar nosso código e fazer uma requisição. Não
estamos mais imprimindo nenhum dado no terminal, então não veremos nenhuma saída
além da saída do Cargo. Quando você carregar _127.0.0.1:7878_ em um navegador
web, deverá obter uma página em branco em vez de um erro. Você acabou de
implementar manualmente o recebimento de uma requisição HTTP e o envio de uma
resposta!

<a id="returning-real-html"></a>
### Retornando HTML Real

Vamos implementar a funcionalidade para retornar mais do que uma página em
branco. Crie o novo arquivo _hello.html_ na raiz do diretório do seu projeto,
não no diretório _src_. Você pode colocar o HTML que quiser; a Listagem 21-4
mostra uma possibilidade.

<Listing number="21-4" file-name="hello.html" caption="Um arquivo HTML de exemplo para retornar em uma resposta">

```html
{{#include ../listings/ch21-web-server/listing-21-05/hello.html}}
```

</Listing>

Este é um documento HTML5 mínimo com um título e algum texto. Para retornar
isso do servidor quando uma requisição for recebida, modificaremos
`handle_connection` conforme mostrado na Listagem 21-5 para ler o arquivo HTML,
adicioná-lo à resposta como corpo e enviá-lo.

<Listing number="21-5" file-name="src/main.rs" caption="Enviando o conteúdo de *hello.html* como o corpo da resposta">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-05/src/main.rs:here}}
```

</Listing>

Adicionamos `fs` à instrução `use` para trazer o módulo de sistema de arquivos
da biblioteca padrão para o escopo. O código para ler o conteúdo de um arquivo
para uma string deve parecer familiar; nós o usamos quando lemos o conteúdo de
um arquivo no nosso projeto de E/S na Listagem 12-4.

Em seguida, usamos `format!` para adicionar o conteúdo do arquivo como o corpo
da resposta de sucesso. Para garantir uma resposta HTTP válida, adicionamos o
cabeçalho `Content-Length`, que é definido como o tamanho do corpo da nossa
resposta — neste caso, o tamanho de `hello.html`.

Execute este código com `cargo run` e carregue _127.0.0.1:7878_ no seu
navegador; você deverá ver seu HTML renderizado!

Atualmente, estamos ignorando os dados da requisição em `http_request` e apenas
enviando de volta o conteúdo do arquivo HTML incondicionalmente. Isso significa
que, se você tentar requisitar _127.0.0.1:7878/something-else_ no seu navegador,
ainda receberá essa mesma resposta HTML. No momento, nosso servidor é muito
limitado e não faz o que a maioria dos servidores web faz. Queremos personalizar
nossas respostas dependendo da requisição e enviar o arquivo HTML de volta
apenas para uma requisição bem-formada para _/_.

<a id="validating-the-request-and-selectively-responding"></a>
### Validando a Requisição e Respondendo Seletivamente

Neste momento, nosso servidor web retornará o HTML do arquivo não importa o que
o cliente requisitou. Vamos adicionar uma funcionalidade para verificar se o
navegador está requisitando _/_ antes de retornar o arquivo HTML e para retornar
um erro se o navegador requisitar qualquer outra coisa. Para isso, precisamos
modificar `handle_connection`, conforme mostrado na Listagem 21-6. Esse novo
código verifica o conteúdo da requisição recebida em comparação com o que
sabemos ser uma requisição para _/_ e adiciona blocos `if` e `else` para tratar
as requisições de maneira diferente.

<Listing number="21-6" file-name="src/main.rs" caption="Tratando requisições para */* de forma diferente de outras requisições">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-06/src/main.rs:here}}
```

</Listing>

Vamos olhar apenas para a primeira linha da requisição HTTP, então, em vez de
ler toda a requisição em um vetor, estamos chamando `next` para obter o primeiro
item do iterador. O primeiro `unwrap` cuida do `Option` e interrompe o programa
se o iterador não tiver nenhum item. O segundo `unwrap` lida com o `Result` e
tem o mesmo efeito que o `unwrap` que estava no `map` adicionado na Listagem
21-2.

Em seguida, verificamos `request_line` para ver se ela é igual à linha de
requisição de uma requisição GET para o caminho _/_. Se for, o bloco `if`
retorna o conteúdo do nosso arquivo HTML.

Se `request_line` _não_ for igual à requisição GET para o caminho _/_, isso
significa que recebemos alguma outra requisição. Adicionaremos código ao bloco
`else` em um instante para responder a todas as outras requisições.

Rode este código agora e requisite _127.0.0.1:7878_; você deverá obter o HTML
de _hello.html_. Se você fizer qualquer outra requisição, como
_127.0.0.1:7878/something-else_, receberá um erro de conexão como aqueles que
viu ao rodar o código da Listagem 21-1 e da Listagem 21-2.

Agora vamos adicionar o código da Listagem 21-7 ao bloco `else` para retornar
uma resposta com o código de status 404, que sinaliza que o conteúdo para a
requisição não foi encontrado. Também retornaremos algum HTML para uma página a
ser renderizada no navegador indicando a resposta ao usuário final.

<Listing number="21-7" file-name="src/main.rs" caption="Respondendo com código de status 404 e uma página de erro se qualquer coisa diferente de */* for requisitada">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-07/src/main.rs:here}}
```

</Listing>

Aqui, nossa resposta tem uma linha de status com o código de status 404 e a
frase de motivo `NOT FOUND`. O corpo da resposta será o HTML do arquivo
_404.html_. Você precisará criar um arquivo _404.html_ ao lado de _hello.html_
para a página de erro; novamente, fique à vontade para usar qualquer HTML que
quiser, ou use o HTML de exemplo da Listagem 21-8.

<Listing number="21-8" file-name="404.html" caption="Conteúdo de exemplo para a página a ser enviada de volta com qualquer resposta 404">

```html
{{#include ../listings/ch21-web-server/listing-21-07/404.html}}
```

</Listing>

Com essas alterações, rode seu servidor novamente. Requisitar _127.0.0.1:7878_
deve retornar o conteúdo de _hello.html_, e qualquer outra requisição, como
_127.0.0.1:7878/foo_, deve retornar o HTML de erro de _404.html_.

<!-- Old headings. Do not remove or links may break. -->

<a id="a-touch-of-refactoring"></a>
<a id="refactoring"></a>

### Refatoração

No momento, os blocos `if` e `else` têm muita repetição: ambos estão lendo
arquivos e escrevendo o conteúdo dos arquivos no *stream*. As únicas diferenças
são a linha de status e o nome do arquivo. Vamos tornar o código mais conciso
extraindo essas diferenças para linhas `if` e `else` separadas que atribuirão os
valores da linha de status e do nome do arquivo a variáveis; poderemos então
usar essas variáveis incondicionalmente no código para ler o arquivo e escrever
a resposta. A Listagem 21-9 mostra o código resultante após substituir os grandes
blocos `if` e `else`.

<Listing number="21-9" file-name="src/main.rs" caption="Refatorando os blocos `if` e `else` para conter apenas o código que difere entre os dois casos">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-09/src/main.rs:here}}
```

</Listing>

Agora os blocos `if` e `else` retornam apenas os valores apropriados para a
linha de status e o nome do arquivo em uma tupla; então usamos desestruturação
para atribuir esses dois valores a `status_line` e `filename` usando um padrão na
instrução `let`, conforme discutido no Capítulo 19.

O código anteriormente duplicado agora está fora dos blocos `if` e `else` e usa
as variáveis `status_line` e `filename`. Isso torna mais fácil ver a diferença
entre os dois casos e significa que temos apenas um lugar para atualizar o
código se quisermos mudar a forma como a leitura de arquivos e a escrita de
respostas funcionam. O comportamento do código da Listagem 21-9 será o mesmo que
o da Listagem 21-7.

Incrível! Agora temos um servidor web simples em aproximadamente 40 linhas de
código Rust que responde a uma requisição com uma página de conteúdo e responde
a todas as outras requisições com uma resposta 404.

Atualmente, nosso servidor roda em uma única *thread*, o que significa que ele
só pode atender uma requisição por vez. Vamos examinar como isso pode ser um
problema simulando algumas requisições lentas. Depois, corrigiremos isso para
que nosso servidor possa lidar com múltiplas requisições de uma só vez.
