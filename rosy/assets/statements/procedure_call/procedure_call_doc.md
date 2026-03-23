# PROCEDURE CALL

## ROSY Test

```rosy
BEGIN;
    PROCEDURE SHOW X;
        WRITE 6 X;
    ENDPROCEDURE;
    SHOW 99;
END;
```

## Expected Output

```
 99.00000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    PROCEDURE SHOW X;
        WRITE 6 X;
    ENDPROCEDURE;
    SHOW 99;
ENDPROCEDURE;
RUN;
END;
```
