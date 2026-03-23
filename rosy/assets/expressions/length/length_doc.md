# LENGTH

## ROSY Test

```rosy
BEGIN;
    VARIABLE (ST) S;
    VARIABLE (RE) L;
    S := 'hello';
    L := LENGTH(S);
    WRITE 6 L;
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
    VARIABLE S 80;
    VARIABLE L 1;
    S := 'hello';
    L := LENGTH(S);
    WRITE 6 L;
ENDPROCEDURE;
RUN;
END;
```
