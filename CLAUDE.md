# Rosy

Rosy transpiles ROSY source code (.rosy) into self-contained Rust executables for beam physics and differential algebra. It is a modern reimplementation of the COSY INFINITY language.

## Pipeline

Parse (pest PEG) -> AST (FromRule) -> Type Resolution (TypeResolver) -> Transpile (Transpile trait) -> Compile (cargo build)

## Build & Test

```bash
cargo build --release                              # Build transpiler
cargo test                                         # Run unit tests
cargo run --bin rosy -- run examples/basic.rosy     # Run a ROSY script
cargo run --bin rosy -- build examples/basic.rosy   # Build standalone binary
```

## Project Layout

```
rosy/assets/rosy.pest              PEG grammar (syntax source of truth)
rosy/src/ast.rs                    FromRule trait, PrattParser, CosyParser
rosy/src/program/expressions/      Expression AST nodes
  operators/                         Binary/unary: add.rs, sub.rs, mult.rs, ...
  functions/math/                    Intrinsics: sqr.rs, sin.rs, exp.rs, ...
  functions/conversion/              Type conversions: CM(), ST(), LO()
  types/                             Literals: number.rs, string.rs, da.rs, ...
  core/                              Variable references
rosy/src/program/statements/        Statement AST nodes
  core/                              var_decl, assign, if, loop, function, procedure, ...
  io/                                write, read, openf, closef, ...
  da/                                da_init, daprv, darev
  math/                              fit
rosy/src/rosy_lib/operators/        Runtime operator traits + TypeRule registries
rosy/src/rosy_lib/intrinsics/       Runtime intrinsic traits + IntrinsicTypeRule registries
rosy/src/transpile.rs               Transpile trait, TranspilationInputContext
rosy/src/resolve.rs                 Type resolution pass (dependency graph)
rosy/build.rs                       Build-time codegen from registries
rosy/codegen.rs                     Test file generation (.rosy + .fox)
rosy/assets/output_template/        Template for generated Rust projects
examples/                           Integration test .rosy programs
cosy_manual/                       COSY INFINITY 10 manual (split by chapter)
  A2_operators.md                    Operator type compatibility tables
  A3_intrinsic_functions.md          Intrinsic function type tables
  A4_intrinsic_procedures.md         Intrinsic procedure signatures
  03_cosyscript.md                   Language syntax, flow control, I/O
  02_types.md                        Type system overview
```

## Type System

RE (f64), ST (String), LO (bool), CM (Complex64), VE (Vec<f64>), DA (Taylor series), CD (Complex DA)

Multi-dimensional arrays: `(RE ** 2)` = `Vec<Vec<f64>>`

## Key Traits

- **FromRule** -- parse pest pair -> AST node
- **Transpile** -- AST node -> Rust code string (returns `Result<TranspilationOutput, Vec<Error>>`)
- **TranspileableExpr** -- `type_of()`, `build_expr_recipe()`, `discover_expr_function_calls()`
- **TranspileableStatement** -- `register_declaration()`, `discover_dependencies()`, `apply_resolved_types()`

## TDD Rule

Every operator/intrinsic must pass COSY/ROSY output diffing. TypeRule registries auto-generate `.rosy` and `.fox` test files via `codegen.rs` at build time. Run both, diff outputs -- must match exactly.

## Versioning

Uses [Semantic Versioning](https://semver.org/). The version is in `rosy/Cargo.toml`.

- **Every commit to `master`** must bump the version in `rosy/Cargo.toml`
  - Patch bump (0.1.0 -> 0.1.1): bug fixes, small improvements
  - Minor bump (0.1.0 -> 0.2.0): new language constructs, features, or breaking changes
  - Major bump: reserved for 1.0 stable release
- **To create a release**: push a tag matching `v*.*.*` (e.g. `git tag v0.2.0 && git push --tags`)
- The GitHub Actions release workflow (`.github/workflows/release.yml`) automatically builds binaries and creates a GitHub Release on tag push

## Conventions

- Worktrees: `../rosy-work/<item-name>`
- Work output: `work/manifest.json` -- `{ item, status, worktree, result_path }`
- Agent models: Orchestrator -> Opus 4.6, Implementers/Reviewer/Tester/Migrator/NewUser -> Sonnet 4.6, DevilsAdvocate/Synthesizer -> Opus 4.6
- Personality and detailed workflows: see `.github/copilot-instructions.md`
