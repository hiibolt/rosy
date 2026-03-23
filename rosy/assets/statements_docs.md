# ASSIGN

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := 42;
    X := X + 8;
    WRITE 6 X;
END;
```

## Expected Output

```
 50.00000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := 42;
    X := X + 8;
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

# BREAK STATEMENT

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) I;
    I := 0;
    WHILE I < 100;
        I := I + 1;
        IF I = 5;
            BREAK;
        ENDIF;
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
    WHILE I < 100;
        I := I + 1;
        IF I = 5;
            BREAK;
        ENDIF;
    ENDWHILE;
    WRITE 6 I;
ENDPROCEDURE;
RUN;
END;
```

---

# CLOSEF

## ROSY Test

```rosy
BEGIN;
    OPENF 20 'test_closef_tmp.dat' 'UNKNOWN';
    WRITE 20 'data';
    CLOSEF 20;
    WRITE 6 'closef ok';
END;
```

## Expected Output

```
closef ok
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    OPENF 20 'test_closef_tmp.dat' 'UNKNOWN';
    WRITE 20 'data';
    CLOSEF 20;
    WRITE 6 'closef ok';
ENDPROCEDURE;
RUN;
END;
```

---

# CPUSEC

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) T;
    CPUSEC T;
    IF T >= 0;
        WRITE 6 'cpusec ok';
    ENDIF;
END;
```

## Expected Output

```
cpusec ok
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE T 1;
    CPUSEC T;
    IF T >= 0;
        WRITE 6 'cpusec ok';
    ENDIF;
ENDPROCEDURE;
RUN;
END;
```

---

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

---

# DAEPS

## ROSY Test

```rosy
BEGIN;
    DAINI 3 2 0 0;
    DAEPS 0.0000000001;
    WRITE 6 'daeps ok';
END;
```

## Expected Output

```
daeps ok
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE NM 1;
    OV 3 2 0 NM;
    DAEPS 0.0000000001;
    WRITE 6 'daeps ok';
ENDPROCEDURE;
RUN;
END;
```

---

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

---

# DAPRV

## ROSY Test

```rosy
BEGIN;
    DAINI 3 2 0 0;
    VARIABLE (DA 1) A;
    A(1) := DA(1) + 2;
    DAPRV A 1 2 2 6;
END;
```

## Expected Output

```
  I  COEFFICIENT                1             ORDER EXPONENTS
  1   2.000000000000000         0    0 0
  2   1.000000000000000         1    1 0
------------------------------------------------------
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE NM 1;
    VARIABLE A 2000;
    OV 3 2 0 NM;
    A(1) := DA(1) + 2;
    DAPRV A 1 2 2 6;
ENDPROCEDURE;
RUN;
END;
```

---

# DAREV

## ROSY Test

```rosy
BEGIN;
    DAINI 3 2 0 0;
    VARIABLE (DA 1) A;
    VARIABLE (DA 1) B;
    A(1) := DA(1) + 3;
    OPENF 20 'test_darev_tmp.dat' 'UNKNOWN';
    DAPRV A 1 2 2 20;
    CLOSEF 20;
    OPENF 20 'test_darev_tmp.dat' 'OLD';
    DAREV B 1 2 2 20;
    CLOSEF 20;
    WRITE 6 'darev ok';
END;
```

## Expected Output

```
darev ok
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE NM 1;
    VARIABLE A 2000;
    VARIABLE B 2000;
    OV 3 2 0 NM;
    A(1) := DA(1) + 3;
    OPENF 20 'test_darev_tmp.dat' 'UNKNOWN';
    DAPRV A 1 2 2 20;
    CLOSEF 20;
    OPENF 20 'test_darev_tmp.dat' 'OLD';
    DAREV B 1 2 2 20;
    CLOSEF 20;
    WRITE 6 'darev ok';
ENDPROCEDURE;
RUN;
END;
```

---

# DATRN

## ROSY Test

```rosy
BEGIN;
    DAINI 3 2 0 0;
    VARIABLE (DA 1) INPUT;
    VARIABLE (DA 1) OUTPUT;
    VARIABLE (VE) SCALES;
    VARIABLE (VE) SHIFTS;
    INPUT(1) := DA(1);
    SCALES := 2.0 & 1.0;
    SHIFTS := 1.0 & 0.0;
    DATRN INPUT SCALES SHIFTS 1 1 OUTPUT;
    DAPRV OUTPUT 1 2 2 6;
END;
```

## Expected Output

```
  I  COEFFICIENT                1             ORDER EXPONENTS
  1   1.000000000000000         0    0 0
  2   2.000000000000000         1    1 0
------------------------------------------------------
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE NM 1;
    VARIABLE INPUT 2000;
    VARIABLE OUTPUT 2000;
    VARIABLE SCALES 100;
    VARIABLE SHIFTS 100;
    OV 3 2 0 NM;
    INPUT(1) := DA(1);
    SCALES := 2.0 & 1.0;
    SHIFTS := 1.0 & 0.0;
    DATRN INPUT SCALES SHIFTS 1 1 OUTPUT;
    DAPRV OUTPUT 1 2 2 6;
ENDPROCEDURE;
RUN;
END;
```

---

# FIT

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    VARIABLE (RE) OBJ;
    X := 0;
    FIT X;
        OBJ := (X - 3) * (X - 3);
    ENDFIT 0.0000000001 1000 1 OBJ;
    WRITE 6 X;
END;
```

## Expected Output

```
 2.999750000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    VARIABLE OBJ 1;
    X := 0;
    FIT X;
        OBJ := (X - 3) * (X - 3);
    ENDFIT 0.0000000001 1000 1 OBJ;
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

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

---

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

---

# IF

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := 5;
    IF X > 3;
        WRITE 6 'X is greater than 3';
    ENDIF;
    IF X < 3;
        WRITE 6 'should not print';
    ELSEIF X = 5;
        WRITE 6 'X equals 5';
    ELSE;
        WRITE 6 'should not print either';
    ENDIF;
END;
```

## Expected Output

```
X is greater than 3
X equals 5
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := 5;
    IF X > 3;
        WRITE 6 'X is greater than 3';
    ENDIF;
    IF X < 3;
        WRITE 6 'should not print';
    ELSEIF X = 5;
        WRITE 6 'X equals 5';
    ELSE;
        WRITE 6 'should not print either';
    ENDIF;
ENDPROCEDURE;
RUN;
END;
```

---

# IMUNIT

## ROSY Test

```rosy
BEGIN;
    VARIABLE (CM) Z;
    IMUNIT Z;
    WRITE 6 ST(Z);
END;
```

## Expected Output

```
 (  0.00000000     ,  1.00000000     )
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE Z 2;
    IMUNIT Z;
    WRITE 6 Z;
ENDPROCEDURE;
RUN;
END;
```

---

# LDET

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE 10 10) M;
    VARIABLE (RE) D;
    M(1)(1) := 1;
    M(1)(2) := 2;
    M(2)(1) := 3;
    M(2)(2) := 4;
    LDET M 2 10 D;
    WRITE 6 D;
END;
```

## Expected Output

```
-2.000000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE M 100;
    VARIABLE D 1;
    M(1)(1) := 1;
    M(1)(2) := 2;
    M(2)(1) := 3;
    M(2)(2) := 4;
    LDET M 2 10 D;
    WRITE 6 D;
ENDPROCEDURE;
RUN;
END;
```

---

# LEV

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE 10 10) M;
    VARIABLE (RE 10) ER;
    VARIABLE (RE 10) EI;
    VARIABLE (RE 10 10) V;
    M(1)(1) := 2;
    M(1)(2) := 1;
    M(2)(1) := 1;
    M(2)(2) := 2;
    LEV M ER EI V 2 10;
    VARIABLE (RE) S;
    S := ER(1) + ER(2);
    WRITE 6 S;
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
    VARIABLE M 100;
    VARIABLE ER 10;
    VARIABLE EI 10;
    VARIABLE V 100;
    VARIABLE S 1;
    M(1)(1) := 2;
    M(1)(2) := 1;
    M(2)(1) := 1;
    M(2)(2) := 2;
    LEV M ER EI V 2 10;
    S := ER(1) + ER(2);
    WRITE 6 S;
ENDPROCEDURE;
RUN;
END;
```

---

# LINV

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE 10 10) M;
    VARIABLE (RE 10 10) INV;
    VARIABLE (RE) ERR;
    M(1)(1) := 1;
    M(1)(2) := 2;
    M(2)(1) := 3;
    M(2)(2) := 4;
    LINV M INV 2 10 ERR;
    WRITE 6 ERR;
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
    VARIABLE M 100;
    VARIABLE INV 100;
    VARIABLE ERR 1;
    M(1)(1) := 1;
    M(1)(2) := 2;
    M(2)(1) := 3;
    M(2)(2) := 4;
    LINV M INV 2 10 ERR;
    WRITE 6 ERR;
ENDPROCEDURE;
RUN;
END;
```

---

# LOOP

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) SUM;
    SUM := 0;
    LOOP I 1 5;
        SUM := SUM + I;
    ENDLOOP;
    WRITE 6 SUM;
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
    VARIABLE SUM 1;
    SUM := 0;
    LOOP I 1 5;
        SUM := SUM + I;
    ENDLOOP;
    WRITE 6 SUM;
ENDPROCEDURE;
RUN;
END;
```

---

# MBLOCK

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE 10 10) M;
    VARIABLE (RE 10 10) T;
    VARIABLE (RE 10 10) TI;
    M(1)(1) := 2;
    M(1)(2) := 1;
    M(2)(1) := 1;
    M(2)(2) := 2;
    MBLOCK M T TI 10 2;
    WRITE 6 'mblock ok';
END;
```

## Expected Output

```
mblock ok
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE M 100;
    VARIABLE T 100;
    VARIABLE TI 100;
    M(1)(1) := 2;
    M(1)(2) := 1;
    M(2)(1) := 1;
    M(2)(2) := 2;
    MBLOCK M T TI 10 2;
    WRITE 6 'mblock ok';
ENDPROCEDURE;
RUN;
END;
```

---

# MTREE

## ROSY Test

```rosy
BEGIN;
    DAINI 3 2 0 0;
    VARIABLE (DA 2) MAP;
    MAP(1) := DA(1) + 0.1 * DA(2);
    MAP(2) := DA(2) + 0.1 * DA(1);
    VARIABLE (RE 100) COEFF;
    VARIABLE (RE 100) STEER1;
    VARIABLE (RE 100) STEER2;
    VARIABLE (RE) ELEM2;
    VARIABLE (RE) TLEN;
    MTREE MAP 2 COEFF STEER1 STEER2 ELEM2 TLEN;
    WRITE 6 'mtree ok';
END;
```

## Expected Output

```
mtree ok
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE NM 1;
    VARIABLE MAP 4000;
    VARIABLE COEFF 100;
    VARIABLE STEER1 100;
    VARIABLE STEER2 100;
    VARIABLE ELEM2 1;
    VARIABLE TLEN 1;
    OV 3 2 0 NM;
    MAP(1) := DA(1) + 0.1 * DA(2);
    MAP(2) := DA(2) + 0.1 * DA(1);
    MTREE MAP 2 COEFF STEER1 STEER2 ELEM2 TLEN;
    WRITE 6 'mtree ok';
ENDPROCEDURE;
RUN;
END;
```

---

# OPENF

## ROSY Test

```rosy
BEGIN;
    OPENF 20 'test_openf_tmp.dat' 'UNKNOWN';
    WRITE 20 'hello from file';
    CLOSEF 20;
    WRITE 6 'openf ok';
END;
```

## Expected Output

```
openf ok
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    OPENF 20 'test_openf_tmp.dat' 'UNKNOWN';
    WRITE 20 'hello from file';
    CLOSEF 20;
    WRITE 6 'openf ok';
ENDPROCEDURE;
RUN;
END;
```

---

# OPENFB

## ROSY Test

```rosy
BEGIN;
    VARIABLE (VE) V;
    V := 1&2&3;
    OPENFB 21 'test_openfb_tmp.bin' 'UNKNOWN';
    WRITEB 21 V;
    CLOSEF 21;
    WRITE 6 'openfb ok';
END;
```

## Expected Output

```
openfb ok
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE V 100;
    V := 1&2&3;
    OPENFB 21 'test_openfb_tmp.bin' 'UNKNOWN';
    WRITEB 21 V;
    CLOSEF 21;
    WRITE 6 'openfb ok';
ENDPROCEDURE;
RUN;
END;
```

---

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

---

# PNPRO

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) NP;
    PNPRO NP;
    WRITE 6 NP;
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
    VARIABLE NP 1;
    PNPRO NP;
    WRITE 6 NP;
ENDPROCEDURE;
RUN;
END;
```

---

# POLVAL

## ROSY Test

```rosy
BEGIN;
    DAINI 3 2 0 0;
    VARIABLE (DA 2) P;
    VARIABLE (RE 100) A;
    VARIABLE (RE 100) R;
    P(1) := 1 + DA(1);
    P(2) := 1 + DA(2);
    A(1) := 2;
    A(2) := 3;
    POLVAL 1 P 2 A 2 R 2;
    WRITE 6 'polval ok';
END;
```

## Expected Output

```
polval ok
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE NM 1;
    VARIABLE P 4000;
    VARIABLE A 100;
    VARIABLE R 100;
    OV 3 2 0 NM;
    P(1) := 1 + DA(1);
    P(2) := 1 + DA(2);
    A(1) := 2;
    A(2) := 3;
    POLVAL 1 P 2 A 2 R 2;
    WRITE 6 'polval ok';
ENDPROCEDURE;
RUN;
END;
```

---

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

---

# PROCEDURE CALL

## ROSY Test

```rosy
BEGIN;
    PROCEDURE SHOW X;
        WRITE 6 X;
    ENDPROCEDURE;
    SHOW 99;
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
    PROCEDURE SHOW X;
        WRITE 6 X;
    ENDPROCEDURE;
    SHOW 99;
ENDPROCEDURE;
RUN;
END;
```

---

# PWTIME

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) T;
    PWTIME T;
    IF T >= 0;
        WRITE 6 'pwtime ok';
    ENDIF;
END;
```

## Expected Output

```
pwtime ok
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE T 1;
    PWTIME T;
    IF T >= 0;
        WRITE 6 'pwtime ok';
    ENDIF;
ENDPROCEDURE;
RUN;
END;
```

---

# QUIT

## ROSY Test

```rosy
BEGIN;
    WRITE 6 'before quit';
    QUIT 0;
    WRITE 6 'after quit';
END;
```

## Expected Output

```
before quit
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    WRITE 6 'before quit';
    QUIT 0;
    WRITE 6 'after quit';
ENDPROCEDURE;
RUN;
END;
```

---

# READ

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    OPENF 20 'test_read_tmp.dat' 'UNKNOWN';
    WRITE 20 42;
    CLOSEF 20;
    OPENF 20 'test_read_tmp.dat' 'OLD';
    READ 20 X;
    CLOSEF 20;
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
    OPENF 20 'test_read_tmp.dat' 'UNKNOWN';
    WRITE 20 42;
    CLOSEF 20;
    OPENF 20 'test_read_tmp.dat' 'OLD';
    READ 20 X;
    CLOSEF 20;
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

# READB

## ROSY Test

```rosy
BEGIN;
    VARIABLE (VE) V;
    VARIABLE (VE) V2;
    V := 10&20&30;
    OPENFB 23 'test_readb_tmp.bin' 'UNKNOWN';
    WRITEB 23 V;
    CLOSEF 23;
    OPENFB 23 'test_readb_tmp.bin' 'OLD';
    READB 23 V2;
    CLOSEF 23;
    WRITE 6 ST(V2);
END;
```

## Expected Output

```
  10.00000       20.00000       30.00000
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE V 100;
    VARIABLE V2 100;
    V := 10&20&30;
    OPENFB 23 'test_readb_tmp.bin' 'UNKNOWN';
    WRITEB 23 V;
    CLOSEF 23;
    OPENFB 23 'test_readb_tmp.bin' 'OLD';
    READB 23 V2;
    CLOSEF 23;
    WRITE 6 V2;
ENDPROCEDURE;
RUN;
END;
```

---

# RECST

## ROSY Test

```rosy
BEGIN;
    VARIABLE (ST) S;
    RECST 3.14 '(F10.4)' S;
    WRITE 6 S;
END;
```

## Expected Output

```
    3.1400
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE S 80;
    RECST 3.14 '(F10.4)' S;
    WRITE 6 S;
ENDPROCEDURE;
RUN;
END;
```

---

# RERAN

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) R;
    RERAN R;
    IF R >= 0;
        IF R < 1;
            WRITE 6 'reran ok';
        ENDIF;
    ENDIF;
END;
```

## Expected Output

```
reran ok
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE R 1;
    RERAN R;
    IF R >= 0;
        IF R < 1;
            WRITE 6 'reran ok';
        ENDIF;
    ENDIF;
ENDPROCEDURE;
RUN;
END;
```

---

# SCRLEN

## ROSY Test

```rosy
BEGIN;
    SCRLEN 100;
    WRITE 6 'scrlen ok';
END;
```

## Expected Output

```
scrlen ok
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    SCRLEN 100;
    WRITE 6 'scrlen ok';
ENDPROCEDURE;
RUN;
END;
```

---

# STCRE

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    STCRE '3.14' X;
    WRITE 6 X;
END;
```

## Expected Output

```
 3.140000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    STCRE '3.14' X;
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

# SUBSTR

## ROSY Test

```rosy
BEGIN;
    VARIABLE (ST) S;
    VARIABLE (ST) SUB;
    S := 'hello world';
    SUBSTR S 1 5 SUB;
    WRITE 6 SUB;
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
    VARIABLE SUB 80;
    S := 'hello world';
    SUBSTR S 1 5 SUB;
    WRITE 6 SUB;
ENDPROCEDURE;
RUN;
END;
```

---

# VAR DECL

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    VARIABLE (ST) S;
    VARIABLE (LO) B;
    X := 42;
    S := 'hello';
    B := TRUE;
    WRITE 6 X;
    WRITE 6 S;
    WRITE 6 B;
END;
```

## Expected Output

```
 42.00000000000000    
hello
TRUE
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    VARIABLE S 80;
    VARIABLE B 1;
    X := 42;
    S := 'hello';
    B := TRUE;
    WRITE 6 X;
    WRITE 6 S;
    WRITE 6 B;
ENDPROCEDURE;
RUN;
END;
```

---

# VEDOT

## ROSY Test

```rosy
BEGIN;
    VARIABLE (VE) A;
    VARIABLE (VE) B;
    VARIABLE (RE) D;
    A := 1&2&3;
    B := 4&5&6;
    VEDOT A B D;
    WRITE 6 D;
END;
```

## Expected Output

```
 32.00000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE A 100;
    VARIABLE B 100;
    VARIABLE D 1;
    A := 1&2&3;
    B := 4&5&6;
    VEDOT A B D;
    WRITE 6 D;
ENDPROCEDURE;
RUN;
END;
```

---

# VELGET

## ROSY Test

```rosy
BEGIN;
    VARIABLE (VE) V;
    VARIABLE (RE) X;
    V := 10&20&30;
    VELGET V 2 X;
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
    V := 10&20&30;
    VELGET V 2 X;
    WRITE 6 X;
ENDPROCEDURE;
RUN;
END;
```

---

# VELSET

## ROSY Test

```rosy
BEGIN;
    VARIABLE (VE) V;
    V := 1&2&3;
    VELSET V 2 99;
    WRITE 6 ST(V);
END;
```

## Expected Output

```
  1.000000       99.00000       3.000000
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE V 100;
    V := 1&2&3;
    VELSET V 2 99;
    WRITE 6 V;
ENDPROCEDURE;
RUN;
END;
```

---

# VEUNIT

## ROSY Test

```rosy
BEGIN;
    VARIABLE (VE) V;
    VARIABLE (VE) U;
    V := 3&4;
    VEUNIT V U;
    WRITE 6 ST(U);
END;
```

## Expected Output

```
 0.6000000      0.8000000
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE V 100;
    VARIABLE U 100;
    V := 3&4;
    VEUNIT V U;
    WRITE 6 U;
ENDPROCEDURE;
RUN;
END;
```

---

# VEZERO

## ROSY Test

```rosy
BEGIN;
    VARIABLE (VE) V;
    V := 0.001 & 5 & 0.0001;
    VEZERO V 3 0.01;
    WRITE 6 ST(V);
END;
```

## Expected Output

```
 0.0000000       0.000000      0.0000000
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE V 100;
    V := 0.001 & 5 & 0.0001;
    VEZERO V 3 0.01;
    WRITE 6 V;
ENDPROCEDURE;
RUN;
END;
```

---

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

---

# WRITE

## ROSY Test

```rosy
BEGIN;
    VARIABLE (RE) X;
    X := 42;
    WRITE 6 'Value: ' X;
END;
```

## Expected Output

```
Value:  42.00000000000000    
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE X 1;
    X := 42;
    WRITE 6 'Value: ' X;
ENDPROCEDURE;
RUN;
END;
```

---

# WRITEB

## ROSY Test

```rosy
BEGIN;
    VARIABLE (VE) V;
    V := 1&2&3;
    OPENFB 22 'test_writeb_tmp.bin' 'UNKNOWN';
    WRITEB 22 V;
    CLOSEF 22;
    WRITE 6 'writeb ok';
END;
```

## Expected Output

```
writeb ok
```

## COSY Equivalent

```cosy
BEGIN;
PROCEDURE RUN;
    VARIABLE V 100;
    V := 1&2&3;
    OPENFB 22 'test_writeb_tmp.bin' 'UNKNOWN';
    WRITEB 22 V;
    CLOSEF 22;
    WRITE 6 'writeb ok';
ENDPROCEDURE;
RUN;
END;
```

---

