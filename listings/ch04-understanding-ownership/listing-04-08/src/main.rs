fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// ANCHOR: here
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word receberá o valor 5

    s.clear(); // isso esvazia a String, tornando-a igual a ""

    // word ainda tem o valor 5 aqui, mas s não tem mais nenhum conteúdo que
    // possamos usar de forma significativa com o valor 5, então word agora é totalmente inválida!
}
// ANCHOR_END: here
