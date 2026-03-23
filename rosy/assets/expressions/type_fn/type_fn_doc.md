# TYPE FN

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    VARIABLE (RE) T;
    X := 42;
    T := TYPE(X);
    WRITE 6 T;
END;
```

## Expected Output

```
 1.000000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    VARIABLE T 1;
    X := 42;
    T := TYPE(X);
    WRITE 6 T;
ENDPROCEDURE;
RUN;
END;
```
