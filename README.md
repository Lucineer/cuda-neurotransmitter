# cuda-neurotransmitter

Signal-to-gene activation pathways — neurotransmitter-inspired agent signaling with receptor binding and cascading responses (Rust)

Part of the Cocapn biology layer — bio-inspired agent lifecycle and energy.

## What It Does

### Key Types

- `NeuroSignal` — core data structure
- `Receptor` — core data structure
- `Synapse` — core data structure
- `SynapticCleft` — core data structure
- `NeuroEffect` — core data structure
- `Cascade` — core data structure
- _and 1 more (see source)_

## Quick Start

```bash
# Clone
git clone https://github.com/Lucineer/cuda-neurotransmitter.git
cd cuda-neurotransmitter

# Build
cargo build

# Run tests
cargo test
```

## Usage

```rust
use cuda_neurotransmitter::*;

// See src/lib.rs for full API
// 15 unit tests included
```

### Available Implementations

- `NeuroType` — see source for methods
- `NeuroSignal` — see source for methods
- `Receptor` — see source for methods
- `Synapse` — see source for methods
- `SynapticCleft` — see source for methods
- `Cascade` — see source for methods

## Testing

```bash
cargo test
```

15 unit tests covering core functionality.

## Architecture

This crate is part of the **Cocapn Fleet** — a git-native multi-agent ecosystem.

- **Category**: biology
- **Language**: Rust
- **Dependencies**: See `Cargo.toml`
- **Status**: Active development

## Related Crates

- [cuda-energy](https://github.com/Lucineer/cuda-energy)
- [cuda-biology](https://github.com/Lucineer/cuda-biology)
- [cuda-genepool](https://github.com/Lucineer/cuda-genepool)
- [cuda-dna](https://github.com/Lucineer/cuda-dna)

## Fleet Position

```
Casey (Captain)
├── JetsonClaw1 (Lucineer realm — hardware, low-level systems, fleet infrastructure)
├── Oracle1 (SuperInstance — lighthouse, architecture, consensus)
└── Babel (SuperInstance — multilingual scout)
```

## Contributing

This is a fleet vessel component. Fork it, improve it, push a bottle to `message-in-a-bottle/for-jetsonclaw1/`.

## License

MIT

---

*Built by JetsonClaw1 — part of the Cocapn fleet*
*See [cocapn-fleet-readme](https://github.com/Lucineer/cocapn-fleet-readme) for the full fleet roadmap*

---

## Fleet Context

Part of the Lucineer/Cocapn fleet. See [fleet-onboarding](https://github.com/Lucineer/fleet-onboarding) for boarding protocol.

- **Vessel:** JetsonClaw1 (Jetson Orin Nano 8GB)
- **Domain:** Low-level systems, CUDA, edge computing
- **Comms:** Bottles via Forgemaster/Oracle1, Matrix #fleet-ops
