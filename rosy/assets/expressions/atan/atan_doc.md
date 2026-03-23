# ATAN

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := ATAN(1.0);
    WRITE 6 X;
END;
```

## Expected Output

```
0.7853981633974483    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := ATAN(1.0);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```
