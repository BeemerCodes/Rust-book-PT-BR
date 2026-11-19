// ANCHOR: here
use gui::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // código para realmente desenhar uma caixa de seleção
    }
}
// ANCHOR_END: here

fn main() {}
