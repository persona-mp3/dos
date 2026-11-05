cargo build --target thumbv7em-none-eabihf

https://os.phil-opp.com/minimal-rust-kernel/
when building for custom target, rust expects the core and dependencies 
of the core, update the `./cargo/config.toml` with 

```toml
build-std = ["core", "compiler_builtins"]
```
[00:33] nixos::dos (main %) | rustup component add rust-src
[00:33] nixos::dos (main %) | cargo build --target x86_64-dos.json
