

<!-- fix(#18): add usage examples for the deposit api -->
<!-- fix(#1): attester authorization check missing on submit_interaction -->
# ⭐ Stellar Reputation Oracle

A decentralized reputation protocol on the Stellar network that builds verifiable, on-chain reputation scores from real economic activity.

## Overview

Stellar Reputation Oracle allows any protocol (escrow, lending, marketplace) to submit attested interactions (successful trade, loan repayment, verified delivery). These interactions accumulate into a **transparent, algorithmic reputation score** that other protocols can query — no centralized rating authority required.

## Why

Existing reputation systems (eBay, Upwork, Fiverr) are:
- **Platform-locked** — your reputation dies when you leave the platform
- **Centralized** — the platform can alter or suppress ratings
- **Non-portable** — no way to carry reputation across protocols

On Stellar with Soroban, reputation becomes:
- **Portable** — your on-chain identity carries its score everywhere
- **Verifiable** — every score component traces to a real on-chain interaction
- **Tamper-proof** — only authorized attesters can submit interactions, and scores are computed deterministically

## Architecture

### Soroban Contract (`contracts/reputation`)
| Function | Description |
|---|---|
| `initialize` | Set admin, authorized attesters list |
| `register_attester` | Admin adds a protocol that can submit interactions |
| `submit_interaction` | Authorized attester logs an interaction (address, type, outcome, weight) |
| `get_score` | Compute and return the reputation score for an address |
| `get_interactions` | List all interactions for an address |
| `get_attesters` | List all authorized attesters |

### Scoring Algorithm

Each interaction has a **type** (trade, loan, delivery) and **outcome** (positive/negative). The score is a weighted sum:

```
score = Σ(weight_i × outcome_i) / Σ(weight_i) × 100
```

A decay factor is applied so recent interactions carry more weight.

### Frontend (`frontend`)
React + Vite + Freighter for viewing scores, browsing attesters, and exploring interaction history.

## Flows

1. **Admin** calls `initialize(admin)` and `register_attester(attester_address, protocol_name)`
2. **Attester** (e.g., an escrow contract) calls `submit_interaction(user_address, "trade", "positive", weight)`
3. **Any protocol** calls `get_score(user_address)` to retrieve reputation
4. **User** can call `get_interactions(user_address)` to see their full history

## Build & Test

```bash
cd contracts/reputation && cargo test
cd ../../frontend && npm install && npm run dev
```

## License

MIT
