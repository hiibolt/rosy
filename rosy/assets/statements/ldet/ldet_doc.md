# LDET

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE 10 10) M;
    VARIABLE (RE) D;
    M(1)(1) := 1;
    M(1)(2) := 2;
    M(2)(1) := 3;
    M(2)(2) := 4;
    LDET M 2 10 D;
    WRITE 6 D;
END;
```

## Expected Output

```
-2.000000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE M 100;
    VARIABLE D 1;
    M(1)(1) := 1;
    M(1)(2) := 2;
    M(2)(1) := 3;
    M(2)(2) := 4;
    LDET M 2 10 D;
    WRITE 6 D;
ENDPROCEDURE;
RUN;
END;
```
