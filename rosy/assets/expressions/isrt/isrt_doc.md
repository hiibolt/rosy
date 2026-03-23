# ISRT

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := ISRT(4.0);
    WRITE 6 X;
END;
```

## Expected Output

```
0.5000000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := ISRT(4.0);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```
