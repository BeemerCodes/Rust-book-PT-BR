static mut COUNTER: u32 = 0;

/// SAFETY: Chamar isto a partir de mais de uma thread por vez é comportamento
/// indefinido, então você *precisa* garantir que só a chamará a partir de uma
/// única thread por vez.
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    unsafe {
        // SAFETY: Isto só é chamado a partir de uma única thread em `main`.
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }
}
