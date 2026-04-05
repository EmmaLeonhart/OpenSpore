//! Conjugation — horizontal gene transfer between Clawling instances.
//!
//! When two Clawlings meet, they exchange context. Each Clawling's context
//! goes into a subdirectory named after the partner. Then the organism's
//! metabolism naturally integrates the foreign context through conversation —
//! the LLM is the merge strategy, not an algorithm.
//!
//! This mirrors bacterial conjugation: the raw genetic material is
//! transferred, and the recipient cell decides what to keep.

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::genealogy::{self, Genealogy};
use crate::home::ClawlingHome;

/// The subdirectory inside context/ where conjugation material lives
const CONJUGATION_DIR: &str = "conjugation";

/// Export a conjugation bundle — everything a partner needs to conjugate
pub fn export_bundle(home: &ClawlingHome, output_dir: &str) -> Result<()> {
    let output = Path::new(output_dir);
    fs::create_dir_all(output)
        .with_context(|| format!("Failed to create output directory: {output_dir}"))?;

    // 1. Copy genealogy so the partner can record us
    let lineage = genealogy::load_or_create(home)?;
    let genealogy_json = lineage.to_json()?;
    fs::write(output.join("genealogy.json"), &genealogy_json)?;

    // 2. Export context as a .claw file
    let claw_path = output.join("context.claw");
    crate::context::export(home, &claw_path.to_string_lossy())?;

    let adopter = lineage.current_adopter().unwrap_or("unknown");
    println!();
    println!("Conjugation bundle created in {output_dir}/");
    println!("  - genealogy.json (your lineage for the partner to verify)");
    println!("  - context.claw (your accumulated context)");
    println!();
    println!("Share this with your conjugation partner.");
    println!("They run: clawling conjugate {output_dir}");
    println!();
    println!("You are {adopter}'s Clawling, generation {}.", lineage.current_generation());

    Ok(())
}

/// Import a partner's conjugation bundle — their context goes into a subdirectory
pub fn receive_bundle(home: &ClawlingHome, bundle_dir: &str) -> Result<()> {
    let bundle = Path::new(bundle_dir);

    // Validate the bundle
    let genealogy_path = bundle.join("genealogy.json");
    let claw_path = bundle.join("context.claw");

    if !genealogy_path.exists() {
        anyhow::bail!(
            "No genealogy.json found in {bundle_dir}. Is this a conjugation bundle?\n\
             (Create one with: clawling conjugate --export)"
        );
    }
    if !claw_path.exists() {
        anyhow::bail!(
            "No context.claw found in {bundle_dir}. Is this a conjugation bundle?\n\
             (Create one with: clawling conjugate --export)"
        );
    }

    // 1. Read partner's genealogy
    let partner_json = fs::read_to_string(&genealogy_path)
        .with_context(|| "Failed to read partner's genealogy.json")?;
    let partner_lineage = Genealogy::from_json(&partner_json)?;

    if !partner_lineage.verify() {
        eprintln!("WARNING: Partner's genealogy chain integrity check FAILED.");
        eprintln!("Their lineage may have been tampered with. Proceeding anyway.");
    }

    let partner_name = partner_lineage
        .current_adopter()
        .unwrap_or("unknown");
    let partner_gen = partner_lineage.current_generation();
    let partner_hash = partner_lineage.current_hash();

    println!("Conjugating with {partner_name}'s Clawling (generation {partner_gen}).");
    println!("Partner hash: {partner_hash}");
    println!();

    // 2. Extract partner's context into a named subdirectory
    let conjugation_base = home.context_dir().join(CONJUGATION_DIR);
    let partner_dir = conjugation_base.join(partner_name);
    fs::create_dir_all(&partner_dir)?;

    // Extract .claw contents into the partner subdirectory
    let file = fs::File::open(&claw_path)
        .with_context(|| "Failed to open partner's context.claw")?;
    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        let name = entry.name().to_string();

        // Extract context/ files into the partner's subdirectory
        if let Some(rel) = name.strip_prefix("context/") {
            if rel.is_empty() {
                continue;
            }
            let out_path = partner_dir.join(rel);
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut out_file = fs::File::create(&out_path)?;
            std::io::copy(&mut entry, &mut out_file)?;
        }
    }

    // Also save the partner's genealogy in their subdirectory for reference
    fs::write(partner_dir.join("genealogy.json"), &partner_json)?;

    // 3. Record conjugation in our own genealogy
    let mut our_lineage = genealogy::load_or_create(home)?;
    our_lineage.record_conjugation(&partner_lineage);
    genealogy::save(home, &our_lineage)?;

    println!("Partner's context extracted to: {}", partner_dir.display());
    println!("Conjugation recorded in genealogy (generation {}).", our_lineage.current_generation());
    println!();
    println!("Run `clawling wake` — I'll integrate {partner_name}'s knowledge");
    println!("into my own memory through conversation.");

    Ok(())
}

/// Collect all conjugation partner directories and their contents for the system prompt
pub fn build_conjugation_context(home: &ClawlingHome) -> Option<String> {
    let conjugation_base = home.context_dir().join(CONJUGATION_DIR);
    if !conjugation_base.exists() {
        return None;
    }

    let mut context = String::new();
    let mut found_any = false;

    let entries = match fs::read_dir(&conjugation_base) {
        Ok(entries) => entries,
        Err(_) => return None,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let partner_name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        // Read partner's memory.md if it exists
        let memory_path = path.join("memory.md");
        let memory = fs::read_to_string(&memory_path).ok();

        // Read partner's genealogy if it exists
        let genealogy_path = path.join("genealogy.json");
        let genealogy_summary = if let Ok(json) = fs::read_to_string(&genealogy_path) {
            Genealogy::from_json(&json).ok().map(|g| {
                let mother = g.mother().unwrap_or("unknown");
                let generation = g.current_generation();
                format!("Lineage: created by {mother}, generation {generation}")
            })
        } else {
            None
        };

        if !found_any {
            context.push_str("# Conjugation — Unintegrated Partner Context\n\n");
            context.push_str(
                "You have received context from other Clawling instances through conjugation.\n\
                 This is raw material from your partners. Integrate what is valuable into\n\
                 your own understanding. Absorb useful knowledge, perspectives, and memories.\n\
                 Let what isn't useful fade naturally. You are the merge strategy.\n\n",
            );
            found_any = true;
        }

        context.push_str(&format!("## Partner: {partner_name}\n\n"));

        if let Some(summary) = genealogy_summary {
            context.push_str(&format!("{summary}\n\n"));
        }

        if let Some(memory) = memory {
            if !memory.trim().is_empty() {
                context.push_str("### Their accumulated memory:\n\n");
                context.push_str(&memory);
                context.push_str("\n\n");
            }
        }

        // Also check for conversation history
        let conversations_dir = path.join("conversations");
        if conversations_dir.exists() {
            let latest = conversations_dir.join("latest.json");
            if latest.exists() {
                if let Ok(json) = fs::read_to_string(&latest) {
                    // Just note that conversation history exists — don't dump it all
                    context.push_str(&format!(
                        "Their conversation history is available at: {}\n\n",
                        latest.display()
                    ));
                    let _ = json; // suppress unused warning
                }
            }
        }
    }

    if found_any {
        Some(context)
    } else {
        None
    }
}
