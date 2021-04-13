# Rust examples for STM B-L475E-IOT01A

This is a collection of Rust examples for the [B-L475E-IOT01A](https://www.st.com/en/evaluation-tools/b-l475e-iot01a.html) (STM32L4 Discovery kit IoT node) board.

# Prerequisites

* [Rust](https://rustup.rs) 1.31 or newer.

* Compilation target for Cortex-M4F processor.

```rustup target add thumbv7em-none-eabihf```

* [probe-rs](https://probe.rs/) toolkit to interact with the board.

```cargo install cargo-embed```

# Getting started

To run specific example, run

```cargo embed --chip STM32L475VGTx --example blinky```

To just build an example locally, run

```cargo build --target thumbv7em-none-eabihf --example blinky```
