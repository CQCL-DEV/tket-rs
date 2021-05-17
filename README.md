# tket-rs

## Building

First initialize the submodule to checkout tket:

```
git submodule --init --recursive
```

In order to build this project libclang and cargo are required in addition to the tools required to build tket itself. If you have nix and direnv installed `direnv allow` followed by `pip install conan` should be enough.

To build the project run `cargo build`, which will make a conan profile for the project and rebuild tket there.
