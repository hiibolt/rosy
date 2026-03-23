# LST

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := LST(100);
    WRITE 6 X;
END;
```

## Expected Output

```
 100.0000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := LST(100);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```
