use glob::glob;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use bothan_core::registry::Registry;
use bothan_core::tasks::TaskSet;

#[tokio::test]
async fn test_registry_validity() {
    for glob in glob("registry/*.json").unwrap() {
        let path = glob.unwrap();
        let mut file = File::open(&path).await.unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).await.unwrap();

        let registry =
            serde_json::from_str::<Registry>(&buffer).expect("Failed to parse registry file");
        TaskSet::try_from(registry).expect("Failed to create tasks from registry");
    }
}
