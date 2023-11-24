use crate::error::Error;
use crate::types::Mapper;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// A struct representing a static mapper using a HashMap of String keys to Values.
pub struct BandStaticMapper {
    mapping: HashMap<String, Value>,
}

impl BandStaticMapper {
    /// Constructor to create a new BandStaticMapper from a pre-existing mapping.
    pub fn new(mapping: HashMap<String, Value>) -> Self {
        Self { mapping }
    }

    /// Constructor to create a BandStaticMapper from a source file.
    pub fn from_source(source: &str) -> Result<Self, Error> {
        let content = match source {
            "binance" => Ok(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/resources/binance.json"
            ))),
            "coingecko" => Ok(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/resources/coingecko.json"
            ))),
            _ => Err(Error::UnsupportedSource), // Add more matches for other sources
        }?;

        // Deserialize the JSON content into a HashMap<String, Value>.
        let mapping = serde_json::from_str(&content)?;

        Ok(Self { mapping })
    }

    /// Constructor to create a BandStaticMapper from a file path.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        // Attempt to open the file at the specified path.
        let mut file = File::open(&path)?;

        // Read the file content into a String.
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        // Deserialize the JSON content into a HashMap<String, Value>.
        let mapping = serde_json::from_str(&content)?;

        Ok(Self { mapping })
    }
}

/// Implementing the Mapper trait for BandStaticMapper.
#[async_trait::async_trait]
impl Mapper for BandStaticMapper {
    /// Retrieve the mapping as a reference, wrapped in a Result.
    async fn get_mapping(&self) -> Result<&HashMap<String, Value>, Error> {
        Ok(&self.mapping)
    }
}
