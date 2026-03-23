# DAEPS

## ROSY Test

```rosy
BEGIN;
    DAINI 3 2 0 0;
    DAEPS 0.0000000001;
    WRITE 6 'daeps ok';
END;
```

## Expected Output

```
daeps ok
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE NM 1;
    OV 3 2 0 NM;
    DAEPS 0.0000000001;
    WRITE 6 'daeps ok';
ENDPROCEDURE;
RUN;
END;
```
