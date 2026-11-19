// ANCHOR: here
//! # Art
//!
//! Uma biblioteca para modelar conceitos artísticos.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    // --snip--
    // ANCHOR_END: here
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
    // ANCHOR: here
}

pub mod utils {
    // --snip--
    // ANCHOR_END: here
    use crate::kinds::*;

    /// Combina duas cores primárias em quantidades iguais para criar
    /// uma cor secundária.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Orange
    }
    // ANCHOR: here
}
// ANCHOR_END: here
