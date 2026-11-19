<!-- Old headings. Do not remove or links may break. -->

<a id="extensible-concurrency-with-the-sync-and-send-traits"></a>
<a id="extensible-concurrency-with-the-send-and-sync-traits"></a>

<a id="extensible-concurrency-with-send-and-sync"></a>
## Concorrência Extensível com `Send` e `Sync`

Curiosamente, quase todos os recursos de concorrência sobre os quais falamos até
agora neste capítulo fazem parte da biblioteca padrão, não da linguagem. Suas
opções para lidar com concorrência não se limitam à linguagem ou à biblioteca
padrão; você pode escrever seus próprios recursos de concorrência ou usar aqueles
escritos por outras pessoas.

No entanto, entre os conceitos-chave de concorrência que estão embutidos na
linguagem em vez de na biblioteca padrão estão as *traits* `Send` e `Sync` de
`std::marker`.

<!-- Old headings. Do not remove or links may break. -->

<a id="allowing-transference-of-ownership-between-threads-with-send"></a>

<a id="transferring-ownership-between-threads"></a>
### Transferindo *Ownership* Entre *Threads*

A *trait* marcadora `Send` indica que a propriedade (*ownership*) de valores do
tipo que implementa `Send` pode ser transferida entre *threads*. Quase todo tipo
em Rust implementa `Send`, mas há algumas exceções, incluindo `Rc<T>`: ele não
pode implementar `Send` porque, se você clonasse um valor `Rc<T>` e tentasse
transferir a propriedade do clone para outra *thread*, ambas as *threads*
poderiam atualizar a contagem de referências ao mesmo tempo. Por esse motivo,
`Rc<T>` é implementado para uso em situações de *thread* única, em que você não
quer pagar a penalidade de desempenho da segurança entre *threads*.

Portanto, o sistema de tipos e os *trait bounds* do Rust garantem que você nunca
possa enviar acidentalmente um valor `Rc<T>` entre *threads* de forma insegura.
Quando tentamos fazer isso na Listagem 16-14, obtivemos o erro `` the trait
`Send` is not implemented for `Rc<Mutex<i32>>` ``. Quando mudamos para `Arc<T>`,
que implementa `Send`, o código compilou.

Qualquer tipo composto inteiramente de tipos `Send` também é automaticamente
marcado como `Send`. Quase todos os tipos primitivos são `Send`, com exceção de
ponteiros brutos (*raw pointers*), que discutiremos no Capítulo 20.

<!-- Old headings. Do not remove or links may break. -->

<a id="allowing-access-from-multiple-threads-with-sync"></a>

<a id="accessing-from-multiple-threads"></a>
### Acessando a partir de Múltiplas *Threads*

A *trait* marcadora `Sync` indica que é seguro para o tipo que implementa `Sync`
ser referenciado a partir de múltiplas *threads*. Em outras palavras, qualquer
tipo `T` implementa `Sync` se `&T` (uma referência imutável para `T`) implementa
`Send`, o que significa que a referência pode ser enviada com segurança para
outra *thread*. De forma semelhante a `Send`, todos os tipos primitivos
implementam `Sync`, e tipos compostos inteiramente de tipos que implementam
`Sync` também implementam `Sync`.

O *smart pointer* `Rc<T>` também não implementa `Sync` pelos mesmos motivos
pelos quais não implementa `Send`. O tipo `RefCell<T>` (sobre o qual falamos no
Capítulo 15) e a família de tipos `Cell<T>` relacionados não implementam `Sync`.
A implementação de verificação de *borrowing* que `RefCell<T>` faz em tempo de
execução não é *thread-safe*. O *smart pointer* `Mutex<T>` implementa `Sync` e
pode ser usado para compartilhar acesso com múltiplas *threads*, como você viu
em ["Acesso Compartilhado a `Mutex<T>`"][shared-access]<!-- ignore -->.

<a id="implementing-send-and-sync-manually-is-unsafe"></a>
### Implementar `Send` e `Sync` Manualmente É *Unsafe*

Como tipos compostos inteiramente de outros tipos que implementam as *traits*
`Send` e `Sync` também implementam `Send` e `Sync` automaticamente, não
precisamos implementar essas *traits* manualmente. Como *traits* marcadoras,
elas nem mesmo possuem métodos para implementar. Elas são úteis apenas para
impor invariantes relacionadas à concorrência.

Implementar essas *traits* manualmente envolve escrever código Rust *unsafe*.
Falaremos sobre o uso de código Rust *unsafe* no Capítulo 20; por enquanto, a
informação importante é que construir novos tipos concorrentes que não sejam
feitos de partes `Send` e `Sync` exige cuidado para manter as garantias de
segurança. ["The Rustonomicon"][nomicon] tem mais informações sobre essas
garantias e como mantê-las.

<a id="summary"></a>
## Resumo

Esta não é a última vez que você verá concorrência neste livro: o próximo
capítulo foca em programação assíncrona, e o projeto no Capítulo 21 usará os
conceitos deste capítulo em uma situação mais realista do que os exemplos
menores discutidos aqui.

Como mencionado anteriormente, como muito pouco da forma como o Rust lida com
concorrência faz parte da linguagem, muitas soluções de concorrência são
implementadas como *crates*. Elas evoluem mais rapidamente do que a biblioteca
padrão, portanto certifique-se de pesquisar online pelas *crates* mais atuais
para usar em situações *multithreaded*.

A biblioteca padrão do Rust fornece canais para troca de mensagens e tipos de
*smart pointers*, como `Mutex<T>` e `Arc<T>`, que são seguros para uso em
contextos concorrentes. O sistema de tipos e o *borrow checker* garantem que o
código que usa essas soluções não acabe com corridas de dados ou referências
inválidas. Assim que você fizer seu código compilar, pode ficar tranquilo de que
ele executará tranquilamente em múltiplas *threads* sem os tipos de bugs
difíceis de rastrear comuns em outras linguagens. Programação concorrente não é
mais um conceito para se ter medo: vá em frente e torne seus programas
concorrentes, sem medo!

[shared-access]: ch16-03-shared-state.html#shared-access-to-mutext
[nomicon]: ../nomicon/index.html
