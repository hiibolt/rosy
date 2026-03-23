# DANOT

## ROSY Test

```rosy
BEGIN;
    DAINI 5 2 0 0;
    DANOT 3;
    WRITE 6 'danot ok';
END;
```

## Expected Output

```
danot ok
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE NM 1;
    OV 5 2 0 NM;
    DANOT 3;
    WRITE 6 'danot ok';
ENDPROCEDURE;
RUN;
END;
```
