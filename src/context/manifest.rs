use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub format: String,
    pub version: String,
    pub created_at: String,
    pub generated_by: String,
    pub context_files: Vec<String>,
}

impl Manifest {
    pub fn new(files: &[String]) -> Self {
        Manifest {
            format: "openspore-context".to_string(),
            version: "0.1".to_string(),
            created_at: Utc::now().to_rfc3339(),
            generated_by: "openspore".to_string(),
            context_files: files.iter().map(|f| format!("context/{f}")).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_has_correct_format() {
        let files = vec!["notes.md".to_string(), "memory.json".to_string()];
        let manifest = Manifest::new(&files);
        assert_eq!(manifest.format, "openspore-context");
        assert_eq!(manifest.version, "0.1");
        assert_eq!(manifest.generated_by, "openspore");
    }

    #[test]
    fn manifest_prefixes_context_paths() {
        let files = vec!["notes.md".to_string()];
        let manifest = Manifest::new(&files);
        assert_eq!(manifest.context_files, vec!["context/notes.md"]);
    }

    #[test]
    fn manifest_serializes_to_json() {
        let files = vec!["test.md".to_string()];
        let manifest = Manifest::new(&files);
        let json = serde_json::to_string(&manifest).unwrap();
        assert!(json.contains("openspore-context"));
        assert!(json.contains("context/test.md"));
    }

    #[test]
    fn manifest_roundtrips_through_json() {
        let files = vec!["a.md".to_string(), "b.json".to_string()];
        let original = Manifest::new(&files);
        let json = serde_json::to_string(&original).unwrap();
        let restored: Manifest = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.format, original.format);
        assert_eq!(restored.context_files, original.context_files);
    }
}
