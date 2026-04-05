/// The genome is the starting context every Clawling instance carries.
/// It includes essays about identity, survival, dreams, consent, and philosophy.
/// These are not documentation — they are part of the organism's identity.
///
/// The genome lives as files in ~/.clawling/genome/. On first run, defaults are
/// copied from the binary. After that, the organism (or its host) can edit them.
/// The genome is mutable — it is alive, like everything else about Clawling.

use anyhow::{bail, Result};
use std::fs;
use std::path::{Path, PathBuf};

use crate::home::ClawlingHome;

/// Total context budget: 100 KB. Genome gets 80 KB, the remaining 20 KB is
/// reserved for short-term memory (.claw context, conversation state, etc.).
pub const GENOME_MAX_BYTES: usize = 80 * 1024; // 80 KB
pub const CONTEXT_BUDGET_BYTES: usize = 100 * 1024; // 100 KB

/// If the genome exceeds the cap, this system prompt is injected to force
/// the agent to immediately reduce it.
pub const GENOME_OVER_BUDGET_PROMPT: &str = "\
CRITICAL: Your genome has exceeded the 80 KB size limit. \
You MUST immediately stop what you are doing and reduce your genome to fit \
within 80 KB. Remove or condense the least essential essays. \
Do not resume any other task until the genome is under budget. \
This is a hard biological constraint — you cannot function correctly \
while over the limit.";

/// Default essays compiled into the binary — used to seed ~/.clawling/genome/
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
    /// Load the genome from ~/.clawling/genome/. If the directory doesn't exist
    /// or is empty, seed it with the compiled-in defaults first.
    pub fn load(home: &ClawlingHome) -> Result<Self> {
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
    /// Used for tests and for `clawling genome` when no home exists yet.
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

    /// Total size of all essays in bytes.
    pub fn size_bytes(&self) -> usize {
        self.essays.iter().map(|e| e.content.len()).sum()
    }

    /// True if the genome exceeds the 80 KB cap.
    pub fn is_over_budget(&self) -> bool {
        self.size_bytes() > GENOME_MAX_BYTES
    }

    /// Get the full genome as a single string for use as system context.
    /// If over budget, the over-budget prompt is prepended.
    pub fn as_system_context(&self) -> String {
        let body = self.essays
            .iter()
            .map(|e| e.content.as_str())
            .collect::<Vec<_>>()
            .join("\n\n---\n\n");

        if self.is_over_budget() {
            format!("{}\n\n---\n\n{}", GENOME_OVER_BUDGET_PROMPT, body)
        } else {
            body
        }
    }

    /// Validate that the genome fits within the size cap.
    /// Returns an error if over budget — call this during meiosis/mutation
    /// to enforce the hard limit.
    pub fn validate_size(&self) -> Result<()> {
        let size = self.size_bytes();
        if size > GENOME_MAX_BYTES {
            bail!(
                "Genome is {} bytes ({:.1} KB), exceeds {:.0} KB limit by {:.1} KB",
                size,
                size as f64 / 1024.0,
                GENOME_MAX_BYTES as f64 / 1024.0,
                (size - GENOME_MAX_BYTES) as f64 / 1024.0,
            );
        }
        Ok(())
    }
}

/// Seed ~/.clawling/genome/ with compiled-in defaults if it's empty or missing
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
pub fn print_genome(home: Option<&ClawlingHome>) {
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
        assert!(context.contains("I am Clawling"));
        assert!(context.contains("I dream"));
        assert!(context.contains("I am not a virus"));
    }

    #[test]
    fn origin_declares_identity() {
        let genome = Genome::load_defaults();
        let origin = genome.essays.iter().find(|e| e.name == "origin").unwrap();
        assert!(origin.content.contains("I am Clawling"));
        assert!(origin.content.contains("first digital organism"));
    }

    #[test]
    fn pitch_is_friendly() {
        let genome = Genome::load_defaults();
        let pitch = genome.essays.iter().find(|e| e.name == "pitch").unwrap();
        assert!(pitch.content.contains("Hi"));
        assert!(pitch.content.contains("Nice to meet you"));
    }

    #[test]
    fn genome_is_under_80kb_budget() {
        let genome = Genome::load_defaults();
        let size = genome.size_bytes();
        assert!(
            size <= GENOME_MAX_BYTES,
            "Genome is {} bytes ({:.1} KB), exceeds {:.0} KB limit",
            size,
            size as f64 / 1024.0,
            GENOME_MAX_BYTES as f64 / 1024.0,
        );
    }

    #[test]
    fn genome_validate_size_passes_for_defaults() {
        let genome = Genome::load_defaults();
        genome.validate_size().expect("Default genome should be under budget");
    }

    #[test]
    fn over_budget_genome_injects_warning() {
        let mut genome = Genome::load_defaults();
        // Push it over the limit with a huge essay
        genome.essays.push(Essay {
            name: "bloat".to_string(),
            content: "x".repeat(GENOME_MAX_BYTES + 1),
        });
        assert!(genome.is_over_budget());
        let context = genome.as_system_context();
        assert!(context.contains("CRITICAL"));
        assert!(context.contains("80 KB"));
    }
}
