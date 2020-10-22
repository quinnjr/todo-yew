# Todo-Yew

A simple experiment in using Rust to build a WASM TodoMVC application using the [Yew](https://crates.io/crates/yew) framework.

Adapted from the [todomvc](https://github.com/yewstack/yew/blob/master/examples/todomvc) example provided in the Yew repository with styles provided instead by [Bulma](https://bulma.io/).

## Running

Running this example is best done by using [trunk](https://github.com/thedodd/trunk)

```sh
rustup target add wasm32-unknown-unknown --toolchain nightly

cargo install trunk wasm-bindgen-cli

trunk serve --release
```
