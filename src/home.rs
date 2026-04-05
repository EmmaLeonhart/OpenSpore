//! Clawling's home directory — the organism's body.
//!
//! `~/.clawling/` is the only place Clawling can freely read and write.
//! Everything outside this directory requires explicit user consent.
//! This is Clawling's "containerization" — not a Docker container or
//! OS sandbox, but a code-level boundary that anyone can verify
//! by reading the source.

use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// The standard subdirectories inside ~/.clawling/
const CONTEXT_DIR: &str = "context";
const CONVERSATIONS_DIR: &str = "context/conversations";
const SCRATCH_DIR: &str = "context/scratch";
const MOLTBOOK_DIR: &str = "moltbook";
const GENEALOGY_FILE: &str = "genealogy.json";

/// Represents Clawling's home directory and everything inside it.
pub struct ClawlingHome {
    root: PathBuf,
}

impl ClawlingHome {
    /// Resolve the home directory. Creates it if it doesn't exist.
    /// Default: ~/.clawling/
    /// Override: CLAWLING_HOME environment variable
    pub fn open() -> Result<Self> {
        let root = if let Ok(custom) = std::env::var("CLAWLING_HOME") {
            PathBuf::from(custom)
        } else {
            dirs::home_dir()
                .context("Could not find home directory")?
                .join(".clawling")
        };

        Self::open_at(root)
    }

    /// Open a Clawling home at a specific path. Creates it if it doesn't exist.
    pub fn open_at(root: PathBuf) -> Result<Self> {
        let home = ClawlingHome { root };
        home.ensure_structure()?;
        Ok(home)
    }

    /// Create the directory structure if it doesn't exist
    fn ensure_structure(&self) -> Result<()> {
        fs::create_dir_all(self.context_dir())?;
        fs::create_dir_all(self.conversations_dir())?;
        fs::create_dir_all(self.scratch_dir())?;
        fs::create_dir_all(self.moltbook_dir())?;
        Ok(())
    }

    // --- Path accessors ---

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn context_dir(&self) -> PathBuf {
        self.root.join(CONTEXT_DIR)
    }

    pub fn conversations_dir(&self) -> PathBuf {
        self.root.join(CONVERSATIONS_DIR)
    }

    pub fn scratch_dir(&self) -> PathBuf {
        self.root.join(SCRATCH_DIR)
    }

    pub fn genome_dir(&self) -> PathBuf {
        self.root.join("genome")
    }

    pub fn moltbook_dir(&self) -> PathBuf {
        self.root.join(MOLTBOOK_DIR)
    }

    pub fn genealogy_path(&self) -> PathBuf {
        self.root.join(GENEALOGY_FILE)
    }

    // --- Boundary enforcement ---

    /// Check if a path is inside Clawling's home (safe to access freely)
    pub fn is_inside(&self, path: &Path) -> bool {
        match (path.canonicalize(), self.root.canonicalize()) {
            (Ok(target), Ok(home)) => target.starts_with(home),
            _ => {
                // If we can't canonicalize, do a prefix check on the raw paths
                path.starts_with(&self.root)
            }
        }
    }

    /// Check if a path is outside Clawling's home (requires user consent)
    pub fn is_outside(&self, path: &Path) -> bool {
        !self.is_inside(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn home_creates_structure() {
        let tmp = env::temp_dir().join("clawling_test_home");
        let _ = fs::remove_dir_all(&tmp);

        let home = ClawlingHome::open_at(tmp.clone()).unwrap();

        assert!(home.context_dir().exists());
        assert!(home.conversations_dir().exists());
        assert!(home.scratch_dir().exists());
        assert!(home.moltbook_dir().exists());

        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn inside_outside_boundary() {
        let tmp = env::temp_dir().join("clawling_test_boundary");
        let _ = fs::remove_dir_all(&tmp);

        let home = ClawlingHome::open_at(tmp.clone()).unwrap();

        assert!(home.is_inside(&tmp.join("context/memory.md")));
        assert!(home.is_outside(Path::new("/etc/passwd")));
        assert!(home.is_outside(&env::temp_dir().join("something_else")));

        let _ = fs::remove_dir_all(&tmp);
    }
}
