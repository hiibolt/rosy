# MBLOCK

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE 10 10) M;
    VARIABLE (RE 10 10) T;
    VARIABLE (RE 10 10) TI;
    M(1)(1) := 2;
    M(1)(2) := 1;
    M(2)(1) := 1;
    M(2)(2) := 2;
    MBLOCK M T TI 10 2;
    WRITE 6 'mblock ok';
END;
```

## Expected Output

```
mblock ok
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE M 100;
    VARIABLE T 100;
    VARIABLE TI 100;
    M(1)(1) := 2;
    M(1)(2) := 1;
    M(2)(1) := 1;
    M(2)(2) := 2;
    MBLOCK M T TI 10 2;
    WRITE 6 'mblock ok';
ENDPROCEDURE;
RUN;
END;
```
