# WRITE

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := 42;
    WRITE 6 'Value: ' X;
END;
```

## Expected Output

```
Value:  42.00000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := 42;
    WRITE 6 'Value: ' X;
ENDPROCEDURE;
RUN;
END;
```
