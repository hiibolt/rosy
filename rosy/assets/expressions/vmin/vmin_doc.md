# VMIN

## ROSY Test

```rosy
BEGIN;
    VARIABLE (VE) V;
    VARIABLE (RE) M;
    V := 3 & 1 & 4 & 1 & 5;
    M := VMIN(V);
    WRITE 6 M;
END;
```

## Expected Output

```
 1.000000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE V 100;
    VARIABLE M 1;
    V := 3 & 1 & 4 & 1 & 5;
    M := VMIN(V);
    WRITE 6 M;
ENDPROCEDURE;
RUN;
END;
```
