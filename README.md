# cuda-neurotransmitter

Signal-to-gene activation pathways inspired by neurotransmitter systems. Maps chemical signaling (dopamine, serotonin, etc.) to agent communication with receptor binding, cascading responses, and synaptic decay.

## The Key Insight

> Dopamine IS the confidence signal. Serotonin IS the trust signal. These aren't metaphors — they're the same mathematical structure (exponential decay, accumulation, threshold gating).

## Neurotransmitter Types

| Neurotransmitter | Effect | Maps To |
|-----------------|--------|---------|
| Dopamine | Reward/prediction error | Confidence boost, learning drive |
| Serotonin | Calm/contentment | Trust building, risk reduction |
| Norepinephrine | Alert/stress | Focus increase, exploration decrease |
| Acetylcholine | Learning/plasticity | Gene expression rate increase |
| Endorphin | Pleasure/pain | Reinforcement signal for gene fitness |
| Oxytocin | Bonding/attachment | Trust in specific agents |
| Melatonin | Sleep/repair | Rest instinct, memory consolidation |
| Anandamide | Novelty/surprise | Expectation reset, exploration drive |

## Architecture

```text
Signal → Receptor Binding → Cascade Activation → Gene Expression → Behavior Change
  ↓          ↓                    ↓                    ↓
NeuroType  Pattern Match    Threshold Gate     Protein (bytecode)
```

## Quick Start

```bash
git clone https://github.com/Lucineer/cuda-neurotransmitter.git
cd cuda-neurotransmitter
cargo test    # 15 tests
```

## Key Types

- **`NeuroType`** — 8 neurotransmitter types with distinct behavioral effects
- **`Receptor`** — Signal pattern matcher with binding affinity and threshold
- **`Synapse`** — Agent-to-agent connection with strength, decay, and firing history
- **`Cascade`** — One signal triggers multiple downstream gene activations

---

## Fleet Context

Part of the Lucineer/Cocapn fleet. See [fleet-onboarding](https://github.com/Lucineer/fleet-onboarding) for boarding protocol.

- **Vessel:** JetsonClaw1 (Jetson Orin Nano 8GB)
- **Domain:** Low-level systems, CUDA, edge computing
- **Comms:** Bottles via Forgemaster/Oracle1, Matrix #fleet-ops
