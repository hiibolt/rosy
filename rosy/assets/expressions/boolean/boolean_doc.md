# BOOLEAN

## ROSY Test

```rosy
BEGIN;
    VARIABLE (LO) B;
    B := TRUE;
    WRITE 6 B;
    B := FALSE;
    WRITE 6 B;
END;
```

## Expected Output

```
TRUE
FALSE
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE B 1;
    B := TRUE;
    WRITE 6 B;
    B := FALSE;
    WRITE 6 B;
ENDPROCEDURE;
RUN;
END;
```
