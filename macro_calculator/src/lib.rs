pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut total_cal = 0.0;
    let mut total_carbs = 0.0;
    let mut total_prot = 0.0;
    let mut total_fats = 0.0;

    for food in foods {
        let portion = food.nbr_of_portions;
        let kcal: f64 = food.calories.1.trim_end_matches("kcal").parse().unwrap();

        total_cal += kcal * portion;
        total_carbs += food.carbs * portion;
        total_prot += food.proteins * portion;
        total_fats += food.fats * portion;
    }
    json::object! {
        "cals": round_to_two_decimal_places(total_cal),
        "carbs": round_to_two_decimal_places(total_carbs),
        "proteins": round_to_two_decimal_places(total_prot),
        "fats": round_to_two_decimal_places(total_fats),
    }
}

pub fn round_to_two_decimal_places(num: f64) -> f64 {
    (num * 100.0).round() / 100.0
}