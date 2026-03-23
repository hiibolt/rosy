# NEQ

## ROSY Test

```rosy
BEGIN;
    VARIABLE (LO) R;
    R := 5 # 3;
    WRITE 6 R;
END;
```

## Expected Output

```
TRUE
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE R 1;
    R := 5 # 3;
    WRITE 6 R;
ENDPROCEDURE;
RUN;
END;
```
