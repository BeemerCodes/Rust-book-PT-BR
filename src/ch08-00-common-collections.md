<a id="common-collections"></a>

# Coleções Comuns

A biblioteca padrão do Rust inclui diversas estruturas de dados muito úteis chamadas
_coleções_. A maioria dos outros tipos de dados representa um único valor específico, mas
coleções podem conter múltiplos valores. Ao contrário dos tipos embutidos de array e tupla,
os dados apontados por essas coleções ficam armazenados no *heap*, o que
significa que a quantidade de dados não precisa ser conhecida em tempo de compilação e pode crescer
ou diminuir conforme o programa executa. Cada tipo de coleção tem diferentes
capacidades e custos, e escolher a mais adequada para a sua situação atual
é uma habilidade que você vai desenvolver com o tempo. Neste capítulo, discutiremos
três coleções usadas com muita frequência em programas Rust:

- Um _vetor_ permite armazenar um número variável de valores lado a lado.
- Uma _string_ é uma coleção de caracteres. Já mencionamos o tipo `String`
  antes, mas neste capítulo falaremos sobre ele em profundidade.
- Um _hash map_ permite associar um valor a uma chave específica. É uma
  implementação particular de uma estrutura de dados mais geral chamada _map_.

Para conhecer os outros tipos de coleções fornecidos pela biblioteca padrão,
consulte [a documentação][collections].

Veremos como criar e atualizar vetores, strings e hash maps, bem como
o que torna cada um deles especial.

[collections]: ../std/collections/index.html
