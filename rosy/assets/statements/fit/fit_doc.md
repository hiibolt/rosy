# FIT

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    VARIABLE (RE) OBJ;
    X := 0;
    FIT X;
        OBJ := (X - 3) * (X - 3);
    ENDFIT 0.0000000001 1000 1 OBJ;
    WRITE 6 X;
END;
```

## Expected Output

```
 2.999750000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    VARIABLE OBJ 1;
    X := 0;
    FIT X;
        OBJ := (X - 3) * (X - 3);
    ENDFIT 0.0000000001 1000 1 OBJ;
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```
