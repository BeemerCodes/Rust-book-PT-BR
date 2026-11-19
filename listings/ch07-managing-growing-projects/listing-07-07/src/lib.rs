// ANCHOR: here
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// -- snip --
// ANCHOR_END: here
pub fn eat_at_restaurant() {
    // Caminho absoluto
    crate::front_of_house::hosting::add_to_waitlist();

    // Caminho relativo
    front_of_house::hosting::add_to_waitlist();
}
