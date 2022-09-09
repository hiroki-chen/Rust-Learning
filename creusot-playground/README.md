# How to `Creusot` to generate formal proofs for Rust Applications

1. You need to install `creusot` from source: https://github.com/xldenis/creusot
2. You need to install some SMT solver, e.g., Z3, CVC.
3. Build `creusot` from source and add it as a cargo component.
4. Run `cargo creusot -- --features creusot-contracts/contracts` in your own Rust project.
5. The generated MLCFG file will be located under `target/debug` directory.
6. Run `ide /target/debug/*.mlcfg` in the `creusot` directory.
7. Pick up any solver and check if the proof succeeds.