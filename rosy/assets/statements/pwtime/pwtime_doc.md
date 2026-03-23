# PWTIME

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) T;
    PWTIME T;
    IF T >= 0;
        WRITE 6 'pwtime ok';
    ENDIF;
END;
```

## Expected Output

```
pwtime ok
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE T 1;
    PWTIME T;
    IF T >= 0;
        WRITE 6 'pwtime ok';
    ENDIF;
ENDPROCEDURE;
RUN;
END;
```
