# VEDOT

## ROSY Test

```rosy
BEGIN;
    VARIABLE (VE) A;
    VARIABLE (VE) B;
    VARIABLE (RE) D;
    A := 1&2&3;
    B := 4&5&6;
    VEDOT A B D;
    WRITE 6 D;
END;
```

## Expected Output

```
 32.00000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE A 100;
    VARIABLE B 100;
    VARIABLE D 1;
    A := 1&2&3;
    B := 4&5&6;
    VEDOT A B D;
    WRITE 6 D;
ENDPROCEDURE;
RUN;
END;
```
