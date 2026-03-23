# DAPRV

## ROSY Test

```rosy
BEGIN;
    DAINI 3 2 0 0;
    VARIABLE (DA 1) A;
    A(1) := DA(1) + 2;
    DAPRV A 1 2 2 6;
END;
```

## Expected Output

```
  I  COEFFICIENT                1             ORDER EXPONENTS
  1   2.000000000000000         0    0 0
  2   1.000000000000000         1    1 0
------------------------------------------------------
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE NM 1;
    VARIABLE A 2000;
    OV 3 2 0 NM;
    A(1) := DA(1) + 2;
    DAPRV A 1 2 2 6;
ENDPROCEDURE;
RUN;
END;
```
