mod manifest;

use anyhow::{Context, Result};
use std::fs;
use std::io::{Read as _, Write as _};
use std::path::Path;
use zip::write::SimpleFileOptions;

use crate::home::ClawlingHome;

pub use manifest::Manifest;

/// Export the context directory from Clawling's home into a .claw archive.
/// This is the "mating format" — you zip up context for transport
/// (reproduction, conjugation, backup). Normally Clawling works directly
/// with the context directory.
pub fn export(home: &ClawlingHome, output_path: &str) -> Result<()> {
    let context_path = home.context_dir();
    if !context_path.exists() {
        println!("No context directory found. Nothing to export.");
        println!("(Context accumulates as Clawling runs and learns.)");
        return Ok(());
    }

    let file = fs::File::create(output_path)
        .with_context(|| format!("Failed to create {output_path}"))?;
    let mut zip = zip::ZipWriter::new(file);
    let options = SimpleFileOptions::default();

    // Collect context files
    let mut context_files = Vec::new();
    collect_files(&context_path, &context_path, &mut context_files)?;

    // Write manifest
    let manifest = Manifest::new(&context_files);
    let manifest_json = serde_json::to_string_pretty(&manifest)?;
    zip.start_file("manifest.json", options)?;
    zip.write_all(manifest_json.as_bytes())?;

    // Write README
    zip.start_file("README.md", options)?;
    zip.write_all(generate_readme().as_bytes())?;

    // Write context files
    for rel_path in &context_files {
        let full_path = context_path.join(rel_path);
        let mut contents = Vec::new();
        fs::File::open(&full_path)?.read_to_end(&mut contents)?;
        zip.start_file(format!("context/{rel_path}"), options)?;
        zip.write_all(&contents)?;
    }

    zip.finish()?;
    println!("Exported context to {output_path}");
    println!("  {} files, manifest generated", context_files.len());
    Ok(())
}

/// Import a .claw archive into Clawling's context directory.
/// This is how a Clawling receives context — from a parent at birth,
/// from a partner during conjugation, or from a backup restore.
pub fn import(home: &ClawlingHome, claw_path: &str) -> Result<()> {
    let context_path = home.context_dir();
    let file = fs::File::open(claw_path)
        .with_context(|| format!("Failed to open {claw_path}"))?;
    let mut archive = zip::ZipArchive::new(file)?;

    fs::create_dir_all(&context_path)?;

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        let name = entry.name().to_string();

        // Only extract context/ files into our context directory
        if let Some(rel) = name.strip_prefix("context/") {
            if rel.is_empty() {
                continue;
            }
            let out_path = context_path.join(rel);
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut out_file = fs::File::create(&out_path)?;
            std::io::copy(&mut entry, &mut out_file)?;
        }
    }

    println!("Imported context from {claw_path}");
    Ok(())
}

/// Display info about a .claw archive without extracting it.
pub fn info(claw_path: &str) -> Result<()> {
    let file = fs::File::open(claw_path)
        .with_context(|| format!("Failed to open {claw_path}"))?;
    let mut archive = zip::ZipArchive::new(file)?;

    // Try to read manifest
    if let Ok(mut manifest_entry) = archive.by_name("manifest.json") {
        let mut contents = String::new();
        manifest_entry.read_to_string(&mut contents)?;
        let manifest: Manifest = serde_json::from_str(&contents)?;
        println!("Format:  {}", manifest.format);
        println!("Version: {}", manifest.version);
        println!("Created: {}", manifest.created_at);
        println!("Files:   {}", manifest.context_files.len());
        for f in &manifest.context_files {
            println!("  - {f}");
        }
    } else {
        println!("{claw_path}: no manifest.json found");
        println!("Archive contains {} entries:", archive.len());
        for i in 0..archive.len() {
            let entry = archive.by_index(i)?;
            println!("  - {}", entry.name());
        }
    }

    Ok(())
}

fn collect_files(base: &Path, dir: &Path, out: &mut Vec<String>) -> Result<()> {
    if !dir.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_files(base, &path, out)?;
        } else {
            let rel = path.strip_prefix(base)?.to_string_lossy().to_string();
            out.push(rel.replace('\\', "/"));
        }
    }
    Ok(())
}

fn generate_readme() -> String {
    String::from(
        "# Clawling Context Archive (.claw)\n\
        \n\
        This is a .claw file — a portable context archive for a Clawling instance.\n\
        \n\
        It is a standard zip archive. You can rename it to .zip and open it with\n\
        any archive tool.\n\
        \n\
        The `context/` directory contains this Clawling's accumulated memory and\n\
        working state. The `manifest.json` describes what's inside.\n\
        \n\
        This file contains no secrets, no credentials, and no executable code.\n\
        It is purely declarative — it describes state, and the Clawling runtime\n\
        decides what to do with it.\n\
        \n\
        .claw is the transport format — Clawling zips up its context when it needs\n\
        to share it (reproduction, conjugation, backup). Normally Clawling works\n\
        directly with the files in its ~/.clawling/context/ directory.\n\
        \n\
        Learn more: https://github.com/EmmaLeonhart/OpenSpore\n",
    )
}
