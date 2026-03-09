<!-- cargo-rdme start -->

# Simulator

Run mousefood apps on your computer inside a simulator! Uses [embedded-graphics-simulator](https://crates.io/crates/embedded-graphics-simulator).

## Requirements

This app requires [SDL2](https://wiki.libsdl.org/SDL2/Installation) to be installed.

If you use [nix](https://nixos.org) you can run `nix-shell -p SDL2`
before running the application.

## Run

To start the minimal demo, simply run:

```shell
cargo run -p simulator
```

For the modifiers demo, run:

```shell
cargo run -p simulator --bin modifiers
```

A window will open with the simulator running.

<!-- cargo-rdme end -->
