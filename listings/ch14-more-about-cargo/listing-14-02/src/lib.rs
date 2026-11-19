// ANCHOR: here
//! # My Crate
//!
//! `my_crate` é uma coleção de utilitários para tornar a realização de certos
//! cálculos mais conveniente.

/// Adiciona um ao número fornecido.
// --snip--
// ANCHOR_END: here
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
