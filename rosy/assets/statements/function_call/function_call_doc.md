# FUNCTION CALL

## ROSY Test

```rosy
BEGIN;
    FUNCTION DOUBLE X;
        DOUBLE := X + X;
    ENDFUNCTION;
    VARIABLE (RE) R;
    R := DOUBLE(21);
    WRITE 6 R;
END;
```

## Expected Output

```
 42.00000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE R 1;
    FUNCTION DOUBLE X;
        DOUBLE := X + X;
    ENDFUNCTION;
    R := DOUBLE(21);
    WRITE 6 R;
ENDPROCEDURE;
RUN;
END;
```
