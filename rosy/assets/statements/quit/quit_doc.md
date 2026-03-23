# QUIT

## ROSY Test

```rosy
BEGIN;
    WRITE 6 'before quit';
    QUIT 0;
    WRITE 6 'after quit';
END;
```

## Expected Output

```
before quit
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    WRITE 6 'before quit';
    QUIT 0;
    WRITE 6 'after quit';
ENDPROCEDURE;
RUN;
END;
```
