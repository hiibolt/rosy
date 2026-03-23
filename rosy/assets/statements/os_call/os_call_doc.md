# OS CALL

## ROSY Test

```rosy
BEGIN;
    OS 'echo os_call ok';
END;
```

## Expected Output

```
os_call ok
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    OS 'echo os_call ok';
ENDPROCEDURE;
RUN;
END;
```
