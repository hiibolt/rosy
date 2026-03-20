# Changelog

All notable changes to Rosy are documented here. Versions follow [Semantic Versioning](https://semver.org/).

## [0.3.2] - 2026-03-20

### Fixed
- PLOOP codegen: MPI `get_group_num`/`coordinate` calls now correctly pass a mutable value instead of a shared reference
- Update notice now appears between transpilation and program output instead of after

### Added
- Clickable changelog link (OSC 8 hyperlink) in the update notice for supported terminals
- Update notice download URL now points to the specific version release

## [0.3.0] - 2026-03-20

### Changed
- **Docs overhaul**: all module docs rewritten with user-intent "Looking for something?" navigation tables
- **Module reorganization**: operators grouped into `arithmetic/`, `comparison/`, `unary/`, `collection/`; math functions grouped into `trig/`, `exponential/`, `complex/`, `rounding/`, `vector/`, `query/`, `memory/`
- README slimmed to showcase + install, pointing users to the full Rustdoc reference

## [0.2.0] - 2026-03-20

### Added
- CI workflow (`cargo check` + tests on every push/PR)
- Version-gated docs and release workflows (trigger on `Cargo.toml` version changes, not tags)
- Auto-injected version in rustdoc header via `env!("CARGO_PKG_VERSION")`
- Semver versioning with `semver` crate
- Background update check against GitHub releases (warns if a newer version exists)
- GitHub Actions release workflow (builds Linux x86_64, macOS x86_64, macOS aarch64)
- Colored step-by-step transpiler progress output (`[1/5] Parsing...`, etc.)
- Real-time cargo build output piping (no more silent compilation)
- `rosy --version` flag
- Install docs: `cargo install --path rosy`, GitHub Releases, NIU Metis MPI instructions

## [0.1.0] - 2026-03-20

Initial release with core language support.

### Language Features
- 7 base types: `RE`, `ST`, `LO`, `CM`, `VE`, `DA`, `CD`
- Multi-dimensional arrays
- 13 binary operators with full type compatibility
- 40+ intrinsic functions (trig, exponential, complex, rounding, vector, query, conversion, string)
- 13 intrinsic procedures (SCRLEN, CPUSEC, QUIT, OS, DA operations, linear algebra, etc.)
- Control flow: `IF`/`ELSEIF`/`ELSE`, `LOOP`, `WHILE`, `PLOOP` (MPI), `BREAK`
- `FUNCTION` and `PROCEDURE` definitions with recursion
- File I/O: `WRITE`, `READ`, `OPENF`, `CLOSEF`, binary I/O
- `FIT` optimization loop
- Differential Algebra: `OV`, `DA(n)`, `CD(n)`, `DAPRV`, `DAREV`
- Optional MPI support via `PLOOP`
