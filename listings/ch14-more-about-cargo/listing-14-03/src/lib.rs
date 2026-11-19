// ANCHOR: here
//! # Art
//!
//! Uma biblioteca para modelar conceitos artísticos.

pub mod kinds {
    /// As cores primárias de acordo com o modelo de cores RYB.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// As cores secundárias de acordo com o modelo de cores RYB.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combina duas cores primárias em quantidades iguais para criar
    /// uma cor secundária.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
        // ANCHOR_END: here
        unimplemented!();
        // ANCHOR: here
    }
}
// ANCHOR_END: here
