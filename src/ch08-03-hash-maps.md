<a id="storing-keys-with-associated-values-in-hash-maps"></a>

## Armazenando Chaves com Valores Associados em Hash Maps

A última das nossas coleções comuns é o hash map. O tipo `HashMap<K, V>`
armazena um mapeamento de chaves do tipo `K` para valores do tipo `V` usando uma _função de espalhamento_
(_hashing function_), que determina como ele posiciona essas chaves e valores na memória.
Muitas linguagens de programação oferecem suporte a esse tipo de estrutura de dados, embora
costumem usar nomes diferentes, tais como _hash_, _map_, _objeto_, _tabela hash_,
_dicionário_ ou _array associativo_, para citar apenas alguns.

Hash maps são úteis quando você quer buscar dados não usando um índice numérico, como
pode fazer com vetores, mas sim usando uma chave que pode ser de qualquer tipo. Por exemplo,
em um jogo, você poderia rastrear a pontuação de cada time em um hash map no qual
cada chave é o nome de um time e os valores são a pontuação do respectivo time. Dado o nome
de um time, você pode recuperar a sua pontuação.

Passaremos pela API básica de hash maps nesta seção, mas muitas outras funcionalidades
interessantes estão escondidas nas funções definidas em `HashMap<K, V>` pela biblioteca padrão.
Como sempre, consulte a documentação da biblioteca padrão para mais detalhes.

<a id="creating-a-new-hash-map"></a>

### Criando um Novo Hash Map

Uma maneira de criar um hash map vazio é usar `new` e adicionar elementos com
`insert`. Na Listagem 8-20, estamos registrando as pontuações de dois times cujos
nomes são _Azul_ (*Blue*) e _Amarelo_ (*Yellow*). O time Azul começa com 10 pontos e o
time Amarelo começa com 50.

<Listing number="8-20" caption="Criando um novo hash map e inserindo algumas chaves e valores">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-20/src/main.rs:here}}
```

</Listing>

Observe que primeiro precisamos trazer o `HashMap` para o escopo com `use` a partir da parte de coleções
da biblioteca padrão. Das nossas três coleções comuns, esta é a usada com menor
frequência, por isso não está incluída nas funcionalidades trazidas para o escopo
automaticamente no prelúdio (*prelude*). Os hash maps também contam com menos suporte da
biblioteca padrão; não existe uma macro embutida para construí-los, por exemplo.

Assim como os vetores, os hash maps armazenam seus dados no *heap*. Este `HashMap` possui
chaves do tipo `String` e valores do tipo `i32`. Assim como vetores, hash maps são
homogêneos: todas as chaves devem ter o mesmo tipo e todos os valores
devem ter o mesmo tipo.

<a id="accessing-values-in-a-hash-map"></a>

### Acessando Valores em um Hash Map

Podemos obter um valor do hash map fornecendo sua chave ao método `get`,
como mostrado na Listagem 8-21.

<Listing number="8-21" caption="Acessando a pontuação do time Blue armazenada no hash map">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-21/src/main.rs:here}}
```

</Listing>

Aqui, `score` terá o valor associado ao time Azul, e o
resultado será `10`. O método `get` retorna um `Option<&V>`; se não houver um
valor para aquela chave no hash map, `get` retornará `None`. Esse programa
lida com o `Option` chamando `copied` para obter um `Option<i32>` em vez de um
`Option<&i32>`, e depois `unwrap_or` para definir `score` como zero se `scores` não
tiver uma entrada para a chave.

Podemos iterar sobre cada par chave-valor em um hash map de forma semelhante à que
fazemos com vetores, usando um laço `for`:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-03-iterate-over-hashmap/src/main.rs:here}}
```

Esse código imprimirá cada par em uma ordem arbitrária:

```text
Yellow: 50
Blue: 10
```

<!-- Old headings. Do not remove or links may break. -->

<a id="hash-maps-and-ownership"></a>

<a id="managing-ownership-in-hash-maps"></a>

### Gerenciando Ownership em Hash Maps

Para tipos que implementam a *trait* `Copy`, como `i32`, os valores são copiados
para o hash map. Para valores com posse (*owned*), como `String`, os valores serão movidos e
o hash map passará a ser o dono desses valores, conforme demonstrado na Listagem 8-22.

<Listing number="8-22" caption="Mostrando que chaves e valores pertencem ao hash map uma vez que são inseridos">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-22/src/main.rs:here}}
```

</Listing>

Não podemos usar as variáveis `field_name` e `field_value` depois
que elas foram movidas para dentro do hash map com a chamada a `insert`.

Se inserirmos referências a valores no hash map, os valores não serão movidos
para dentro dele. Os valores para os quais as referências apontam devem ser válidos por
pelo menos tanto tempo quanto o hash map for válido. Falaremos mais sobre essas questões na seção
[“Validando Referências com
Lifetimes”][validating-references-with-lifetimes]<!-- ignore --> no Capítulo 10.

<a id="updating-a-hash-map"></a>

### Atualizando um Hash Map

Embora o número de pares de chave e valor possa crescer, cada chave única só pode
ter um único valor associado a ela por vez (mas o oposto não é verdadeiro: por
exemplo, tanto o time Azul quanto o time Amarelo poderiam ter o valor `10`
armazenado no hash map `scores`).

Quando você quer alterar os dados em um hash map, precisa decidir como
tratar o caso em que uma chave já possui um valor atribuído. Você poderia substituir o
valor antigo pelo novo valor, desconsiderando completamente o antigo. Poderia
manter o valor antigo e ignorar o novo, adicionando o novo valor apenas se a
chave *não* possuir um valor ainda. Ou você poderia combinar o valor antigo e o
novo. Vamos ver como fazer cada uma dessas coisas!

<a id="overwriting-a-value"></a>

#### Sobrescrevendo um Valor

Se inserirmos uma chave e um valor em um hash map e, em seguida, inserirmos essa mesma chave
com um valor diferente, o valor associado a essa chave será substituído.
Mesmo que o código na Listagem 8-23 chame `insert` duas vezes, o hash map só
conterá um par chave-valor, porque estamos inserindo o valor para a chave do time
Azul em ambas as vezes.

<Listing number="8-23" caption="Substituindo um valor armazenado com uma determinada chave">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-23/src/main.rs:here}}
```

</Listing>

Esse código imprimirá `{"Blue": 25}`. O valor original de `10` foi
sobrescrito.

<!-- Old headings. Do not remove or links may break. -->

<a id="only-inserting-a-value-if-the-key-has-no-value"></a>

<a id="adding-a-key-and-value-only-if-a-key-isnt-present"></a>

#### Adicionando uma Chave e um Valor Apenas se a Chave Não Estiver Presente

É comum verificar se uma determinada chave já existe no hash map
com um valor e então tomar as seguintes ações: se a chave existir
no hash map, o valor existente deve permanecer como está; se a chave
não existir, insira-a com um valor correspondente.

Os hash maps têm uma API especial para isso chamada `entry`, que recebe como parâmetro a chave
que você deseja verificar. O valor de retorno do método `entry` é um *enum*
chamado `Entry`, que representa um valor que pode existir ou não. Digamos
que queiramos verificar se a chave do time Amarelo possui um valor associado
a ela. Se não tiver, queremos inserir o valor `50`, e o mesmo para o
time Azul. Usando a API `entry`, o código fica como na Listagem 8-24.

<Listing number="8-24" caption="Usando o método `entry` para inserir somente se a chave ainda não tiver um valor">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-24/src/main.rs:here}}
```

</Listing>

O método `or_insert` em `Entry` é definido para retornar uma referência mutável ao
valor da respectiva chave de `Entry` caso essa chave exista; se não existir, ele
insere o parâmetro como o novo valor para essa chave e retorna uma referência mutável
ao novo valor. Essa técnica é muito mais limpa do que escrevermos a
lógica manualmente e, além disso, interage muito melhor com o *borrow checker*.

Executar o código na Listagem 8-24 imprimirá `{"Yellow": 50, "Blue": 10}`. A
primeira chamada a `entry` inserirá a chave para o time Amarelo com o valor
`50`, pois o time Amarelo ainda não possui um valor. A segunda chamada a
`entry` não alterará o hash map, já que o time Azul já possui o
valor `10`.

<a id="updating-a-value-based-on-the-old-value"></a>

#### Atualizando um Valor com Base no Valor Antigo

Outro caso de uso comum para hash maps é buscar o valor de uma chave e, em seguida,
atualizá-lo com base no valor antigo. Por exemplo, a Listagem 8-25 mostra um código que
conta quantas vezes cada palavra aparece em um texto. Usamos um hash map com
as palavras como chaves e incrementamos o valor para acompanhar quantas vezes
já vimos aquela palavra. Se for a primeira vez que vemos uma palavra, primeiro inserimos
o valor `0`.

<Listing number="8-25" caption="Contando ocorrências de palavras usando um hash map que armazena palavras e contagens">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-25/src/main.rs:here}}
```

</Listing>

Esse código imprimirá `{"world": 2, "hello": 1, "wonderful": 1}`. Você pode ver
os mesmos pares chave-valor impressos em uma ordem diferente: lembre-se da seção [“Acessando
Valores em um Hash Map”][access]<!-- ignore --> que iterar sobre um hash map
ocorre em uma ordem arbitrária.

O método `split_whitespace` retorna um iterador sobre subfatias (*subslices*), separadas por
espaço em branco, do valor em `text`. O método `or_insert` retorna uma referência
mutável (`&mut V`) para o valor da chave especificada. Aqui, armazenamos essa
referência mutável na variável `count`, de modo que, para atribuir a esse valor,
precisamos primeiro desreferenciar `count` usando o asterisco (`*`). A referência
mutável sai de escopo ao final do laço `for`, portanto todas essas
alterações são seguras e permitidas pelas regras de empréstimo (*borrowing*).

<a id="hashing-functions"></a>

### Funções de Hashing

Por padrão, `HashMap` usa uma função de *hashing* chamada _SipHash_, que oferece
resistência a ataques de negação de serviço (DoS) envolvendo tabelas
hash[^siphash]<!-- ignore -->. Esse não é o algoritmo de *hashing* mais rápido
disponível, mas a compensação de maior segurança que acompanha a ligeira queda de
desempenho vale a pena. Se você analisar o perfil do seu código e descobrir que a função
padrão de *hashing* é lenta demais para suas necessidades, você pode mudar para outra função
especificando um *hasher* diferente. Um _hasher_ é um tipo que implementa a
*trait* `BuildHasher`. Falaremos sobre *traits* e como implementá-las no
[Capítulo 10][traits]<!-- ignore -->. Você não precisa necessariamente implementar
seu próprio *hasher* do zero; o [crates.io](https://crates.io/)<!-- ignore -->
possui bibliotecas compartilhadas por outros usuários do Rust que fornecem *hashers* implementando muitos
algoritmos comuns de *hashing*.

[^siphash]: [https://en.wikipedia.org/wiki/SipHash](https://en.wikipedia.org/wiki/SipHash)

<a id="summary"></a>

## Resumo

Vetores, strings e hash maps fornecerão uma grande quantidade de funcionalidades
essenciais em programas quando você precisar armazenar, acessar e modificar dados. Aqui estão
alguns exercícios que você agora deve estar apto a resolver:

1. Dada uma lista de inteiros, use um vetor e retorne a mediana (quando ordenado,
   o valor na posição central) e a moda (o valor que ocorre com mais
   frequência; um hash map será útil aqui) da lista.
2. Converta strings para o Pig Latin. A primeira consoante de cada palavra é movida para
   o fim da palavra e _ay_ é adicionado, logo _first_ vira _irst-fay_. Palavras
   que começam com vogal recebem _hay_ no final (_apple_ vira
   _apple-hay_). Lembre-se dos detalhes sobre a codificação UTF-8!
3. Usando um hash map e vetores, crie uma interface de texto para permitir que o usuário adicione
   nomes de funcionários a um departamento em uma empresa; por exemplo, “Add Sally to
   Engineering” ou “Add Amir to Sales”. Em seguida, permita que o usuário obtenha uma lista de
   todas as pessoas de um departamento ou de todas as pessoas da empresa agrupadas por departamento, ordenadas
   em ordem alfabética.

A documentação da API da biblioteca padrão descreve métodos que vetores, strings
e hash maps possuem e que serão muito úteis para esses exercícios!

Estamos avançando para programas mais complexos nos quais operações podem falhar, portanto este é
um momento perfeito para discutir tratamento de erros. Faremos isso a seguir!

[validating-references-with-lifetimes]: ch10-03-lifetime-syntax.html#validating-references-with-lifetimes
[access]: #accessing-values-in-a-hash-map
[traits]: ch10-02-traits.html
