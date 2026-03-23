# RERAN

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) R;
    RERAN R;
    IF R >= 0;
        IF R < 1;
            WRITE 6 'reran ok';
        ENDIF;
    ENDIF;
END;
```

## Expected Output

```
reran ok
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE R 1;
    RERAN R;
    IF R >= 0;
        IF R < 1;
            WRITE 6 'reran ok';
        ENDIF;
    ENDIF;
ENDPROCEDURE;
RUN;
END;
```
