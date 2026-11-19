<a id="error-handling"></a>

# Tratamento de Erros

Erros são uma realidade em qualquer software, e por isso o Rust oferece diversos recursos para lidar com situações em que algo dá errado. Em muitos casos, o Rust exige que você reconheça a possibilidade de um erro e tome alguma providência antes que seu código compile. Essa exigência torna seus programas mais robustos, garantindo que você descubra os erros e os trate adequadamente antes de colocar seu código em produção!

O Rust classifica os erros em duas grandes categorias: erros recuperáveis e erros irrecuperáveis. Para um _erro recuperável_, como um erro de _arquivo não encontrado_, normalmente queremos apenas informar o problema ao usuário e tentar a operação novamente. _Erros irrecuperáveis_ são sempre sintomas de bugs, como tentar acessar uma posição além do final de um vetor, e nesse caso queremos interromper o programa imediatamente.

A maioria das linguagens não distingue entre esses dois tipos de erro e os trata da mesma forma, usando mecanismos como exceções. O Rust não tem exceções. Em vez disso, ele possui o tipo `Result<T, E>` para erros recuperáveis e a *macro* `panic!`, que interrompe a execução quando o programa encontra um erro irrecuperável. Este capítulo aborda primeiro a chamada ao `panic!` e depois fala sobre como retornar valores `Result<T, E>`. Além disso, vamos explorar as considerações a fazer na hora de decidir se devemos tentar se recuperar de um erro ou parar a execução.
