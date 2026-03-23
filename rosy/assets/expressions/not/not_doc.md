# NOT

## ROSY Test

```rosy
BEGIN;
    VARIABLE (LO) B;
    B := !TRUE;
    WRITE 6 B;
    B := !FALSE;
    WRITE 6 B;
END;
```

## Expected Output

```
FALSE
TRUE
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE B 1;
    B := NOT TRUE;
    WRITE 6 B;
    B := NOT FALSE;
    WRITE 6 B;
ENDPROCEDURE;
RUN;
END;
```
