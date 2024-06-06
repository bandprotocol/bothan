use bothan_api::registry::{Registry, Validator};
use glob::glob;

#[tokio::test]
async fn test_registry() {
    for glob in glob("registry/*.json").unwrap() {
        let path = glob.unwrap();
        let file = std::fs::File::open(path).unwrap();
        let registry: Registry = serde_json::from_reader(file).unwrap();
        assert!(registry.validate());
    }
}
