# NINT

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := NINT(2.6);
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
    X := NINT(2.6);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```
