# RE CONVERT

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := RE(42);
    WRITE 6 X;
END;
```

## Expected Output

```
 42.00000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := RE(42);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```
