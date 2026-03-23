# INT FN

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := INT(2.9);
    WRITE 6 X;
END;
```

## Expected Output

```
 2.000000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := INT(2.9);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```
