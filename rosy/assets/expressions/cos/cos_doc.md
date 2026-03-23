# COS

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := COS(1.0);
    WRITE 6 X;
END;
```

## Expected Output

```
0.5403023058681398    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := COS(1.0);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```
