use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Allergen {
    pub(crate) code: String,
    pub(crate) name: String,
}

#[derive(Debug, Deserialize)]
pub struct Language {
    pub(crate) code: String,
    pub(crate) name: String,
}


#[derive(Debug, Deserialize)]
pub struct Location {
    pub(crate) code: String,
    pub(crate) name: String,
    pub(crate) city: String,
    pub(crate) available_languages: Option<Vec<Language>>
}

#[derive(Debug, Deserialize)]
pub struct Meal {
    pub(crate) name: String,
    pub(crate) price: MealPrice,
    pub(crate) vegan: bool,
    pub(crate) vegetarian: bool,
    pub(crate) location: Location,
}

#[derive(Debug, Deserialize)]
pub struct MealPrice {
    pub(crate) students: f32,
    pub(crate) guests: f32,
    pub(crate) employees: f32,
}

#[derive(Debug, Deserialize)]
pub struct Data<T> {
    pub(crate)last_updated: chrono::DateTime<chrono::Utc>,
    pub(crate)data: T
}


