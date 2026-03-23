# EXP

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := EXP(1.0);
    WRITE 6 X;
END;
```

## Expected Output

```
 2.718281828459045    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := EXP(1.0);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```
