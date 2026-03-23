# FUNCTION

## ROSY Test

```rosy
BEGIN;
    FUNCTION SQUARE X;
        SQUARE := X * X;
    ENDFUNCTION;
    VARIABLE (RE) R;
    R := SQUARE(5);
    WRITE 6 R;
END;
```

## Expected Output

```
 25.00000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE R 1;
    FUNCTION SQUARE X;
        SQUARE := X * X;
    ENDFUNCTION;
    R := SQUARE(5);
    WRITE 6 R;
ENDPROCEDURE;
RUN;
END;
```
