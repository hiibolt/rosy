# SCRLEN

## ROSY Test

```rosy
BEGIN;
    SCRLEN 100;
    WRITE 6 'scrlen ok';
END;
```

## Expected Output

```
scrlen ok
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    SCRLEN 100;
    WRITE 6 'scrlen ok';
ENDPROCEDURE;
RUN;
END;
```
