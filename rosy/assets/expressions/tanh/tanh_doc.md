# TANH

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := TANH(1.0);
    WRITE 6 X;
END;
```

## Expected Output

```
0.7615941559557649    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := TANH(1.0);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```
