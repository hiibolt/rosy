# SUBSTR

## ROSY Test

```rosy
BEGIN;
    VARIABLE (ST) S;
    VARIABLE (ST) SUB;
    S := 'hello world';
    SUBSTR S 1 5 SUB;
    WRITE 6 SUB;
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
    VARIABLE SUB 80;
    S := 'hello world';
    SUBSTR S 1 5 SUB;
    WRITE 6 SUB;
ENDPROCEDURE;
RUN;
END;
```
