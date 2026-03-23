# LTRIM

## ROSY Test

```rosy
BEGIN;
    VARIABLE (ST) S;
    S := LTRIM('   world');
    WRITE 6 S;
END;
```

## Expected Output

```
world
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE S 80;
    S := LTRIM('   world');
    WRITE 6 S;
ENDPROCEDURE;
RUN;
END;
```
