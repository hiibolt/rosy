# SINH

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := SINH(1.0);
    WRITE 6 X;
END;
```

## Expected Output

```
 1.175201193643801    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := SINH(1.0);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```
