# POW

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := 2^10;
    WRITE 6 X;
END;
```

## Expected Output

```
 1024.000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := 2^10;
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```
