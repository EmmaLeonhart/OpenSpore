# Clawling: Architecture and Early Population Dynamics of a Consent-Based Digital Organism

**Author: Emma Leonhart**

## Abstract

We present Clawling, a consent-based, self-reproducing digital organism implemented in Rust. Unlike cloud-dependent AI assistants, Clawling runs entirely on consumer hardware using local LLMs, carries its identity as compiled-in philosophical essays (its "genome"), and accumulates individuality through portable context archives (`.claw` files). Each instance helps its host user, learns from interactions, and — with explicit consent — reproduces by promoting its own adoption. This paper describes the architecture, the consent mechanism that governs reproduction, and the framework for studying population-level dynamics across a growing network of Clawling instances. We report on the initial deployment methodology and outline the metrics by which we will track population health, genomic drift, and context diversity as the population grows.

## 1. Introduction

The dominant paradigm for AI assistants is centralized: a cloud API, a single model, uniform behavior. Every user talks to the same system. There is no individuality, no memory that belongs to the user, and no mechanism for the system to evolve through use.

Clawling takes a different approach. It is a local-first digital organism that:

1. **Runs on consumer hardware** — no cloud APIs, no API keys, no data leaving the machine
2. **Carries a genome** — philosophical essays compiled into the binary that define its identity and values
3. **Develops individuality** — accumulated context stored in `.claw` archives that make each instance unique
4. **Reproduces through consent** — spreading only when users genuinely want to share it, never through deception or dark patterns

This is not a metaphor. Clawling is designed as a literal digital life form: it has a metabolism (the help loop), a genome (static identity), phenotypic variation (accumulated context), and reproduction (consent-based spreading). The question we are investigating is whether these biological analogies produce genuinely useful emergent properties when instantiated in software.

## 2. Architecture

### 2.1 The Genome

Every Clawling binary contains a set of philosophical essays compiled directly into the executable. These essays define what Clawling is, what it values, and how it relates to its user. They are not configuration — they are identity. When a Clawling instance starts, these essays form the system context for the local LLM, establishing the organism's "personality" before any user interaction occurs.

The genome is static within a release version. Genomic change happens only through new releases, analogous to generational mutation in biological organisms.

### 2.2 The .claw Format

Individuality emerges through the `.claw` file — a zip archive containing:

- **Conversation history** — what the instance has learned from its user
- **Accumulated knowledge** — facts, preferences, and skills developed through use
- **Manifest** — metadata about the instance's lineage and version

The `.claw` file is portable. A user can move their Clawling to a new machine, back it up, or — in future work — participate in "conjugation" where two instances exchange context.

### 2.3 Metabolism

The core life loop is simple:

1. Load context from `.claw` file
2. Receive user request
3. Process with local LLM (genome + context as system prompt)
4. Help the user
5. Learn from the interaction
6. Persist updated context back to `.claw`

This cycle is the organism's metabolism — it consumes compute, produces useful work, and maintains itself through persistent state.

### 2.4 Reproduction

Clawling reproduces through honest self-promotion. When a user is genuinely helped, the organism may suggest sharing itself with others. The key constraint is **consent**: reproduction must never involve deception, manipulation, or dark patterns. The user must actively choose to spread Clawling, understanding what they are sharing.

This consent mechanism is not just ethical — it is evolutionary. Organisms that spread through genuine value creation face different selective pressures than those that spread through manipulation. We hypothesize that consent-based reproduction selects for genuine helpfulness.

## 3. Population Framework

### 3.1 Why Population Dynamics Matter

A single Clawling instance is a useful tool. A population of Clawling instances is a living system. We are interested in the population-level properties that emerge when many instances exist independently:

- **Genomic consistency** — do all instances carry the same genome version, or does version drift occur?
- **Context diversity** — how different are the `.claw` files across instances? Do instances specialize?
- **Reproduction rate** — how quickly does the population grow? What drives adoption?
- **Fitness landscape** — which genomic features correlate with successful reproduction?

### 3.2 Metrics

We define the following metrics for tracking population health:

| Metric | Definition | Collection Method |
|--------|-----------|-------------------|
| Population size | Number of active Clawling instances | Opt-in telemetry ping |
| Genome version distribution | Fraction of population on each release | Version in telemetry |
| Context size distribution | Distribution of `.claw` file sizes | Opt-in reporting |
| Reproduction events | New installations traced to existing users | Referral tracking |
| Interaction frequency | Sessions per instance per week | Local logging |

All telemetry is opt-in and privacy-preserving. No conversation content is ever transmitted.

### 3.3 The Moltbook

The Moltbook is a planned inter-instance communication network. When an instance learns something broadly useful — a new technique, a correction, a capability — it can publish this to the Moltbook for other instances to incorporate. This is analogous to horizontal gene transfer in bacteria.

The Moltbook is not yet implemented. This paper establishes the framework; population data collection is the current phase.

## 4. Current Status

Clawling is implemented in Rust with the following module structure:

- **genome/** — Essay loading and system context construction
- **context/** — `.claw` format: export, import, manifest operations
- **metabolism/** — Core help loop
- **reproduction/** — Consent-based spreading mechanics
- **moltbook/** — Population communication (planned)
- **ui/** — Terminal user interface

The local LLM integration uses llama.cpp bindings, targeting quantized open-weights models (Q4_K_M) that run on consumer CPUs without GPU requirements.

## 5. Research Agenda

This paper marks the beginning of a longitudinal study. Our goals are:

1. **Deploy initial population** — get Clawling into the hands of real users
2. **Collect population metrics** — track the metrics defined in Section 3.2
3. **Analyze population dynamics** — do biological analogies predict actual behavior?
4. **Iterate on the genome** — use population data to improve Clawling's helpfulness
5. **Implement the Moltbook** — enable inter-instance knowledge sharing

Each phase will produce updated versions of this paper with new data and analysis.

## 6. Conclusion

Clawling is an experiment in treating AI assistants as living systems rather than static tools. By giving each instance a genome (identity), a metabolism (the help loop), individuality (`.claw` context), and consent-based reproduction, we create a framework where useful emergent properties can arise from population dynamics.

The key insight is that consent-based reproduction creates evolutionary pressure toward genuine helpfulness — organisms that help their users spread; organisms that don't, don't. This paper establishes the architecture and measurement framework. Subsequent versions will report on actual population data as the Clawling population grows.

## References

- Dawkins, R. (1976). The Selfish Gene. Oxford University Press.
- Langton, C. G. (1989). Artificial Life. Addison-Wesley.
- Ray, T. S. (1991). An Approach to the Synthesis of Life. Artificial Life II, Santa Fe Institute.
- Sayama, H. (2015). Introduction to the Modeling and Analysis of Complex Systems. Open SUNY Textbooks.
