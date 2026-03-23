# RECST

## ROSY Test

```rosy
BEGIN;
    VARIABLE (ST) S;
    RECST 3.14 '(F10.4)' S;
    WRITE 6 S;
END;
```

## Expected Output

```
    3.1400
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE S 80;
    RECST 3.14 '(F10.4)' S;
    WRITE 6 S;
ENDPROCEDURE;
RUN;
END;
```
