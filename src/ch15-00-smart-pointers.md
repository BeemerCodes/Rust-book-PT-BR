<a id="smart-pointers"></a>
# *Smart Pointers*

Um ponteiro é um conceito geral para uma variável que contém um endereço na
memória. Esse endereço se refere a, ou "aponta para", algum outro dado. O tipo
mais comum de ponteiro em Rust é a referência, que você aprendeu no Capítulo 4.
Referências são indicadas pelo símbolo `&` e realizam empréstimo (*borrowing*) do
valor para o qual apontam. Elas não têm nenhuma capacidade especial além de se
referenciar a dados, e não possuem custo adicional (*overhead*).

Os *smart pointers* (ponteiros inteligentes), por outro lado, são estruturas de
dados que agem como um ponteiro, mas também possuem metadados adicionais e
capacidades extras. O conceito de *smart pointers* não é exclusivo do Rust: eles
se originaram em C++ e existem em outras linguagens também. O Rust tem uma
variedade de *smart pointers* definidos na biblioteca padrão que fornecem
funcionalidades além das proporcionadas pelas referências. Para explorar o
conceito geral, vamos olhar alguns exemplos diferentes de *smart pointers*,
incluindo um tipo de *smart pointer* com *contagem de referências* (*reference
counting*). Esse ponteiro permite que você tenha múltiplos proprietários de um
dado, mantendo o controle do número de proprietários e, quando não restar
nenhum, limpando os dados.

Em Rust, com seus conceitos de *ownership* e *borrowing*, há uma diferença
adicional entre referências e *smart pointers*: enquanto referências apenas
realizam *borrowing* dos dados, em muitos casos os *smart pointers* *possuem* os
dados para os quais apontam.

Os *smart pointers* geralmente são implementados usando *structs*. Ao contrário
de uma *struct* comum, os *smart pointers* implementam as *traits* `Deref` e
`Drop`. A *trait* `Deref` permite que uma instância da *struct* de *smart pointer*
se comporte como uma referência, de modo que você possa escrever código que
funcione tanto com referências quanto com *smart pointers*. A *trait* `Drop`
permite que você personalize o código executado quando uma instância do *smart
pointer* sai de escopo. Neste capítulo, discutiremos ambas as *traits* e
demonstraremos por que elas são importantes para os *smart pointers*.

Como o padrão de *smart pointer* é um padrão de projeto geral usado com
frequência em Rust, este capítulo não cobrirá todos os *smart pointers*
existentes. Muitas bibliotecas têm seus próprios *smart pointers*, e você pode
até escrever os seus. Vamos abordar os *smart pointers* mais comuns da biblioteca
padrão:

- `Box<T>`, para alocar valores no *heap*
- `Rc<T>`, um tipo de *reference counting* que permite múltiplos proprietários
- `Ref<T>` e `RefMut<T>`, acessados por meio de `RefCell<T>`, um tipo que aplica
  as regras de *borrowing* em tempo de execução em vez de em tempo de compilação

Além disso, abordaremos o padrão de *mutabilidade interior* (*interior
mutability*), em que um tipo imutável expõe uma API para mutar um valor interno.
Também discutiremos *ciclos de referência* (*reference cycles*): como eles podem
vazar memória e como evitá-los.

Vamos começar!
