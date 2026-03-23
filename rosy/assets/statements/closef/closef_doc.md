# CLOSEF

## ROSY Test

```rosy
BEGIN;
    OPENF 20 'test_closef_tmp.dat' 'UNKNOWN';
    WRITE 20 'data';
    CLOSEF 20;
    WRITE 6 'closef ok';
END;
```

## Expected Output

```
closef ok
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    OPENF 20 'test_closef_tmp.dat' 'UNKNOWN';
    WRITE 20 'data';
    CLOSEF 20;
    WRITE 6 'closef ok';
ENDPROCEDURE;
RUN;
END;
```
