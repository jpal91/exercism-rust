pub struct Allergies {
    allergies: Vec<Allergen>
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let mut score = score % 256;
        let mut allergies: Vec<Allergen> = vec![];

        for a in Allergen::iter() {
            let a_score = a as u32;
            
            if a_score > score {
                continue
            }

            score -= a_score;
            allergies.push(a)
        };

        Self { allergies }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergies.clone()
    }
}

impl Allergen {
    fn iter() -> std::array::IntoIter<Allergen, 8> {
        [
            Allergen::Cats,
            Allergen::Pollen,
            Allergen::Chocolate,
            Allergen::Tomatoes,
            Allergen::Strawberries,
            Allergen::Shellfish,
            Allergen::Peanuts,
            Allergen::Eggs
        ].into_iter()
    }
}