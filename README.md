# gd_bind_sync

A Rust workspace that generates Godot `Resource` wrappers for your data types.

This project defines derive macros and traits for converting between pure Rust
structs/enums and Godot's runtime types. The included example demonstrates how
these macros can be used inside a small Godot extension library.

## Workspace Layout

- **`as_gd_res`** – Library providing the `AsGdRes`, `AsSimpleGdEnum` and
  `ExtractGd` traits, along with helpers for common engine types.
- **`as_gd_res_derive`** – Procedural macro for `#[derive(AsGdRes)]`.
- **`as_simple_gd_enum_derive`** – Procedural macro for `#[derive(AsSimpleGdEnum)]`.
- **`resource_test_rust`** – Example crate compiled as a `cdylib` to test the
  derives from Godot. It is paired with the `resource_test_godot_project`
  directory which contains a minimal Godot project.

Every crate that defines macros has a `rust-toolchain.toml` pointing to the
`nightly` toolchain.

## Building

1. Ensure you have a nightly Rust toolchain installed (`rustup toolchain install nightly`).
2. The `godot` dependency uses a local path in `Cargo.toml`:

   ```toml
   godot = { path = "/data/code_projects/rust/gdext/godot", features = ["register-docs", "experimental-threads"] }
   ```

   Update this path to your own `godot-rust` checkout if necessary.
3. Build all crates with Cargo:

   ```bash
   cargo build --workspace --all-targets
   ```

4. To run the example Godot project, open the `resource_test_godot_project`
   folder with the Godot editor and enable the compiled extension library.

## Usage Overview

Implement the traits or use the derive macros to bridge between Rust and Godot
resources. For example:

```rust
#[derive(as_gd_res::AsGdRes, Debug, Clone)]
struct MyData {
    pub name: String,
    pub value: i32,
}

#[derive(as_gd_res::AsSimpleGdEnum, Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Element {
    #[default]
    Fire,
    Water,
}
```

The macros generate `Resource` structs compatible with Godot and implement
`ExtractGd` so you can convert the generated resources back into your original
Rust types.

## License

This project is licensed under the MIT License.
