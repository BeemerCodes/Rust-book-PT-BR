mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Pede um café da manhã no verão com torrada de centeio.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Muda de ideia sobre qual pão queremos.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // A próxima linha não compilará se a descomentarmos; não temos permissão
    // para ver ou modificar a fruta da estação que acompanha a refeição.
    // meal.seasonal_fruit = String::from("blueberries");
}
