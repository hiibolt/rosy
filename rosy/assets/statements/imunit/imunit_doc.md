# IMUNIT

## ROSY Test

```rosy
BEGIN;
    VARIABLE (CM) Z;
    IMUNIT Z;
    WRITE 6 ST(Z);
END;
```

## Expected Output

```
 (  0.00000000     ,  1.00000000     )
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE Z 2;
    IMUNIT Z;
    WRITE 6 Z;
ENDPROCEDURE;
RUN;
END;
```
