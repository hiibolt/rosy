# TRIM

## ROSY Test

```rosy
BEGIN;
    VARIABLE (ST) S;
    S := TRIM('hello   ');
    WRITE 6 S;
END;
```

## Expected Output

```
hello
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE S 80;
    S := TRIM('hello   ');
    WRITE 6 S;
ENDPROCEDURE;
RUN;
END;
```
