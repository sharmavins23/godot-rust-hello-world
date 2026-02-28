# Godot-Rust - Hello World

This project is a quick (and super simple!) Hello World project in Godot, using
Rust bindings for a scripting language. It's following
[this tutorial](https://godot-rust.github.io/book/intro/hello-world.html#creating-a-rust-class),
and performing some light refactoring atop it.

## Development

As this is a Rust project, running `cargo build` in the `/rust` folder will
re-build the project. This also includes generating the C-based dynamically
linked libraries (DLLs) that Godot looks for when loading in the extensions.

Unfortunately, because these DLL files control Godot's interaction with Rust,
`bacon` is not able to consistently re-build (as `bacon` only runs `cargo check`
and not the final build step). Instead, you can run `watchexec` as follows:

```zsh
cargo install watchexec-cli
watchexec -c -w src -e rs "cargo build"
```
