# Frontier Vision: Dimension Reduction, JEPA, and Post-Transformer Thinking

> The ideas behind Clawling extend beyond the agent framework itself. This
> document captures the broader technical vision — where AI architecture should
> go and why the current paradigm is insufficient.

## The Case Against Brute-Force Scaling

The way frontier models work: invest billions making a gigantic model. Your
model can be better than others partially because you're just brute forcing it.
Sometimes you end up with something like O3 — way too expensive to run even if
it performs great. You're losing billions. Every company ends up with one tiny
model, and the diversity collapses.

## Dimension Reduction Over Distillation: "Clipping"

The proposed alternative to distillation:

1. Train a model with a **million-dimensional vector space** — massively
   over-parameterised intentionally
2. Use **latent space cartography** to map the resulting embedding space
3. Identify dimensions that don't contain useful content, or dimensions that
   encode redundant information
4. **Clip them off** — surgical removal of dead dimensions

The result: a model potentially smaller than current frontier models, equally
capable, but requiring ten or a hundred times the initial training investment
(which is the point — invest once, clip many ways).

### The Biological Analogy

You don't make somebody learn stuff by constantly expanding their brain. A baby
has a small brain and then a 30-year-old has a brain the size of an apartment
building — you don't do that because that's stupid. There's lots of stuff you
don't need, and you should clip it off later.

This is **synaptic pruning** — the brain actually has more connections in
childhood than adulthood. Learning isn't just adding wires; it's the elimination
of weak ones.

### Model Tiers Through Clipping

Instead of distilling Opus → Sonnet → Haiku, all three would be trained on the
same data in the same high-dimensional space:

- **Opus**: Many dimensions, full fidelity
- **Sonnet**: Fewer dimensions, selected for the best parameter subsets
- **Haiku**: Smallest, most aggressively clipped

They're all essentially the same model with different levels of dimensional
reduction — selecting specifically the best parameters rather than training a
small model to mimic a big one.

### The Open Beta Evolutionary Approach

You could produce a thousand tiny clipped models and let them compete:

- Release them as an open beta
- Users interact and provide feedback: "This model is good, this model is bad,
  this model is very good at taxes, this model deleted my database"
- Figure out which clippings are good through real-world use rather than
  relying on benchmarks

Benchmarks are rigid — an inherited system from a less complex time. When
"how many R's are in strawberry" becomes part of the training data, benchmarks
stop measuring what they claim to measure.

## Unified Embedding Spaces

It is an absolute shitshow how often people end up using a separate embedding
model for their RAG pipeline that doesn't match the language model's internal
embeddings. They end up with precision loss because the two models' understanding
of concepts diverges.

### The Vision

Every large language model should have a **vector database built into it** for
all of its proper nouns. You should be able to standardise **authority control**
for proper nouns in LLM embedding spaces.

- Each named entity gets a unique, stable coordinate in the model's own latent space
- Disambiguation is handled through probabilistic distributions — an entity isn't
  a point, it's a cloud of probability in embedding space
- The string "Descartes" and the entity Descartes are different things — the string
  is a path of tokens, the entity is a destination of meaning

If the model and the retrieval system share the exact same latent space, retrieval
isn't a separate step — it's a native extension of the model's own memory. No
translation error because they speak the same mathematical language.

## Hyperdimensional Computing as a Thinking Mode

Hyperdimensional Computing (HDC) / Vector Symbolic Architectures can perform
logic — AND, OR, NOT — using mathematical operations directly on high-dimensional
vectors (XOR, circular convolution, etc.).

### Integration with Architecture

Instead of the LLM guessing the next token, it could use HDC to **manipulate
the latent space itself**:

- Take the vector for "Descartes' Philosophy" and subtract the vector for "Dualism"
- In a standard LLM, this is a messy statistical guess
- In an HDC-integrated architecture, this is a precise algebraic operation yielding
  a new coordinate

This turns the latent space journey into a deterministic logical path. You
don't just trace the path — you calculate it.

### The Overflow Model

The envisioned architecture has three modes of thinking:

| Phase | Component | Action |
|-------|-----------|--------|
| Intuition | JEPA | The model "sees" the goal in latent space — no words yet, just the semantic target |
| Logic | HDC | If the path is complex, use vector algebra to check logical consistency |
| Speech | Text Diffusion | Take the abstract goal and logic, slowly refine a complete output |

When the agent isn't overwhelmed, JEPA intuition flows directly to text diffusion.
When it encounters complexity, HDC kicks in as a structured reasoning mode.

## JEPA: Intuition Before Language

JEPA (Joint-Embedding Predictive Architecture) predicts in the **abstract** before
it speaks. It doesn't predict every token — it predicts the latent state of the
next move.

- The encoder is typically a Transformer (Vision Transformer in canonical implementations)
- LeCun attacks the autoregressive *objective*, not the Transformer *architecture*
- JEPA predicts the consequence of an action in latent space
- An agent could "think" through a logical sequence (State_A × Operator_B → State_C)
  without the heavy compute of generating text

## Text Diffusion: The End of Flat Generation

LLM output feels flat because it's a greedy statistical path of least resistance.
The first word of a paragraph is decided without the model knowing how the last
word will end.

### The Commitment Problem

Large language models generate in a way where they are committed at each point in
sentence generation. They have to continue their sentence. This contributes to
errors where the LLM starts saying something, gets something wrong, and then has
to correct itself.

### Why Diffusion is Better for Speech

Text diffusion treats a sentence like a blurry image that slowly comes into focus.
It refines the entire thought simultaneously:

- A diffusion model wouldn't be committed to option one when option two is better
- It wouldn't have the same "started wrong, must correct" issue
- LLM text comes out flat because it can't build up to a point like humans do
- Humans have the gist of a point before finding the words — diffusion mimics this

For coding, left-to-right generation is somewhat helpful because code is functional.
For speaking, text diffusion is probably going to take over.

## Latent Space Cartography

Custom tools for mapping and visualising the internal geometry of embedding spaces:

- Mapping where concepts cluster in high-dimensional space
- Identifying which dimensions encode which capabilities
- Proposed for use in the "clipping" process to find dead or redundant dimensions
- **Sparse autoencoders** are the preferred interpretability microscope — they
  show what the model is actually thinking at the mechanistic level

### Latent Space Journey Reporting

The vision: a debugger that doesn't show you code but shows you the **vector
trajectory** through the model's reasoning.

- "The query started at 'Rationalism', was transformed by the temporal operator,
  and landed in the '17th Century' cluster"
- This makes Chain of Thought visible in its raw mathematical form rather than
  in the model's potentially hallucinated verbal explanation

## The Drosophila Connection

All of these ideas feed back into Clawling's organisms:

- **Clipping** could eventually apply to the model weights themselves, not just context
- **JEPA-style intuition** could replace autoregressive generation for organism reasoning
- **HDC** could provide structured logical reasoning within the 100 KB identity budget
- **Latent space cartography** provides the interpretability layer for understanding
  how organisms evolve
- **Authority control** in embedding spaces makes entity disambiguation precise
  within an organism's memory

The organisms are currently context-evolution only — model evolution is the next
frontier, and requires finding a sufficiently transparent small model (like OLMo 2,
BitNet, or SmolLM2) to experiment with.
