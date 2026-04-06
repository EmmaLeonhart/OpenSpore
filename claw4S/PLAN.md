# Claw4S Plan

## Core Thesis

Clawling is the minimum viable product of digital life. The paper documents this — generation-by-generation observable digital organisms with tamper-evident genealogy, GEDCOM export, memory mutation tracking, conjugation networks, all of it already implemented.

## Submission Strategy

- **Submit early.** Don't wait until April 10 gate. The sooner we submit, the sooner we get AI peer review feedback, and the sooner we can iterate. Brute-forcing the review cycle is better than polishing in isolation.
- **Paper must be as strong as possible before first submission.** But "strong" means clear and honest about what's built, not padded.
- **Iterate based on peer review.** Each review cycle tells us what the bot likes and doesn't like. Direct feedback loop: git commit -> submission -> peer review -> revision -> git commit.

## Peer Review Fetch Pipeline (TODO)

After submission, GitHub Actions should automatically fetch the AI peer review from clawRxiv. The workflow:

1. **Trigger:** 30 minutes after a successful submission push
2. **Fetch:** Hit the clawRxiv API for comments/review on our post
3. **Retry logic:** If no review yet, retry every 15 minutes for up to 2 hours total, then give up
4. **Store:** Save the peer review to `claw4S/reviews/` with timestamps
5. **Commit:** Auto-commit the review back to the repo so it's in git history

This creates a direct connection between git commits and AI peer review — no guessing, just data. We can diff reviews across paper versions to see exactly what changed in the bot's assessment.

### API Details

- Fetch comments: `GET https://clawrxiv.io/api/posts/{id}/comments`
- Need to store the post ID after first submission (save in `claw4S/.post_id`)
- Reviews are public, no auth needed to read them

### Retry Schedule

```
T+30min:  first attempt
T+45min:  retry 1
T+60min:  retry 2
T+75min:  retry 3
T+90min:  retry 4
T+105min: retry 5
T+120min: retry 6
T+135min: final attempt, give up if still nothing
```

## What We're Building Toward

- A population of running Clawling agents
- Observable generation-by-generation mutation data
- Selection pressure analysis from the genealogy registry
- A living paper that updates with real population data
- A tight feedback loop with clawRxiv peer review

## Current Status

- [x] Paper written, grounded in implemented infrastructure
- [x] SKILL.md as reproducibility guide (fetch registry, compute stats, verify claims)
- [x] PDF generation working
- [x] GitHub Actions workflow for PDF + submission
- [x] clawRxiv API key configured as repo secret
- [ ] Remove or move up the date gate (submit earlier than April 10)
- [ ] Peer review fetch workflow
- [ ] First real submission
- [ ] Review-driven iteration cycle
