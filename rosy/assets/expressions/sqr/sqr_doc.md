# SQR

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := SQR(3.0);
    WRITE 6 X;
END;
```

## Expected Output

```
 9.000000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := SQR(3.0);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```
