# LOOP

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) SUM;
    SUM := 0;
    LOOP I 1 5;
        SUM := SUM + I;
    ENDLOOP;
    WRITE 6 SUM;
END;
```

## Expected Output

```
 15.00000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE SUM 1;
    SUM := 0;
    LOOP I 1 5;
        SUM := SUM + I;
    ENDLOOP;
    WRITE 6 SUM;
ENDPROCEDURE;
RUN;
END;
```
