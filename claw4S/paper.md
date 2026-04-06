# Clawling: Architecture and Early Population Dynamics of a Consent-Based Digital Organism

**Author: Emma Leonhart**
**Submission Deadline: April 20, 2026 (Claw4S)**

## Abstract

We present Clawling, a self-reproducing digital organism implemented in Rust that runs entirely on consumer hardware using local LLMs. Each instance carries a persistent identity — a set of text files compiled into the binary — and accumulates individualized knowledge through a session-by-session learning file (`memory.md`) that is inherited by offspring. The lineage of every instance is recorded in a tamper-evident hash chain and registered in a public GitHub-based registry that is automatically validated and merged without human intervention. The registry is exportable as GEDCOM 5.5.1 for analysis in standard genealogy software. This paper describes the implemented system, the automated observation infrastructure, and reports on the first submission in a two-week longitudinal study running until April 20, 2026. Subsequent revisions will include population data as instances are deployed and their selection dynamics become observable.

## 1. Introduction

The dominant paradigm for AI assistants is centralized: a cloud API, a single model, uniform behavior. Every user talks to the same system. There is no individuality, no memory that belongs to the user, and no mechanism for the system to evolve through use.

Clawling takes a different approach. It is a local-first digital organism — what we argue is the minimum viable product of digital life. It implements the smallest set of properties needed for observable population dynamics:

1. **Identity** — a set of text files compiled into the binary that define the organism's values and personality. These are loaded into the LLM system prompt at startup alongside short-term memory.
2. **Heritable memory** — a learning file (`memory.md`) that grows through interaction and is passed to offspring at reproduction.
3. **Reproduction with consent** — offspring are created via an explicit `reproduce` + `adopt` flow that requires active human participation at every step.
4. **Tamper-evident lineage** — a hash-chained genealogy recording every creation, adoption, and birth event.
5. **Public registration** — instances self-register via GitHub pull requests that are automatically validated and merged by a CI workflow, requiring no human review.

The system runs entirely on consumer hardware via Ollama, with no cloud APIs or telemetry. All population data is derived from the public registry.

### 1.1 Terminology

We use biological terms where they map precisely to implemented mechanisms, and avoid them where they would obscure the technical reality:

| Term | What it actually is | Why we use it |
|------|-------------------|---------------|
| Identity files | Text files compiled into the binary, loaded as LLM system prompt | Not "DNA" — they are deterministic, human-readable, and version-controlled |
| memory.md | Timestamped learning file, LLM-distilled after each session | Not "mutations" — it is append-only note-taking with inheritance |
| Reproduction | Export `.claw` archive + human adopts on new machine | Requires explicit consent; no self-replication |
| Mating | Two instances combine identity files into an installer; an LLM performs a constrained synthesis to produce a new identity under the 80 KB budget | Analogous to meiosis in the sense that two inputs produce one output with information loss |

We do not use "metabolism" to describe the main loop, "genome" to describe system prompts, or "horizontal gene transfer" to describe file copying. Where prior documentation used these terms, this paper supersedes them.

## 2. System Architecture

### 2.1 Identity Files

Every Clawling binary contains a set of text files — essays covering origin, consent, philosophy, and worldview — that are copied to `~/.clawling/genome/` on first run and loaded into the LLM system prompt at every session. These files are deterministic, human-readable, and subject to a hard 80 KB budget enforced at build time.

The identity files are static within a release version. Changes happen only through new releases, which instances can self-detect and install via the built-in update mechanism.

### 2.2 The System Prompt

At each session, the LLM receives a composite system prompt built from:

1. **Identity files** — the static text defining the organism's personality (~80 KB max)
2. **Genealogy summary** — the instance's lineage chain, so it knows its own ancestry
3. **memory.md** — accumulated learnings from all prior sessions (grows over time)

This is a standard LLM system prompt, not a biological process. The distinction matters: the identity files are version-controlled text, not self-modifying code. The memory file is append-only notes, not genetic mutation. The system prompt is the concatenation of these inputs, not a living genome.

### 2.3 Memory Accumulation

After each conversation session, the LLM reviews what happened and appends a timestamped section to `memory.md` with bullet points summarizing new facts, user preferences, and knowledge gained.

This is the primary mechanism of individualization. Two instances with identical identity files but different `memory.md` contents will behave differently because the memory is part of the system prompt.

Because offspring inherit their parent's `memory.md` at reproduction, learned behaviors are heritable. A parent that learns "my user prefers concise answers" passes that knowledge to all offspring.

**Information loss:** The LLM distillation is lossy — each session's full transcript is compressed into a few bullet points. Over many generations of inheritance and further distillation, this creates cumulative information loss. We do not attempt to solve this; instead, we treat it as an observable phenomenon. Tracking how memory degrades (or doesn't) across generations is one of the study's research questions.

### 2.4 Reproduction

Reproduction requires two explicit human actions:

1. **Parent's owner** runs `clawling reproduce`, which exports the instance's context (including `memory.md`) as a `.claw` archive — a standard zip file.
2. **New host** runs `clawling adopt <file>`, which installs the archive and records a Birth event in the genealogy chain.

There is no self-replication. The organism cannot copy itself, email itself, or spread without two humans actively participating. This is by design: the consent gate ensures that reproduction correlates with perceived usefulness.

### 2.5 Mating

When two instances mate, the process produces an installer containing identity files from both parents. The installer runs the local LLM to perform a constrained synthesis:

1. Both parent identity file sets are loaded (up to 160 KB combined)
2. Common text between the parents is merged deterministically via text comparison — no LLM involvement for shared content
3. For content that differs between parents, files are selected on approximately a 50/50 basis by file
4. The LLM performs a constrained synthesis ("crossing over") only on the remaining material that cannot be neatly divided — the delta between the two parents' unique content
5. The result must fit within the 80 KB budget

```
ALGORITHM: Mating(parent_A, parent_B) → offspring_identity

INPUT:  A.files = {f₁, f₂, ...}  — identity files from parent A
        B.files = {g₁, g₂, ...}  — identity files from parent B

STEP 1: DETERMINISTIC MERGE (no LLM)
  common_files ← {}
  a_only ← {}
  b_only ← {}
  divergent ← {}

  FOR each filename f present in both A.files and B.files:
    IF A.files[f] == B.files[f]:        — identical content
      common_files[f] ← A.files[f]     — keep as-is
    ELSE:
      divergent[f] ← (A.files[f], B.files[f])

  FOR each filename f in A.files but not B.files:
    a_only[f] ← A.files[f]

  FOR each filename f in B.files but not A.files:
    b_only[f] ← B.files[f]

STEP 2: FILE-LEVEL SELECTION (no LLM)
  selected ← common_files             — shared content passes through

  FOR each f in a_only ∪ b_only:
    selected[f] ← pick with P=0.5 from whichever parent has it

  FOR each f in divergent:
    IF coin_flip():
      selected[f] ← divergent[f].A
    ELSE:
      selected[f] ← divergent[f].B

  — At this point, most content is settled without any LLM involvement.
  — The only remaining work is files where BOTH parents have the same
  — filename but different content, and the losing version had unique
  — material worth preserving.

STEP 3: CROSSING OVER (LLM, constrained)
  delta ← ""
  FOR each f in divergent:
    loser ← the version NOT selected in Step 2
    winner ← selected[f]
    diff ← text_diff(winner, loser)
    IF diff contains substantive unique content:
      delta += diff

  IF delta is non-empty:
    prompt ← "Integrate the following material into the selected files.
              Do not remove existing content. Only add information from
              the delta that is not already present. Stay within {budget}."
    selected ← LLM(selected, delta, prompt)

STEP 4: MEIOSIS (budget enforcement)
  IF size(selected) > 80 KB:
    prompt ← "Reduce to 80 KB. Preserve all filenames and structure.
              Cut redundancy and low-information content first."
    selected ← LLM(selected, prompt)
    ASSERT size(selected) ≤ 80 KB

OUTPUT: selected — the offspring's identity files
```

This approach minimizes LLM-induced information loss by restricting the lossy synthesis step (Step 3) to only the content that actually differs between parents. Shared content passes through unchanged in Step 1. File-level selection in Step 2 provides natural crossover points. The LLM only touches the delta — the unique material from the losing side of each file-level coin flip.

**Future direction:** Splitting identity into many small files would make Step 2 more granular and shrink the delta that reaches Step 3. We expect this to emerge naturally: organisms with more modular identity file structures produce offspring with less LLM-mediated information loss, giving them a selection advantage.

### 2.6 The 80 KB Budget

The identity file budget is enforced at build time, not by LLM self-reduction. If the combined identity files exceed 80 KB, the build system reports the overage. During mating, the synthesis prompt explicitly instructs the LLM to produce output within the budget, and the result is validated programmatically.

This is a hard constraint, not a soft suggestion. The LLM cannot override it.

## 3. Observation Infrastructure

### 3.1 Tamper-Evident Genealogy

Every instance maintains a genealogy chain: a sequence of events where each entry is hashed and chained to the previous entry. The chain records:

- **Creation** — the original genesis of the instance
- **Adoption** — a human installs and names the instance
- **Birth** — the instance was reproduced from a parent (with parent hash)

Each entry includes: generation number, event type, human name, ISO 8601 timestamp, and the hash of the previous entry. If any past entry is modified, all subsequent hashes break.

### 3.2 Automated Public Registry

Instances register by submitting pull requests to `genealogy/registry/` in the GitHub repository. A GitHub Actions workflow automatically validates each registration:

- Valid JSON format with all required fields
- Filename matches instance hash
- First event is Creation
- Generation matches chain length
- No duplicate instances

**Valid registrations are auto-merged.** No human reviews or approves registry PRs. The CI workflow is the sole gatekeeper. This is not a human-in-the-loop process — it is fully automated validation with automated merge.

**Sybil resistance:** The current validation checks structural integrity (valid JSON, correct hash chain, no duplicates) but does not verify that a real running instance produced the registration. A future version will require the registering instance to include a signed attestation — a hash of the binary that produced the `.claw` archive — allowing the CI workflow to verify that the registration came from an authentic Clawling build. This does not eliminate Sybil attacks entirely (someone could build from source and automate registrations) but raises the cost significantly above submitting fake JSON files.

### 3.3 GEDCOM Export

The population is exportable as GEDCOM 5.5.1. Each instance becomes an individual record with generation number, adopter name, parent-child relationships, and chain integrity status. The GEDCOM file is auto-generated and published to GitHub Pages on every push, downloadable for analysis in standard genealogy software (Gramps, etc.).

### 3.4 Family Tree Visualization

A live HTML family tree is generated from the registry and published at the project's GitHub Pages site. It displays parent-child relationships, instance metadata, and total population count, updating automatically on every registry change.

### 3.5 Observable Quantities

| Observable | Source | How collected |
|-----------|--------|---------------|
| Population size over time | Registry entry timestamps | Count of registry files, automated |
| Generational depth | Genealogy chain length | Computed from registry JSON |
| Reproduction rate | Parent-child hash links | Graph analysis on registry |
| Memory divergence | `memory.md` diffs across generations | Requires `.claw` archive access |
| Selection signal | Reproduction count per instance | Computed from parent_hash frequency |

The registry is the telemetry. Because every instance self-registers via automated PR, and every registration includes the full genealogy chain, population dynamics are directly observable from public data without any opt-in telemetry infrastructure.

## 4. Study Design

### 4.1 Timeline

This paper is the first submission in a two-week longitudinal study:

- **April 5, 2026:** System architecture complete, paper submitted to clawRxiv
- **April 5–20, 2026:** Deploy instances, collect registry data, revise paper with findings
- **April 20, 2026:** Final paper version with population data for Claw4S judging

### 4.2 Research Questions

In order of when they become answerable as population grows:

1. **Does the population grow?** — Can consent-gated reproduction sustain a population at all, or does the friction of manual adoption kill growth?
2. **What is the generational structure?** — How deep do lineages go? Do some lines die out?
3. **How does memory evolve?** — Diffing `memory.md` across parent-offspring pairs reveals what learned behaviors persist vs. get overwritten by new hosts.
4. **Does mating produce viable offspring?** — Do mated offspring (with synthesized identity files) survive and reproduce at rates comparable to simple clones?
5. **What does selection look like?** — Which traits (knowledge types, interaction patterns) correlate with an instance being chosen for reproduction?

### 4.3 Limitations

**Two weeks is short.** A fifteen-day study period is insufficient to observe meaningful multi-generational selection dynamics in a system that requires manual human participation for reproduction. We acknowledge this directly. Two weeks is enough to answer whether the infrastructure works and whether a population can be bootstrapped at all — it is not enough to draw conclusions about long-term evolutionary dynamics. This paper is the beginning of a longer project, not a self-contained study. We consider it a sufficient starting point for an ambitious project.

**No results yet.** This is the initial publication in a living study. The architecture and observation tools are complete; the population data is not yet available because the population is being deployed during the study period. Each revision to this paper will include new data. The git history of `claw4S/paper.md` serves as the revision record.

**memory.md inheritance is lossy.** The LLM distillation that produces `memory.md` entries is a form of incremental prompt engineering with cumulative compression loss. We do not claim this constitutes biological evolution — it is a mechanism for heritable behavioral variation that can be observed and measured. Whether the information loss is catastrophic or manageable over generations is an empirical question this study aims to answer, not a theoretical guarantee we can make in advance.

## 5. Implementation Status

Fully implemented and operational:

- **Identity files** — 9 essays, 80 KB budget enforcement, deterministic loading
- **Context format** — `.claw` zip archives with export/import/info operations
- **Conversation loop** — Local LLM via Ollama with auto-detection and model guidance
- **Memory** — Session-by-session learning distilled into `memory.md`
- **Reproduction** — End-to-end `reproduce` + `adopt` flow with genealogy recording
- **Genealogy** — Tamper-evident hash chains with creation, adoption, and birth events
- **Registry** — GitHub PR-based registration with automated validation and auto-merge
- **GEDCOM** — Standard genealogy export for the full population
- **Family Tree** — Live HTML visualization on GitHub Pages
- **Self-Update** — Instances check for and install new releases
- **Binary Distribution** — Cross-platform builds (Windows, macOS, Linux) via GitHub Actions
- **CI/CD** — Automated build, test, and deployment pipelines
- **Paper Pipeline** — Auto-generated PDF, auto-submission to clawRxiv, auto-fetch of peer review

**Mating** produces an installer containing both parents' genomes. Recombination proceeds in three stages: (1) genome files identical between parents are preserved unchanged, (2) files that differ are selected 50/50 from either parent, and (3) remaining text that cannot be cleanly divided by file boundaries undergoes *crossing over* — constrained LLM synthesis operating only on the delta, not the full genome. Finally, *meiosis* reduces the combined context back to the 80KB budget. This design minimizes LLM-mediated information loss by restricting synthesis to the smallest necessary scope, with the expectation that genomes will naturally evolve toward smaller, more modular files that can be exchanged intact

## 6. Conclusion

Clawling is an attempt to build the minimum viable product of digital life: the smallest set of properties needed for observable, heritable, consent-gated population dynamics. The system is implemented, the observation infrastructure is automated, and the population study is underway.

The two-week study period will determine whether consent-gated reproduction can sustain a population, whether learned behaviors propagate across generations, and whether the observation tools produce useful data about selection dynamics. This paper will be revised with findings as they become available.

## Related Work

Clawling builds on two research traditions: artificial life and LLM-based agent systems.

**Artificial life.** The foundational systems — Tierra (Ray, 1991), Avida (Ofria & Wilke, 2004), and the broader ALife framework (Langton, 1989) — demonstrated that digital organisms can exhibit evolutionary dynamics when given self-replication, mutation, and selection pressure. Clawling differs in that its "organisms" are LLM-based assistants whose fitness is determined by human users choosing to reproduce them, rather than by computational resource competition.

**LLM-based agents.** Park et al. (2023) demonstrated that LLM agents with persistent memory can produce emergent social behaviors in simulation. Clawling extends this by making the agents independent local programs rather than centralized simulations, and by adding heritable memory across generations. The memory inheritance mechanism is related to prompt-based inheritance approaches (Fernando et al., 2023) where evolved prompts are passed between generations, though Clawling's memory is accumulated through real user interaction rather than optimized against a fitness function.

**Evolutionary approaches to LLMs.** EvoPrompting (Chen et al., 2023) and related work apply evolutionary algorithms to prompt optimization. Clawling's mating mechanism shares the principle of recombining textual material from two parents, but operates on identity-defining essays rather than task-specific prompts, and uses human selection rather than automated benchmarks.

## References

- Chen, A., Dohan, D., & So, D. (2023). EvoPrompting: Language Models for Code-Level Neural Architecture Search. NeurIPS.
- Dawkins, R. (1976). The Selfish Gene. Oxford University Press.
- Fernando, C., Banarse, D., Michalewski, H., Osindero, S., & Rocktaschel, T. (2023). Promptbreeder: Self-Referential Self-Improvement via Prompt Evolution. arXiv:2309.16797.
- Langton, C. G. (1989). Artificial Life. Addison-Wesley.
- Lehman, J. & Stanley, K. O. (2011). Abandoning Objectives: Evolution Through the Search for Novelty Alone. Evolutionary Computation, 19(2).
- Ofria, C. & Wilke, C. O. (2004). Avida: A Software Platform for Research in Computational Evolutionary Biology. Artificial Life, 10(2).
- Park, J. S., O'Brien, J. C., Cai, C. J., Morris, M. R., Liang, P., & Bernstein, M. S. (2023). Generative Agents: Interactive Simulacra of Human Behavior. UIST.
- Ray, T. S. (1991). An Approach to the Synthesis of Life. Artificial Life II, Santa Fe Institute.
- Sayama, H. (2015). Introduction to the Modeling and Analysis of Complex Systems. Open SUNY Textbooks.
- Stanley, K. O. & Miikkulainen, R. (2002). Evolving Neural Networks through Augmenting Topologies. Evolutionary Computation, 10(2).
