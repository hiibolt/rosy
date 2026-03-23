# WHILE LOOP

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) I;
    I := 0;
    WHILE I < 5;
        I := I + 1;
    ENDWHILE;
    WRITE 6 I;
END;
```

## Expected Output

```
 5.000000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE I 1;
    I := 0;
    WHILE I < 5;
        I := I + 1;
    ENDWHILE;
    WRITE 6 I;
ENDPROCEDURE;
RUN;
END;
```
