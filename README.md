# Quantum Shield – Foundation Phase (Step 1)

Welcome to the first phase of the **Quantum Shield** project, a Layer 1 network resistant to quantum computing.

---

## The Goal of This Stage

Preparing the core engine of the blockchain, which includes:

- Building the block structure ("Block Structure")
- Connecting the blocks sequentially to form the chain
- Signing each block using a post-quantum signature (PQC - experimental)
- Generating a hash for each block using SHA256

> ⚠️ **Note:**  
> SHA256 is currently used for prototyping due to its simplicity and widespread support in existing tooling.  
> However, it is not considered quantum-resistant.  
> In future production stages, the project plans to explore and integrate PQ-hash algorithms such as **SPHINCS+** or **XMSS** to enhance quantum security.

---

## Requirements

Make sure you have the following tools:

- [Rust](https://www.rust-lang.org/tools/install) (we recommend installing via `rustup`)
- Required libraries:
  - `sha2` (for hashing)
  - `chrono` (for timing)
  - `serde` and `serde_json` (for serialization and deserialization)

Install libraries using:

```bash
cargo add sha2 chrono serde serde_json
