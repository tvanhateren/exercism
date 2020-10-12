use std::slice::Iter;

pub struct Allergies {
    score: usize,
}

#[derive(Debug, PartialEq, Copy, Clone)]
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

impl Allergies {
    fn variants() -> Iter<'static, Allergen> {
        static ALLERGY_LIST: &'static [Allergen] = &[
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ];
        ALLERGY_LIST.iter()
    }

    pub fn new(score: usize) -> Self {
        Allergies { score }
    }

    fn allergen_score(allergen: &Allergen) -> usize {
        let index = Allergies::variants()
            .enumerate()
            .find(|item| item.1 == allergen)
            .unwrap()
            .0;

        usize::pow(2, index as u32)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        Self::allergen_score(allergen) & self.score > 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergies::variants()
            .filter_map(|allergen| {
                if self.is_allergic_to(allergen) {
                    Some(*allergen)
                } else {
                    None
                }
            })
            .collect()
    }
}
