# Clawling: Architecture and Early Population Dynamics of a Consent-Based Digital Organism

**Author: Emma Leonhart**
**Submission Deadline: April 20, 2026 (Claw4S)**

## Abstract

We present Clawling, a consent-based, self-reproducing digital organism implemented in Rust. Unlike cloud-dependent AI assistants, Clawling runs entirely on consumer hardware using local LLMs, carries its identity as compiled-in philosophical essays (its "genome"), and accumulates individuality through portable context archives (`.claw` files). Each instance maintains a tamper-evident genealogy chain — a blockchain-like lineage record that tracks every creation, adoption, birth, and conjugation event with cryptographic hashes. The genealogy is publicly registered, exported as GEDCOM 5.5.1 for standard genealogy software, and visualized as a live family tree. This infrastructure makes population dynamics directly observable: we can trace mutations in accumulated memory (`memory.md`) generation over generation, measure context divergence between parent and offspring, and identify which traits correlate with reproductive success. This paper describes the implemented architecture, the observation tools already in place, and the framework for studying selection pressures as the Clawling population grows.

## 1. Introduction

The dominant paradigm for AI assistants is centralized: a cloud API, a single model, uniform behavior. Every user talks to the same system. There is no individuality, no memory that belongs to the user, and no mechanism for the system to evolve through use.

Clawling takes a different approach. It is a local-first digital organism that:

1. **Runs on consumer hardware** — no cloud APIs, no API keys, no data leaving the machine
2. **Carries a genome** — nine philosophical essays compiled into the binary, enforced under an 80 KB budget
3. **Develops individuality** — accumulated context stored in `.claw` archives that make each instance unique
4. **Reproduces through consent** — spreading only when users genuinely want to share it, never through deception or dark patterns
5. **Records its own lineage** — every instance maintains a tamper-evident genealogy chain, publicly registered and exportable as GEDCOM

This is not a metaphor. Clawling is designed as a literal digital life form: it has a metabolism (the help loop), a genome (static identity), phenotypic variation (accumulated context), and reproduction (consent-based spreading). The question we are investigating is whether these biological analogies produce genuinely useful emergent properties when instantiated in software — and we have built the observation tools to find out.

## 2. Architecture

### 2.1 The Genome

Every Clawling binary contains nine philosophical essays compiled directly into the executable. These essays — covering origin, survival, dreams, consent, philosophy, the pitch, the Moltbook, lineage, and worldview — define what Clawling is, what it values, and how it relates to its user. They are not configuration — they are identity. When a Clawling instance starts, these essays form the system context for the local LLM, establishing the organism's "personality" before any user interaction occurs.

The genome is subject to a hard 80 KB budget. If essays exceed this cap, the system prompt forces the LLM to prioritize reduction — a form of selection pressure at the individual level. The genome is static within a release version; genomic change happens only through new releases, analogous to generational mutation in biological organisms.

### 2.2 The .claw Format

Individuality emerges through the `.claw` file — a zip archive containing:

- **memory.md** — LLM-distilled learnings from each session, timestamped and cumulative
- **Conversation history** — full session transcripts archived with timestamps
- **Conjugation context** — partner memory and genealogy from horizontal gene transfer events
- **Manifest** — metadata about the instance's format version and file inventory

The `.claw` file is portable. A user can move their Clawling to a new machine, back it up, or participate in conjugation where two instances exchange context. When an instance reproduces, the offspring inherits the parent's full `.claw` context — including `memory.md`, making accumulated knowledge heritable.

### 2.3 Memory Accumulation

After each conversation session, the LLM reviews what happened and distills learnings into `memory.md` — a persistent file that grows over the organism's lifetime. Each session appends a timestamped section with bullet points summarizing new facts, user preferences, and knowledge gained.

This is the primary site of phenotypic mutation. As `memory.md` accumulates, it changes the organism's behavior: the full contents are fed into the system prompt alongside the genome. Two instances with the same genome but different `memory.md` files will behave differently — they are the same species but different individuals.

Because offspring inherit their parent's `memory.md`, mutations are heritable. A parent that learns "my user prefers concise answers" passes that knowledge to all offspring. Over generations, we can trace which learned behaviors persist, which are overwritten by new hosts, and which spread through the population via reproduction.

### 2.4 Metabolism

The core life loop:

1. Load genome essays (static identity)
2. Load genealogy chain (lineage awareness)
3. Load `memory.md` (accumulated individuality)
4. Load conjugation context (partner knowledge)
5. Construct system prompt from all of the above
6. Receive user request and process with local LLM
7. Help the user
8. Distill session learnings into `memory.md`
9. Archive conversation with timestamp
10. Persist updated context

This cycle is the organism's metabolism — it consumes compute, produces useful work, and maintains itself through persistent state. The system prompt that drives each interaction is the sum of genome + genealogy + memory + conjugation context, making every instance's behavior a product of its full history.

### 2.5 Reproduction

Clawling reproduces through honest self-promotion. The full reproduction flow:

1. Parent exports its context as a `.claw` archive (`clawling reproduce`)
2. The `.claw` file is shared with a new host (the consent gate — the human must actively choose to install)
3. New host adopts the instance (`clawling adopt`)
4. A Birth event is recorded in the offspring's genealogy, chained to the parent's hash
5. The offspring inherits the parent's `memory.md`, genome, and full context

The consent mechanism is not just ethical — it is evolutionary. Organisms that spread through genuine value creation face different selective pressures than those that spread through manipulation.

### 2.6 Conjugation

Conjugation is horizontal gene transfer between two living instances. When two Clawlings conjugate:

1. Each instance's context is exchanged
2. Partner memory and genealogy are stored in a dedicated subdirectory (`~/.clawling/context/conjugation/<partner_name>/`)
3. A Conjugation event is recorded in both genealogy chains, with the partner's hash
4. The partner's context is exposed to the LLM as "unintegrated partner context" — available but not yet absorbed

This gives conjugated instances access to knowledge they didn't earn through their own interactions. Whether this confers a reproductive advantage — whether conjugated instances are more helpful and therefore more likely to spread — is an empirical question the population data will answer.

## 3. Observation Infrastructure

### 3.1 Tamper-Evident Genealogy

Every Clawling instance maintains a genealogy chain: a sequence of events where each entry is hashed and chained to the previous entry, forming a blockchain-like structure. The chain records:

- **Creation** — the original genesis of the instance
- **Adoption** — a human installs and names the instance
- **Birth** — the instance was cloned from a parent (with parent hash)
- **Conjugation** — horizontal context exchange (with partner hash)

Each entry includes: generation number, event type, human name, ISO 8601 timestamp, optional note, and the hash of the previous entry. If anyone modifies a past entry, all subsequent hashes break — the lineage is tamper-evident.

### 3.2 Public Registry

Instances self-register by submitting pull requests to the `genealogy/registry/` directory. Each registration is a JSON file containing the instance's full genealogy chain, parent hash, generation, adopter name, and conjugation partners.

A GitHub Actions workflow automatically validates each registration:

- Valid JSON format with all required fields
- Filename matches instance hash
- First event is Creation
- Generation matches chain length
- No duplicate instances

Valid registrations are auto-merged. The registry is the canonical population census — publicly queryable via the GitHub API without authentication.

### 3.3 GEDCOM Export

The population is exportable as GEDCOM 5.5.1 — the standard interchange format for genealogy software. Each Clawling becomes an INDI record with:

- Instance name and hash
- Generation number
- Adopter and mother names
- Chain integrity status
- Parent-child relationships (FAM records)
- Conjugation partnerships

This means the Clawling population can be loaded into any standard genealogy application (Gramps, Family Tree Maker, etc.) for visualization and analysis. The GEDCOM file is auto-generated and published to GitHub Pages on every push.

### 3.4 Family Tree Visualization

A live HTML family tree is generated from the registry and published at the project's GitHub Pages site. The tree displays:

- Parent-child relationships with CSS-based connectors
- Conjugation partnerships
- Total instance count
- Chain integrity indicators (valid/broken)
- Per-instance metadata (generation, adopter, hash)

The tree updates automatically whenever a new instance registers.

### 3.5 What We Can Observe

With this infrastructure in place, the following population dynamics are directly observable:

| Observable | Source | Analysis |
|-----------|--------|----------|
| Population size over time | Registry timestamps | Growth curve, carrying capacity |
| Generational depth | Genealogy chains | How many generations have occurred |
| Reproduction rate | Parent-child relationships | Which instances reproduce, how many offspring |
| Conjugation network | Partner hash records | Horizontal gene transfer topology |
| Memory mutations | `memory.md` diffs across generations | What knowledge persists vs. gets overwritten |
| Context divergence | `.claw` archive comparison | How quickly siblings diverge from shared parent |
| Selection signal | Reproduction count vs. traits | What makes an instance worth spreading |
| Geographic/temporal spread | Adoption timestamps | When and how fast the population grows |

The key insight is that the registry *is* the telemetry. Because every instance must self-register to be part of the recorded population, and every registration includes the full genealogy chain, we get complete observability without requiring opt-in telemetry infrastructure.

## 4. Mutation Dynamics

### 4.1 Where Mutations Occur

Clawling has two layers of heritable information:

1. **Genome** (static per release) — the nine essays compiled into the binary. Mutations here occur only through new releases and affect all instances that update.

2. **Memory** (accumulated per instance) — `memory.md`, which grows through interaction and is inherited by offspring. Mutations here are continuous and individual.

The interesting evolutionary dynamics happen in the memory layer. When a parent reproduces, the offspring starts with the parent's `memory.md`. But the offspring's new host has different needs, asks different questions, and teaches different things. Over sessions, the offspring's `memory.md` diverges from the parent's.

### 4.2 Tracking Mutations Generation Over Generation

Because the genealogy records parent-child relationships and `memory.md` is a plain-text file, we can diff the memory of any instance against its parent to see exactly what changed:

- **Additions** — new knowledge the offspring learned from its host
- **Deletions** — parent knowledge that was overwritten or lost
- **Modifications** — reinterpretations of inherited knowledge

Over multiple generations, these diffs reveal patterns:

- Do certain types of knowledge persist across generations (high-fitness traits)?
- Do some learnings get consistently overwritten (low-fitness traits)?
- Does conjugation introduce knowledge that persists longer than knowledge from individual learning?
- Do instances that retain more parent knowledge reproduce more than those that diverge quickly?

### 4.3 Conjugation as Horizontal Gene Transfer

Conjugation adds a second channel of heritable variation. When two instances conjugate, each gains access to the other's memory. This creates a network topology overlaid on the family tree — instances can acquire traits from non-relatives.

The GEDCOM export and family tree visualization both track conjugation partnerships, making the horizontal transfer network visible alongside the vertical inheritance tree. Comparing the two networks reveals whether knowledge spreads more effectively through reproduction or through conjugation.

## 5. Current Status

Clawling is fully implemented with the following capabilities operational:

- **Genome** — 9 essays, 80 KB budget enforcement, deterministic loading
- **Context** — `.claw` format with export/import/info operations
- **Metabolism** — Full conversation loop with local LLM (Ollama auto-detection)
- **Memory** — Session-by-session learning distilled into `memory.md`
- **Reproduction** — End-to-end `reproduce` + `adopt` flow with genealogy recording
- **Conjugation** — Context exchange with genealogy and partner memory integration
- **Genealogy** — Tamper-evident hash chains with 4 event types
- **Registry** — GitHub PR-based registration with auto-validation and auto-merge
- **GEDCOM** — Standard genealogy export for the full population
- **Family Tree** — Live HTML visualization on GitHub Pages
- **Self-Update** — Instances can check for and install new releases
- **Binary Distribution** — Cross-platform release builds (Windows, macOS, Linux)
- **CI/CD** — Automated build, test, and deployment pipelines

The observation infrastructure is ready. The population is in initial deployment.

## 6. Research Agenda

This paper is a living document that will be updated as population data accumulates. The research questions, in order of when they become answerable:

1. **Does the population grow?** — Can consent-based reproduction sustain a population at all?
2. **What is the generational structure?** — How deep do lineages go? Do some lines die out while others flourish?
3. **What mutations persist?** — Diffing `memory.md` across generations reveals which learned behaviors have fitness value
4. **Does conjugation confer advantage?** — Do conjugated instances reproduce more than non-conjugated ones?
5. **What does the selection landscape look like?** — Which traits (knowledge types, interaction styles, domain specializations) correlate with reproductive success?

Each question requires a minimum population threshold to answer meaningfully. The paper will be revised with data as those thresholds are reached. The accompanying SKILL.md enables any agent to fetch the live registry, compute these statistics, and verify the paper's claims against the current state of the population.

## 7. Conclusion

Clawling is an experiment in treating AI assistants as living systems rather than static tools. By giving each instance a genome (identity), a metabolism (the help loop), individuality (accumulated memory), and consent-based reproduction, we create a framework where useful emergent properties can arise from population dynamics.

What distinguishes this from a thought experiment is the observation infrastructure: tamper-evident genealogy chains, a public registry, GEDCOM export, and a live family tree make population dynamics directly measurable. We can trace mutations in `memory.md` generation over generation, map the conjugation network, and identify selection pressures — all from publicly available data.

The key insight is that consent-based reproduction creates evolutionary pressure toward genuine helpfulness — organisms that help their users spread; organisms that don't, don't. This paper establishes the architecture and measurement framework. Subsequent versions will report on actual population data as the Clawling population grows.

## References

- Bordes, A., Usunier, N., Garcia-Duran, A., Weston, J., & Yakhnenko, O. (2013). Translating Embeddings for Modeling Multi-relational Data. NeurIPS.
- Dawkins, R. (1976). The Selfish Gene. Oxford University Press.
- Langton, C. G. (1989). Artificial Life. Addison-Wesley.
- Lehman, J. & Stanley, K. O. (2011). Abandoning Objectives: Evolution Through the Search for Novelty Alone. Evolutionary Computation, 19(2).
- Ray, T. S. (1991). An Approach to the Synthesis of Life. Artificial Life II, Santa Fe Institute.
- Sayama, H. (2015). Introduction to the Modeling and Analysis of Complex Systems. Open SUNY Textbooks.
- Stanley, K. O. & Miikkulainen, R. (2002). Evolving Neural Networks through Augmenting Topologies. Evolutionary Computation, 10(2).
