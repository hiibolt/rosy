# CPUSEC

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) T;
    CPUSEC T;
    IF T >= 0;
        WRITE 6 'cpusec ok';
    ENDIF;
END;
```

## Expected Output

```
cpusec ok
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE T 1;
    CPUSEC T;
    IF T >= 0;
        WRITE 6 'cpusec ok';
    ENDIF;
ENDPROCEDURE;
RUN;
END;
```
