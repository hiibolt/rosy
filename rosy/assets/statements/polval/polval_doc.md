# POLVAL

## ROSY Test

```rosy
BEGIN;
    DAINI 3 2 0 0;
    VARIABLE (DA 2) P;
    VARIABLE (RE 100) A;
    VARIABLE (RE 100) R;
    P(1) := 1 + DA(1);
    P(2) := 1 + DA(2);
    A(1) := 2;
    A(2) := 3;
    POLVAL 1 P 2 A 2 R 2;
    WRITE 6 'polval ok';
END;
```

## Expected Output

```
polval ok
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE NM 1;
    VARIABLE P 4000;
    VARIABLE A 100;
    VARIABLE R 100;
    OV 3 2 0 NM;
    P(1) := 1 + DA(1);
    P(2) := 1 + DA(2);
    A(1) := 2;
    A(2) := 3;
    POLVAL 1 P 2 A 2 R 2;
    WRITE 6 'polval ok';
ENDPROCEDURE;
RUN;
END;
```
