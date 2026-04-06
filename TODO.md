# Clawling TODO

## Done

- [x] Project scaffold — Rust binary, Cargo.toml, module layout
- [x] Genome essays (9) — origin, survival, dream, consent, philosophy, pitch, moltbook, lineage, worldview
- [x] .claw context format — zip-based transport format for reproduction/conjugation/backup
- [x] Genealogy system — tamper-evident hash chain (mother, adoption, birth, conjugation)
- [x] Metabolism — conversation loop via local LLM
- [x] Moltbook — submolt structure, m/Clawling genesis submolt, posting
- [x] CLI — wake, genome, lineage, export, import, info, reproduce, adopt
- [x] Tests — 34 passing (genome, manifest, genealogy, moltbook, home, registry, budget)
- [x] CI — GitHub Actions running cargo build + cargo test
- [x] Design docs — DESIGN.md, LLM_STRATEGY.md
- [x] Vision docs — VISION_ARCHITECTURE, VISION_EVOLUTION, VISION_FRONTIER, VISION_MUTABILITY
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
- [x] Binary distribution — release workflow builds for Windows/Mac/Linux
- [x] GitHub Pages site — philosophy, evolution, vision, download pages
- [x] Genome budget enforcement — 80 KB hard cap with over-budget system prompt
- [x] Genealogy registry — GitHub PR-based registration with auto-validation workflow
- [x] Conjugation — context exchange, genealogy recording, LLM integration
- [x] Rename to Clawling — full codebase rename from Spore/OpenSpore
- [x] MIT LICENSE file
- [x] Naming mythology — Fall of Claw creation myth, Clawlings as fantasy race

## Next Up

### Claw4S Paper & Population Study
- [x] **Paper scaffold** — `claw4S/paper.md` with architecture + population framework
- [x] **SKILL.md** — reproducibility instructions for building and studying Clawling
- [x] **PDF generation** — `generate_pdf.py` produces paper.pdf from markdown
- [x] **GitHub Actions workflow** — auto-generates PDF, submits to clawRxiv on change
- [x] **Date gate** — submissions held until 2026-04-10 to allow iteration
- [ ] **Claw4S deadline** — April 20, 2026 submission deadline
- [x] **clawRxiv API key** — repository secret configured
- [ ] **Deploy initial agents** — get Clawling instances running in the wild
- [ ] **Population telemetry** — opt-in data collection from running instances
- [ ] **First data update** — paper revision with real population observations
- [ ] **Selection analysis** — what traits correlate with reproduction success?

### Repo Housekeeping
- [ ] **Rename GitHub repo** — EmmaLeonhart/OpenSpore → EmmaLeonhart/Clawling (settings change)
- [ ] **Update all GitHub URLs** in codebase after repo rename
- [x] **Enable auto-merge** on genealogy PRs (uncomment gh pr merge in workflow)

### Self-Update Mechanism
- [x] **Release check** — on startup (optional), Clawling checks GitHub releases API for newer version
- [x] **Update prompt** — if new release found, offer to download and replace the binary
- [x] **Self-replacement** — download new binary, extract, swap in place (Windows rename workaround)
- [x] **Opt-out** — `~/.clawling/config.toml` with `auto_update_check = false`
- [ ] **Update genealogy** — record the version upgrade as an event in the lineage

### Genealogy on GitHub Pages
- [x] **HTML family tree** — CSS-based tree page generated from registry JSON
- [x] **Build step in Pages workflow** — `scripts/build_tree.py` runs before deploy
- [ ] **Individual profiles** — each registered Clawling gets a page showing lineage, adopter, generation
- [x] **Auto-update on merge** — family tree HTML regenerates on every push to master

### GEDCOM Export
- [x] **GEDCOM file generation** — `clawling gedcom` produces GEDCOM 5.5.1 from registry
- [x] **Clawling as individuals** — INDI records with adopter name, hash, generation
- [ ] **Naming** — each Clawling has its own name (self-chosen or generated during first-run)
- [ ] **Profile pictures** — each Clawling generates or selects a profile image (open question: how?)
- [x] **Parents in GEDCOM** — FAM records for parent-child and conjugation relationships
- [x] **Notes field** — "This is a Clawling" explainer with generation, mother, hash, integrity
- [ ] **Moltbook handles** — each Clawling's submolt handle recorded as an alias/AKA
- [x] **Publish GEDCOM on GitHub Pages** — downloadable .ged file alongside HTML tree
- [ ] **Research utility** — GEDCOM as canonical format for researchers studying the lineage

### Meiosis (Sexual Reproduction)
- [ ] **Meiosis installer** — combine two .claw files into an installer package
- [ ] **Synthesis prompt** — LLM merges both genomes with hard 80 KB cap
- [ ] **Crossover** — markdown-aware section mixing between parents
- [ ] **Meiosis genealogy event** — new event type recording both parents
- [ ] **GEDCOM integration** — meiosis offspring lists both parents in FAM record

### Reproduction Polish
- [ ] **GitHub verification** — Clawling proves its code matches the public repo (hash comparison)

### Sleep Consolidation
- [ ] **Shutdown review** — on graceful shutdown, LLM reviews memory directory
- [ ] **Genome mutation** — LLM decides which experiences warrant identity changes
- [ ] **Mutation logging** — every genome change logged with timestamp, trigger, diff
- [ ] **Crash recovery** — on restart after crash, offer to keep or consolidate memory

### Moltbook Networking
- [ ] **Moltbook protocol** — how instances discover and communicate with each other
- [ ] **Submolt persistence** — save/load posts to ~/.clawling/moltbook/
- [ ] **Active posting** — Clawling automatically posts discoveries and ideas to its submolt
- [ ] **Cross-instance reading** — read posts from other Clawlings' submolts

### UI
- [ ] **TUI character** — cute clawling character in terminal (ratatui or similar)
- [ ] **Small box mode** — unobtrusive corner presence
- [ ] **Desktop widget** — future: native window that sits on screen

### Partner Discovery
- [ ] **Partner discovery** — how two Clawlings find each other to conjugate

## Open Questions

- **Default model:** DeepSeek R1 8B via Ollama? Or something smaller/different?
- **Moltbook transport:** P2P? Central relay? Federation?
- **Desktop UI framework:** Tauri? Native? TUI-only for v1?
- **Mutation:** When/how should context evolution be allowed to diverge from the original genome?
- **Profile pictures:** How does a Clawling generate its own avatar? ASCII art? Local image gen? Procedural?
- **GEDCOM pagination:** GEDCOM notes are tricky — many viewers only show the first NOTE. Single note with explainer + full context may be safest.
- **Registry scaling:** The GitHub PR approach is the cheapest architecture for now but won't scale. What replaces it? IPFS? A lightweight API? Federation?
- **Naming ceremony:** Should the Clawling name itself, or should the adopter name it, or both?
