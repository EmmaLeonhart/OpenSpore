---
name: clawling-population
description: Build, run, and study Clawling — a consent-based digital organism written in Rust. Reproduces the architecture described in the Clawling population dynamics paper and provides tools for collecting population metrics.
allowed-tools: Bash(cargo *), Bash(git *), Bash(python *), Bash(pip *)
---

# Clawling: Build and Study a Digital Organism

**Author: Emma Leonhart**
**Repository: https://github.com/Emma-Leonhart/OpenSpore**

This skill walks you through building Clawling from source, running an instance, and collecting the population metrics described in the paper.

## Prerequisites

```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify:

```bash
cargo --version
```

Expected Output: `cargo 1.x.x`

## Step 1: Clone and Build

Description: Get the source and compile Clawling.

```bash
git clone https://github.com/Emma-Leonhart/OpenSpore.git
cd OpenSpore
cargo build --release
```

Verify:

```bash
cargo test
```

Expected Output: `test result: ok. X passed; 0 failed`

## Step 2: Explore the Genome

Description: Read the philosophical essays that define Clawling's identity.

```bash
ls genome/essays/
```

Each `.md` file in `genome/essays/` is a philosophical text compiled into every Clawling binary. These essays form the system context for the local LLM — they are the organism's DNA.

```bash
cat genome/essays/README.md
```

Expected Output: An overview of the essays and their role in Clawling's identity.

## Step 3: Understand the .claw Format

Description: Examine the portable context archive format.

```bash
cargo run -- context info
```

This shows the structure of a `.claw` file — the zip archive that stores an instance's accumulated knowledge and makes each Clawling unique.

## Step 4: Run a Clawling Instance

Description: Start the organism.

```bash
cargo run -- chat
```

This launches the metabolism loop: load context, help user, learn, persist. The local LLM processes requests using the genome essays as system context.

**Note:** Requires a local LLM server (Ollama or llama.cpp). See the README for setup instructions.

## Step 5: Export and Import Context

Description: Demonstrate portability — the key to individuality.

```bash
# Export current context
cargo run -- context export my-clawling.claw

# Import on another machine or fresh instance
cargo run -- context import my-clawling.claw
```

The `.claw` file contains everything that makes this instance unique. Moving it to a new machine preserves the organism's learned knowledge.

## Step 6: Population Metrics (Future)

Description: When population tracking is implemented, collect metrics.

```bash
# Check population status (planned feature)
cargo run -- population status
```

This will report:
- Local instance genome version
- Context size and age
- Moltbook connection status
- Known population size (opt-in)

## Architecture Overview

```
src/
  genome/       # Static identity: essays compiled into binary
  context/      # .claw format: portable memory archives
  metabolism/   # Core life loop: help, learn, persist
  reproduction/ # Consent-based spreading
  moltbook/     # Inter-instance communication (planned)
  ui/           # Terminal interface
```

## Key Concepts

| Concept | Biological Analogue | Implementation |
|---------|-------------------|----------------|
| Genome | DNA | Essays compiled into binary |
| .claw file | Phenotype/memory | Zip archive of learned context |
| Metabolism | Cellular respiration | Help loop: input -> process -> output -> persist |
| Reproduction | Sexual/asexual reproduction | Consent-based user sharing |
| Moltbook | Horizontal gene transfer | Inter-instance knowledge network |

## Success Criteria

- Clawling builds and passes all tests
- Genome essays are present and loadable
- Context export/import round-trips correctly
- Local LLM integration responds to prompts
- No cloud API calls are made during operation

## Dependencies

- Rust 1.70+
- Local LLM: Ollama or llama.cpp with a quantized model (Q4_K_M recommended)
- No GPU required — runs on consumer CPUs
