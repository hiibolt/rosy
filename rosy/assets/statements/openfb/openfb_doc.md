# OPENFB

## ROSY Test

```rosy
BEGIN;
    VARIABLE (VE) V;
    V := 1&2&3;
    OPENFB 21 'test_openfb_tmp.bin' 'UNKNOWN';
    WRITEB 21 V;
    CLOSEF 21;
    WRITE 6 'openfb ok';
END;
```

## Expected Output

```
openfb ok
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE V 100;
    V := 1&2&3;
    OPENFB 21 'test_openfb_tmp.bin' 'UNKNOWN';
    WRITEB 21 V;
    CLOSEF 21;
    WRITE 6 'openfb ok';
ENDPROCEDURE;
RUN;
END;
```
