<a id="appendix-e-editions"></a>
## Apêndice E: Edições

No Capítulo 1, você viu que `cargo new` adiciona alguns metadados ao seu arquivo
_Cargo.toml_ sobre uma edição (*edition*). Este apêndice explica o que isso
significa!

A linguagem e o compilador Rust têm um ciclo de lançamento de seis semanas, o
que significa que as pessoas usuárias recebem um fluxo constante de novos
recursos. Outras linguagens de programação lançam mudanças maiores com menos
frequência; o Rust lança atualizações menores com mais frequência. Depois de um
tempo, todas essas pequenas mudanças se acumulam. Mas, de um lançamento para o
outro, pode ser difícil olhar para trás e dizer: “Nossa, entre o Rust 1.10 e o
Rust 1.31, o Rust mudou bastante!”

Aproximadamente a cada três anos, a equipe do Rust produz uma nova _edição_ do
Rust. Cada edição reúne os recursos que chegaram à linguagem em um conjunto
claro, com documentação e ferramentas totalmente atualizadas. As novas edições
são entregues como parte do processo normal de lançamento a cada seis semanas.

As edições atendem a propósitos diferentes para pessoas diferentes:

- Para quem já usa Rust ativamente, uma nova edição reúne mudanças incrementais
  em um conjunto fácil de entender.
- Para quem ainda não usa Rust, uma nova edição sinaliza que avanços
  importantes chegaram, o que pode fazer com que valha a pena dar uma nova
  olhada na linguagem.
- Para quem desenvolve o Rust, uma nova edição oferece um ponto de convergência
  para o projeto como um todo.

No momento em que este livro foi escrito, há quatro edições do Rust
disponíveis: Rust 2015, Rust 2018, Rust 2021 e Rust 2024. Este livro foi escrito
utilizando os idiomas da edição Rust 2024.

A chave `edition` no _Cargo.toml_ indica qual edição o compilador deve usar para
o seu código. Se a chave não existir, o Rust usa `2015` como valor da edição por
motivos de compatibilidade retroativa.

Cada projeto pode optar por uma edição diferente da edição padrão 2015. As
edições podem conter mudanças incompatíveis, como a inclusão de uma nova
palavra-chave que entre em conflito com identificadores no código. No entanto, a
menos que você opte por adotar essas mudanças, seu código continuará compilando
mesmo quando você atualizar a versão do compilador Rust utilizada.

Todas as versões do compilador Rust dão suporte a qualquer edição que existia
antes do lançamento daquela versão do compilador, e elas conseguem vincular
*crates* de quaisquer edições suportadas entre si. Mudanças de edição afetam
apenas a forma como o compilador faz o parsing inicial do código. Portanto, se
você estiver usando Rust 2015 e uma de suas dependências usar Rust 2018, seu
projeto compilará e conseguirá usar essa dependência. A situação oposta, em que
seu projeto usa Rust 2018 e uma dependência usa Rust 2015, também funciona.

Para deixar claro: a maioria dos recursos estará disponível em todas as edições.
Pessoas desenvolvedoras que usam qualquer edição do Rust continuarão vendo
melhorias à medida que novos lançamentos estáveis forem feitos. Contudo, em
alguns casos — principalmente quando novas palavras-chave são adicionadas —,
certos recursos novos podem ficar disponíveis apenas em edições posteriores.
Você precisará mudar de edição se quiser aproveitar esses recursos.

Para mais detalhes, consulte [_The Rust Edition Guide_][edition-guide]. Trata-se
de um livro completo que enumera as diferenças entre as edições e explica como
atualizar automaticamente seu código para uma nova edição por meio do comando
`cargo fix`.

[edition-guide]: https://doc.rust-lang.org/stable/edition-guide
