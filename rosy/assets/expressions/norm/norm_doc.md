# NORM

## ROSY Test

```rosy
BEGIN;
    VARIABLE (VE) V;
    VARIABLE (VE) N;
    V := 1.5 & -2.5 & 3.5;
    N := NORM(V);
    WRITE 6 ST(N);
END;
```

## Expected Output

```
  1.500000       2.500000       3.500000
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE V 100;
    VARIABLE N 100;
    V := 1.5 & -2.5 & 3.5;
    N := NORM(V);
    WRITE 6 N;
ENDPROCEDURE;
RUN;
END;
```
