# PROCEDURE

## ROSY Test

```rosy
BEGIN;
    PROCEDURE GREET NAME;
        WRITE 6 'Hello ' NAME;
    ENDPROCEDURE;
    GREET 'World';
END;
```

## Expected Output

```
Hello World
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    PROCEDURE GREET NAME;
        WRITE 6 'Hello ' NAME;
    ENDPROCEDURE;
    GREET 'World';
ENDPROCEDURE;
RUN;
END;
```
