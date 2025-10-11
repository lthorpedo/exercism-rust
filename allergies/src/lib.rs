fn convert_dec_to_binary(number: u32) -> Vec<u32> {
    let nums: Vec<u32> = number.to_string()
        .split("")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let from_base = 10;
    let to_base = 2;

    let mut base_10 = 0;

    for digit in nums {
        base_10 *= from_base;
        base_10 += digit;
    }

    if base_10 == 0 {
        return vec![0];
    }

    let mut n = base_10;
    let mut vec = Vec::new();

    while n > 0 {
        vec.insert(0, n % to_base);
        n /= to_base;
    }

    vec
}

pub struct Allergies {
    allers: Vec<Allergen>,
}

#[derive(Debug, PartialEq, Eq)]
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

    pub fn new(score: u32) -> Self {
        //todo!("Given the '{score}' score, construct a new Allergies struct.");
        let boost = convert_dec_to_binary(score);

        println!("{:?}", boost);
        todo!("yep");
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        todo!("Determine if the patient is allergic to the '{allergen:?}' allergen.");
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        todo!(
            "Return the list of allergens contained within the score with which the Allergies struct was made."
        );
    }


}
