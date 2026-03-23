# CONCAT

## ROSY Test

```rosy
BEGIN;
    VARIABLE (VE) V;
    V := 1 & 2 & 3;
    WRITE 6 ST(V);
END;
```

## Expected Output

```
  1.000000       2.000000       3.000000
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE V 100;
    V := 1 & 2 & 3;
    WRITE 6 V;
ENDPROCEDURE;
RUN;
END;
```
