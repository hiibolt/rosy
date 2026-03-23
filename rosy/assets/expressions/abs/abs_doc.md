# ABS

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := ABS(-3.5);
    WRITE 6 X;
END;
```

## Expected Output

```
 3.500000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := ABS(-3.5);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```
