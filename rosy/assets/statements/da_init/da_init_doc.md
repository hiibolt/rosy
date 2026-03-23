# DA INIT

## ROSY Test

```rosy
BEGIN;
    DAINI 3 2 0 0;
    VARIABLE (DA) X;
    X := DA(1);
    WRITE 6 'daini ok';
END;
```

## Expected Output

```
daini ok
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE NM 1;
    VARIABLE X 2000;
    OV 3 2 0 NM;
    X := DA(1);
    WRITE 6 'daini ok';
ENDPROCEDURE;
RUN;
END;
```
