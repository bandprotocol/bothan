use glob::glob;

use bothan_api::registry::Registry;

#[tokio::test]
async fn test_registry() {
    for glob in glob("registry/*.json").unwrap() {
        let path = glob.unwrap();
        let registry = Registry::try_from_path(path).unwrap();
        assert!(registry.is_valid());
    }
}
