# VARIABLE IDENTIFIER

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := 7;
    WRITE 6 X;
END;
```

## Expected Output

```
 7.000000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := 7;
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```
