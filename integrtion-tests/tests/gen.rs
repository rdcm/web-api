use rand::Rng;
use uuid::Uuid;

pub struct Gen {}

impl Gen {
    pub fn random_string() -> String {
        Uuid::new_v4().to_string()
    }

    pub fn random_u8() -> u8 {
        let mut rng = rand::thread_rng();
        rng.gen()
    }
}
