use crate::core::utils::string_to_u8_40_array;
use rand::Rng;

pub struct User {
    age: u8,
    id: Vec<u8>,
    country: String,
}

impl User {
    pub fn new(age: u8, country: &str) -> Self {
        let mut rng = rand::thread_rng();
        User {
            age,
            id: (0..4).map(|_| rng.gen_range(u8::MIN..=u8::MAX)).collect(),
            country: country.to_owned(),
        }
    }

    pub fn age(&self) -> u8 {
        self.age
    }

    pub fn id(&self) -> Vec<u8> {
        self.id.to_owned()
    }

    pub fn country(&self) -> [u8; 40] {
        string_to_u8_40_array(&self.country)
    }
}
