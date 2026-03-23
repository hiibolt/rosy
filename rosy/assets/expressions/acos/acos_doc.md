# ACOS

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := ACOS(0.5);
    WRITE 6 X;
END;
```

## Expected Output

```
 1.047197551196597    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := ACOS(0.5);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```
