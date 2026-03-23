# CD

## ROSY Test

```rosy
BEGIN;
    DAINI 3 2 0 0;
    VARIABLE (CD) Z;
    Z := CD(1);
    WRITE 6 ST(CONS(REAL(Z)));
END;
```

## Expected Output

```
 0.000000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE NM 1;
    VARIABLE Z 4000;
    OV 3 2 0 NM;
    Z := CD(1);
    WRITE 6 CONS(REAL(Z));
ENDPROCEDURE;
RUN;
END;
```
