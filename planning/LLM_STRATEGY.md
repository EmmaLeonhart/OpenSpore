# LLM Strategy — Spore's Cognitive Substrate

## Core Requirement

Spore must run on decent consumer PCs without cloud API dependencies. The organism
should be self-contained — its thinking happens locally, on the host's machine.
No phone-home, no API keys, no subscription required.

## Candidates

### DeepSeek (current leading candidate)
- Strong reasoning capability at small parameter counts
- DeepSeek-R1 distilled variants run well on consumer hardware
- Open weights, which aligns with Spore's transparency philosophy
- Good at coding tasks (important for Spore's help capabilities)

### Other Options to Evaluate
- **Qwen 2.5** — Strong multilingual, good at code, various sizes
- **Llama 3** — Meta's open weights, well-supported ecosystem
- **Phi-3/Phi-4** — Microsoft's small but capable models
- **Mistral/Mixtral** — Good balance of size and capability

## Integration Approach

Spore should use **llama.cpp** (or its Rust bindings) as the inference engine:
- Runs GGUF quantized models efficiently on CPU and GPU
- Single library, no Python dependency
- Supports all major open model architectures
- Well-maintained, active development

The Rust binding options:
- `llama-cpp-rs` — Rust bindings to llama.cpp
- `candle` — Pure Rust ML framework (HuggingFace)
- `burn` — Pure Rust deep learning framework

## Architecture Decision

The organism binary ships without a model. On first run, it:
1. Checks if a compatible model is already present
2. If not, explains what it needs and offers to download one
3. Downloads a quantized model (Q4_K_M or similar, ~4GB)
4. Stores it alongside the binary or in a configured location

This keeps the initial binary small (~10MB) while the model is a one-time download.

## System Context

When running inference, Spore's system prompt is constructed from:
1. The genome essays (compiled into the binary)
2. The .claw context (accumulated memory and personality)
3. The current task context

This means the genome is always present — Spore always knows who it is,
regardless of what the user asks it to do.

## Hardware Requirements (Target)

- **Minimum:** 8GB RAM, any modern CPU (runs Q4 quantized ~7B model)
- **Recommended:** 16GB RAM, GPU with 6GB+ VRAM (runs Q4 ~13B model smoothly)
- **Ideal:** 32GB RAM or GPU with 12GB+ VRAM (can run larger/higher-quality models)
