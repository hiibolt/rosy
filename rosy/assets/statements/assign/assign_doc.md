# ASSIGN

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := 42;
    X := X + 8;
    WRITE 6 X;
END;
```

## Expected Output

```
 50.00000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := 42;
    X := X + 8;
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```
