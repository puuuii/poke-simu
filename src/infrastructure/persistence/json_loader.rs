use log::{error, info};
use serde::de::DeserializeOwned;
use std::fs;
use std::path::Path;

/// 指定されたディレクトリからJSONファイルを読み込んで、指定された型にデシリアライズする
pub fn load_json_from_directory<T>(directory: &str, type_name: &str) -> Vec<T>
where
    T: DeserializeOwned,
{
    info!("Loading {} from file system...", type_name);
    let mut items = Vec::new();
    let path = Path::new(directory);

    if path.is_dir() {
        for entry in fs::read_dir(path).expect(&format!("Failed to read {} directory", directory)) {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                let content =
                    fs::read_to_string(&path).expect(&format!("Failed to read file: {:?}", path));
                match serde_json::from_str::<T>(&content) {
                    Ok(item) => items.push(item),
                    Err(e) => error!("Error parsing JSON from {:?}: {}", path, e),
                }
            }
        }
    } else {
        error!("{} directory not found.", directory);
    }

    info!("Loaded {} {}", items.len(), type_name);
    items
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::fs::{self, File};
    use std::io::Write;

    fn setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[derive(Deserialize, Debug, PartialEq, Clone, Ord, PartialOrd, Eq)]
    struct TestData {
        name: String,
    }

    #[test]
    fn test_load_json_from_directory() {
        setup();
        let test_dir = Path::new("test_data_for_json_loader");
        fs::create_dir_all(test_dir).unwrap();

        let mut file1 = File::create(test_dir.join("test1.json")).unwrap();
        file1.write_all(b"{\"name\": \"test1\"}").unwrap();

        let mut file2 = File::create(test_dir.join("test2.json")).unwrap();
        file2.write_all(b"{\"name\": \"test2\"}").unwrap();

        let mut file3 = File::create(test_dir.join("test3.txt")).unwrap();
        file3.write_all(b"some text").unwrap();

        let mut result: Vec<TestData> =
            load_json_from_directory(test_dir.to_str().unwrap(), "test data");
        result.sort();

        assert_eq!(result.len(), 2);
        assert_eq!(
            result[0],
            TestData {
                name: "test1".to_string()
            }
        );
        assert_eq!(
            result[1],
            TestData {
                name: "test2".to_string()
            }
        );

        fs::remove_dir_all(test_dir).unwrap();
    }

    #[test]
    fn test_load_from_non_existent_directory() {
        setup();
        let result: Vec<TestData> =
            load_json_from_directory("non_existent_dir_for_test", "test data");
        assert!(result.is_empty());
    }
}
