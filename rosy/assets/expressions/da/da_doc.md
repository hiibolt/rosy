# DA

## ROSY Test

```rosy
BEGIN;
    DAINI 3 2 0 0;
    VARIABLE (DA) X;
    X := DA(1);
    WRITE 6 ST(CONS(X));
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
    VARIABLE X 2000;
    OV 3 2 0 NM;
    X := DA(1);
    WRITE 6 CONS(X);
ENDPROCEDURE;
RUN;
END;
```
