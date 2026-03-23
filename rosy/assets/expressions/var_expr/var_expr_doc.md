# VAR EXPR

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := 99;
    WRITE 6 X;
END;
```

## Expected Output

```
 99.00000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := 99;
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```
