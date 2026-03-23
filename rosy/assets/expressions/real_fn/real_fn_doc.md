# REAL FN

## ROSY Test

```rosy
BEGIN;
    VARIABLE (CM) Z;
    VARIABLE (RE) R;
    Z := CM(3&4);
    R := REAL(Z);
    WRITE 6 R;
END;
```

## Expected Output

```
 3.000000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE Z 2;
    VARIABLE R 1;
    Z := CM(3&4);
    R := REAL(Z);
    WRITE 6 R;
ENDPROCEDURE;
RUN;
END;
```
