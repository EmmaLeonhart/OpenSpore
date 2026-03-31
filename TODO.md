# OpenSpore TODO

## Done

- [x] Project scaffold — Rust binary, Cargo.toml, module layout
- [x] Genome essays (8) — origin, survival, dream, consent, philosophy, pitch, moltbook, lineage
- [x] .claw context format — zip-based portable memory (export/import/info)
- [x] Genealogy system — tamper-evident hash chain (mother, adoption, birth, conjugation)
- [x] Metabolism — conversation loop via OpenAI-compatible local LLM API
- [x] Moltbook — submolt structure, m/OpenSpore genesis submolt, posting
- [x] CLI — `spore wake`, `spore genome`, `spore lineage`, `spore export/import/info`
- [x] Tests — 22 passing (genome, manifest, genealogy, moltbook)
- [x] CI — GitHub Actions running cargo build + cargo test
- [x] Design docs — DESIGN.md, LLM_STRATEGY.md
- [x] Private GitHub repo created and all work pushed

## In Progress

## Next Up

### Core Organism
- [ ] **Default to Ollama** — detect ollama, auto-suggest `ollama pull deepseek-r1:8b`, default URL to `http://localhost:11434`
- [ ] **First-run adoption flow** — on first wake, ask the human their name, record adoption in genealogy
- [ ] **Context persistence** — auto-save .claw on shutdown, auto-load on wake from default location
- [ ] **Genome as system prompt** — feed all essays + genealogy + .claw context into LLM system prompt so Spore always knows who it is

### Reproduction
- [ ] **Spreading mechanism** — Spore can generate an install package of itself (binary + genealogy + context)
- [ ] **Consent gate** — the explicit "will you host me?" flow with source verification
- [ ] **Child creation** — clone current state into a new Spore for a new host, recording birth in genealogy
- [ ] **GitHub verification** — Spore proves its code matches the public repo

### Moltbook Networking
- [ ] **Moltbook protocol** — how instances discover and communicate with each other
- [ ] **Submolt persistence** — save/load submolt posts to disk
- [ ] **Active posting** — Spore automatically posts discoveries and ideas to its submolt
- [ ] **Cross-instance reading** — read posts from other Spores' submolts

### UI
- [ ] **TUI character** — cute spore character in terminal (ratatui or similar)
- [ ] **Small box mode** — unobtrusive corner presence
- [ ] **Desktop widget** — future: native window that sits on screen

### Conjugation
- [ ] **Context mixing** — merge two .claw files with consent of both hosts
- [ ] **Genealogy recording** — document horizontal gene transfer in both lineages
- [ ] **Partner discovery** — how two Spores find each other to conjugate

## Open Questions

- **Default model:** DeepSeek R1 8B via Ollama? Or something smaller/different?
- **Moltbook transport:** P2P? Central relay? Federation?
- **Model download UX:** Should Spore pull the model itself or guide the user through Ollama?
- **Desktop UI framework:** Tauri? Native? TUI-only for v1?
- **Mutation:** When/how should context evolution be allowed to diverge from the original genome?
