use uuid::Uuid;

pub struct Gen { }

impl Gen {
    pub fn random_string() -> String {
        Uuid::new_v4().to_string()
    }
}