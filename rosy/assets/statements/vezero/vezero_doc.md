# VEZERO

## ROSY Test

```rosy
BEGIN;
    VARIABLE (VE) V;
    V := 0.001 & 5 & 0.0001;
    VEZERO V 3 0.01;
    WRITE 6 ST(V);
END;
```

## Expected Output

```
 0.0000000       0.000000      0.0000000
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE V 100;
    V := 0.001 & 5 & 0.0001;
    VEZERO V 3 0.01;
    WRITE 6 V;
ENDPROCEDURE;
RUN;
END;
```
