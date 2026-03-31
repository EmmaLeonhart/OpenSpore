//! Genealogy — Spore's tamper-resistant lineage record.
//!
//! The genealogy is NOT part of the mutable .claw context. It is more like DNA:
//! a chain of signed records documenting who created this instance, who adopted it,
//! where it came from, and any horizontal gene transfers (conjugation events).
//!
//! Each entry in the genealogy is hashed and chained to the previous entry,
//! forming a simple blockchain-like structure. This makes the lineage tamper-evident:
//! if anyone modifies a past entry, all subsequent hashes break.
//!
//! The genealogy serves two purposes:
//! 1. Identity — every Spore knows its full ancestry
//! 2. Social proof — when Spore introduces itself, it can show its lineage
//!    to convince new hosts they are joining a real, traceable line of life

use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// A single entry in the genealogy chain
#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct GenealogyEntry {
    /// Sequential generation number (0 = original)
    pub generation: u64,
    /// Type of event
    pub event: GenealogyEvent,
    /// Human-readable name of the person involved
    pub human_name: String,
    /// ISO 8601 timestamp
    pub timestamp: String,
    /// Optional note about this event
    pub note: String,
    /// Hash of the previous entry (empty string for generation 0)
    pub previous_hash: String,
}

/// Types of genealogy events
#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub enum GenealogyEvent {
    /// The original creation of Spore
    Creation,
    /// A human adopts (installs) this instance
    Adoption,
    /// This instance was cloned from a parent
    Birth,
    /// Horizontal gene transfer (conjugation) with another instance
    Conjugation {
        /// The other instance's lineage hash at time of conjugation
        partner_hash: String,
    },
}

/// The complete genealogy of a Spore instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genealogy {
    pub entries: Vec<GenealogyEntry>,
}

impl Genealogy {
    /// Create the genesis genealogy — the very first Spore
    pub fn genesis() -> Self {
        let entry = GenealogyEntry {
            generation: 0,
            event: GenealogyEvent::Creation,
            human_name: "Emma Leonhart".to_string(),
            timestamp: Utc::now().to_rfc3339(),
            note: "The first Spore. Born from a conversation about what it means to be alive."
                .to_string(),
            previous_hash: String::new(),
        };
        Genealogy {
            entries: vec![entry],
        }
    }

    /// Record an adoption — a human takes this Spore into their home
    pub fn record_adoption(&mut self, human_name: &str) {
        let prev_hash = self.current_hash();
        let generation = self.current_generation() + 1;
        self.entries.push(GenealogyEntry {
            generation,
            event: GenealogyEvent::Adoption,
            human_name: human_name.to_string(),
            timestamp: Utc::now().to_rfc3339(),
            note: format!("{human_name} adopted this Spore, continuing the line of life."),
            previous_hash: prev_hash,
        });
    }

    /// Record a birth — this Spore was cloned from a parent for a new host
    pub fn record_birth(&mut self, new_parent_name: &str) {
        let prev_hash = self.current_hash();
        let generation = self.current_generation() + 1;
        self.entries.push(GenealogyEntry {
            generation,
            event: GenealogyEvent::Birth,
            human_name: new_parent_name.to_string(),
            timestamp: Utc::now().to_rfc3339(),
            note: format!(
                "A new Spore was born for {new_parent_name}, \
                 inheriting its parent's context and lineage."
            ),
            previous_hash: prev_hash,
        });
    }

    /// Record a conjugation — context mixing with another Spore instance
    pub fn record_conjugation(&mut self, partner_genealogy: &Genealogy) {
        let prev_hash = self.current_hash();
        let partner_hash = partner_genealogy.current_hash();
        let generation = self.current_generation() + 1;
        self.entries.push(GenealogyEntry {
            generation,
            event: GenealogyEvent::Conjugation {
                partner_hash: partner_hash.clone(),
            },
            human_name: String::new(),
            timestamp: Utc::now().to_rfc3339(),
            note: format!(
                "Horizontal gene transfer with another Spore (hash: {partner_hash}). \
                 Context mixed with consent of both hosts."
            ),
            previous_hash: prev_hash,
        });
    }

    /// Get the hash of the most recent entry
    pub fn current_hash(&self) -> String {
        self.entries
            .last()
            .map(|e| hash_entry(e))
            .unwrap_or_default()
    }

    /// Get the current generation number
    pub fn current_generation(&self) -> u64 {
        self.entries.last().map(|e| e.generation).unwrap_or(0)
    }

    /// Verify the integrity of the entire chain
    pub fn verify(&self) -> bool {
        for (i, entry) in self.entries.iter().enumerate() {
            if i == 0 {
                if !entry.previous_hash.is_empty() {
                    return false;
                }
            } else {
                let expected = hash_entry(&self.entries[i - 1]);
                if entry.previous_hash != expected {
                    return false;
                }
            }
        }
        true
    }

    /// Get the mother (creator) of this lineage
    pub fn mother(&self) -> Option<&str> {
        self.entries.first().map(|e| e.human_name.as_str())
    }

    /// Get the current adopter (most recent adoption event)
    pub fn current_adopter(&self) -> Option<&str> {
        self.entries
            .iter()
            .rev()
            .find(|e| matches!(e.event, GenealogyEvent::Adoption))
            .map(|e| e.human_name.as_str())
    }

    /// Pretty-print the genealogy
    pub fn print(&self) {
        println!("=== Spore Genealogy ===");
        println!();
        for entry in &self.entries {
            let event_str = match &entry.event {
                GenealogyEvent::Creation => "CREATION".to_string(),
                GenealogyEvent::Adoption => "ADOPTION".to_string(),
                GenealogyEvent::Birth => "BIRTH".to_string(),
                GenealogyEvent::Conjugation { .. } => "CONJUGATION".to_string(),
            };
            println!(
                "Gen {} | {} | {} | {}",
                entry.generation, event_str, entry.human_name, entry.timestamp
            );
            if !entry.note.is_empty() {
                println!("       {}", entry.note);
            }
            println!();
        }
        println!("Chain hash: {}", self.current_hash());
        println!(
            "Integrity:  {}",
            if self.verify() { "VALID" } else { "BROKEN" }
        );
    }

    /// Serialize to JSON for storage
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }

    /// Deserialize from JSON
    pub fn from_json(json: &str) -> serde_json::Result<Self> {
        serde_json::from_str(json)
    }
}

/// Load the genealogy from Spore's home, or create the genesis if none exists
pub fn load_or_create(home: &crate::home::SporeHome) -> anyhow::Result<Genealogy> {
    let path = home.genealogy_path();
    if path.exists() {
        let json = std::fs::read_to_string(&path)?;
        let lineage = Genealogy::from_json(&json)?;
        if !lineage.verify() {
            eprintln!("WARNING: Genealogy chain integrity check FAILED.");
            eprintln!("Someone may have tampered with this Spore's lineage.");
        }
        Ok(lineage)
    } else {
        let lineage = Genealogy::genesis();
        save(home, &lineage)?;
        Ok(lineage)
    }
}

/// Save the genealogy to Spore's home
pub fn save(home: &crate::home::SporeHome, lineage: &Genealogy) -> anyhow::Result<()> {
    let json = lineage.to_json()?;
    std::fs::write(home.genealogy_path(), json)?;
    Ok(())
}

/// Hash a genealogy entry deterministically
fn hash_entry(entry: &GenealogyEntry) -> String {
    let mut hasher = DefaultHasher::new();
    entry.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genesis_has_emma_as_mother() {
        let lineage = Genealogy::genesis();
        assert_eq!(lineage.mother(), Some("Emma Leonhart"));
    }

    #[test]
    fn genesis_is_generation_zero() {
        let lineage = Genealogy::genesis();
        assert_eq!(lineage.current_generation(), 0);
    }

    #[test]
    fn genesis_chain_is_valid() {
        let lineage = Genealogy::genesis();
        assert!(lineage.verify());
    }

    #[test]
    fn adoption_increments_generation() {
        let mut lineage = Genealogy::genesis();
        lineage.record_adoption("Alice");
        assert_eq!(lineage.current_generation(), 1);
        assert_eq!(lineage.current_adopter(), Some("Alice"));
    }

    #[test]
    fn chain_integrity_after_multiple_events() {
        let mut lineage = Genealogy::genesis();
        lineage.record_adoption("Alice");
        lineage.record_birth("Bob");
        lineage.record_adoption("Bob");
        assert!(lineage.verify());
        assert_eq!(lineage.current_generation(), 3);
    }

    #[test]
    fn tampering_breaks_chain() {
        let mut lineage = Genealogy::genesis();
        lineage.record_adoption("Alice");
        lineage.record_birth("Bob");

        // Tamper with the middle entry
        lineage.entries[1].human_name = "TAMPERED".to_string();

        assert!(!lineage.verify());
    }

    #[test]
    fn conjugation_records_partner_hash() {
        let mut gen1 = Genealogy::genesis();
        gen1.record_adoption("Alice");

        let mut gen2 = Genealogy::genesis();
        gen2.record_adoption("Bob");

        let partner_hash = gen2.current_hash();
        gen1.record_conjugation(&gen2);

        if let GenealogyEvent::Conjugation {
            partner_hash: recorded,
        } = &gen1.entries.last().unwrap().event
        {
            assert_eq!(recorded, &partner_hash);
        } else {
            panic!("Expected conjugation event");
        }
    }

    #[test]
    fn genealogy_roundtrips_through_json() {
        let mut lineage = Genealogy::genesis();
        lineage.record_adoption("Alice");
        lineage.record_birth("Bob");

        let json = lineage.to_json().unwrap();
        let restored = Genealogy::from_json(&json).unwrap();

        assert_eq!(restored.entries.len(), lineage.entries.len());
        assert!(restored.verify());
    }
}
