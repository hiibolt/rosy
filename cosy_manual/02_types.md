## 2 COSY Types

This section should be read together with Appendix A, which lists the elementary operations, procedures,
and functions defined for COSY objects.

COSY INFINITY is an environment with dynamic typing, also called polymorphism. Thus, the same
expression can be evaluated with different types, and the same variable can assume different types at
different times in the execution.

In this section, we will discuss the corresponding COSY functions and procedures that allow the explicit
initialization of COSY variables to various types, and illustrate some of the most important tools for the
manipulation of these types.

All examples are given in COSYScript, but readily translate to the syntax of C++ and/or F90, using
the same names for intrinsic functions and procedures.

### 2.1 Reals, Complex, Strings, and Logicals

Real number variables are created by assignment. Initially, all variables are of typeREand are initialized
to 0. Thus, the following fragment declares two variablesXandYwith enough space for a single double
precision number and initializes them to 1 and 1=e^3 , respectively.

```
VARIABLE X 1 ; VARIABLE Y 1 ;
X := 1 ; fAssigns value 1 to variable Xg
Y := EXP(-3) ;
```
Details on the allowed operations and their return types for real variables can be found in Appendix
A.

Complex numbers are created with the help of the COSY intrinsic functionCM. The following two
fragments each create a variableZand initialize it toz= 2- 3 i:Note that the variablesZandIhave to
be declared with enough space to hold two double precision numbers.

```
VARIABLE Z 2 ; VARIABLE I 2 ; VARIABLE X 1 ; VARIABLE Y 1 ;
I := CM(0&1); fAssigns imaginary unit to variable ig
Z := 2 - 3*I ; fAssigns complex result by mixing real and complexg
```
or

```
Z := CM(2&(-3)) ; fAssigns complex number (2,-3) directlyg
X := RE(Z) ; fDetermines the real part of Zg
X := Zj1 ; Y := Zj 2 fExtracts the real and imaginary parts of Zg
```
Once initialized, complex numbers can be used in most mathematical expressions and evaluations
(refer to Appendix A for details).

Strings can be created either by assignment, or by concatenation of other strings, or by conversion
from other types. As an example, consider the following code fragment:

```
VARIABLE S 80 ; VARIABLE T 80 2 ;
T(1) := 'HELLO ' ; fAssigns values to stringsg
T(2) := 'WORLD' ;
S := T(1)&T(2) ; fConcatenates the two stringsg
S := ST(4*atan(1)) ;fContains an approximation of the leading digits of PIg
```


It creates two string variables by assignment and initializes the variableSby assigning the union of
the two variablesT(1)andT(2). Other procedures operating on strings are described in Appendix A.

Logical variables can be created by assignment using operators that return results of type logical, or
by the use of the intrinsic functionLOdescribed in Appendix A. The following code fragments illustrates
this:

```
VARIABLE L 1 ;
L := 1=1 ;
L := LO(1) ;
```
Note that logical values can be stored in variables of any size. Appendix A describes the operations
and functions defined for logical variables.

### 2.2 Vectors

COSY INFINITY has vector data types that are similar to one-dimensional arrays, but differ in that
elementary operations and functions are defined on them (generally, the operations act component-wise).
The appropriate use of vectors allow performance gains on processors utilizing hyperthreading or multiple
cores, in OpenMP environments, and also in other environments due to simplifications in memory access.

Several different vector types exist, distinguished by the type of the components. Vectors can be
created with the concatenation operator \&" and utility functions exist to extract components. The
following fragments demonstrate the creation of a real number vector.

```
VARIABLE V 4 ; VARIABLE X 1 ;
V := 22&33 ; fCreates Vector V from two components 22 and 33g
V := 11&V&44 ; fTurns V into a vector with four componentsg
```
```
X := Vj3 ; fExtracts third component from V and stores in Xg
X := VMIN(V) ; fReturns the minimum of the entries in Vg
X := VMAX(V) ; fReturns the maximum of the entries in V)
```
```
X := RE(V) ; fComputes the arithmetic mean of the entries of Vg
```
More details on the operations and functions defined on the various vector data types are given in
Appendix A.

### 2.3 DA Vectors

DA vectors can be created in several ways. First, it is important to distinguish DA Vectors from the usual
vector data types: DA vectors are multiplied according to the rule of an algebra (in fact, a differential
algebra), while Vectors are multiplied component-wise. Also, DA vectors support the derivation and
anti-derivation operations characteristic of differential algebraic structures.

DA vectors can be created by evaluating expressions with the return values of theDAfunction. Use
of DA vectors requires prior initialization of the DA system of COSY INFINITY by using the procedure
DAINI. As an example of creating a DA vector, consider the following code fragment. It initializes the
DA system to order three in two variables and assigns the third-order Taylor expansion ofx 1 exp(x 1 +x 2 )
around the origin to the variableD.


##### VARIABLE D 100 ; VARIABLE NM 1 ;

```
DAINI 3 2 0 NM ; fInitializes DA for order 3 and 2 variablesg
D := DA(1)*EXP(DA(1)+DA(2)) ; fAssigns D to be a DA vectorg
```
The differential algebraic structure induces a derivation and an anti-derivation operation. These can
be used in the following way.

```
VARIABLE D2 100 ; VARIABLE DI 100 ;
D2 := D%2 ; fAssigns D2 to be the DA vector of the partial
derivate of D with respect to variable 2g
DI := D%(-1) ; fAssigns DI to be the DA vector of the integral
of D with respect to variable 1g
```
```
It is possible to extract individual coefficients from DA vectors:
```
```
X := RE(D2) ; fExtracts constant part from D2g
X := DIj(2&1) ; fExtracts coefficient x^2 y from DIg
```
```
More details on the operations and functions defined for DA vectors are given in Appendix A.
```
### 2.4 Taylor Models (RDA Objects)

Taylor model variables [16] [18] [17] should be created evaluating expressions with elementary Taylor
models. The latter can be created with the intrinsic procedureTMVAR or the convenience function
TMI. Like in the case of DA vectors, use of Taylor models requires prior initialization of the DA system.
The following fragment creates a 10th order Taylor model forf(x 1 ; x 2 ) =x 1 exp(x 1 +x 2 ), defined over
the domain (2 + [- 1 = 4 ; 1 =4])(5 + [- 1 = 2 ; 1 =2]) with reference point of (2;5) to the variableD.

```
VARIABLE D 1000 ; VARIABLE NM 1 ; VARIABLE X1 100 ; VARIABLE X2 100 ;
DAINI 10 2 0 NM ;
X1 := 2 + TM(1)/4 ; X2 := 5 + TM(2)/2 ;
D := X1*EXP(X1+X2) ;
```
Coefficients from Taylor models can be extracted in the same way as for DA vectors.

```
Note that Taylor models are not supported in the current version of COSY INFINITY.
```
### 2.5 The Intrinsic Procedure POLVAL

An important COSY intrinsic procedure for DA vectors and Taylor models is the tool POLVAL. It has
the formal syntax

POLVAL<L> <P> <NP> <A> <NA> <R> <NR>;

where<P>; <A>;and<R>are arrays, andPOLVALlets the polynomial described by the NP DA
vectors or Taylor models stored in the array P act on the NA arguments A, and the result is stored in the
NR Vectors R.

In the normal situation, L should be set 1. AfterPOLVALhas already been called with L= 1;and
if it is called with the same polynomial array P again, a certain part of internal analysis of P can be
avoided by callingPOLVALwith L=-1 or L= 0:(There are other advanced settings for L, but their use
is discouraged for normal users because they may interfere with the internal use ofPOLVALof various
COSY tools.)


The type of the array A is free, but all elements of A have to be the same type. It can be either DA,
or CD, in which case the procedure acts as a concatenator, it can be real or complex, in which case it acts
like a polynomial evaluator, or it can be of vector type VE, in which case it acts as an efficient vectorizing
polynomial evaluator, which is used for example for repetitive tracking in Beam Physics applications. If
necessary, adding0*A(1)to subsequent array elementsA(I)can make the type of the argument array
element agree to that type ofA(1).

### 2.6 Verification of COSY

The operations on the various types have been verified for correctness in a variety of ways.

```
The intrinsic operations of the Real, Complex, and DA data types have been verified for various
complex examples in Beam Physics against the code COSY 5.0 [5]. Despite the similar name, COSY
5.0 uses analytic formulas developed by a custom-made high performance formula manipulator [9]
and not DA tools to compute flows of particle accelerators up to order five. Agreement to near
machine precision has been obtained for all terms in the flow expansion up to order five for a large
class of different particle optical systems. Since the computation of these flow expansions requires
virtually all COSY intrinsic operations and functions for the Real, Complex, and DA data types, any
errors in their implementation would be expected to lead to some discrepancies. Since all operations
in the DA data types are independent of order, agreement of up to order five also provides confidence
for agreement to higher order.
```
```
Flows for various specific ODEs that possess certain invariants of motion have been cross checked
against these invariants. In particular, a large class of flows of systems in Beam Physics up to orders
15 has been checked for satisfaction of symplecticity as well as energy conservation. Similar to the
previous test, any errors in implementation of the Real, Complex, and DA data types would be
expected to lead to violations of these invariants.
```
```
Advanced arguments involving symplectic representations and geometric symmetries allow to devise
nonlinear systems for which all nonlinearities of the flows up to a given order cancel at certain values
of the independent variable [22] [23]. Following these prescriptions, such systems have been designed
with COSY, and as predicted in the theory, the advertised nonlinearities do indeed vanish [24].
This provides confidence in the ability to compute the underlying flows properly, and again provide
confidence in their correctness.
```
```
The Taylor model data types have been verified via rather extensive tests against high-precision
arithmetic packages by Corliss and Yun [10]. Further extensive automated tests have been performed
by Nathalie Revol against other high-precision packages (unpublished). The theoretical soundness
of their implementation has been verified [21]. Since the underlying Taylor models utilize those of
the DA type, this also provides verification of those operations.
```
