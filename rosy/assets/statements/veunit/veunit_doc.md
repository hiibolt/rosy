# VEUNIT

## ROSY Test

```rosy
BEGIN;
    VARIABLE (VE) V;
    VARIABLE (VE) U;
    V := 3&4;
    VEUNIT V U;
    WRITE 6 ST(U);
END;
```

## Expected Output

```
 0.6000000      0.8000000
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE V 100;
    VARIABLE U 100;
    V := 3&4;
    VEUNIT V U;
    WRITE 6 U;
ENDPROCEDURE;
RUN;
END;
```
