<a id="hello-world"></a>
## Olá, Mundo!

Agora que você instalou o Rust, é hora de escrever seu primeiro programa Rust. É tradição, ao aprender uma nova linguagem, escrever um pequeno programa que imprime o texto `Hello, world!` na tela, então faremos o mesmo aqui!

> Nota: Este livro assume familiaridade básica com a linha de comando. O Rust não faz exigências específicas sobre seu editor ou ferramentas, ou onde seu código fica, então, se você preferir usar um IDE em vez da linha de comando, fique à vontade para usar seu IDE favorito. Muitos IDEs agora têm algum grau de suporte a Rust; consulte a documentação do IDE para detalhes. A equipe do Rust tem focado em habilitar um excelente suporte a IDEs via `rust-analyzer`. Veja o [Apêndice D][devtools]<!-- ignore --> para mais detalhes.

<!-- Old headings. Do not remove or links may break. -->
<a id="creating-a-project-directory"></a>

<a id="project-directory-setup"></a>
### Configurando o Diretório do Projeto

Você começará criando um diretório para armazenar seu código Rust. Para o Rust, não importa onde seu código fica, mas para os exercícios e projetos deste livro, sugerimos criar um diretório _projects_ no seu diretório home e manter todos os seus projetos lá.

Abra um terminal e digite os seguintes comandos para criar um diretório _projects_ e um diretório para o projeto "Hello, world!" dentro do diretório _projects_.

Para Linux, macOS e PowerShell no Windows, digite:

```console
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

Para o CMD do Windows, digite:

```cmd
> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir hello_world
> cd hello_world
```

<!-- Old headings. Do not remove or links may break. -->
<a id="writing-and-running-a-rust-program"></a>

<a id="rust-program-basics"></a>
### Noções Básicas de um Programa Rust

Em seguida, crie um novo arquivo-fonte e chame-o de _main.rs_. Arquivos Rust sempre terminam com a extensão _.rs_. Se você estiver usando mais de uma palavra no nome do arquivo, a convenção é usar um sublinhado para separá-las. Por exemplo, use _hello_world.rs_ em vez de _helloworld.rs_.

Agora abra o arquivo _main.rs_ que você acabou de criar e insira o código da Listagem 1-1.

<Listing number="1-1" file-name="main.rs" caption="Um programa que imprime `Hello, world!`">

```rust
fn main() {
    println!("Hello, world!");
}
```

</Listing>

Salve o arquivo e volte para a janela do terminal no diretório _~/projects/hello_world_. No Linux ou macOS, digite os seguintes comandos para compilar e executar o arquivo:

```console
$ rustc main.rs
$ ./main
Hello, world!
```

No Windows, digite o comando `.\main` em vez de `./main`:

```powershell
> rustc main.rs
> .\main
Hello, world!
```

Independentemente do sistema operacional, a string `Hello, world!` deve ser impressa no terminal. Se você não ver essa saída, consulte a parte ["Solução de Problemas"][troubleshooting]<!-- ignore --> da seção de Instalação para obter ajuda.

Se `Hello, world!` foi impresso, parabéns! Você escreveu oficialmente um programa Rust. Isso faz de você um programador Rust — seja bem-vindo!

<!-- Old headings. Do not remove or links may break. -->

<a id="anatomy-of-a-rust-program"></a>

<a id="the-anatomy-of-a-rust-program"></a>
### A Anatomia de um Programa Rust

Vamos revisar este programa "Hello, world!" em detalhes. Aqui está a primeira peça do quebra-cabeça:

```rust
fn main() {

}
```

Essas linhas definem uma função chamada `main`. A função `main` é especial: ela é sempre o primeiro código que é executado em todo programa Rust executável. Aqui, a primeira linha declara uma função chamada `main` que não tem parâmetros e não retorna nada. Se houvesse parâmetros, eles estariam dentro dos parênteses (`()`).

O corpo da função está envolto em `{}`. O Rust exige chaves ao redor de todos os corpos de função. É uma boa prática colocar a chave de abertura na mesma linha da declaração da função, com um espaço entre elas.

> Nota: Se você quiser manter um estilo padrão em todos os projetos Rust, pode usar uma ferramenta de formatação automática chamada `rustfmt` para formatar seu código em um estilo específico (mais sobre o `rustfmt` no [Apêndice D][devtools]<!-- ignore -->). A equipe do Rust incluiu essa ferramenta na distribuição padrão do Rust, assim como o `rustc`, então ela já deve estar instalada no seu computador!

O corpo da função `main` contém o seguinte código:

```rust
println!("Hello, world!");
```

Esta linha faz todo o trabalho neste pequeno programa: imprime texto na tela. Há três detalhes importantes a notar aqui.

Primeiro, `println!` chama uma *macro* Rust. Se tivesse chamado uma função, seria escrito como `println` (sem o `!`). *Macros* Rust são uma forma de escrever código que gera código para estender a sintaxe do Rust, e vamos discuti-las com mais detalhe no [Capítulo 20][ch20-macros]<!-- ignore -->. Por enquanto, você só precisa saber que usar `!` significa que você está chamando uma *macro* em vez de uma função normal, e que *macros* nem sempre seguem as mesmas regras que funções.

Segundo, você vê a string `"Hello, world!"`. Passamos essa string como argumento para `println!`, e a string é impressa na tela.

Terceiro, terminamos a linha com um ponto e vírgula (`;`), o que indica que esta expressão terminou e a próxima está pronta para começar. A maioria das linhas de código Rust termina com ponto e vírgula.

<!-- Old headings. Do not remove or links may break. -->
<a id="compiling-and-running-are-separate-steps"></a>

<a id="compilation-and-execution"></a>
### Compilação e Execução

Você acabou de executar um programa recém-criado, então vamos examinar cada etapa do processo.

Antes de executar um programa Rust, você deve compilá-lo usando o compilador Rust, digitando o comando `rustc` e passando o nome do seu arquivo-fonte, assim:

```console
$ rustc main.rs
```

Se você tem experiência com C ou C++, vai notar que isso é similar ao `gcc` ou `clang`. Após compilar com sucesso, o Rust gera um executável binário.

No Linux, macOS e PowerShell no Windows, você pode ver o executável digitando o comando `ls` no shell:

```console
$ ls
main  main.rs
```

No Linux e macOS, você verá dois arquivos. Com o PowerShell no Windows, você verá os mesmos três arquivos que veria usando CMD. Com o CMD no Windows, você digitaria o seguinte:

```cmd
> dir /B %= a opção /B diz para mostrar apenas os nomes dos arquivos =%
main.exe
main.pdb
main.rs
```

Isso mostra o arquivo de código-fonte com a extensão _.rs_, o arquivo executável (_main.exe_ no Windows, mas _main_ em todas as outras plataformas) e, quando no Windows, um arquivo com informações de depuração com a extensão _.pdb_. A partir daí, você executa o arquivo _main_ ou _main.exe_, assim:

```console
$ ./main # ou .\main no Windows
```

Se seu _main.rs_ for o seu programa "Hello, world!", esta linha imprime `Hello, world!` no seu terminal.

Se você está mais familiarizado com uma linguagem dinâmica, como Ruby, Python ou JavaScript, pode não estar acostumado a compilar e executar um programa como etapas separadas. O Rust é uma linguagem _compilada antecipadamente_ (*ahead-of-time compiled*), o que significa que você pode compilar um programa e dar o executável para outra pessoa, e ela pode executá-lo mesmo sem ter o Rust instalado. Se você der a alguém um arquivo _.rb_, _.py_ ou _.js_, elas precisarão ter uma implementação de Ruby, Python ou JavaScript instalada (respectivamente). Mas nessas linguagens, você só precisa de um comando para compilar e executar seu programa. Tudo é uma troca no design de linguagens.

Compilar apenas com `rustc` é bom para programas simples, mas à medida que seu projeto cresce, você vai querer gerenciar todas as opções e facilitar o compartilhamento do seu código. A seguir, apresentaremos a ferramenta Cargo, que vai ajudá-lo a escrever programas Rust do mundo real.

[troubleshooting]: ch01-01-installation.html#troubleshooting
[devtools]: appendix-04-useful-development-tools.html
[ch20-macros]: ch20-05-macros.html
