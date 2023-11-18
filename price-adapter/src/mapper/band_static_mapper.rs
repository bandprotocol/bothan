use super::Mapper;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub struct BandStaticMapper {
    mapping: HashMap<String, Value>,
}

impl BandStaticMapper {
    pub fn new(source: String) -> Self {
        // Read the JSON file content
        let mut file = File::open(format!("resources/{}.json", source)).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        // Deserialize the JSON content into a Vec<Mapping>
        let mapping = serde_json::from_str(&content).unwrap();

        Self { mapping }
    }
}

impl Mapper for BandStaticMapper {
    fn get_mapping(&self) -> &HashMap<String, Value> {
        &self.mapping
    }
}
