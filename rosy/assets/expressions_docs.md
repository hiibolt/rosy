# ABS

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := ABS(-3.5);
    WRITE 6 X;
END;
```

## Expected Output

```
 3.500000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := ABS(-3.5);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

# ACOS

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := ACOS(0.5);
    WRITE 6 X;
END;
```

## Expected Output

```
 1.047197551196597    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := ACOS(0.5);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

# ADD

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := 3 + 4;
    WRITE 6 X;
END;
```

## Expected Output

```
 7.000000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := 3 + 4;
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

# ASIN

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := ASIN(0.5);
    WRITE 6 X;
END;
```

## Expected Output

```
0.5235987755982989    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := ASIN(0.5);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

# ATAN

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := ATAN(1.0);
    WRITE 6 X;
END;
```

## Expected Output

```
0.7853981633974483    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := ATAN(1.0);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

# BOOLEAN

## ROSY Test

```rosy
BEGIN;
    VARIABLE (LO) B;
    B := TRUE;
    WRITE 6 B;
    B := FALSE;
    WRITE 6 B;
END;
```

## Expected Output

```
TRUE
FALSE
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE B 1;
    B := TRUE;
    WRITE 6 B;
    B := FALSE;
    WRITE 6 B;
ENDPROCEDURE;
RUN;
END;
```

---

# CD

## ROSY Test

```rosy
BEGIN;
    DAINI 3 2 0 0;
    VARIABLE (CD) Z;
    Z := CD(1);
    WRITE 6 ST(CONS(REAL(Z)));
END;
```

## Expected Output

```
 0.000000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE NM 1;
    VARIABLE Z 4000;
    OV 3 2 0 NM;
    Z := CD(1);
    WRITE 6 CONS(REAL(Z));
ENDPROCEDURE;
RUN;
END;
```

---

# CMPLX

## ROSY Test

```rosy
BEGIN;
    VARIABLE (CM) Z;
    Z := CMPLX(3.0);
    WRITE 6 ST(Z);
END;
```

## Expected Output

```
 (  3.00000000     ,  0.00000000     )
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE Z 2;
    Z := CMPLX(3.0);
    WRITE 6 Z;
ENDPROCEDURE;
RUN;
END;
```

---

# COMPLEX CONVERT

## ROSY Test

```rosy
BEGIN;
    VARIABLE (CM) Z;
    Z := CM(3&4);
    WRITE 6 ST(Z);
END;
```

## Expected Output

```
 (  3.00000000     ,  4.00000000     )
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE Z 2;
    Z := CM(3&4);
    WRITE 6 Z;
ENDPROCEDURE;
RUN;
END;
```

---

# CONCAT

## ROSY Test

```rosy
BEGIN;
    VARIABLE (VE) V;
    V := 1 & 2 & 3;
    WRITE 6 ST(V);
END;
```

## Expected Output

```
  1.000000       2.000000       3.000000
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE V 100;
    V := 1 & 2 & 3;
    WRITE 6 V;
ENDPROCEDURE;
RUN;
END;
```

---

# CONJ

## ROSY Test

```rosy
BEGIN;
    VARIABLE (CM) Z;
    VARIABLE (CM) C;
    Z := CM(3&4);
    C := CONJ(Z);
    WRITE 6 ST(C);
END;
```

## Expected Output

```
 (  3.00000000     , -4.00000000     )
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE Z 2;
    VARIABLE C 2;
    Z := CM(3&4);
    C := CONJ(Z);
    WRITE 6 C;
ENDPROCEDURE;
RUN;
END;
```

---

# CONS

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := CONS(2.5);
    WRITE 6 X;
END;
```

## Expected Output

```
 2.500000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := CONS(2.5);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

# COS

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := COS(1.0);
    WRITE 6 X;
END;
```

## Expected Output

```
0.5403023058681398    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := COS(1.0);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

# COSH

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := COSH(1.0);
    WRITE 6 X;
END;
```

## Expected Output

```
 1.543080634815243    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := COSH(1.0);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

# DA

## ROSY Test

```rosy
BEGIN;
    DAINI 3 2 0 0;
    VARIABLE (DA) X;
    X := DA(1);
    WRITE 6 ST(CONS(X));
END;
```

## Expected Output

```
 0.000000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE NM 1;
    VARIABLE X 2000;
    OV 3 2 0 NM;
    X := DA(1);
    WRITE 6 CONS(X);
ENDPROCEDURE;
RUN;
END;
```

---

# DERIVE

## ROSY Test

```rosy
BEGIN;
    DAINI 3 2 0 0;
    VARIABLE (DA) F;
    VARIABLE (DA) DF;
    F := DA(1) * DA(1);
    DF := F % 1;
    WRITE 6 ST(CONS(DF));
END;
```

## Expected Output

```
 0.000000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE NM 1;
    VARIABLE F 2000;
    VARIABLE DF 2000;
    OV 3 2 0 NM;
    F := DA(1) * DA(1);
    DF := F % 1;
    WRITE 6 CONS(DF);
ENDPROCEDURE;
RUN;
END;
```

---

# DIV

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := 10 / 4;
    WRITE 6 X;
END;
```

## Expected Output

```
 2.500000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := 10 / 4;
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

# EQ

## ROSY Test

```rosy
BEGIN;
    VARIABLE (LO) R;
    R := 5 = 5;
    WRITE 6 R;
END;
```

## Expected Output

```
TRUE
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE R 1;
    R := 5 = 5;
    WRITE 6 R;
ENDPROCEDURE;
RUN;
END;
```

---

# EXP

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := EXP(1.0);
    WRITE 6 X;
END;
```

## Expected Output

```
 2.718281828459045    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := EXP(1.0);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

# EXTRACT

## ROSY Test

```rosy
BEGIN;
    VARIABLE (VE) V;
    VARIABLE (RE) X;
    V := 10 & 20 & 30;
    X := V|2;
    WRITE 6 X;
END;
```

## Expected Output

```
 20.00000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE V 100;
    VARIABLE X 1;
    V := 10 & 20 & 30;
    X := V|2;
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

# GT

## ROSY Test

```rosy
BEGIN;
    VARIABLE (LO) R;
    R := 5 > 3;
    WRITE 6 R;
END;
```

## Expected Output

```
TRUE
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE R 1;
    R := 5 > 3;
    WRITE 6 R;
ENDPROCEDURE;
RUN;
END;
```

---

# GTE

## ROSY Test

```rosy
BEGIN;
    VARIABLE (LO) R;
    R := 5 >= 3;
    WRITE 6 R;
END;
```

## Expected Output

```
TRUE
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE R 1;
    R := 5 >= 3;
    WRITE 6 R;
ENDPROCEDURE;
RUN;
END;
```

---

# IMAG FN

## ROSY Test

```rosy
BEGIN;
    VARIABLE (CM) Z;
    VARIABLE (RE) R;
    Z := CM(3&4);
    R := IMAG(Z);
    WRITE 6 R;
END;
```

## Expected Output

```
 4.000000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE Z 2;
    VARIABLE R 1;
    Z := CM(3&4);
    R := IMAG(Z);
    WRITE 6 R;
ENDPROCEDURE;
RUN;
END;
```

---

# INT FN

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := INT(2.9);
    WRITE 6 X;
END;
```

## Expected Output

```
 2.000000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := INT(2.9);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

# ISRT

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := ISRT(4.0);
    WRITE 6 X;
END;
```

## Expected Output

```
0.5000000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := ISRT(4.0);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

# ISRT3

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := ISRT3(4.0);
    WRITE 6 X;
END;
```

## Expected Output

```
0.1250000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := ISRT3(4.0);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

# LCD

## ROSY Test

```rosy
BEGIN;
    VARIABLE (VE) V;
    VARIABLE (RE) N;
    V := 3 & 2;
    N := LCD(V);
    WRITE 6 N;
END;
```

## Expected Output

```
 15.00000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE V 100;
    VARIABLE N 1;
    V := 3 & 2;
    N := LCD(V);
    WRITE 6 N;
ENDPROCEDURE;
RUN;
END;
```

---

# LCM

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := LCM(50);
    WRITE 6 X;
END;
```

## Expected Output

```
 100.0000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := LCM(50);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

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

---

# LOG

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := LOG(2.718281828);
    WRITE 6 X;
END;
```

## Expected Output

```
0.9999999998311266    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := LOG(2.718281828);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

# LOGICAL CONVERT

## ROSY Test

```rosy
BEGIN;
    VARIABLE (LO) B;
    B := LO(1);
    WRITE 6 B;
    B := LO(0);
    WRITE 6 B;
END;
```

## Expected Output

```
TRUE
FALSE
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE B 1;
    B := LO(1);
    WRITE 6 B;
    B := LO(0);
    WRITE 6 B;
ENDPROCEDURE;
RUN;
END;
```

---

# LST

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := LST(100);
    WRITE 6 X;
END;
```

## Expected Output

```
 100.0000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := LST(100);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

# LT

## ROSY Test

```rosy
BEGIN;
    VARIABLE (LO) R;
    R := 3 < 5;
    WRITE 6 R;
END;
```

## Expected Output

```
TRUE
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE R 1;
    R := 3 < 5;
    WRITE 6 R;
ENDPROCEDURE;
RUN;
END;
```

---

# LTE

## ROSY Test

```rosy
BEGIN;
    VARIABLE (LO) R;
    R := 3 <= 5;
    WRITE 6 R;
END;
```

## Expected Output

```
TRUE
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE R 1;
    R := 3 <= 5;
    WRITE 6 R;
ENDPROCEDURE;
RUN;
END;
```

---

# LTRIM

## ROSY Test

```rosy
BEGIN;
    VARIABLE (ST) S;
    S := LTRIM('   world');
    WRITE 6 S;
END;
```

## Expected Output

```
world
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE S 80;
    S := LTRIM('   world');
    WRITE 6 S;
ENDPROCEDURE;
RUN;
END;
```

---

# MULT

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := 6 * 7;
    WRITE 6 X;
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
    VARIABLE X 1;
    X := 6 * 7;
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

# NEG

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := -(5);
    WRITE 6 X;
END;
```

## Expected Output

```
-5.000000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := -(5);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

# NEQ

## ROSY Test

```rosy
BEGIN;
    VARIABLE (LO) R;
    R := 5 # 3;
    WRITE 6 R;
END;
```

## Expected Output

```
TRUE
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE R 1;
    R := 5 # 3;
    WRITE 6 R;
ENDPROCEDURE;
RUN;
END;
```

---

# NINT

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := NINT(2.6);
    WRITE 6 X;
END;
```

## Expected Output

```
 3.000000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := NINT(2.6);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

# NORM

## ROSY Test

```rosy
BEGIN;
    VARIABLE (VE) V;
    VARIABLE (VE) N;
    V := 1.5 & -2.5 & 3.5;
    N := NORM(V);
    WRITE 6 ST(N);
END;
```

## Expected Output

```
  1.500000       2.500000       3.500000
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE V 100;
    VARIABLE N 100;
    V := 1.5 & -2.5 & 3.5;
    N := NORM(V);
    WRITE 6 N;
ENDPROCEDURE;
RUN;
END;
```

---

# NOT

## ROSY Test

```rosy
BEGIN;
    VARIABLE (LO) B;
    B := !TRUE;
    WRITE 6 B;
    B := !FALSE;
    WRITE 6 B;
END;
```

## Expected Output

```
FALSE
TRUE
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE B 1;
    B := NOT TRUE;
    WRITE 6 B;
    B := NOT FALSE;
    WRITE 6 B;
ENDPROCEDURE;
RUN;
END;
```

---

# NUMBER

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := 42;
    WRITE 6 X;
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
    VARIABLE X 1;
    X := 42;
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

# POW

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := 2^10;
    WRITE 6 X;
END;
```

## Expected Output

```
 1024.000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := 2^10;
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

# RE CONVERT

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := RE(42);
    WRITE 6 X;
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
    VARIABLE X 1;
    X := RE(42);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

# REAL FN

## ROSY Test

```rosy
BEGIN;
    VARIABLE (CM) Z;
    VARIABLE (RE) R;
    Z := CM(3&4);
    R := REAL(Z);
    WRITE 6 R;
END;
```

## Expected Output

```
 3.000000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE Z 2;
    VARIABLE R 1;
    Z := CM(3&4);
    R := REAL(Z);
    WRITE 6 R;
ENDPROCEDURE;
RUN;
END;
```

---

# SIN

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := SIN(1.0);
    WRITE 6 X;
END;
```

## Expected Output

```
0.8414709848078965    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := SIN(1.0);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

# SINH

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := SINH(1.0);
    WRITE 6 X;
END;
```

## Expected Output

```
 1.175201193643801    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := SINH(1.0);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

# SQR

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := SQR(3.0);
    WRITE 6 X;
END;
```

## Expected Output

```
 9.000000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := SQR(3.0);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

# SQRT

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := SQRT(9.0);
    WRITE 6 X;
END;
```

## Expected Output

```
 3.000000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := SQRT(9.0);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

# STRING

## ROSY Test

```rosy
BEGIN;
    VARIABLE (ST) S;
    S := 'hello';
    WRITE 6 S;
END;
```

## Expected Output

```
hello
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE S 80;
    S := 'hello';
    WRITE 6 S;
ENDPROCEDURE;
RUN;
END;
```

---

# STRING CONVERT

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := 42;
    WRITE 6 ST(X);
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
    VARIABLE X 1;
    X := 42;
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

# SUB

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := 10 - 3;
    WRITE 6 X;
END;
```

## Expected Output

```
 7.000000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := 10 - 3;
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

# TAN

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := TAN(1.0);
    WRITE 6 X;
END;
```

## Expected Output

```
 1.557407724654902    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := TAN(1.0);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

# TANH

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := TANH(1.0);
    WRITE 6 X;
END;
```

## Expected Output

```
0.7615941559557649    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := TANH(1.0);
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

# TRIM

## ROSY Test

```rosy
BEGIN;
    VARIABLE (ST) S;
    S := TRIM('hello   ');
    WRITE 6 S;
END;
```

## Expected Output

```
hello
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE S 80;
    S := TRIM('hello   ');
    WRITE 6 S;
ENDPROCEDURE;
RUN;
END;
```

---

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

---

# VAR EXPR

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := 99;
    WRITE 6 X;
END;
```

## Expected Output

```
 99.00000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := 99;
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

# VARIABLE IDENTIFIER

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := 7;
    WRITE 6 X;
END;
```

## Expected Output

```
 7.000000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := 7;
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

# VE CONVERT

## ROSY Test

```rosy
BEGIN;
    VARIABLE (VE) V;
    V := VE(CM(3&4));
    WRITE 6 ST(V);
END;
```

## Expected Output

```
  3.000000       4.000000
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE V 100;
    V := VE(CM(3&4));
    WRITE 6 V;
ENDPROCEDURE;
RUN;
END;
```

---

# VMAX

## ROSY Test

```rosy
BEGIN;
    VARIABLE (VE) V;
    VARIABLE (RE) M;
    V := 3 & 1 & 4 & 1 & 5;
    M := VMAX(V);
    WRITE 6 M;
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
    VARIABLE V 100;
    VARIABLE M 1;
    V := 3 & 1 & 4 & 1 & 5;
    M := VMAX(V);
    WRITE 6 M;
ENDPROCEDURE;
RUN;
END;
```

---

# VMIN

## ROSY Test

```rosy
BEGIN;
    VARIABLE (VE) V;
    VARIABLE (RE) M;
    V := 3 & 1 & 4 & 1 & 5;
    M := VMIN(V);
    WRITE 6 M;
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
    VARIABLE V 100;
    VARIABLE M 1;
    V := 3 & 1 & 4 & 1 & 5;
    M := VMIN(V);
    WRITE 6 M;
ENDPROCEDURE;
RUN;
END;
```

---

