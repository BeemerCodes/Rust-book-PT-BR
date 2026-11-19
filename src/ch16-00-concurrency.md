<a id="fearless-concurrency"></a>
# Concorrência Sem Medo

Lidar com programação concorrente de forma segura e eficiente é outro dos
principais objetivos do Rust. A *programação concorrente*, na qual diferentes
partes de um programa são executadas de forma independente, e a *programação
paralela*, na qual diferentes partes de um programa são executadas ao mesmo
tempo, estão se tornando cada vez mais importantes à medida que mais
computadores aproveitam seus múltiplos processadores. Historicamente, programar
nesses contextos tem sido difícil e propenso a erros. O Rust espera mudar isso.

Inicialmente, a equipe do Rust pensava que garantir a segurança de memória e
prevenir problemas de concorrência eram dois desafios separados a serem
resolvidos com métodos diferentes. Com o tempo, a equipe descobriu que os
sistemas de *ownership* e de tipos são um conjunto poderoso de ferramentas para
ajudar a gerenciar segurança de memória *e* problemas de concorrência! Ao
aproveitar o *ownership* e a verificação de tipos, muitos erros de concorrência
se tornam erros em tempo de compilação no Rust em vez de erros em tempo de
execução. Portanto, em vez de fazer você gastar muito tempo tentando reproduzir
as circunstâncias exatas em que um bug de concorrência em tempo de execução
ocorre, um código incorreto simplesmente se recusará a compilar e apresentará um
erro explicando o problema. Como resultado, você pode corrigir seu código
enquanto trabalha nele, em vez de potencialmente depois que ele já foi enviado
para produção. Apelidamos esse aspecto do Rust de *concorrência sem medo*
(*fearless concurrency*). A concorrência sem medo permite escrever código livre
de bugs sutis e fácil de refatorar sem introduzir novos bugs.

> Nota: Por simplicidade, nos referiremos a muitos dos problemas como
> *concorrentes* em vez de sermos mais precisos dizendo *concorrentes e/ou
> paralelos*. Neste capítulo, substitua mentalmente *concorrente* por
> *concorrente e/ou paralelo* sempre que usarmos *concorrente*. No próximo
> capítulo, onde a distinção importa mais, seremos mais específicos.

Muitas linguagens são dogmáticas sobre as soluções que oferecem para lidar com
problemas concorrentes. Por exemplo, Erlang tem uma funcionalidade elegante para
concorrência por troca de mensagens, mas possui apenas maneiras obscuras de
compartilhar estado entre *threads*. Suportar apenas um subconjunto das soluções
possíveis é uma estratégia razoável para linguagens de mais alto nível, porque
uma linguagem de mais alto nível promete benefícios ao abrir mão de algum
controle para ganhar abstrações. No entanto, espera-se que linguagens de mais
baixo nível forneçam a solução com o melhor desempenho em qualquer situação e
tenham menos abstrações sobre o hardware. Portanto, o Rust oferece uma variedade
de ferramentas para modelar problemas da maneira que for mais apropriada para
sua situação e seus requisitos.

Aqui estão os tópicos que abordaremos neste capítulo:

- Como criar *threads* para executar múltiplos trechos de código ao mesmo tempo
- Concorrência por *troca de mensagens* (*message-passing*), em que canais
  enviam mensagens entre *threads*
- Concorrência por *estado compartilhado* (*shared-state*), em que múltiplas
  *threads* têm acesso a algum dado
- As *traits* `Sync` e `Send`, que estendem as garantias de concorrência do Rust
  para tipos definidos pelo usuário, bem como para tipos fornecidos pela
  biblioteca padrão
