# TAN

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := TAN(1.0);
    WRITE 6 X;
END;
```

## Expected Output

```
 1.557407724654902    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := TAN(1.0);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```
