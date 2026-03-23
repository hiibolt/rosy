# DATRN

## ROSY Test

```rosy
BEGIN;
    DAINI 3 2 0 0;
    VARIABLE (DA 1) INPUT;
    VARIABLE (DA 1) OUTPUT;
    VARIABLE (VE) SCALES;
    VARIABLE (VE) SHIFTS;
    INPUT(1) := DA(1);
    SCALES := 2.0 & 1.0;
    SHIFTS := 1.0 & 0.0;
    DATRN INPUT SCALES SHIFTS 1 1 OUTPUT;
    DAPRV OUTPUT 1 2 2 6;
END;
```

## Expected Output

```
  I  COEFFICIENT                1             ORDER EXPONENTS
  1   1.000000000000000         0    0 0
  2   2.000000000000000         1    1 0
------------------------------------------------------
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE NM 1;
    VARIABLE INPUT 2000;
    VARIABLE OUTPUT 2000;
    VARIABLE SCALES 100;
    VARIABLE SHIFTS 100;
    OV 3 2 0 NM;
    INPUT(1) := DA(1);
    SCALES := 2.0 & 1.0;
    SHIFTS := 1.0 & 0.0;
    DATRN INPUT SCALES SHIFTS 1 1 OUTPUT;
    DAPRV OUTPUT 1 2 2 6;
ENDPROCEDURE;
RUN;
END;
```
