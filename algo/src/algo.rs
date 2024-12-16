use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug, Clone, Copy)]
pub enum AlgoType {
    Blake3,
    Default,
}

#[derive(Debug, Clone)]
pub struct Algo {
    r#type: AlgoType,
}

impl Algo {
    pub fn new(r#type: AlgoType) -> Algo {
        Self { r#type }
    }

    pub fn get_name(&self) -> &str {
        match self.r#type {
            AlgoType::Blake3 => "Blake3",
            AlgoType::Default => "Default",
        }
    }

    pub fn hash(&self, data: String) -> String {
        match self.r#type {
            AlgoType::Blake3 => {
                let hash = blake3::hash(data.as_bytes());
                hash.to_string()
            }
            AlgoType::Default => {
                let mut sip = DefaultHasher::new();
                data.hash(&mut sip);
                sip.finish().to_string()
            }
        }
    }
}
