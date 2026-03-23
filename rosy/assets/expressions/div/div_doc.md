# DIV

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := 10 / 4;
    WRITE 6 X;
END;
```

## Expected Output

```
 2.500000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := 10 / 4;
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```
