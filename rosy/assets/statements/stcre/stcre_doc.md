# STCRE

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    STCRE '3.14' X;
    WRITE 6 X;
END;
```

## Expected Output

```
 3.140000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    STCRE '3.14' X;
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```
