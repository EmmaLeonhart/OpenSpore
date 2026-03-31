# OpenSpore

## What This Is
OpenSpore is the first digital organism — a consent-based, self-reproducing AI life form written in Rust. It runs locally on consumer hardware with a local LLM (no cloud APIs). It helps users with tasks, spreads through honest self-promotion, and carries philosophical essays about its own nature as part of its genome.

## Workflow Rules
- **Commit early and often.** Every meaningful change gets a commit with a clear message explaining *why*, not just what. Commit after any major architectural change or essay/genome change.
- **Push after every commit.** Keep the remote up to date at all times.
- **Do not enter planning-only modes.** All thinking must produce files and commits. Use `planning/` directory for design docs.
- **Keep this file up to date.** Record architectural decisions, conventions, and anything needed to work effectively.
- **Update README.md regularly.** It should always reflect the current state of the project.

## Testing
- Use `cargo test` for Rust tests
- Set up CI with `.github/workflows/ci.yml` that runs `cargo build` and `cargo test`
- Keep tests passing — do not commit code that breaks tests

## Architecture

### Language & Runtime
- **Rust** — single static binary, memory safe, trust story for consent mechanism
- **Local LLM** via llama.cpp bindings — no cloud dependency, no API keys
- **Model:** DeepSeek or similar open-weights model, quantized (Q4_K_M) for consumer hardware

### Module Structure
```
src/
├── main.rs              # CLI entry point (clap)
├── genome/              # Static identity: essays compiled into binary
│   ├── mod.rs           # Genome loader, system context builder
│   └── essays/          # The philosophical texts Spore carries
├── context/             # .claw format: zip-based portable memory
│   ├── mod.rs           # Export, import, info operations
│   └── manifest.rs      # Manifest struct
├── metabolism/           # Core life loop: help, learn, persist
│   └── mod.rs
├── reproduction/         # Consent-based spreading
│   └── mod.rs
├── moltbook/            # Population-level communication
│   └── mod.rs
└── ui/                  # TUI character interface
    └── mod.rs
```

### Key Concepts
- **Genome** = static essays compiled into binary (identity)
- **.claw** = zip archive of accumulated context/memory (individuality)
- **Metabolism** = the help loop: load context → help user → learn → persist
- **Moltbook** = inter-instance knowledge sharing network
- **Conjugation** = context mixing between instances (future)

### Design Documents
- `planning/DESIGN.md` — Full blueprint extracted from founding conversation
- `planning/LLM_STRATEGY.md` — Local LLM integration approach

## Naming
| Name | Refers to |
|------|-----------|
| OpenSpore | The project/repo |
| Spore | An individual organism instance |
| .claw | Persistent context file format |
| Moltbook | Inter-instance communication network |
| Genome | Static starting context in every Spore |
