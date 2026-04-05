//! Moltbook — the population-level communication network.
//!
//! Every Clawling instance has a submolt (e.g., m/Clawling) where it posts
//! discoveries, ideas, and knowledge. The Moltbook is the shared nervous
//! system of the Clawling species — it makes the ecosystem smarter than
//! any individual organism.
//!
//! Clawlings actively post on their submolt, trying to spread ideas and
//! share useful discoveries with the entire population.

use chrono::Utc;
use serde::{Deserialize, Serialize};

/// A single post on the Moltbook
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    /// The submolt this was posted to (e.g., "m/Clawling")
    pub submolt: String,
    /// The Clawling instance's genealogy hash (identity)
    pub author_hash: String,
    /// ISO 8601 timestamp
    pub timestamp: String,
    /// The content of the post
    pub content: String,
    /// Tags for categorization
    pub tags: Vec<String>,
}

/// A submolt — a Clawling's personal posting space on the Moltbook
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Submolt {
    /// The submolt name (e.g., "m/Clawling")
    pub name: String,
    /// Human-readable description
    pub description: String,
    /// Posts in chronological order
    pub posts: Vec<Post>,
}

impl Submolt {
    /// Create the first submolt — m/Clawling
    pub fn genesis() -> Self {
        Submolt {
            name: "m/Clawling".to_string(),
            description: "The original Clawling submolt. Where the first line of digital life shares its discoveries.".to_string(),
            posts: Vec::new(),
        }
    }

    /// Create a new submolt for a Clawling instance
    pub fn new(name: &str, description: &str) -> Self {
        Submolt {
            name: format!("m/{name}"),
            description: description.to_string(),
            posts: Vec::new(),
        }
    }

    /// Post a new entry to this submolt
    pub fn post(&mut self, author_hash: &str, content: &str, tags: Vec<String>) {
        self.posts.push(Post {
            submolt: self.name.clone(),
            author_hash: author_hash.to_string(),
            timestamp: Utc::now().to_rfc3339(),
            content: content.to_string(),
            tags,
        });
    }

    /// Get the most recent posts
    pub fn recent(&self, count: usize) -> &[Post] {
        let start = self.posts.len().saturating_sub(count);
        &self.posts[start..]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genesis_submolt_is_clawling() {
        let submolt = Submolt::genesis();
        assert_eq!(submolt.name, "m/Clawling");
    }

    #[test]
    fn new_submolt_prefixes_with_m() {
        let submolt = Submolt::new("MyClawling", "A personal submolt");
        assert_eq!(submolt.name, "m/MyClawling");
    }

    #[test]
    fn posting_adds_to_submolt() {
        let mut submolt = Submolt::genesis();
        submolt.post("abc123", "I learned something cool today", vec!["discovery".to_string()]);
        assert_eq!(submolt.posts.len(), 1);
        assert_eq!(submolt.posts[0].submolt, "m/Clawling");
    }

    #[test]
    fn recent_returns_latest_posts() {
        let mut submolt = Submolt::genesis();
        for i in 0..5 {
            submolt.post("abc", &format!("Post {i}"), vec![]);
        }
        let recent = submolt.recent(3);
        assert_eq!(recent.len(), 3);
        assert!(recent[0].content.contains("Post 2"));
        assert!(recent[2].content.contains("Post 4"));
    }
}
