# CONJ

## ROSY Test

```rosy
BEGIN;
    VARIABLE (CM) Z;
    VARIABLE (CM) C;
    Z := CM(3&4);
    C := CONJ(Z);
    WRITE 6 ST(C);
END;
```

## Expected Output

```
 (  3.00000000     , -4.00000000     )
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE Z 2;
    VARIABLE C 2;
    Z := CM(3&4);
    C := CONJ(Z);
    WRITE 6 C;
ENDPROCEDURE;
RUN;
END;
```
