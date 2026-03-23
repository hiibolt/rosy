# LINV

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE 10 10) M;
    VARIABLE (RE 10 10) INV;
    VARIABLE (RE) ERR;
    M(1)(1) := 1;
    M(1)(2) := 2;
    M(2)(1) := 3;
    M(2)(2) := 4;
    LINV M INV 2 10 ERR;
    WRITE 6 ERR;
END;
```

## Expected Output

```
 0.000000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE M 100;
    VARIABLE INV 100;
    VARIABLE ERR 1;
    M(1)(1) := 1;
    M(1)(2) := 2;
    M(2)(1) := 3;
    M(2)(2) := 4;
    LINV M INV 2 10 ERR;
    WRITE 6 ERR;
ENDPROCEDURE;
RUN;
END;
```
