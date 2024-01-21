use self::Allergen::*;

pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

const ALLERGENS: [Allergen; 8] =
    [Eggs, Peanuts, Shellfish, Strawberries, Tomatoes, Chocolate, Pollen, Cats];

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score: score % 256 }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let bits = 1 << map_to_index(allergen);
        bits == self.score & bits
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        ALLERGENS.into_iter().filter(|allergen| self.is_allergic_to(allergen)).collect()
    }
}

fn map_to_index(allergen: &Allergen) -> u32 {
    match allergen {
        Allergen::Eggs => 0,
        Allergen::Peanuts => 1,
        Allergen::Shellfish => 2,
        Allergen::Strawberries => 3,
        Allergen::Tomatoes => 4,
        Allergen::Chocolate => 5,
        Allergen::Pollen => 6,
        Allergen::Cats => 7,
    }
}
