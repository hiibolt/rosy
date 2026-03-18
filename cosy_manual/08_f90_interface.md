## 8 The Fortran 90 Interface

The Fortran 90 interface to COSY INFINITY gives Fortran 90 programmers easy access to the sophisti-
cated data types of COSY INFINITY. The interface has been implemented in the form of a Fortran 90
module.

### 8.1 Installation

Installation of the Fortran 90 interface module to COSY INFINITY requires a Fortran 90 compiler that
is backwards compatible with Fortran 77.

The distribution contains the four Fortran 77 files that make up the COSY INFINITY system (c.f.
Section 1.5 for details on how to compile these files). However, some changes have been made in the file
foxy.f to enable use in the Fortran 90 module. The file foxy.f must be converted from*NORMto the
*FACEversion using VERSION. Specify*NORMand*FACEas the current ID and the new ID, then
VERSION un-comments all the lines that contain the string*FACEin columns 1 to 5, and comments all
the lines containing the string*NORMin columns 73 to 80. See Section 1.5.4 on how to use VERSION.

The actual implementation of the module is contained in the filescosy.f90andcosydef.f90which
contain all the necessary interfaces to use COSY INFINITY from Fortran 90.

The filemain.f90, which is part of the distribution, contains a small demo program that illustrates
how the COSY module can be used in practice. While it does not use all features of the module, it should
provide a good starting point for the development of new programs with the COSY module. Compilation
of the demo program is accomplished by compiling the individual Fortran files and linking them to the
executable program.

Lastly, a makefile is provided that eases the compilation by allowing the user to type \make cosy".
The makefile has been used on UNIX systems with the Digital Fortran compiler \fort" and can easily
be adopted to other platforms. If users port the build system to a new platform, we would like to hear
about this, so we can include the necessary files in the distribution.

### 8.2 Special Utility Routines

The Fortran 90 interface to COSY INFINITY uses a small number of utility routines for low-level access
to the internals. In this section we describe these routines in detail. The routine

SUBROUTINE COSY INIT [<NTEMP>] [<NSCR>] [<MEMDBG>]

initializes the COSY system. This subroutine has to be called before any COSY objects are used.

NTEMP sets the size of the pool of temporary objects and defaults to 20. This pool of variables is used
for the allocation of temporary COSY objects. Since Fortran 90 does not support automatic destruction
of objects, it is necessary to allocate all temporary objects beforehand and never deallocate them during
the execution of the program. The pool is organized as a circular list; and in the absence of automatic
destruction of objects, if the number of actually used temporary variables ever exceeds NTEMP, memory
corruption will occur. It is the responsibility of the user to set the size appropriately.

NSCR defaults to 50000 and sets the size of the variables in the pool. Additionally, the subroutine
SCRLENis called to set the size of COSY's internal temp variables. MEMDBG may be either 0 (no


debug output) or 1 (print debug information on memory usage). It should never be necessary for users of
the Fortran 90 module to set MEMDBG.

Neither the size of the pool, nor the size of the variables in the pool can be changed after this call.
(Refer to Section 8.7 for more details on the pool of temporary objects.) The command

SUBROUTINE COSY CREATE<SELF>[<LEN>] [<VAL>] [<NDIMS>] [<DIMS>]

creates a variable in the cosy core. All COSY objects have to be created before they can be used! This
routine allocates space for the variable and registers it with the COSY system. SELF is the COSY variable
to be created.

LEN is the desired size of the variable SELF (it determines how many DOUBLE PRECISION values
can be stored in SELF) and defaults to 1. If VAL is given, the variable is initialized to it (VAL defaults
to 0.D0). Independent of the parameters LEN and VAL, the type of the variable is set toRE.

This routine can also be used for the creation of COSY arrays (see also Section 8.8). If NDIMS and
DIMS are specified, the variable SELF is initialized to be an NDIMS-dimensional COSY array with length
DIMS(I) in the i-th direction. Each entry of the array has length LEN and is initialized to VAL with type
RE.

SUBROUTINE COSY DESTROY<SELF>

destructs the COSY object SELF and free the associated memory. If SELF hasn't been initialized with
COSYCREATE, the results of this are undefined.

SUBROUTINE COSY ARRAYGET<SELF> <NDIMS> <IDXS>

returns a copy of an element of the array SELF. NDIMS specifies the dimensionality of the array and
IDXS is an array containing the index of the desired element (refer to Section 8.8 for further details on
COSY arrays).

SUBROUTINE COSY ARRAYSET<SELF> <NDIMS> <IDXS> <ARG>

copies the COSY object ARG into an element of the NDIMS-dimensional array SELF. The target is
specified by the NDIMS-dimensional array IDXS which contains the index of the target (refer to Section
8.8 for further details on COSY arrays).

SUBROUTINE COSY GETTEMP<SELF>

returns the address of the next available temporary object from the circular pool (buffer) of such objects.
While the value of the returned variable is undefined, the type is guaranteed to beRE. Refer to Section
8.7 for more details.

SUBROUTINE COSY DOUBLE<SELF>

extracts the DOUBLE PRECISION value from the variable SELF by calling the function COSY function
CONS.

SUBROUTINE COSY LOGICAL<SELF>

extracts the logical value from the variable SELF. If the type of SELF is notLO, the result of the operation
is undefined.

SUBROUTINE COSY WRITE<SELF>[<IUNIT>]

writes the COSY variable SELF to the unit IUNIT (which defaults to 6). This function uses the same
algorithms employed by the COSY procedureWRITE(c.f. Section 3.5).


##### 64 8 THE FORTRAN 90 INTERFACE

##### SUBROUTINE COSY TMP<ARG>

returns a temporary COSY object initialized with the value ARG (which may be either of type DOUBLE
PRECISION or INTEGER). The main purpose of this function is for the temporary conversion of param-
eters to COSY procedures. As an example, consider the following two equivalent code fragments. They
illustrate that the use of the function COSYTMP leads to simpler and less error prone code.

```
TYPE(COSY) :: A,B,X
CALL COSYCREATE(A)
CALL COSYCREATE(B)
CALL COSYCREATE(X,2)
A = 2
B = 5
CALL INTERV(A,B,X)
CALL COSYDESTROY(A)
CALL COSYDESTROY(B)
```
```
TYPE(COSY) :: X
CALL COSYCREATE(X,2)
CALL INTERV(COSYTMP(2),COSYTMP(5),X)
```
### 8.3 Operations

The Fortran 90 interface to COSY INFINITY offers all operators that the standard COSY system offers.
For the convenience of the user, additional support functions are provided that allow mixed operations
between built-in data types and the COSY objects. The following tables list all the defined operations
between COSY objects and built-in types. All operations involving COSY objects return COSY objects.

```
Addition+
COSY COSY COSY
DOUBLE PRECISION COSY COSY
COSY DOUBLE PRECISION COSY
INTEGER COSY COSY
COSY INTEGER COSY
```
```
Subtraction-
COSY COSY COSY
DOUBLE PRECISION COSY COSY
COSY DOUBLE PRECISION COSY
INTEGER COSY COSY
COSY INTEGER COSY
COSY COSY
```
```
Multiplication*
COSY COSY COSY
DOUBLE PRECISION COSY COSY
COSY DOUBLE PRECISION COSY
INTEGER COSY COSY
COSY INTEGER COSY
```


```
Division/
COSY COSY COSY
DOUBLE PRECISION COSY COSY
COSY DOUBLE PRECISION COSY
INTEGER COSY COSY
COSY INTEGER COSY
```
```
Power**
COSY COSY COSY
DOUBLE PRECISION COSY COSY
COSY DOUBLE PRECISION COSY
INTEGER COSY COSY
COSY INTEGER COSY
```
```
Comparison.LT.
COSY COSY COSY
DOUBLE PRECISION COSY COSY
COSY DOUBLE PRECISION COSY
INTEGER COSY COSY
COSY INTEGER COSY
```
```
Comparison.GT.
COSY COSY COSY
DOUBLE PRECISION COSY COSY
COSY DOUBLE PRECISION COSY
INTEGER COSY COSY
COSY INTEGER COSY
```
```
Comparison.EQ.
COSY COSY COSY
DOUBLE PRECISION COSY COSY
COSY DOUBLE PRECISION COSY
INTEGER COSY COSY
COSY INTEGER COSY
```
```
Comparison.NE.
COSY COSY COSY
DOUBLE PRECISION COSY COSY
COSY DOUBLE PRECISION COSY
INTEGER COSY COSY
COSY INTEGER COSY
```

##### 66 8 THE FORTRAN 90 INTERFACE

```
Concatenation.UN.
COSY COSY COSY
DOUBLE PRECISION COSY COSY
COSY DOUBLE PRECISION COSY
INTEGER COSY COSY
COSY INTEGER COSY
DOUBLE PRECISION DOUBLE PRECISION COSY
DOUBLE PRECISION INTEGER COSY
INTEGER DOUBLE PRECISION COSY
INTEGER INTEGER COSY
```
```
Extraction.EX.
COSY COSY COSY
COSY DOUBLE PRECISION COSY
COSY INTEGER COSY
```
```
Derivation.DI.
COSY COSY COSY
COSY DOUBLE PRECISION COSY
COSY INTEGER COSY
```
### 8.4 Assignment

The Fortran 90 interface to COSY INFINITY provides several assignment operations that allow an easy
transition between built-in data types and COSY objects. This section lists all the defined assignment
operators involving COSY objects. The command

COSY LHS = COSY RHS

copies the COSY object RHS to LHS. If LHS hasn't been created yet, it will be created automatically.

DOUBLE PRECISION LHS = COSY RHS

converts the COSY object RHS to the DOUBLE PRECISION number LHS by calling the function COSY
DOUBLE.

LOGICAL LHS = COSY RHS

converts the COSY object RHS to the LOGICAL variable LHS by calling the function COSY LOGICAL.

COSY LHS = DOUBLE PRECISION RHS

copies the DOUBLE PRECISION variable RHS to the COSY object LHS. If LHS hasn't been created
yet, it will be created automatically. The type of LHS will be set toRE.

COSY LHS = LOGICAL RHS

copies the LOGICAL variable RHS to the COSY object LHS. If LHS hasn't been created yet, it will be
created automatically. The type of LHS will be set toLO.

COSY LHS = INTEGER RHS

copies the INTEGER variable RHS to the COSY object LHS. If LHS hasn't been created yet, it will be
created automatically. The type of LHS will be set toRE.


### 8.5 Functions

The Fortran 90 interface to COSY INFINITY supports most of the functions supported by the COSY
environment; for the few functions not supported, a compiler error message will result. Appendix A lists
details on the COSY INFINITY functions.

### 8.6 Subroutines

All the standard procedures of the COSY INFINITY language environment are available as subroutines
from the Fortran 90 interface to COSY. The names and parameter lists of the subroutines match the
names and parameter lists of the normal COSY INFINITY procedures.

Automatic argument conversion is not available. That means that all arguments have to be either
previously created COSY objects or temporary COSY objects obtained from calls to COSYTMP.

### 8.7 Memory Management

The COSY Fortran 90 module is based on the standard core functions and algorithms of COSY INFINITY.
As such, it uses the fixed size memory buffers of COSY INFINITY for storage of COSY objects. While
this fact is mostly hidden from the user, understanding this concept helps in writing efficient code.

When a COSY object is created by using the routine COSY CREATE, memory is allocate in the
internal COSY memory. This memory is not freed until the routine COSYDESTROY is called for this
object. Moreover, since COSY's internal memory is stack based for utmost computational efficiency (and
not garbage collected), memory occupied by one object will not be freed until all objects that have been
created at a later time have also been destroyed.

Since Fortran 90 does not have automatic constructors and destructors, all objects have to be deleted
manually. While this is generally acceptable for normal objects, this is impossible to guarantee for tem-
porary objects. To allow temporary objects in the COSY module, a circular buffer of temp. objects is
created when the COSY system is initialized with COSYINIT.

As an example on how the pool of temporary objects should be used, consider the following fragment of
code that implements a convenience interface to the COSY procedureRERAN. Internally, the function
CRAN obtains one object from the pool for its return value. This avoids the obvious memory leak that
would result if it was creating a new COSY object.

```
FUNCTION CRAN()
USE COSYMODULE
IMPLICIT NONE
TYPE(COSY) :: CRAN
CALL COSY GETTEMP(CRAN)
CALL RERAN(CRAN)
END FUNCTION CRAN
```
However, it has to be stressed that the fixed size of the pool of temporaries bears a potential problem:
there is no check in place for possible exhaustion of the pool. In other words, the pool has to be sized large
enough to accommodate the maximum number of temp. objects at any given time during the execution
of the program. Since this number is easily underestimated, especially for deeply nested expressions, the
buffer should be sized rather generously.


##### 68 8 THE FORTRAN 90 INTERFACE

### 8.8 COSY Arrays vs. Arrays of COSY objects

In the COSY INFINITY language environment, arrays are collections of objects that may or may not have
the same internal type. Thus, within COSY INFINITY, it is conceivable to have an array with entries
representing strings and real numbers. In that sense, the notion of arrays in COSY INFINITY is quite
similar to the notion of arrays of COSY objects in Fortran 90.

However, there is a fundamental difference between the two concepts: a Fortran 90 array of COSY
objects is not again a COSY object. Due to this difference, the Fortran 90 module does not use Fortran
arrays of COSY objects (although the user obviously has the freedom to declare and use them). As a
consequence, the interface provides two different (and slightly incompatible) notions of arrays. \Arrays
of COSY Objects" are Fortran 90 arrays and they can be used wherever Fortran permits the use of
arrays. \COSY Arrays", on the other hand, are individual COSY objects which themselves contain
COSY objects. Since several important procedures of COSY INFINITY assume their arguments to be
COSY arrays, COSY arrays are quite important in the context of COSY INFINITY and its Fortran 90
interface modules.

```
To access the elements of COSY arrays, users should use the utility routines
```
SUBROUTINE COSY ARRAYGET<SELF> <NDIMS> <IDXS>

and

SUBROUTINE COSY ARRAYSET<SELF> <NDIMS> <IDXS> <ARG>

Finally, we point out that the two different concepts of arrays lead to the possibility of having Fortran
90 arrays of COSY arrays { although it would be quite challenging to maintain a clear distinction between
the various indices needed to access the individual elements.
