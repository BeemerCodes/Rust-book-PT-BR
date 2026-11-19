<a id="appendix-g---how-rust-is-made-and-nightly-rust"></a>
## Apêndice G - Como o Rust é Feito e o “Nightly Rust”

Este apêndice explica como o Rust é desenvolvido e como isso afeta você como
pessoa desenvolvedora Rust.

<a id="stability-without-stagnation"></a>
### Estabilidade Sem Estagnação

Como linguagem, o Rust se preocupa _muito_ com a estabilidade do seu código.
Queremos que o Rust seja uma base sólida sobre a qual você possa construir, e
se as coisas estivessem mudando o tempo todo, isso seria impossível. Ao mesmo
tempo, se não pudermos experimentar novos recursos, talvez não descubramos
falhas importantes até depois do lançamento, quando não podemos mais mudar as
coisas.

Nossa solução para esse problema é o que chamamos de “estabilidade sem
estagnação” (*stability without stagnation*), e nosso princípio orientador é
este: você nunca deve ter receio de atualizar para uma nova versão do Rust
estável (*stable*). Cada atualização deve ser tranquila, mas também deve trazer
novos recursos, menos bugs e tempos de compilação mais rápidos.

<a id="choo-choo-release-channels-and-riding-the-trains"></a>
### Piuí, Piuí! Canais de Lançamento e Embarcando nos Trens

O desenvolvimento do Rust opera em um _cronograma de trens_. Ou seja, todo o
desenvolvimento é feito na branch principal (*main*) do repositório do Rust. Os
lançamentos seguem um modelo de trem de lançamento de software (*release
train*), que já foi usado pelo Cisco IOS e por outros projetos de software.
Existem três _canais de lançamento_ para o Rust:

- Nightly
- Beta
- Stable

A maioria das pessoas desenvolvedoras Rust usa principalmente o canal *stable*,
mas quem deseja experimentar novos recursos experimentais pode usar o *nightly*
ou o *beta*.

Veja um exemplo de como funciona o processo de desenvolvimento e lançamento:
vamos supor que a equipe do Rust esteja trabalhando no lançamento do Rust 1.5.
Esse lançamento aconteceu em dezembro de 2015, mas nos fornecerá números de
versão realistas. Um novo recurso é adicionado ao Rust: um novo commit chega na
branch principal. Toda noite, uma nova versão *nightly* do Rust é produzida.
Todo dia é dia de lançamento, e esses lançamentos são criados automaticamente
pela nossa infraestrutura de lançamento. Assim, com o passar do tempo, nossos
lançamentos ficam assim, um por noite:

```text
nightly: * - - * - - *
```

A cada seis semanas, é hora de preparar um novo lançamento! A branch `beta` do
repositório do Rust ramifica-se a partir da branch principal usada pelo
*nightly*. Agora, há dois lançamentos:

```text
nightly: * - - * - - *
                     |
beta:                *
```

A maioria das pessoas usuárias de Rust não usa versões *beta* ativamente, mas
testa contra o *beta* em seus sistemas de CI para ajudar o Rust a descobrir
possíveis regressões. Enquanto isso, continua havendo um lançamento *nightly*
todas as noites:

```text
nightly: * - - * - - * - - * - - *
                     |
beta:                *
```

Digamos que uma regressão seja encontrada. Ainda bem que tivemos algum tempo
para testar a versão *beta* antes que a regressão entrasse sorrateiramente em
um lançamento *stable*! A correção é aplicada na branch principal, de modo que o
*nightly* seja corrigido, e então a correção é portada de volta (*backported*)
para a branch `beta`, e uma nova versão do *beta* é produzida:

```text
nightly: * - - * - - * - - * - - * - - *
                     |
beta:                * - - - - - - - - *
```

Seis semanas depois da criação do primeiro *beta*, é hora de um lançamento
*stable*! A branch `stable` é produzida a partir da branch `beta`:

```text
nightly: * - - * - - * - - * - - * - - * - * - *
                     |
beta:                * - - - - - - - - *
                                       |
stable:                                *
```

Viva! O Rust 1.5 está pronto! No entanto, esquecemos uma coisa: como as seis
semanas se passaram, também precisamos de um novo *beta* da _próxima_ versão do
Rust, a 1.6. Então, depois que a `stable` se ramifica da `beta`, a próxima
versão da `beta` se ramifica novamente a partir da `nightly`:

```text
nightly: * - - * - - * - - * - - * - - * - * - *
                     |                         |
beta:                * - - - - - - - - *       *
                                       |
stable:                                *
```

Isso é chamado de “modelo de trens” porque, a cada seis semanas, um lançamento
“sai da estação”, mas ainda precisa fazer uma viagem pelo canal *beta* antes
de chegar como um lançamento *stable*.

O Rust lança novas versões a cada seis semanas, com precisão de relógio. Se você
sabe a data de um lançamento do Rust, sabe a data do próximo: é seis semanas
depois. Um aspecto positivo de ter lançamentos agendados a cada seis semanas é
que o próximo trem está chegando em breve. Se por acaso um recurso perder um
lançamento específico, não há motivo para se preocupar: outro acontecerá em
pouco tempo! Isso ajuda a reduzir a pressão para incluir recursos possivelmente
pouco polidos perto do prazo de lançamento.

Graças a esse processo, você sempre pode conferir a próxima compilação do Rust
e verificar por conta própria se é fácil atualizar para ela: se uma versão
*beta* não funcionar como esperado, você pode reportar o problema para a equipe
e vê-lo corrigido antes que o próximo lançamento *stable* aconteça! Quebras em
uma versão *beta* são relativamente raras, mas o `rustc` ainda é um software, e
bugs existem.

<a id="maintenance-time"></a>
### Tempo de Manutenção

O projeto Rust dá suporte à versão *stable* mais recente. Quando uma nova versão
*stable* é lançada, a versão antiga atinge seu fim de vida útil (EOL, de *end
of life*). Isso significa que cada versão recebe suporte por seis semanas.

<a id="unstable-features"></a>
### Recursos Instáveis

Há mais um detalhe nesse modelo de lançamento: recursos instáveis. O Rust usa
uma técnica chamada “*feature flags*” para determinar quais recursos estão
habilitados em um determinado lançamento. Se um novo recurso está em
desenvolvimento ativo, ele chega à branch principal e, portanto, ao *nightly*,
mas atrás de uma _feature flag_. Se você, como pessoa usuária, quiser
experimentar o recurso em andamento, pode fazê-lo, mas precisa estar usando uma
versão *nightly* do Rust e anotar seu código-fonte com a flag apropriada para
habilitá-lo.

Se você estiver usando uma versão *beta* ou *stable* do Rust, não poderá usar
nenhuma *feature flag*. Essa é a chave que nos permite obter uso prático com
novos recursos antes de declará-los estáveis para sempre. Quem deseja estar na
vanguarda pode fazer isso, e quem quer uma experiência sólida como rocha pode
ficar no *stable* e saber que seu código não vai quebrar. Estabilidade sem
estagnação.

Este livro contém apenas informações sobre recursos estáveis, já que os recursos
em andamento ainda estão mudando e certamente estarão diferentes entre o momento
em que este livro foi escrito e o momento em que forem habilitados em builds
*stable*. Você pode encontrar a documentação de recursos exclusivos do *nightly*
online.

<a id="rustup-and-the-role-of-rust-nightly"></a>
### O Rustup e o Papel do Rust Nightly

O Rustup facilita alternar entre diferentes canais de lançamento do Rust, de
forma global ou por projeto. Por padrão, você terá o Rust *stable* instalado.
Para instalar o *nightly*, por exemplo:

```console
$ rustup toolchain install nightly
```

Você também pode ver todos os _toolchains_ (lançamentos do Rust e componentes
associados) que tem instalados com o `rustup`. Veja um exemplo no computador
Windows de uma das pessoas autoras deste livro:

```powershell
> rustup toolchain list
stable-x86_64-pc-windows-msvc (default)
beta-x86_64-pc-windows-msvc
nightly-x86_64-pc-windows-msvc
```

Como você pode ver, o *toolchain* *stable* é o padrão. A maioria das pessoas
usuárias de Rust usa o *stable* na maior parte do tempo. Você pode querer usar o
*stable* quase sempre, mas usar o *nightly* em um projeto específico porque se
interessa por algum recurso de ponta. Para fazer isso, você pode usar
`rustup override` no diretório desse projeto para definir o *toolchain*
*nightly* como aquele que o `rustup` deve usar quando você estiver nesse
diretório:

```console
$ cd ~/projects/needs-nightly
$ rustup override set nightly
```

Agora, toda vez que você chamar `rustc` ou `cargo` dentro de
_~/projects/needs-nightly_, o `rustup` garantirá que você esteja usando o Rust
*nightly*, em vez do seu padrão de Rust *stable*. Isso é muito útil quando você
tem vários projetos em Rust!

<a id="the-rfc-process-and-teams"></a>
### O Processo de RFC e as Equipes

Então, como você fica sabendo desses novos recursos? O modelo de desenvolvimento
do Rust segue um _processo de Request For Comments (RFC)_. Se você gostaria de
uma melhoria no Rust, pode escrever uma proposta, chamada de RFC.

Qualquer pessoa pode escrever RFCs para melhorar o Rust, e as propostas são
revisadas e discutidas pela equipe do Rust, que é composta por várias
subequipes temáticas. Há uma lista completa das equipes [no site do
Rust](https://www.rust-lang.org/governance), que inclui equipes para cada área
do projeto: design da linguagem, implementação do compilador, infraestrutura,
documentação e muito mais. A equipe apropriada lê a proposta e os comentários,
escreve alguns comentários próprios e, eventualmente, chega a um consenso para
aceitar ou rejeitar o recurso.

Se o recurso for aceito, uma *issue* é aberta no repositório do Rust, e alguém
pode implementá-lo. A pessoa que o implementa pode muito bem não ser a mesma
que propôs o recurso originalmente! Quando a implementação fica pronta, ela
chega na branch principal atrás de uma *feature gate*, como discutimos na seção
[“Recursos Instáveis”](#unstable-features)<!-- ignore -->.

Depois de algum tempo, quando pessoas desenvolvedoras Rust que usam versões
*nightly* tiverem tido a oportunidade de testar o novo recurso, os membros da
equipe discutem o recurso, como ele se saiu no *nightly* e decidem se ele deve
ou não entrar no Rust *stable*. Se a decisão for seguir em frente, a *feature
gate* é removida e o recurso passa a ser considerado estável! Ele embarca nos
trens rumo a um novo lançamento *stable* do Rust.
