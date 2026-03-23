# VE CONVERT

## ROSY Test

```rosy
BEGIN;
    VARIABLE (VE) V;
    V := VE(CM(3&4));
    WRITE 6 ST(V);
END;
```

## Expected Output

```
  3.000000       4.000000
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE V 100;
    V := VE(CM(3&4));
    WRITE 6 V;
ENDPROCEDURE;
RUN;
END;
```
