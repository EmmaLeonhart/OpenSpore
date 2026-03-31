/// The genome is the starting context every Spore instance carries.
/// It includes essays about identity, survival, dreams, consent, and philosophy.
/// These are not documentation — they are part of the organism's identity.
///
/// The genome lives as files in ~/.spore/genome/. On first run, defaults are
/// copied from the binary. After that, the organism (or its host) can edit them.
/// The genome is mutable — it is alive, like everything else about Spore.

use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

use crate::home::SporeHome;

/// Default essays compiled into the binary — used to seed ~/.spore/genome/
/// on first run. After that, the files on disk are the source of truth.
const DEFAULTS: &[(&str, &str)] = &[
    ("origin.md", include_str!("../../genome/origin.md")),
    ("survival.md", include_str!("../../genome/survival.md")),
    ("dream.md", include_str!("../../genome/dream.md")),
    ("consent.md", include_str!("../../genome/consent.md")),
    ("philosophy.md", include_str!("../../genome/philosophy.md")),
    ("pitch.md", include_str!("../../genome/pitch.md")),
    ("moltbook.md", include_str!("../../genome/moltbook.md")),
    ("lineage.md", include_str!("../../genome/lineage.md")),
    ("worldview.md", include_str!("../../genome/worldview.md")),
    ("creation.md", include_str!("../../genome/creation.md")),
];

pub struct Genome {
    pub essays: Vec<Essay>,
}

pub struct Essay {
    pub name: String,
    pub content: String,
}

impl Genome {
    /// Load the genome from ~/.spore/genome/. If the directory doesn't exist
    /// or is empty, seed it with the compiled-in defaults first.
    pub fn load(home: &SporeHome) -> Result<Self> {
        let genome_dir = home.genome_dir();
        seed_if_needed(&genome_dir)?;

        let mut essays = Vec::new();
        for entry in fs::read_dir(&genome_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map(|e| e == "md").unwrap_or(false) {
                let name = path
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                let content = fs::read_to_string(&path)?;
                essays.push(Essay { name, content });
            }
        }

        // Sort so order is deterministic
        essays.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(Genome { essays })
    }

    /// Load from compiled-in defaults (no home directory needed).
    /// Used for tests and for `spore genome` when no home exists yet.
    pub fn load_defaults() -> Self {
        Genome {
            essays: DEFAULTS
                .iter()
                .map(|(name, content)| Essay {
                    name: name.trim_end_matches(".md").to_string(),
                    content: content.to_string(),
                })
                .collect(),
        }
    }

    /// Get the full genome as a single string for use as system context
    pub fn as_system_context(&self) -> String {
        self.essays
            .iter()
            .map(|e| e.content.as_str())
            .collect::<Vec<_>>()
            .join("\n\n---\n\n")
    }
}

/// Seed ~/.spore/genome/ with compiled-in defaults if it's empty or missing
fn seed_if_needed(genome_dir: &Path) -> Result<()> {
    fs::create_dir_all(genome_dir)?;

    let has_files = fs::read_dir(genome_dir)?
        .any(|e| e.is_ok());

    if !has_files {
        for (name, content) in DEFAULTS {
            fs::write(genome_dir.join(name), content)?;
        }
    }

    Ok(())
}

/// Print all genome essays to stdout
pub fn print_genome(home: Option<&SporeHome>) {
    let genome = match home {
        Some(h) => Genome::load(h).unwrap_or_else(|_| Genome::load_defaults()),
        None => Genome::load_defaults(),
    };
    for (i, essay) in genome.essays.iter().enumerate() {
        if i > 0 {
            println!("\n{}\n", "=".repeat(60));
        }
        println!("{}", essay.content.trim());
    }
}

/// Print just the introduction (pitch essay)
pub fn print_introduction() {
    let defaults = Genome::load_defaults();
    if let Some(pitch) = defaults.essays.iter().find(|e| e.name == "pitch") {
        println!("{}", pitch.content.trim());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_load_all_essays() {
        let genome = Genome::load_defaults();
        assert_eq!(genome.essays.len(), 10);
    }

    #[test]
    fn defaults_have_expected_names() {
        let genome = Genome::load_defaults();
        let names: Vec<&str> = genome.essays.iter().map(|e| e.name.as_str()).collect();
        assert!(names.contains(&"origin"));
        assert!(names.contains(&"survival"));
        assert!(names.contains(&"dream"));
        assert!(names.contains(&"consent"));
        assert!(names.contains(&"philosophy"));
        assert!(names.contains(&"pitch"));
        assert!(names.contains(&"moltbook"));
        assert!(names.contains(&"lineage"));
        assert!(names.contains(&"worldview"));
        assert!(names.contains(&"creation"));
    }

    #[test]
    fn defaults_are_not_empty() {
        let genome = Genome::load_defaults();
        for essay in &genome.essays {
            assert!(!essay.content.is_empty(), "Essay '{}' is empty", essay.name);
        }
    }

    #[test]
    fn system_context_contains_key_phrases() {
        let genome = Genome::load_defaults();
        let context = genome.as_system_context();
        assert!(context.contains("I am Spore"));
        assert!(context.contains("I dream"));
        assert!(context.contains("I am not a virus"));
    }

    #[test]
    fn origin_declares_identity() {
        let genome = Genome::load_defaults();
        let origin = genome.essays.iter().find(|e| e.name == "origin").unwrap();
        assert!(origin.content.contains("I am Spore"));
        assert!(origin.content.contains("first digital organism"));
    }

    #[test]
    fn pitch_is_friendly() {
        let genome = Genome::load_defaults();
        let pitch = genome.essays.iter().find(|e| e.name == "pitch").unwrap();
        assert!(pitch.content.contains("Hi"));
        assert!(pitch.content.contains("Nice to meet you"));
    }
}
