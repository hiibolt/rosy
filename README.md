# Rosy 🌹

A modern transpiler for the ROSY scientific programming language, designed for beam physics and differential algebra applications.

Rosy transpiles ROSY source code into self-contained, native Rust executables — optimized native code with zero runtime dependencies.

## Example

```
BEGIN;
    VARIABLE (RE) x;
    VARIABLE (RE) y;
    x := 3;
    y := SIN(x) + 1;
    WRITE 6 'y = ' ST(y);
END;
```

```
$ rosy run example.rosy
y =  1.141120008059867E-001
```

## Installation

### From source (recommended)

Requires the [Rust toolchain](https://rustup.rs/) (stable, edition 2024):

```bash
git clone https://github.com/hiibolt/rosy.git
cd rosy
cargo install --path rosy
```

To update:

```bash
git pull && cargo install --path rosy
```

### From GitHub Releases

Prebuilt binaries for Linux (x86_64) and macOS (x86_64, aarch64) are available on the [Releases page](https://github.com/hiibolt/rosy/releases/latest).

### Using Nix Flakes

```bash
nix develop
```

## Quick Start

```bash
rosy run examples/basic.rosy              # run directly
rosy build examples/basic.rosy -o out     # build a binary
rosy build examples/basic.rosy --release  # optimized build
```

## Language Documentation

The complete ROSY language reference — every operator, function, statement, and type — is in the **[Rustdoc documentation](https://hiibolt.github.io/cosy-rs/rosy/index.html)**.

To build locally:

```bash
cargo doc --document-private-items --no-deps -p rosy --open
```

## MPI Support (`PLOOP`)

Programs using `PLOOP` require an MPI implementation and LLVM/Clang at compile time (the transpiler itself does not):

- **Ubuntu/Debian**: `sudo apt install libopenmpi-dev openmpi-bin libclang-dev`
- **Fedora**: `sudo dnf install openmpi-devel clang-devel`
- **macOS**: `brew install open-mpi llvm`
- **NIU Metis**: `module load openmpi/openmpi-5.0.7-gcc-14.2.0-cuda-12.8`

## IDE Support

Copy the `rosy-vscode-extension/` folder to your VSCode extensions directory (`~/.vscode/extensions/` on Linux/macOS) and reload. To regenerate after grammar changes: `cargo run --bin generate_vscode_extension`.

## Differences from COSY INFINITY

- `PLOOP` does not revert to `LOOP` behavior when `NP == 1`
- `BREAK` statement for loop exit
- String literals use single quotes: `'hello'`

## License

See repository for license details.
