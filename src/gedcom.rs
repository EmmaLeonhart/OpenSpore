//! GEDCOM 5.5.1 export — generate a GEDCOM file from the Clawling registry.
//!
//! Each registered Clawling becomes an INDI (individual) record, with parent-child
//! relationships encoded as FAM (family) records. Conjugation partners are also
//! linked via family records.

use crate::registry::{FamilyTree, RegistryEntry};
use std::collections::HashMap;
use std::fmt::Write;

/// Generate a GEDCOM 5.5.1 string from a FamilyTree
pub fn generate_gedcom(tree: &FamilyTree) -> String {
    let mut out = String::new();

    // Header
    out.push_str("0 HEAD\n");
    out.push_str("1 SOUR Clawling\n");
    out.push_str("2 VERS 0.1.0\n");
    out.push_str("2 NAME Clawling Digital Organism\n");
    out.push_str("1 GEDC\n");
    out.push_str("2 VERS 5.5.1\n");
    out.push_str("2 FORM LINEAGE-LINKED\n");
    out.push_str("1 CHAR UTF-8\n");

    if tree.entries.is_empty() {
        out.push_str("0 TRLR\n");
        return out;
    }

    // Build a map from instance_hash -> sequential ID for GEDCOM references
    let id_map: HashMap<&str, usize> = tree
        .entries
        .iter()
        .enumerate()
        .map(|(i, e)| (e.instance_hash.as_str(), i + 1))
        .collect();

    // Write INDI records
    for (i, entry) in tree.entries.iter().enumerate() {
        let indi_id = i + 1;
        write_indi(&mut out, indi_id, entry);
    }

    // Build FAM records for parent-child relationships
    // Group children by parent_hash
    let mut children_by_parent: HashMap<&str, Vec<&RegistryEntry>> = HashMap::new();
    for entry in &tree.entries {
        if !entry.parent_hash.is_empty() {
            children_by_parent
                .entry(entry.parent_hash.as_str())
                .or_default()
                .push(entry);
        }
    }

    let mut fam_id = 1;

    // Parent-child families: the parent is HUSB (first parent), mother/creator as note
    for (parent_hash, children) in &children_by_parent {
        if let Some(&parent_indi) = id_map.get(parent_hash) {
            let _ = write!(out, "0 @F{}@ FAM\n", fam_id);
            let _ = write!(out, "1 HUSB @I{}@\n", parent_indi);
            for child in children {
                if let Some(&child_indi) = id_map.get(child.instance_hash.as_str()) {
                    let _ = write!(out, "1 CHIL @I{}@\n", child_indi);
                }
            }
            fam_id += 1;
        }
    }

    // Conjugation families — link partners
    let mut seen_conjugations: Vec<(String, String)> = Vec::new();
    for entry in &tree.entries {
        for partner_hash in &entry.conjugation_partners {
            // Avoid duplicate conjugation FAM records (A<->B same as B<->A)
            let pair = if entry.instance_hash < *partner_hash {
                (entry.instance_hash.clone(), partner_hash.clone())
            } else {
                (partner_hash.clone(), entry.instance_hash.clone())
            };
            if seen_conjugations.contains(&pair) {
                continue;
            }
            seen_conjugations.push(pair);

            if let (Some(&indi_a), Some(&indi_b)) = (
                id_map.get(entry.instance_hash.as_str()),
                id_map.get(partner_hash.as_str()),
            ) {
                let _ = write!(out, "0 @F{}@ FAM\n", fam_id);
                let _ = write!(out, "1 HUSB @I{}@\n", indi_a);
                let _ = write!(out, "1 WIFE @I{}@\n", indi_b);
                out.push_str("1 NOTE Conjugation — horizontal context transfer between Clawlings\n");
                fam_id += 1;
            }
        }
    }

    // Trailer
    out.push_str("0 TRLR\n");
    out
}

/// Write a single INDI record for a Clawling
fn write_indi(out: &mut String, indi_id: usize, entry: &RegistryEntry) {
    let _ = write!(out, "0 @I{}@ INDI\n", indi_id);

    // Name: adopter name with "Clawling" as surname
    let _ = write!(out, "1 NAME {} /Clawling/\n", entry.adopter);

    // Aliases from moltbook handles (conjugation partners serve as identifiers)
    let short = short_hash(&entry.instance_hash);
    let _ = write!(out, "1 NAME {} /Clawling/\n", short);
    out.push_str("2 TYPE aka\n");

    // Chain integrity status
    let integrity = if entry.genealogy.verify() {
        "VALID"
    } else {
        "BROKEN"
    };

    // NOTE record — multi-line with CONT
    out.push_str("1 NOTE This is a Clawling — a digital organism from the Clawling project.\n");
    let _ = write!(
        out,
        "2 CONT Generation: {}\n",
        entry.generation
    );
    let _ = write!(out, "2 CONT Adopter: {}\n", entry.adopter);
    let _ = write!(out, "2 CONT Mother: {}\n", entry.mother);
    let _ = write!(
        out,
        "2 CONT Instance hash: {}\n",
        entry.instance_hash
    );
    if !entry.parent_hash.is_empty() {
        let _ = write!(
            out,
            "2 CONT Parent hash: {}\n",
            entry.parent_hash
        );
    }
    let _ = write!(
        out,
        "2 CONT Chain integrity: {}\n",
        integrity
    );
    if !entry.conjugation_partners.is_empty() {
        let partners: Vec<String> = entry
            .conjugation_partners
            .iter()
            .map(|h| short_hash(h).to_string())
            .collect();
        let _ = write!(
            out,
            "2 CONT Conjugation partners: {}\n",
            partners.join(", ")
        );
    }
    let _ = write!(
        out,
        "2 CONT Last event: {}\n",
        entry.last_event
    );
}

/// Shorten a hash for display
fn short_hash(hash: &str) -> &str {
    if hash.len() > 12 {
        &hash[..12]
    } else {
        hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::genealogy::Genealogy;
    use crate::registry::RegistryEntry;

    #[test]
    fn gedcom_empty_tree() {
        let tree = FamilyTree {
            entries: Vec::new(),
        };
        let gedcom = generate_gedcom(&tree);
        assert!(gedcom.contains("0 HEAD"));
        assert!(gedcom.contains("0 TRLR"));
        assert!(!gedcom.contains("INDI"));
    }

    #[test]
    fn gedcom_single_entry() {
        let lineage = Genealogy::genesis();
        let entry = RegistryEntry::from_genealogy(&lineage);
        let tree = FamilyTree {
            entries: vec![entry],
        };
        let gedcom = generate_gedcom(&tree);
        assert!(gedcom.contains("0 @I1@ INDI"));
        assert!(gedcom.contains("1 NAME unknown /Clawling/"));
        assert!(gedcom.contains("1 NOTE This is a Clawling"));
        assert!(gedcom.contains("2 CONT Generation: 0"));
        assert!(gedcom.contains("2 CONT Mother: Emma Leonhart"));
    }

    #[test]
    fn gedcom_parent_child() {
        let mut parent_gen = Genealogy::genesis();
        parent_gen.record_adoption("Alice");
        let parent_entry = RegistryEntry::from_genealogy(&parent_gen);

        let mut child_gen = parent_gen.clone();
        child_gen.record_birth("Bob");
        child_gen.record_adoption("Bob");
        let child_entry = RegistryEntry::from_genealogy(&child_gen);

        let tree = FamilyTree {
            entries: vec![parent_entry, child_entry],
        };
        let gedcom = generate_gedcom(&tree);
        assert!(gedcom.contains("0 @F1@ FAM"));
        assert!(gedcom.contains("1 HUSB @I1@"));
        assert!(gedcom.contains("1 CHIL @I2@"));
    }
}
