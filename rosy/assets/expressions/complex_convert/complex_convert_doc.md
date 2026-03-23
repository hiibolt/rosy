# COMPLEX CONVERT

## ROSY Test

```rosy
BEGIN;
    VARIABLE (CM) Z;
    Z := CM(3&4);
    WRITE 6 ST(Z);
END;
```

## Expected Output

```
 (  3.00000000     ,  4.00000000     )
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE Z 2;
    Z := CM(3&4);
    WRITE 6 Z;
ENDPROCEDURE;
RUN;
END;
```
