<!-- Old headings. Do not remove or links may break. -->

<a id="managing-growing-projects-with-packages-crates-and-modules"></a>

<a id="packages-crates-and-modules"></a>
# Packages, Crates e Módulos

À medida que você escreve programas grandes, organizar o seu código se torna
cada vez mais importante. Ao agrupar funcionalidades relacionadas e separar
códigos com recursos distintos, fica muito mais claro onde encontrar o código
que implementa uma determinada funcionalidade e onde ir para alterar como ela
funciona.

Os programas que escrevemos até agora ficaram em um único módulo dentro de um
único arquivo. Conforme um projeto cresce, você deve organizar o código
dividindo-o em vários módulos e, depois, em vários arquivos. Um *package* pode
conter vários *crates* binários e, opcionalmente, um *crate* de biblioteca. À
medida que um *package* cresce, você pode extrair partes dele para *crates*
separados que se tornam dependências externas. Este capítulo aborda todas essas
técnicas. Para projetos muito grandes compostos por um conjunto de *packages*
inter-relacionados que evoluem juntos, o Cargo oferece workspaces, que
abordaremos em [“Workspaces do Cargo”][workspaces]<!-- ignore --> no Capítulo
14.

Também discutiremos o encapsulamento de detalhes de implementação, o que permite
reutilizar código em um nível mais alto: depois de implementar uma operação,
outro código pode chamá-la por meio de sua interface pública sem precisar saber
como a implementação funciona. A forma como você escreve o código define quais
partes são públicas para outros códigos usarem e quais partes são detalhes
privados de implementação que você se reserva o direito de alterar. Essa é outra
maneira de limitar a quantidade de detalhes que você precisa manter na cabeça.

Um conceito relacionado é o de escopo: o contexto aninhado no qual o código é
escrito possui um conjunto de nomes definidos como “em escopo”. Ao ler, escrever
e compilar código, programadores e compiladores precisam saber se um determinado
nome em um determinado ponto se refere a uma variável, função, *struct*, *enum*,
módulo, constante ou outro item, e o que esse item significa. Você pode criar
escopos e alterar quais nomes estão dentro ou fora de escopo. Não é possível ter
dois itens com o mesmo nome no mesmo escopo; existem ferramentas disponíveis
para resolver conflitos de nomes.

O Rust possui vários recursos que permitem gerenciar a organização do seu
código, incluindo quais detalhes são expostos, quais detalhes são privados e
quais nomes estão em cada escopo dos seus programas. Esses recursos, às vezes
chamados coletivamente de _sistema de módulos_, incluem:

* **Packages**: Um recurso do Cargo que permite compilar, testar e compartilhar
  *crates*
* **Crates**: Uma árvore de módulos que produz uma biblioteca ou um executável
* **Módulos e `use`**: Permitem controlar a organização, o escopo e a
  privacidade dos caminhos
* **Caminhos**: Uma forma de nomear um item, como uma *struct*, função ou módulo

Neste capítulo, abordaremos todos esses recursos, discutiremos como eles
interagem e explicaremos como usá-los para gerenciar escopos. Ao final, você
terá um entendimento sólido do sistema de módulos e conseguirá trabalhar com
escopos como um profissional!

[workspaces]: ch14-03-cargo-workspaces.html
