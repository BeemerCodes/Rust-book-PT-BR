<a id="installation"></a>
## Instalação

O primeiro passo é instalar o Rust. Vamos baixar o Rust por meio do `rustup`, uma ferramenta de linha de comando para gerenciar versões do Rust e ferramentas associadas. Você precisará de uma conexão com a internet para fazer o download.

> Nota: Se preferir não usar o `rustup` por algum motivo, consulte a [página de Outros Métodos de Instalação do Rust][otherinstall] para mais opções.

As etapas a seguir instalam a versão estável mais recente do compilador Rust. As garantias de estabilidade do Rust asseguram que todos os exemplos do livro que compilam continuarão compilando com versões mais recentes do Rust. A saída pode diferir ligeiramente entre versões porque o Rust frequentemente aprimora mensagens de erro e avisos. Em outras palavras, qualquer versão estável mais recente do Rust que você instalar usando estas etapas deve funcionar conforme esperado com o conteúdo deste livro.

> ### Notação de Linha de Comando
>
> Neste capítulo e ao longo do livro, mostraremos alguns comandos usados no terminal. Linhas que você deve digitar no terminal sempre começam com `$`. Você não precisa digitar o caractere `$`; ele é o prompt de linha de comando exibido para indicar o início de cada comando. Linhas que não começam com `$` normalmente mostram a saída do comando anterior. Além disso, exemplos específicos para PowerShell usarão `>` em vez de `$`.

<a id="installing-rustup-on-linux-or-macos"></a>
### Instalando o `rustup` no Linux ou macOS

Se você estiver usando Linux ou macOS, abra um terminal e digite o seguinte comando:

```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

O comando baixa um script e inicia a instalação da ferramenta `rustup`, que instala a versão estável mais recente do Rust. Pode ser que você seja solicitado a fornecer sua senha. Se a instalação for bem-sucedida, a seguinte linha aparecerá:

```text
Rust is installed now. Great!
```

Você também precisará de um _linker_, que é um programa que o Rust usa para juntar suas saídas compiladas em um único arquivo. É provável que você já tenha um. Se você obtiver erros de linker, deverá instalar um compilador C, que geralmente inclui um linker. Um compilador C também é útil porque alguns *packages* Rust comuns dependem de código C e precisarão de um compilador C.

No macOS, você pode obter um compilador C executando:

```console
$ xcode-select --install
```

Usuários de Linux geralmente devem instalar GCC ou Clang, de acordo com a documentação de sua distribuição. Por exemplo, se você usa Ubuntu, pode instalar o pacote `build-essential`.

<a id="installing-rustup-on-windows"></a>
### Instalando o `rustup` no Windows

No Windows, acesse [https://www.rust-lang.org/tools/install][install]<!-- ignore
--> e siga as instruções para instalar o Rust. Em algum momento da instalação, será solicitado que você instale o Visual Studio. Isso fornece um linker e as bibliotecas nativas necessárias para compilar programas. Se precisar de mais ajuda com esta etapa, consulte
[https://rust-lang.github.io/rustup/installation/windows-msvc.html][msvc]<!--
ignore -->.

O restante deste livro usa comandos que funcionam tanto no _cmd.exe_ quanto no PowerShell. Se houver diferenças específicas, explicaremos qual usar.

<a id="troubleshooting"></a>
### Solução de Problemas

Para verificar se o Rust está instalado corretamente, abra um shell e digite esta linha:

```console
$ rustc --version
```

Você deverá ver o número da versão, o hash do commit e a data do commit para a versão estável mais recente lançada, no seguinte formato:

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

Se você ver essa informação, instalou o Rust com sucesso! Se não ver essa informação, verifique se o Rust está na variável de sistema `%PATH%` da seguinte forma.

No Windows CMD, use:

```console
> echo %PATH%
```

No PowerShell, use:

```powershell
> echo $env:Path
```

No Linux e macOS, use:

```console
$ echo $PATH
```

Se isso estiver tudo correto e o Rust ainda não estiver funcionando, há vários lugares onde você pode obter ajuda. Descubra como entrar em contato com outros Rustaceans (apelido informal que nos damos) na [página da comunidade][community].

<a id="updating-and-uninstalling"></a>
### Atualizando e Desinstalando

Uma vez que o Rust está instalado via `rustup`, atualizar para uma versão recém-lançada é fácil. No seu shell, execute o seguinte script de atualização:

```console
$ rustup update
```

Para desinstalar o Rust e o `rustup`, execute o seguinte script de desinstalação no seu shell:

```console
$ rustup self uninstall
```

<!-- Old headings. Do not remove or links may break. -->
<a id="local-documentation"></a>

<a id="reading-the-local-documentation"></a>
### Lendo a Documentação Local

A instalação do Rust também inclui uma cópia local da documentação para que você possa lê-la offline. Execute `rustup doc` para abrir a documentação local no seu navegador.

Sempre que um tipo ou função for fornecido pela biblioteca padrão e você não tiver certeza do que faz ou como usar, use a documentação da interface de programação de aplicativos (API) para descobrir!

<!-- Old headings. Do not remove or links may break. -->
<a id="text-editors-and-integrated-development-environments"></a>

<a id="using-text-editors-and-ides"></a>
### Usando Editores de Texto e IDEs

Este livro não faz suposições sobre quais ferramentas você usa para escrever código Rust. Praticamente qualquer editor de texto resolverá o problema! No entanto, muitos editores de texto e ambientes de desenvolvimento integrado (IDEs) têm suporte embutido para Rust. Você sempre pode encontrar uma lista bastante atualizada de muitos editores e IDEs na [página de ferramentas][tools] do site do Rust.

<a id="working-offline-with-this-book"></a>
### Trabalhando Offline com Este Livro

Em vários exemplos, usaremos *packages* Rust além da biblioteca padrão. Para acompanhar esses exemplos, você precisará de uma conexão com a internet ou ter baixado essas dependências antecipadamente. Para baixar as dependências com antecedência, você pode executar os seguintes comandos. (Explicaremos o que é o `cargo` e o que cada um desses comandos faz com mais detalhe posteriormente.)

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:

* ch02-00-guessing-game-tutorial.md
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
* ch14-03-cargo-workspaces.md
-->

```console
$ cargo new get-dependencies
$ cd get-dependencies
$ cargo add rand@0.10.1 trpl@0.2.0
```

Isso fará cache dos downloads para esses *packages*, para que você não precise baixá-los mais tarde. Uma vez que você tenha executado este comando, não precisa manter a pasta `get-dependencies`. Se você tiver executado este comando, poderá usar a flag `--offline` com todos os comandos `cargo` no restante do livro para usar essas versões em cache em vez de tentar usar a rede.

[otherinstall]: https://forge.rust-lang.org/infra/other-installation-methods.html
[install]: https://www.rust-lang.org/tools/install
[msvc]: https://rust-lang.github.io/rustup/installation/windows-msvc.html
[community]: https://www.rust-lang.org/community
[tools]: https://www.rust-lang.org/tools
