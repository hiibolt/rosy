# PNPRO

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) NP;
    PNPRO NP;
    WRITE 6 NP;
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
    VARIABLE NP 1;
    PNPRO NP;
    WRITE 6 NP;
ENDPROCEDURE;
RUN;
END;
```
