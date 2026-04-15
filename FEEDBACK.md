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

## Issue 1: Unused functions with untyped parameters fail to compile
**Severity: Medium**

The type resolver infers function parameter types from call sites. If a function is never called, untyped parameters have no inference source and remain unresolved. Options: require explicit types on function parameters (matching COSY behavior), or default unused untyped parameters to RE.

## Issue 2: Concat (`&`) doesn't support DA operands
**Severity: High — blocks DA vector construction**

The `CONCAT_REGISTRY` in `rosy/src/rosy_lib/operators/concat.rs` only defines rules for RE, VE, and ST combinations. DA is entirely absent. In COSY, DA arrays (vectors of Taylor series) are common for phase-space maps. New `TypeRule` entries and trait impls need to be added for DA & DA, DA & VE-of-DA, etc. No upcasting is involved — this is strictly about adding missing type rules.

## ~~Issue 3: Case sensitivity~~ (Intentional — not a bug)

## Issue 4: Error messages not sorted by source order
**Severity: Low**

The resolver uses topological sort (Kahn's algorithm), so errors surface in dependency-graph order, not source order. Sorting errors by source location before display would improve readability.

## Issue 5: Inference chain through untyped variables can fail
**Severity: Medium — needs investigation**

When multiple untyped variables form a chain (`gamma0 := 2.06; P0 := gamma0 * V0; K0 := gamma0 - 1;`), the resolver may fail to propagate types through the chain even though the root (`gamma0`) has a literal assignment. Could be a dependency-graph edge issue or a topological-sort ordering problem.

## Issue 6: Indexed array conflict check doesn't account for LHS dimensions
**Severity: High — blocks multi-element DA arrays**

When checking for conflicting types on a second indexed assignment (e.g. `MAP1(2) := DA(2)/P0` after `MAP1(1) := DA(1)`), the conflict check at `assign/mod.rs:304` evaluates the new recipe *without* the `WithDimensions` wrapper that accounts for LHS indexing. The first assignment correctly infers `(DA 1D)` via `WithDimensions` at line 425, but the conflict check compares against the raw RHS type `(DA)`, causing a false mismatch. The fix: wrap the new recipe with `WithDimensions` (using `num_index_dimensions()`) before comparing.
