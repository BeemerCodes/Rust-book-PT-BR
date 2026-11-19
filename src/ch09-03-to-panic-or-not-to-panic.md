<a id="to-panic-or-not-to-panic"></a>

## Usar `panic!` ou Não Usar `panic!`

Então, como você decide quando deve chamar `panic!` e quando deve retornar `Result`? Quando o código entra em pânico, não há como se recuperar. Você poderia chamar `panic!` para qualquer situação de erro, independentemente de haver uma possível forma de recuperação ou não, mas aí você estaria tomando a decisão de que uma situação é irrecuperável em nome do código chamador. Quando você opta por retornar um valor `Result`, você dá opções ao código chamador. O código chamador pode optar por tentar se recuperar de uma forma adequada à sua situação, ou pode decidir que um valor `Err` nesse caso é irrecuperável, e então chamar `panic!`, transformando seu erro recuperável em irrecuperável. Portanto, retornar `Result` é uma boa escolha padrão quando você está definindo uma função que pode falhar.

Em situações como exemplos, código de protótipo e testes, é mais apropriado escrever código que entra em pânico em vez de retornar `Result`. Vamos explorar o motivo disso e depois discutir situações em que o compilador não consegue perceber que a falha é impossível, mas você, como humano, consegue. O capítulo concluirá com algumas diretrizes gerais sobre como decidir se é preciso entrar em pânico em código de biblioteca.

<a id="examples-prototype-code-and-tests"></a>

### Exemplos, Código de Protótipo e Testes

Quando você está escrevendo um exemplo para ilustrar algum conceito, incluir também código robusto de tratamento de erros pode tornar o exemplo menos claro. Em exemplos, entende-se que uma chamada a um método como `unwrap`, que pode entrar em pânico, é um marcador para a forma como você gostaria que sua aplicação tratasse os erros, o que pode variar dependendo do que o resto do seu código está fazendo.

Da mesma forma, os métodos `unwrap` e `expect` são muito úteis quando você está prototipando e ainda não está pronto para decidir como tratar os erros. Eles deixam marcadores claros no seu código para quando você estiver pronto para tornar seu programa mais robusto.

Se uma chamada de método falhar em um teste, você vai querer que o teste inteiro falhe, mesmo que esse método não seja a funcionalidade sendo testada. Como `panic!` é a forma de marcar um teste como falho, chamar `unwrap` ou `expect` é exatamente o que deve acontecer.

<!-- Old headings. Do not remove or links may break. -->

<a id="cases-in-which-you-have-more-information-than-the-compiler"></a>

### Quando Você Tem Mais Informações que o Compilador

Também seria apropriado chamar `expect` quando você tem alguma outra lógica que garante que o `Result` terá um valor `Ok`, mas essa lógica não é algo que o compilador entende. Você ainda terá um valor `Result` que precisa tratar: qualquer operação que você está chamando ainda tem a possibilidade de falhar em geral, mesmo que seja logicamente impossível na sua situação específica. Se você consegue garantir por inspeção manual do código que nunca terá uma variante `Err`, é perfeitamente aceitável chamar `expect` e documentar o motivo pelo qual você acredita que nunca terá uma variante `Err` no texto do argumento. Veja um exemplo:

```rust
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-08-unwrap-that-cant-fail/src/main.rs:here}}
```

Estamos criando uma instância de `IpAddr` ao analisar uma *string* fixada no código. Podemos ver que `127.0.0.1` é um endereço IP válido, portanto é aceitável usar `expect` aqui. No entanto, ter uma *string* válida fixada no código não muda o tipo de retorno do método `parse`: ainda obtemos um valor `Result`, e o compilador ainda nos fará tratar o `Result` como se a variante `Err` fosse uma possibilidade, porque o compilador não é inteligente o suficiente para ver que essa *string* é sempre um endereço IP válido. Se a *string* do endereço IP viesse de um usuário em vez de estar fixada no programa e, portanto, _tivesse_ uma possibilidade de falha, definitivamente vamos querer tratar o `Result` de forma mais robusta. Mencionar a suposição de que esse endereço IP está fixado no código nos incentivará a mudar `expect` para um código de tratamento de erros melhor se, no futuro, precisarmos obter o endereço IP de outra fonte.

<a id="guidelines-for-error-handling"></a>

### Diretrizes para Tratamento de Erros

É aconselhável que seu código entre em pânico quando é possível que ele acabe em um estado ruim. Nesse contexto, um _estado ruim_ é quando alguma suposição, garantia, contrato ou invariante foi violado, como quando valores inválidos, contraditórios ou ausentes são passados para o seu código — mais uma ou mais das seguintes condições:

- O estado ruim é algo inesperado, ao contrário de algo que provavelmente ocorrerá ocasionalmente, como um usuário inserindo dados em formato incorreto.
- Seu código após esse ponto precisa confiar em não estar nesse estado ruim, em vez de verificar o problema a cada etapa.
- Não há uma boa forma de codificar essas informações nos tipos que você usa. Trabalharemos em um exemplo do que queremos dizer em ["Codificando Estados e Comportamentos como Tipos"][encoding]<!-- ignore --> no Capítulo 18.

Se alguém chamar seu código e passar valores que não fazem sentido, é melhor retornar um erro se puder para que o usuário da biblioteca possa decidir o que fazer nesse caso. No entanto, em casos onde continuar poderia ser inseguro ou prejudicial, a melhor escolha pode ser chamar `panic!` e alertar a pessoa que usa sua biblioteca sobre o bug no código dela para que ela possa corrigi-lo durante o desenvolvimento. Da mesma forma, `panic!` costuma ser adequado se você estiver chamando código externo fora do seu controle que retorna um estado inválido que você não tem como corrigir.

No entanto, quando a falha é esperada, é mais apropriado retornar um `Result` do que fazer uma chamada a `panic!`. Exemplos incluem um analisador recebendo dados malformados ou uma requisição HTTP retornando um status que indica que você atingiu um limite de taxa. Nesses casos, retornar um `Result` indica que a falha é uma possibilidade esperada que o código chamador deve decidir como tratar.

Quando seu código realiza uma operação que pode colocar um usuário em risco se for chamada com valores inválidos, seu código deve verificar primeiro se os valores são válidos e entrar em pânico se não forem. Isso é principalmente por razões de segurança: tentar operar com dados inválidos pode expor seu código a vulnerabilidades. Essa é a principal razão pela qual a biblioteca padrão chamará `panic!` se você tentar um acesso à memória fora dos limites: tentar acessar memória que não pertence à estrutura de dados atual é um problema de segurança comum. Funções frequentemente têm _contratos_: seu comportamento só é garantido se as entradas atenderem a requisitos específicos. Entrar em pânico quando o contrato é violado faz sentido porque uma violação de contrato sempre indica um bug do lado do chamador, e não é um tipo de erro que você quer que o código chamador precise tratar explicitamente. Na verdade, não há maneira razoável para o código chamador se recuperar; os _programadores_ chamadores precisam corrigir o código. Os contratos de uma função, especialmente quando uma violação causará pânico, devem ser explicados na documentação da API da função.

No entanto, ter muitas verificações de erro em todas as suas funções seria verboso e irritante. Felizmente, você pode usar o sistema de tipos do Rust (e, portanto, a verificação de tipos feita pelo compilador) para fazer muitas das verificações por você. Se sua função tem um tipo específico como parâmetro, você pode prosseguir com a lógica do seu código sabendo que o compilador já garantiu que você tem um valor válido. Por exemplo, se você tem um tipo em vez de um `Option`, seu programa espera ter _alguma coisa_ em vez de _nada_. Seu código não precisa tratar dois casos para as variantes `Some` e `None`: ele só terá um caso para definitivamente ter um valor. Código tentando passar nada para sua função nem mesmo compilará, portanto sua função não precisa verificar esse caso em tempo de execução. Outro exemplo é usar um tipo inteiro sem sinal como `u32`, que garante que o parâmetro nunca seja negativo.

<!-- Old headings. Do not remove or links may break. -->

<a id="creating-custom-types-for-validation"></a>

### Tipos Personalizados para Validação

Vamos levar a ideia de usar o sistema de tipos do Rust para garantir que temos um valor válido um passo além e ver como criar um tipo personalizado para validação. Lembre-se do jogo de adivinhação do Capítulo 2, em que nosso código pedia ao usuário para adivinhar um número entre 1 e 100. Nunca validamos que o palpite do usuário estava entre esses números antes de verificá-lo contra nosso número secreto; apenas validamos que o palpite era positivo. Nesse caso, as consequências não eram muito graves: nossa saída de "Alto demais" ou "Baixo demais" ainda estaria correta. Mas seria uma melhoria útil guiar o usuário em direção a palpites válidos e ter comportamento diferente quando o usuário adivinha um número fora do intervalo versus quando o usuário digita, por exemplo, letras.

Uma maneira de fazer isso seria analisar o palpite como um `i32` em vez de apenas um `u32` para permitir números potencialmente negativos e, em seguida, adicionar uma verificação para o número estar no intervalo, assim:

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-09-guess-out-of-range/src/main.rs:here}}
```

</Listing>

A expressão `if` verifica se nosso valor está fora do intervalo, informa o usuário sobre o problema e chama `continue` para iniciar a próxima iteração do *loop* e pedir outro palpite. Após a expressão `if`, podemos prosseguir com as comparações entre `guess` e o número secreto sabendo que `guess` está entre 1 e 100.

No entanto, esta não é uma solução ideal: se fosse absolutamente crítico que o programa operasse apenas com valores entre 1 e 100, e ele tivesse muitas funções com esse requisito, ter uma verificação como essa em cada função seria tedioso (e poderia impactar o desempenho).

Em vez disso, podemos criar um novo tipo em um módulo dedicado e colocar as validações em uma função para criar uma instância do tipo, em vez de repetir as validações em todo lugar. Dessa forma, é seguro para as funções usar o novo tipo em suas assinaturas e usar com confiança os valores que recebem. A Listagem 9-13 mostra uma forma de definir um tipo `Guess` que só criará uma instância de `Guess` se a função `new` receber um valor entre 1 e 100.

<Listing number="9-13" caption="Um tipo `Guess` que só continuará com valores entre 1 e 100" file-name="src/guessing_game.rs">

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-13/src/guessing_game.rs}}
```

</Listing>

Note que esse código em *src/guessing_game.rs* depende de adicionar uma declaração de módulo `mod guessing_game;` em *src/lib.rs* que não mostramos aqui. Dentro do arquivo desse novo módulo, definimos uma *struct* chamada `Guess` que tem um campo chamado `value` que armazena um `i32`. É aqui que o número será armazenado.

Em seguida, implementamos uma função associada chamada `new` em `Guess` que cria instâncias de valores `Guess`. A função `new` é definida para ter um parâmetro chamado `value` do tipo `i32` e para retornar um `Guess`. O código no corpo da função `new` testa `value` para garantir que está entre 1 e 100. Se `value` não passar nesse teste, fazemos uma chamada a `panic!`, que alertará o programador que está escrevendo o código chamador de que tem um bug que precisa ser corrigido, porque criar um `Guess` com um `value` fora desse intervalo violaria o contrato que `Guess::new` está usando. As condições em que `Guess::new` pode entrar em pânico devem ser discutidas na documentação da API voltada ao público; abordaremos as convenções de documentação que indicam a possibilidade de um `panic!` na documentação da API que você cria no Capítulo 14. Se `value` passar no teste, criamos um novo `Guess` com seu campo `value` definido como o parâmetro `value` e retornamos o `Guess`.

Em seguida, implementamos um método chamado `value` que faz *borrowing* de `self`, não tem outros parâmetros e retorna um `i32`. Esse tipo de método às vezes é chamado de _getter_ porque seu propósito é obter alguns dados de seus campos e retorná-los. Esse método público é necessário porque o campo `value` da *struct* `Guess` é privado. É importante que o campo `value` seja privado para que código que usa a *struct* `Guess` não possa definir `value` diretamente: o código fora do módulo `guessing_game` _deve_ usar a função `Guess::new` para criar uma instância de `Guess`, garantindo assim que não há como um `Guess` ter um `value` que não foi verificado pelas condições da função `Guess::new`.

Uma função que tem um parâmetro ou retorna apenas números entre 1 e 100 poderia, então, declarar em sua assinatura que recebe ou retorna um `Guess` em vez de um `i32` e não precisaria fazer nenhuma verificação adicional em seu corpo.

<a id="summary"></a>

## Resumo

Os recursos de tratamento de erros do Rust são projetados para ajudá-lo a escrever código mais robusto. A *macro* `panic!` sinaliza que seu programa está em um estado que não consegue lidar e permite que você diga ao processo para parar em vez de tentar prosseguir com valores inválidos ou incorretos. O *enum* `Result` usa o sistema de tipos do Rust para indicar que operações podem falhar de uma forma da qual seu código pode se recuperar. Você pode usar `Result` para dizer ao código que chama o seu código que ele também precisa tratar o sucesso ou falha potencial. Usar `panic!` e `Result` nas situações adequadas tornará seu código mais confiável diante de problemas inevitáveis.

Agora que você viu formas úteis pelas quais a biblioteca padrão usa genéricos com os *enums* `Option` e `Result`, falaremos sobre como os genéricos funcionam e como você pode usá-los no seu código.

[encoding]: ch18-03-oo-design-patterns.html#encoding-states-and-behavior-as-types
