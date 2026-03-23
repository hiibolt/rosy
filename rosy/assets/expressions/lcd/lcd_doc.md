# LCD

## ROSY Test

```rosy
BEGIN;
    VARIABLE (VE) V;
    VARIABLE (RE) N;
    V := 3 & 2;
    N := LCD(V);
    WRITE 6 N;
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
    VARIABLE V 100;
    VARIABLE N 1;
    V := 3 & 2;
    N := LCD(V);
    WRITE 6 N;
ENDPROCEDURE;
RUN;
END;
```
