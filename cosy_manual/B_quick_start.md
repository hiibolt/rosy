## B Quick Start Guide for COSY INFINITY

This guide is intended to assist new users to quickly get started with COSY INFINITY. The main emphasis
is placed on writing a meaningful COSYScript program (with the file extension \.fox"), especially for
performing Beam Physics computations. Some examples in this guide require to use the Beam Physics
macro package cosy.fox.

For the information on how to install and execute COSY INFINITY, refer to the web page
cosyinfinity.org, and Section 1.5 (page 8) for the installation, and Section 1.7 (page 23) for the execution,
and especially Section 1.7.6 (page 26) for executing cosy.fox.

### B.1 Basic Structure of a COSYScript Program

#### B.1.1 Program Segments

A complete COSYScript program consists of a tree-structured arrangement of nested program segments.
There are three types of program segments.

MAIN Program

There has to be one main program in a complete COSYScript program. The main program begins at
the beginning and ends at the end of the whole program.

```
Main Program
BEGIN ;
...
END ;
```
Procedure Program and Function Program

A COSYScript program can contain many procedures and functions which can be called by the main
program and the other procedures or functions. A procedure program and a function program must
contain at least one executable statement.

```
Procedure Program
PROCEDUREnamef name1 ...g;
...
ENDPROCEDURE ;
```
```
Function Program
FUNCTIONname name1f... g;
...
ENDFUNCTION ;
```
Note:f gindicates an optional expression.

name: The name of the procedure or the function.


name1 ...: The local name(s) of variable(s) that are passed into the procedure or into the function. These
variables are to not be declared inside the procedure or the function.

```
A call to a procedure program
namefname1 ...g;
```
```
A call to a function program
name( name1f, ... g)
```
name: The name of the procedure or the function.

name1, ...: The argument(s) that are passed into the procedure or the function.

```
The number of arguments in the procedure program or in the function program has to agree with
the number of arguments in its calling statements.
```
```
A call to a function program can be made in an arithmetic expression.
```
Examples

```
1.A call to the procedure DL.
DL .1 ;
This is a drift of length .1 m.
```
```
2.A call to the function ME.
ME(1,2)
This is the (x; a) element of the map.
```
DL, ME are available via cosy.fox; refer to the Beam Physics Manual for DL, ME.

#### B.1.2 Three Sections inside each Program Segment

Inside each program segment, there are three sections.

1. Declaration of Local Variables

The types of variables are free at the declaration time. There is no distinction among integer, real and
double precision numbers. All locally declared variables are visible inside the program segment.

```
Variable Declaration
VARIABLEname expfexp1 ...g;
```
name: The name of the variable to be declared.

exp: The amount of memory to be allocated to the variable.

exp1, ...: In case of an array with indices, it specifies the different dimension.

Examples


```
1.A real number variable X.
VARIABLE X 1 ;
```
```
2.A 57 array Y of memory length 100 per array element.
VARIABLE Y 100 5 7 ;
```
2. Local Procedures and Functions Any local procedures and local functions are coded inside the
program segment. Any local program is visible in the segment, as long as a call statement to it is made
below the local program.
3. Executable Statements Executable statements are assignment statements, call statements to
procedures, flow control statements, input/output statements.

```
Assignment Statement
variable :=expression;
```
variable : The name of a variable or an array element.

expression : A combination of variables and array elements visible in the segment, combined with operands
and grouped by parentheses.

Examples

```
1.An assignment of .5 to a variable Q1.
Q1 := .5 ;
```
```
2.An assignment of the summation of the absolute values of (x; a) and (y; b) elements of the map to
a variable OBJ.
OBJ := ABS(ME(1,2))+ABS(ME(3,4)) ;
```
ME is available via cosy.fox; refer to the Beam Physics Manual for ME.

### B.2 Input and Output

The basic Input and Output statements are as follows.

```
READ statement
READunit name ;
```
unit: The device unit number. 5 denotes the keyboard.

name: The name of the variable to be input.

```
WRITE statement
WRITE unit namefname1 ...g ;
```


unit: The device unit number. 6 denotes the display.

name, name1, ...: The name(s) of the variable(s) or the string(s) to be output.

```
A PM statement prints the map.
```
```
map printing statement
PM unit;
```
unit: The device unit number. PM is available via cosy.fox; refer to the Beam Physics Manual for PM.

### B.3 How to use COSY INFINITY in Beam Physics Computations

There is a COSYScript macro program cosy.fox, which contains many procedures and functions for Beam
Physics computations. It forms a portion of a complete COSYScript program. To access those procedures
and functions, the user has to include cosy.fox into the user's own COSYScript code. Since cosy.fox starts
with the \BEGIN ;" statement, the user code has to have the executable code for the main program and
the \END ;" statement to complete the whole COSYScript program.

To include a COSYScript macro program into the user's code, an include statement has to be placed
in the beginning.

```
Include Statement
INCLUDE 'name' ;
```
name: The name of a previously compiled macro program to be included.

Examples

```
1.Include the compiled version of cosy.fox.
INCLUDE 'COSY' ;
```
```
2.A user's COSYScript code may look as follows.
```
##### INCLUDE 'COSY';

##### PROCEDURE RUN;

##### ...

##### ENDPROCEDURE;

##### RUN;

##### END;

Tips

```
Refer to the Beam Physics Manual for the available procedures and functions in cosy.fox.
```


### B.4 Example: a Sequence of Elements

As a practical example of Beam Physics computations, we set up a sequence of beam elements consisting
of a few drifts and a few quadrupoles, and compute a nonlinear transfer map of the sequence. OV, RP,
UM, DL, MQ, PM in the example are available via cosy.fox; refer to the Beam Physics Manual. This
example program is available as beamdemoele.fox at the COSY INFINITY download site.

##### INCLUDE 'COSY' ;

##### PROCEDURE RUN ;

OV 5 2 0 ; forder 5, phase space dim 2, # of parameters 0g
RP 10 4 2 ;fkinetic energy 10MeV, mass 4 amu, charge 2g
UM ; fsets map to unityg
DL .1 ;fdrift of length .1 mg
MQ .2 .1 .05 ;ffocusing quad; length .2 m, field .1 T, aperture .05 mg
DL .1 ;
MQ .2 -.1 .05 ; fdefocusingg
DL .1 ;
PM 6 ; fprints map to displayg
ENDPROCEDURE ;
RUN ; END ;

```
The first few lines of the resulting transfer map look like this:
```
##### 0.7084973 -0.1798231 0.000000 0.000000 0.000000 100000

##### 0.6952214 1.234984 0.000000 0.000000 0.000000 010000

##### 0.000000 0.000000 1.234984 -0.1798231 0.000000 001000

##### 0.000000 0.000000 0.6952214 0.7084973 0.000000 000100

##### -0.7552786E-01-0.5173667E-01 0.000000 0.000000 0.000000 300000

##### 0.2751173 0.1728297 0.000000 0.000000 0.000000 210000

##### -0.4105720 -0.2057599 0.000000 0.000000 0.000000 120000

##### 0.3541071 0.8137949E-01 0.000000 0.000000 0.000000 030000

##### 0.000000 0.000000 0.5676314E-01-0.5150461E-01 0.000000 201000

The different columns correspond to the final coordinatesx; a; y; bandt:The lines contain the various
expansion coefficients, which are identified by the exponents of the initial condition. For example, the
third column, hence the final coordinatey;of the last line is the number0.5676314E-01, where the
exponents are noted as 201000 , which meansxxy:So, the value of the expansion coefficient (y; xxy) is
0.05676314.

Tips

```
A comment in COSYScript can be written inside a pair of curly brackets.
Example
fThis is a comment in COSY.g
```
```
Any user executable code for a Beam Physics calculation should start with \OV", then \RP" ( or
\RPP" or \RPE"), then \UM". A definition of the beam system consisting of elements like \DL", \DI",
\MQ" ... follows afterward.
```
```
demo.fox includes many example calculations with COSY INFINITY. It is a good starting point to
refer to demo.fox to find some COSYScript example programs.
```


The following are typicaltipsfor COSY beginners.

```
An input COSYScript file name has to have the extension \.fox".
```
```
Don't forget to use the delimiter \;" at the end of each statement.
```
```
COSYScript expressions are not case sensitive (except for strings treated as STring data type ob-
jects).
```
### B.5 Flow Control

Like other computer languages, COSYScript has branching and looping statements. \FIT - ENDFIT
structure" is a unique and unusual feature not found in other languages. It enables nonlinear optimization
as a part of the syntax of the language.

IF - (ELSEIF) - ENDIF Structure

```
IF - (ELSEIF) - ENDIF structure
IFlogical-expression;
...
fELSEIFlogical-expression;
...g
ENDIF ;
```
Example

```
1.If the value of X is not zero, assign the multiplicative inverse of X to Y.
IF X#0 ; Y:= 1/X ; ENDIF ;
```
WHILE - ENDWHILE Structure

```
WHILE - ENDWHILE structure
WHILElogical-expression;
...
ENDWHILE ;
```
Example

```
1.While the value of N is positive, add N to a variable SUM.
SUM := 0 ; READ 5 N ;
WHILE N>0 ; SUM := SUM+N ; READ 5 N ; ENDWHILE ;
```


LOOP - ENDLOOP Structure

```
LOOP - ENDLOOP structure
LOOPname start endf stepg;
```
```
ENDLOOP ;
```
name: The name of the loop counter.

start: The starting value of the counter.

end: The ending value of the counter.

step: The step size of the counter.

Examples

```
1.Compute 10! and store the result in a variable N.
N := 1 ;
LOOP I 1 10 ; N := N*I ; ENDLOOP ;
```
FIT - ENDFIT Structure

```
FIT - ENDFIT structure
FITname1f ...g;
```
```
ENDFIT eps max algo o1fo2 ... g;
```
name1 ...: The variables to be fit.

eps: The tolerance.

max: The maximum number of iterations.

algo: The number of optimizing algorithm to be used.

```
1: The Simplex algorithm.
```
```
4: The LMDIF optimizer. Several objective quantities can be specified.
```
o1f, o2, ...g: The name(s) of objective quantity (quantities) to be minimized.

Examples

```
1.See the next example COSYScript program.
```
### B.6 Example: Fitting a System

As another practical example of Beam Physics computations, we set up a triplet system consisting of three
quadrupoles and drifts, and optimize the triplet system to fulfill some conditions, in this case, to form
a stigmatic imaging system. The program is set so that we can monitor the process of optimization by


beam trajectories through graphics output. OV, RP, UM, DL, MQ, SB, ER, CR, BP, EP, PG, ME in the
example are available via cosy.fox; refer to the Beam Physics Manual. This example program is available
as beamdemofit.fox at the COSY INFINITY download site.

##### INCLUDE 'COSY' ;

##### PROCEDURE RUN ;

##### VARIABLE Q1 1 ; VARIABLE Q2 1 ; VARIABLE OBJ 1 ;

##### PROCEDURE TRIPLET A B ;

##### MQ .1 A .05 ; DL .05 ; MQ .1 -B .05 ; DL .05 ; MQ .1 A .05 ;

##### ENDPROCEDURE ;

##### OV 1 2 0 ; RP 1 1 1 ;

##### SB .15 .15 0 .15 .15 0 0 0 0 0 0 ;

fsets half widths of beam .15 m in x, y and .15 rad in a, bg
Q1 := .5 ; Q2 := .5 ;fstart values of Q1, Q2g
FIT Q1 Q2 ;
UM ; CR ;fclears the raysg
ER 1 3 1 3 1 1 1 1 ;fensemble of rays, 3 in a, bg
BP ;fbegins a pictureg
DL .2 ; TRIPLET Q1 Q2 ; DL .2 ;
EP ;fends the pictureg
PG -1 -2 ;foutputs the x,y pictures to default windowsg
OBJ := ABS(ME(1,2))+ABS(ME(3,4)) ;
fdefines the objective OBJ.
ME(1,2): map element (x,a), ME(3,4): map element (y,b)g
WRITE 6 'Q1, Q2: ' Q1 Q2 'OBJECTIVE: ' OBJ ;
ENDFIT 1E-5 1000 1 OBJ ;
ffits OBJ by Simplex algorithm. This is point-to-point for both x, yg
PG -12 -12 ;
foutput final pictures to PDF files pic001.pdf and pic002.pdfg
ENDPROCEDURE ;
RUN ; END ;

```
The following finalx; ypictures are created in PDF files pic001.pdf and pic002.pdf.
```
```
MQ MQ MQ 0.40
```
```
0.10
```
```
0.10
```
```
X-motion
```
```
0.10MQ MQ MQ 0.40
```
```
0.10 Y-motion
```
Tip: Refer to Section 5.2 (page 42) for information on the device unit numbers for various graphics
drivers.
