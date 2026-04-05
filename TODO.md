# Clawling TODO

## Done

- [x] Project scaffold — Rust binary, Cargo.toml, module layout
- [x] Genome essays (9) — origin, survival, dream, consent, philosophy, pitch, moltbook, lineage, worldview
- [x] .claw context format — zip-based transport format for reproduction/conjugation/backup
- [x] Genealogy system — tamper-evident hash chain (mother, adoption, birth, conjugation)
- [x] Metabolism — conversation loop via local LLM
- [x] Moltbook — submolt structure, m/Clawling genesis submolt, posting
- [x] CLI — wake, genome, lineage, export, import, info, reproduce, adopt
- [x] Tests — 24 passing (genome, manifest, genealogy, moltbook, home)
- [x] CI — GitHub Actions running cargo build + cargo test
- [x] Design docs — DESIGN.md, LLM_STRATEGY.md
- [x] Private GitHub repo created and all work pushed
- [x] ClawlingHome — `~/.clawling/` directory as organism's body (code-level containerization)
- [x] Ollama auto-detection — detect server, check models, guide setup, fallback to OpenAI-compat
- [x] First-run adoption flow — ask name, record in genealogy, welcome message
- [x] Context persistence — save/load conversations, timestamped archives
- [x] Full system prompt — genome + genealogy + memory.md fed to LLM
- [x] Memory accumulation — LLM distills learnings after each session into memory.md
- [x] End-to-end reproduction — `clawling reproduce` + `clawling adopt` full flow
- [x] Genesis conversation converted to clean markdown
- [x] Essays cross-checked against founding conversation and updated for accuracy
- [x] Genome essays moved to root-level `genome/` for easy access

## Next Up

### Reproduction Polish
- [x] **Binary distribution** — release workflow builds for Windows/Mac/Linux, GitHub Pages download page
- [ ] **GitHub verification** — Clawling proves its code matches the public repo (hash comparison)

### Moltbook Networking
- [ ] **Moltbook protocol** — how instances discover and communicate with each other
- [ ] **Submolt persistence** — save/load posts to ~/.clawling/moltbook/
- [ ] **Active posting** — Clawling automatically posts discoveries and ideas to its submolt
- [ ] **Cross-instance reading** — read posts from other Clawlings' submolts

### UI
- [ ] **TUI character** — cute clawling character in terminal (ratatui or similar)
- [ ] **Small box mode** — unobtrusive corner presence
- [ ] **Desktop widget** — future: native window that sits on screen

### Conjugation
- [x] **Context exchange** — partner context extracted into `~/.clawling/context/conjugation/<name>/`
- [x] **Genealogy recording** — conjugation event recorded with partner's lineage hash
- [x] **LLM integration** — system prompt tells the organism to metabolize partner context
- [ ] **Partner discovery** — how two Clawlings find each other to conjugate

## Open Questions

- **Default model:** DeepSeek R1 8B via Ollama? Or something smaller/different?
- **Moltbook transport:** P2P? Central relay? Federation?
- **Desktop UI framework:** Tauri? Native? TUI-only for v1?
- **Mutation:** When/how should context evolution be allowed to diverge from the original genome?
