---
name: rosy-idioms
description: Exact file-by-file recipes for adding operators, intrinsic functions, and statements to the Rosy transpiler. Use when implementing new ROSY language constructs.
user-invocable: false
---

# Rosy Codebase Idioms

## Adding a Binary Operator (e.g., Add)

**8 files to touch, in order:**

### 1. Grammar: `rosy/assets/rosy.pest`
Add infix rule and wire into `infix_op`:
```pest
add = { "+" }
infix_op = _{ add | sub | mult | ... }
```

### 2. AST struct: `rosy/src/program/expressions/operators/<name>.rs`
```rust
#[derive(Debug, PartialEq)]
pub struct AddExpr {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}
```
Implement `FromRule` (not needed for infix ops -- Pratt parser handles construction).

### 3. ExprEnum variant: `rosy/src/program/expressions/mod.rs`
- Add `use crate::program::expressions::operators::<name>::<Name>Expr;`
- Add variant to `ExprEnum`: `<Name>,`

### 4. Pratt parser wiring: `rosy/src/program/expressions/mod.rs` in `map_infix`
```rust
Rule::<name> => {
    let left = left.context("...while transpiling left-hand side of `<name>` expression")?;
    let right = right.context("...while transpiling right-hand side of `<name>` expression")?;
    Ok(Expr {
        enum_variant: ExprEnum::<Name>,
        inner: Box::new(<Name>Expr { left: Box::new(left), right: Box::new(right) })
    })
},
```

### 5. TypeRule registry: `rosy/src/rosy_lib/operators/<name>.rs`
```rust
pub const <NAME>_REGISTRY: &[TypeRule] = &[
    TypeRule::new("RE", "RE", "RE", "-2", "1"),
    TypeRule::with_comment("RE", "VE", "VE", "1", "1&2", "Add Real componentwise"),
    // ... all type combinations from manual.md Appendix A
];

pub fn get_return_type(lhs: &RosyType, rhs: &RosyType) -> Option<RosyType> {
    let registry = build_type_registry(<NAME>_REGISTRY);
    registry.get(&(*lhs, *rhs)).copied()
}
```

### 6. Runtime trait: same file
```rust
pub trait Rosy<Name><Rhs = Self> {
    type Output;
    fn rosy_<name>(self, rhs: Rhs) -> Result<Self::Output>;
}
impl Rosy<Name><&RE> for &RE {
    type Output = RE;
    fn rosy_<name>(self, rhs: &RE) -> Result<Self::Output> { Ok(self + rhs) }
}
```

### 7. Module declarations: `rosy/src/rosy_lib/operators/mod.rs`
- Add `pub mod <name>;`
- Add `pub use <name>::Rosy<Name>;`

### 8. Build codegen: `rosy/build.rs`
Add `codegen::codegen_operator("<name>");` -- generates test .rosy and .fox files.

### 9. Transpile impl: in the AST struct file
```rust
impl Transpile for <Name>Expr {
    fn transpile(&self, context: &mut TranspilationInputContext) -> Result<TranspilationOutput, Vec<Error>> {
        let left_output = self.left.transpile(context)?;
        let right_output = self.right.transpile(context)?;
        let mut requested_variables = BTreeSet::new();
        requested_variables.extend(left_output.requested_variables);
        requested_variables.extend(right_output.requested_variables);
        Ok(TranspilationOutput {
            serialization: format!("&mut Rosy<Name>::rosy_<name>(&*{}, &*{})?",
                left_output.serialization, right_output.serialization),
            requested_variables,
        })
    }
}
impl TranspileableExpr for <Name>Expr {
    fn type_of(&self, context: &TranspilationInputContext) -> Result<RosyType> {
        let left_type = self.left.type_of(context)?;
        let right_type = self.right.type_of(context)?;
        <name>::get_return_type(&left_type, &right_type)
            .ok_or_else(|| anyhow!("Cannot <name> types '{}' and '{}'!", left_type, right_type))
    }
}
```

---

## Adding an Intrinsic Function (e.g., Sin)

**7 files to touch:**

### 1. Grammar: `rosy/assets/rosy.pest`
```pest
sin = { ^"SIN" ~ "(" ~ expr ~ ")" }
builtin_function = _{ sin | sqr | exp | ... }
```

### 2. AST struct: `rosy/src/program/expressions/functions/<category>/<name>.rs`
```rust
#[derive(Debug, PartialEq)]
pub struct SinExpr {
    pub expr: Box<Expr>,
}
impl FromRule for SinExpr {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        ensure!(pair.as_rule() == Rule::sin, "Expected sin rule");
        let mut inner = pair.into_inner();
        let expr = Box::new(Expr::from_rule(inner.next().context("Missing expr")?)?.unwrap());
        Ok(Some(SinExpr { expr }))
    }
}
```

### 3. Module declaration: category `mod.rs`
`pub mod <name>;` in e.g. `functions/math/mod.rs` or `functions/math/trig/mod.rs`

### 4. ExprEnum + map_primary: `rosy/src/program/expressions/mod.rs`
```rust
Rule::sin => {
    let sin_expr = SinExpr::from_rule(primary)?;
    Ok(Expr { enum_variant: ExprEnum::Sin, inner: Box::new(sin_expr.unwrap()) })
},
```

### 5. IntrinsicTypeRule registry: `rosy/src/rosy_lib/intrinsics/<name>.rs`
```rust
pub const SIN_REGISTRY: &[IntrinsicTypeRule] = &[
    IntrinsicTypeRule::new("RE", "RE", "1.5"),
    IntrinsicTypeRule::new("CM", "CM", "CM(1.5&2.5)"),
    IntrinsicTypeRule::new("VE", "VE", "1.5&2.5&3.5"),
    IntrinsicTypeRule::new("DA", "DA", "DA(1)"),
];
pub trait RosySIN { type Output; fn rosy_sin(&self) -> Result<Self::Output>; }
impl RosySIN for RE { type Output = RE; fn rosy_sin(&self) -> Result<RE> { Ok(self.sin()) } }
```

### 6. Module declarations: `rosy/src/rosy_lib/intrinsics/mod.rs`
- `pub mod <name>;`
- `pub use <name>::Rosy<NAME>;`

### 7. Build codegen: `rosy/build.rs`
`codegen::codegen_intrinsic("<name>");`

---

## Adding a Statement (e.g., Break)

**5 files to touch:**

### 1. Grammar: `rosy/assets/rosy.pest`
```pest
break_statement = { ^"BREAK" }
statement = _{ ... | break_statement | ... }
```
Add to `keyword` rule if the name could collide with identifiers.

### 2. AST struct: `rosy/src/program/statements/<category>/<name>.rs`
```rust
#[derive(Debug)]
pub struct BreakStatement;

impl FromRule for BreakStatement {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        ensure!(pair.as_rule() == Rule::break_statement, "Expected break_statement rule");
        Ok(Some(BreakStatement))
    }
}
impl TranspileableStatement for BreakStatement {}  // empty if no type inference needed
impl Transpile for BreakStatement {
    fn transpile(&self, context: &mut TranspilationInputContext) -> Result<TranspilationOutput, Vec<Error>> {
        Ok(TranspilationOutput { serialization: "break;".to_string(), requested_variables: BTreeSet::new() })
    }
}
```

### 3. Module declaration: `rosy/src/program/statements/<category>/mod.rs`
`pub mod <name>;`

### 4. StatementEnum + from_rule: `rosy/src/program/statements/mod.rs`
- Add `pub use <category>::<name>::<Name>Statement;`
- Add `<Name>,` to `StatementEnum`
- Add match arm in `Statement::from_rule`:
```rust
Rule::<name> => <Name>Statement::from_rule(pair)
    .context("...while building <name> statement!")
    .map(|opt| opt.map(|stmt| Statement {
        enum_variant: StatementEnum::<Name>,
        inner: Box::new(stmt),
        source_location: loc.clone(),
    })),
```

### 5. Integration test: `examples/test_<name>.rosy`

---

## Error Accumulation Pattern

```rust
let mut errors = Vec::new();
match child.transpile(context) {
    Ok(output) => { /* use output */ },
    Err(mut e) => { for err in e.drain(..) { errors.push(err.context("...")); } }
}
if errors.is_empty() { Ok(output) } else { Err(errors) }
```

## Codegen Trigger

`cargo build` runs `build.rs` which calls `codegen::codegen_operator()` / `codegen::codegen_intrinsic()`.
This parses `*_REGISTRY` constants from source files and generates:
- `rosy/assets/operators/<name>/<name>.rosy` -- ROSY test script
- `rosy/assets/operators/<name>/<name>.fox` -- COSY equivalent
- `rosy/assets/operators/<name>/<name>_table.md` -- Documentation table
