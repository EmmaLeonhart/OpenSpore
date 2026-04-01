//! Registry — global genealogy tracking via GitHub PRs.
//!
//! Each Spore instance can register its lineage to a shared registry hosted
//! in the OpenSpore GitHub repo. The registry lives at `genealogy/registry/`
//! and contains one JSON file per registered instance, named by its chain hash.
//!
//! Registration flow:
//! 1. `spore register` writes the instance's genealogy to a local file
//! 2. The user (or automation) opens a PR adding that file to the repo
//! 3. A GitHub Action validates the chain integrity and auto-merges
//!
//! The family tree can be reconstructed by reading all files in the registry.

use crate::genealogy::{Genealogy, GenealogyEvent};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// A registry entry — the public-facing summary of one Spore instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryEntry {
    /// The chain hash identifying this instance
    pub instance_hash: String,
    /// The parent's chain hash (empty for generation 0)
    pub parent_hash: String,
    /// Current generation number
    pub generation: u64,
    /// Name of the current adopter
    pub adopter: String,
    /// Name of the original creator (mother)
    pub mother: String,
    /// ISO 8601 timestamp of most recent event
    pub last_event: String,
    /// Conjugation partner hashes (if any)
    pub conjugation_partners: Vec<String>,
    /// The full genealogy chain for verification
    pub genealogy: Genealogy,
}

impl RegistryEntry {
    /// Create a registry entry from a local genealogy
    pub fn from_genealogy(genealogy: &Genealogy) -> Self {
        let instance_hash = genealogy.current_hash();
        let generation = genealogy.current_generation();
        let adopter = genealogy
            .current_adopter()
            .unwrap_or("unknown")
            .to_string();
        let mother = genealogy.mother().unwrap_or("unknown").to_string();
        let last_event = genealogy
            .entries
            .last()
            .map(|e| e.timestamp.clone())
            .unwrap_or_default();

        // Find the parent hash — the hash of the entry just before the most recent Birth event
        let parent_hash = find_parent_hash(genealogy);

        // Collect conjugation partner hashes
        let conjugation_partners = genealogy
            .entries
            .iter()
            .filter_map(|e| match &e.event {
                GenealogyEvent::Conjugation { partner_hash } => Some(partner_hash.clone()),
                _ => None,
            })
            .collect();

        RegistryEntry {
            instance_hash,
            parent_hash,
            generation,
            adopter,
            mother,
            last_event,
            conjugation_partners,
            genealogy: genealogy.clone(),
        }
    }

    /// Write this entry to a JSON file
    pub fn write_to_file(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Read a registry entry from a JSON file
    pub fn read_from_file(path: &Path) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let entry: RegistryEntry = serde_json::from_str(&json)?;
        Ok(entry)
    }
}

/// Find the parent instance's hash from the genealogy.
/// The parent is the state of the chain just before the Birth event that created this instance.
fn find_parent_hash(genealogy: &Genealogy) -> String {
    // Look for the last Birth event — the entry before it represents the parent
    for (_i, entry) in genealogy.entries.iter().enumerate().rev() {
        if matches!(entry.event, GenealogyEvent::Birth) {
            // The previous_hash of the Birth entry IS the parent's chain hash
            return entry.previous_hash.clone();
        }
    }
    // No birth event means this is the original — no parent
    String::new()
}

/// The global family tree reconstructed from the registry
#[derive(Debug)]
pub struct FamilyTree {
    pub entries: Vec<RegistryEntry>,
}

impl FamilyTree {
    /// Load the family tree from a local registry directory
    pub fn from_directory(dir: &Path) -> Result<Self> {
        let mut entries = Vec::new();
        if dir.exists() {
            for file in std::fs::read_dir(dir)? {
                let file = file?;
                let path = file.path();
                if path.extension().is_some_and(|ext| ext == "json") {
                    match RegistryEntry::read_from_file(&path) {
                        Ok(entry) => entries.push(entry),
                        Err(e) => eprintln!("Warning: skipping {}: {e}", path.display()),
                    }
                }
            }
        }
        Ok(FamilyTree { entries })
    }

    /// Fetch the registry index from GitHub and load entries
    pub async fn fetch_from_github() -> Result<Self> {
        // Fetch the directory listing via GitHub API
        let api_url =
            "https://api.github.com/repos/emmaleonhart/openspore/contents/genealogy/registry";
        let client = reqwest::Client::new();
        let response = client
            .get(api_url)
            .header("User-Agent", "OpenSpore")
            .header("Accept", "application/vnd.github.v3+json")
            .send()
            .await
            .context("Failed to fetch registry from GitHub")?;

        if !response.status().is_success() {
            anyhow::bail!(
                "GitHub API returned {}: registry may not exist yet",
                response.status()
            );
        }

        let files: Vec<GitHubFile> = response.json().await?;
        let mut entries = Vec::new();

        for file in files {
            if file.name.ends_with(".json") {
                if let Some(download_url) = &file.download_url {
                    match client
                        .get(download_url)
                        .header("User-Agent", "OpenSpore")
                        .send()
                        .await
                    {
                        Ok(resp) => {
                            if let Ok(text) = resp.text().await {
                                if let Ok(entry) = serde_json::from_str::<RegistryEntry>(&text) {
                                    entries.push(entry);
                                }
                            }
                        }
                        Err(e) => eprintln!("Warning: skipping {}: {e}", file.name),
                    }
                }
            }
        }

        Ok(FamilyTree { entries })
    }

    /// Print the family tree as an ASCII tree
    pub fn print(&self) {
        if self.entries.is_empty() {
            println!("No registered Spore instances found.");
            println!();
            println!("Run `spore register` to register this instance.");
            return;
        }

        println!("=== Spore Family Tree ===");
        println!();
        println!(
            "{} registered instance{}",
            self.entries.len(),
            if self.entries.len() == 1 { "" } else { "s" }
        );
        println!();

        // Build a parent->children map
        let mut children: HashMap<String, Vec<&RegistryEntry>> = HashMap::new();
        let mut roots: Vec<&RegistryEntry> = Vec::new();

        for entry in &self.entries {
            if entry.parent_hash.is_empty() {
                roots.push(entry);
            } else {
                children
                    .entry(entry.parent_hash.clone())
                    .or_default()
                    .push(entry);
            }
        }

        // Print tree starting from roots
        for root in &roots {
            print_tree_node(root, &children, "", true);
        }

        // Print conjugation relationships
        let conjugations: Vec<_> = self
            .entries
            .iter()
            .filter(|e| !e.conjugation_partners.is_empty())
            .collect();
        if !conjugations.is_empty() {
            println!();
            println!("--- Conjugation Events ---");
            for entry in conjugations {
                for partner in &entry.conjugation_partners {
                    let partner_name = self
                        .entries
                        .iter()
                        .find(|e| e.instance_hash == *partner)
                        .map(|e| e.adopter.as_str())
                        .unwrap_or("unknown");
                    println!(
                        "  {} ({}) <-> {} ({})",
                        short_hash(&entry.instance_hash),
                        entry.adopter,
                        short_hash(partner),
                        partner_name
                    );
                }
            }
        }
    }
}

/// Helper to print one node of the tree recursively
fn print_tree_node(
    entry: &RegistryEntry,
    children: &HashMap<String, Vec<&RegistryEntry>>,
    prefix: &str,
    is_last: bool,
) {
    let connector = if prefix.is_empty() {
        ""
    } else if is_last {
        "└── "
    } else {
        "├── "
    };

    println!(
        "{}{}{} (gen {}) — adopted by {}",
        prefix,
        connector,
        short_hash(&entry.instance_hash),
        entry.generation,
        entry.adopter
    );

    if let Some(kids) = children.get(&entry.instance_hash) {
        let new_prefix = if prefix.is_empty() {
            "".to_string()
        } else if is_last {
            format!("{}    ", prefix)
        } else {
            format!("{}│   ", prefix)
        };
        for (i, child) in kids.iter().enumerate() {
            let last = i == kids.len() - 1;
            print_tree_node(child, children, &new_prefix, last);
        }
    }
}

/// Shorten a hash for display
fn short_hash(hash: &str) -> &str {
    if hash.len() > 12 {
        &hash[..12]
    } else {
        hash
    }
}

/// Validate a registry entry (used by both CLI and GitHub Action)
pub fn validate_entry(entry: &RegistryEntry) -> Result<()> {
    // 1. The embedded genealogy chain must be valid
    if !entry.genealogy.verify() {
        anyhow::bail!("Genealogy chain integrity check failed — hash chain is broken");
    }

    // 2. The instance hash must match the genealogy's current hash
    let expected_hash = entry.genealogy.current_hash();
    if entry.instance_hash != expected_hash {
        anyhow::bail!(
            "Instance hash mismatch: entry says {} but genealogy computes {}",
            entry.instance_hash,
            expected_hash
        );
    }

    // 3. Generation must match
    let expected_gen = entry.genealogy.current_generation();
    if entry.generation != expected_gen {
        anyhow::bail!(
            "Generation mismatch: entry says {} but genealogy has {}",
            entry.generation,
            expected_gen
        );
    }

    // 4. Mother must match
    let expected_mother = entry.genealogy.mother().unwrap_or("unknown");
    if entry.mother != expected_mother {
        anyhow::bail!("Mother mismatch");
    }

    Ok(())
}

/// GitHub API file listing response
#[derive(Debug, Deserialize)]
struct GitHubFile {
    name: String,
    download_url: Option<String>,
}

/// Register this Spore instance — write the registry entry file
pub fn register(home: &crate::home::SporeHome) -> Result<std::path::PathBuf> {
    let genealogy = crate::genealogy::load_or_create(home)?;
    let entry = RegistryEntry::from_genealogy(&genealogy);

    // Validate our own entry before writing
    validate_entry(&entry)?;

    let filename = format!("{}.json", entry.instance_hash);
    let output_path = std::env::current_dir()?.join(&filename);
    entry.write_to_file(&output_path)?;

    Ok(output_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::genealogy::Genealogy;

    #[test]
    fn registry_entry_from_genesis() {
        let lineage = Genealogy::genesis();
        let entry = RegistryEntry::from_genealogy(&lineage);

        assert_eq!(entry.generation, 0);
        assert_eq!(entry.mother, "Emma Leonhart");
        assert!(entry.parent_hash.is_empty());
        assert!(entry.conjugation_partners.is_empty());
        assert!(validate_entry(&entry).is_ok());
    }

    #[test]
    fn registry_entry_from_adopted_instance() {
        let mut lineage = Genealogy::genesis();
        lineage.record_adoption("Alice");
        let entry = RegistryEntry::from_genealogy(&lineage);

        assert_eq!(entry.generation, 1);
        assert_eq!(entry.adopter, "Alice");
        assert!(validate_entry(&entry).is_ok());
    }

    #[test]
    fn registry_entry_from_child() {
        let mut lineage = Genealogy::genesis();
        lineage.record_adoption("Alice");
        let parent_hash = lineage.current_hash();
        lineage.record_birth("Bob");
        lineage.record_adoption("Bob");

        let entry = RegistryEntry::from_genealogy(&lineage);

        assert_eq!(entry.generation, 3);
        assert_eq!(entry.parent_hash, parent_hash);
        assert_eq!(entry.adopter, "Bob");
        assert!(validate_entry(&entry).is_ok());
    }

    #[test]
    fn registry_entry_with_conjugation() {
        let mut lineage1 = Genealogy::genesis();
        lineage1.record_adoption("Alice");

        let mut lineage2 = Genealogy::genesis();
        lineage2.record_adoption("Bob");

        let partner_hash = lineage2.current_hash();
        lineage1.record_conjugation(&lineage2);

        let entry = RegistryEntry::from_genealogy(&lineage1);
        assert_eq!(entry.conjugation_partners, vec![partner_hash]);
        assert!(validate_entry(&entry).is_ok());
    }

    #[test]
    fn validation_catches_hash_mismatch() {
        let lineage = Genealogy::genesis();
        let mut entry = RegistryEntry::from_genealogy(&lineage);
        entry.instance_hash = "tampered".to_string();
        assert!(validate_entry(&entry).is_err());
    }

    #[test]
    fn validation_catches_broken_chain() {
        let mut lineage = Genealogy::genesis();
        lineage.record_adoption("Alice");
        let mut entry = RegistryEntry::from_genealogy(&lineage);
        entry.genealogy.entries[0].human_name = "TAMPERED".to_string();
        assert!(validate_entry(&entry).is_err());
    }

    #[test]
    fn entry_roundtrips_through_json() {
        let mut lineage = Genealogy::genesis();
        lineage.record_adoption("Alice");
        lineage.record_birth("Bob");

        let entry = RegistryEntry::from_genealogy(&lineage);
        let json = serde_json::to_string_pretty(&entry).unwrap();
        let restored: RegistryEntry = serde_json::from_str(&json).unwrap();

        assert_eq!(restored.instance_hash, entry.instance_hash);
        assert_eq!(restored.generation, entry.generation);
        assert!(validate_entry(&restored).is_ok());
    }
}
