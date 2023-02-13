pub struct Allergies(u32);

#[derive(Debug, PartialEq, Clone)]
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

const ALLERGENS: [Allergen; 8] = [
    Allergen::Eggs,
    Allergen::Peanuts,
    Allergen::Shellfish,
    Allergen::Strawberries,
    Allergen::Tomatoes,
    Allergen::Chocolate,
    Allergen::Pollen,
    Allergen::Cats,
];

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies(score)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let i = ALLERGENS.iter().position(|a| a == allergen).unwrap();
        self.0 & (1 << i) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        (0..8)
            .filter(|i| (self.0 & (1 << i)) != 0)
            .map(|i| ALLERGENS[i as usize].clone())
            .collect()
    }
}
