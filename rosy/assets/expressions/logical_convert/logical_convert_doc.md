# LOGICAL CONVERT

## ROSY Test

```rosy
BEGIN;
    VARIABLE (LO) B;
    B := LO(1);
    WRITE 6 B;
    B := LO(0);
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
    B := LO(1);
    WRITE 6 B;
    B := LO(0);
    WRITE 6 B;
ENDPROCEDURE;
RUN;
END;
```
