# ❒Fails when a function is defined but not used (mainly because it cannot figure out the types when

# optional types are not included).

# ❒Cannot create a DA variable that is a vector. Gives a type conflict when doing something like this:

## LOOP I 1 NP;

## IF I=1;

## COORD (1):=(X|I);

## ELSEIF I>1;

## COORD (1):= COORD (1)&(X|I);

## ENDIF;

## ENDLOOP;

# The error message:

```
Variable ’COORD ’ (in ’RUN ’) is assigned conflicting types:
First inferred as: (RE 1D)
Then assigned as: (VE)
```
# Adding DA when first assigned like this:

## LOOP I 1 NP;

## IF I=1;

## COORD (1):=DA(X|I);

## ELSEIF I>1;

## COORD (1):= COORD (1)&(X|I);

## ENDIF;

## ENDLOOP;

# results in this:

```
line 80, col 17: COORD (1):=COORD (1)&(X|I);: Cannot concatenate types ’(DA)’
and ’(RE)’ together!
```
# Adding DA to the next assignments:

## LOOP I 1 NP;

## IF I=1;

## COORD (1):=DA((X|I));

## ELSEIF I>1;

## COORD (1):= COORD (1)&DA(X|I);

## ENDIF;

## ENDLOOP;

# gives this error message:

```
line 80, col 17: COORD (1):=COORD (1)&DA(X|I);: Cannot concatenate types ’(DA
)’ and ’(DA)’ together!
```
# ❒Something might happend in version rosy 0.33.2 as the variables now are case sensitive! For example,

## VARIABLE P0 ;

## VARIABLE GAMMA0 ;

## VARIABLE V0 ;

## VARIABLE K0 ;

```
gamma0 :=2.065789024930937;
V0 :=0.8750256596748323;
P0:= gamma0*V0;
K0:=gamma0 -1;
```

# gives the following error message:

```
K0:=gamma0 -1;: while resolving variable ’K0’ (in RUN): Cannot evaluate
unknown expression recipe
(inferred from assignment)
```
# ❒In addition to the previous issue, it seems thatROSYdoes not give the error messages in the order

# they occur. In the previous issue,ROSYcomplained aboutK0while it was assigned a value afterP

# (both were not declared with the optional types). If the optional type is added toK0(without fixing

# the letters case):VARIABLE (RE)K0 ;, then we get:

```
P0:= gamma0*V0;: while resolving variable ’P0’ (in RUN): Cannot evaluate
unknown expression recipe
(inferred from assignment)
```
# ❒Following the previous two points, if the case of the declared and assigned variable is fixed to match,

# ROSYasks to add the types:

## VARIABLE K0 ;:

```
Type Resolution Failed
2 unresolved types found:
Could not determine the type of variable ’K0 ’ (in ’RUN ’)
It is declared but never assigned a value with a known type.
Declared at: line 31, col 9: VARIABLE K0 ;
Attempted: inferred from assignment
Try assigning it a value (e.g. K0 := 0;) or adding an explicit type.
Add an explicit type: VARIABLE (RE) K0 ;
```
```
Could not determine the type of variable ’P0 ’ (in ’RUN ’)
It is declared but never assigned a value with a known type.
Declared at: line 28, col 9: VARIABLE P0 ;
Attempted: inferred from assignment
Try assigning it a value (e.g. P0 := 0;) or adding an explicit type.
Add an explicit type: VARIABLE (RE) P0 ;
```
```
The type resolver builds a dependency graph and resolves
types from leaves inward. If a slot has no path to a
known type , or is part of a cycle , it cannot be resolved.
```
# ❒The following issue might be similar to the vectors case. Once the first array of a DA is assigned,

# ROSYconsiders it as a 1D array and fails to assign the second array. For example,

```
MAP1 (1):=DA(1); {x}
MAP1 (2):=DA(2)/P0; {a=px/P0}
```
# gives the error message:

## MAP1 (2):=DA(2)/P0;:

```
Type Conflict
Variable ’MAP1 ’ (in ’RUN ’) is assigned conflicting types:
First inferred as: (DA 1D)
Then assigned as: (DA)
First assigned at: line 95, col 9: MAP1 (1):=DA(1);
Then assigned at: line 96, col 9: MAP1 (2):=DA(2)/P0;
```
```
Type elision requires each variable to have exactly one type.
```

Either:
Add an explicit type: VARIABLE (DA) MAP1 ;
Split into separate variables: MAP1_DA and MAP1_DA

---

# Summary

## ~~Issue 1: Unused functions with untyped parameters fail to compile~~ (Fixed in v0.42.14)
**Severity: Medium**

Root cause: in the topological resolver loop (`resolve.rs:topological_resolve`), when a slot was defaulted to RE via the unused-variable fallback, the code `continue`d without decrementing dependent slots' in-degrees. So an untyped function-return variable depending on an unused argument stayed stuck. Fix: removed the early `continue`, letting the dependents block run after defaulting. Verified by `examples/tests/test_issue1_unused_function.rosy`.

## ~~Issue 2: Concat (`&`) doesn't support DA operands~~ (Fixed pre-v0.34.0)
**Severity: High — blocks DA vector construction**

`CONCAT_REGISTRY` in `rosy/src/rosy_lib/operators/concat.rs:33-48` now defines full DA × DA, DA × Vec<DA>, CD × CD, and CD × Vec<CD> rules. The original FEEDBACK reproducer (`COORD(1) := COORD(1) & Y`) was actually a correctly-rejected dimensional mismatch (LHS-indexed concat builds 2D-of-DA), not a missing operator rule. The intended pattern is `COORD := COORD & Y` (no LHS index → flat 1D-of-DA).

## ~~Issue 3: Case sensitivity~~ (Intentional — not a bug)

## ~~Issue 4: Error messages not sorted by source order~~ (Fixed in v0.42.15)
**Severity: Low**

`build_resolution_error` in `rosy/src/resolve.rs` now sorts both `cycle_slots` and `no_info_slots` by source location (`declared_at`, falling back to `assigned_at`, then slot-name for slots with no location). The shared `by_source` comparator + `order_key` extractor mean both partitions emit deterministically in the same line/column order. Verified by `examples/tests/test_issue4_error_order.rosy` (cycle case) — variable `A` (line 8) is now reliably reported before variable `B` (line 11) instead of in HashMap-traversal order.

## ~~Issue 5: Inference chain through untyped variables can fail~~ (Fixed)
**Severity: Medium**

Verified by `examples/tests/test_issue5_inference_chain.rosy` — the chain `GAMMA0 := 2.06; V0 := 0.87; P0 := GAMMA0*V0; K0 := GAMMA0 - 1;` now resolves cleanly with all four variables untyped. Likely fixed alongside the topological-resolver hardening.

## ~~Issue 6: Indexed array conflict check doesn't account for LHS dimensions~~ (Fixed)
**Severity: High — was blocking multi-element DA arrays**

The conflict check at `rosy/src/program/statements/core/assign/mod.rs:268-273` now wraps the new recipe with `WithDimensions(num_index_dimensions())` before comparing against the existing recipe's evaluated type. Verified by `examples/tests/test_issue6_da_array.rosy` — the FEEDBACK reproducer `MAP1(1) := DA(1); MAP1(2) := DA(2)/P0;` compiles and runs.
