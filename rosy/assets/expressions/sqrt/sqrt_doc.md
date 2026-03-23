# SQRT

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := SQRT(9.0);
    WRITE 6 X;
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
    VARIABLE X 1;
    X := SQRT(9.0);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```
