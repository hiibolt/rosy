# ASIN

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := ASIN(0.5);
    WRITE 6 X;
END;
```

## Expected Output

```
0.5235987755982989    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := ASIN(0.5);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```
