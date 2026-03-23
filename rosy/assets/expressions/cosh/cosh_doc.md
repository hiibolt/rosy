# COSH

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := COSH(1.0);
    WRITE 6 X;
END;
```

## Expected Output

```
 1.543080634815243    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := COSH(1.0);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```
