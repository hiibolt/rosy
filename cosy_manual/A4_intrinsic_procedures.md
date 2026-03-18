### A.4 Intrinsic Procedures

The following is a list of all available intrinsic procedures. The arguments and their properties are listed
behind each name. For each of the arguments, 'v' denotes that it has to be passed as a variable, usually
because a value is assigned to it, and a 'c' denotes that it can either be passed as a constant or a variable,
and no value is assigned to it.

```
MEMALL( v )
Returns the total amount of COSY memory that is currently allocated.
```
```
MEMFRE( v )
Returns the total amount of COSY memory that is currently still available.
```
```
MEMDPV( cc )
Performs a dump of the memory contents of a variable. Arguments are the output unit number and
the variable name.
```
```
MEMWRT( c )
Writes memory to file : I, NBEG, NEND, NMAX, NTYP, CC, NC in first lines, and CC, NC in
subsequent ones. Argument is the unit number.
```
```
SCRLEN( c )
Sets the amount of space scratch variables are allocated with. When needed, use this before calling
the corresponding procedure or function. When a negative number is given, it returns the current
amount.
```
```
CPUSEC( v )
Returns the elapsed CPU time in the process. It may be necessary to adjust the subroutine CPUSEC
in dafox.f depending on the local system.
```
```
PWTIME( v )
Returns the elapsed wall-clock time (sec) on the local node in parallel execution. In serial execution,
returns the same time as CPUSEC.
```
```
PNPRO ( v )
Returns the total number of concurrent processes in parallel execution, which is in most cases
equivalent to the total number of processors used to run the parallel COSY program. In serial
execution, the number returned is 1.
```
```
PROOT ( v )
Returns 1 if the calling process is a root process in parallel execution, and 0 otherwise. In serial
execution, the number returned is 1.
```


```
QUIT ( c )
Terminates execution; argument = 1 triggers whatever system traceback is available by performing
the deliberate illegal operation sqrt(-1.D0).
```
```
SLEEPM( c )
Suspends program execution for a given duration (milli-sec).
```
```
OS ( c )
Triggers a system call. For example, a Unix/Linux command 'date' can be called by \ OS 'date' ;
".
```
```
ARGGET( cv )
Returns then-th command line argument. This interfaces to the GETARG intrinsic subroutine in
FORTRAN. Arguments arenand the resulting string. If then-th command line argument does not
exist, an empty string is returned.
```
```
OPENF ( ccc )
Opens a file. Arguments are unit number, filename (string), and status (string, using same syntax
as the Fortran open).
```
```
OPENFB( ccc )
Opens a binary file. Arguments are unit number, filename (string), and status (string, same as in
Fortran open).
```
```
CLOSEF( c )
Closes a file. Argument is the unit number.
```
```
REWF( c )
Rewinds a file. Argument is the unit number.
```
```
BACKF( c )
Backspaces a file. Argument is the unit number.
```
```
READS ( cv )
Reads a string without attempting to convert it to RE. The arguments are the unit number and the
variable name.
```
```
READB ( cv )
Reads a variable in binary form. The arguments are the unit number and the variable name.
```
```
WRITEB( cc )
Writes a variable in binary form. The arguments are the unit number and the variable name.
```
```
READM( vccccc )
Reads arrays for a variable in the form of the COSY memory contents. The arguments are (1) the
variable name (any data type), (2) the variable information (VE), (3) the length of arrays (RE),
(4) the array for the COSY memory double precision part (RE array), (5) the array for the COSY
memory integer part (RE array), (6) the DA parameters if DA or CD (VE); else 0 (RE). READM is
meant to input the output contents by WRITEM. Refer to WRITEM. The supplied DA parameters
(6) are checked for the compatibility against the current DAINI setup.
```
```
WRITEM( cvcvvv )
Writes the COSY memory contents of a variable in arrays. The arguments are (1) the variable name
(any data type), (2) the variable information (VE), (3) the length of arrays (RE), (4) the array for
the COSY memory double precision part (RE array), (5) the array for the COSY memory integer
part (RE array), (6) the DA parameters if DA or CD (VE); else 0 (RE). The variable information (2)
```


```
consists of the data type, the length in the COSY memory, and the WRITEM version identification
number. The DA parameters (6) consists of the order, the number of variables, and when weighted
DA is setup, the weight factors.
```
```
DAINI ( cccv )
Initializes the order and number of variables of DA or CD. Arguments are order, number of variables,
output unit number (nonzero value will trigger output of internally used addressing arrays to the
given unit), and the number of resulting monomials (on return).
```
```
DANOT( c )
Sets momentary truncation order for DA and CD.
```
```
DANOTW( cc )
Sets weighted order factor of each independent variable for DA and CD. Arguments are the array
containing the weight factors and the size of the array. Must be called before DAINI if needed;
incorrect use of DANOTW may void the entire DA, CD computations. Consult us if it is necessary
to use this procedure.
```
```
DAEPS( c )
Sets garbage collection tolerance, also called cutoff threshold, for coefficients of DA and CD vectors.
```
```
DAEPSM( v )
Returns the garbage collection tolerance, also called cutoff threshold, for coefficients of DA and CD
vectors.
```
```
EPSMIN( v )
Returns the under
ow threshold, the smallest positive number representable on the system.
```
```
DAFSET( c )
Sets the DA filtering mode. Provide a template DA vector for filtering operations DAFILT and
some others including DA multiplications for DA and CD. If the argument is 0 or DAINI is called,
the filtering mode is turned off.
```
```
DAFILT( cv )
Filters a DA or CD vector through the template DA vector specified by DAFSET. Arguments are
the incoming and the result DA or CD vectors.
```
```
DAPEW( cccc )
Prints the part of DA vector that has a certain ordernin a specified independent variablexi:
Arguments are the unit number, the DA vector, the independent variable numberi;and the order
n:
```
```
DAREA ( cvc )
Reads a DA vector. Arguments are the unit number, the variable name and the number of indepen-
dent variables.
```
```
DAPRV ( ccccc )
Writes an array of DA vectors. Arguments are the array, the number of components, maximum and
current main variable number, and the unit number.
```
```
DAREV( vcccc )
Reads an array of DA vectors. Arguments are the array, the number of components (limited to 5
currently), maximum and current main variable number, and the unit number.
```


```
DAFLO( ccvc )
Computes the DA representation of the flow ofx′=f(x) for time step 1 to nearly machine accuracy.
Arguments: array of right hand sides, the initial condition, result, and dimension off.
```
```
CDFLO( ccvc )
Same as DAFLO but with complex arguments.
```
```
DAGMD( ccvc )
Computes∇gfArguments:gas a DA,fas an array of DA, the result DA, and the dimension of
f:
```
```
RERAN ( v )
Returns a random number between-1 and 1:
```
```
DARAN( vc )
Fills a DA vector with random entries between-1 and 1:Arguments are DA vector and the sparsity
fill factor, i.e. the fraction of the coefficients that will actually be set nonzero.
```
```
DADIU( ccv )
Performs a division by a DA independent variablexiif possible. Arguments are the number of the
independent variablei;and the incoming and the result DA or CD vectors. If the division is not
possible, 0 is returned.
```
```
DADMU( cccv )
Performs a division then a multiplication by a DA independent variablexi(division) if possible,
then byxj(multiplication). Arguments are the numbers of the independent variablesi; j;and the
incoming and the result DA or CD vectors. If the division is not possible, 0 is returned.
```
```
DADER( ccv )
Performs the derivation operation on a DA or CD vector. Arguments are the number with respect
to which to differentiate and the incoming and the resulting DA or CD vectors.
```
```
DAINT( ccv )
Performs an integration of a DA vector. Arguments are the number with respect to which to integrate
and the incoming and the result DA or CD vectors.
```
```
DAPLU( cccv )
Replaces power of independent variablexiby constantC:Arguments are the DA or CD vector,i;
C;and the resulting DA or CD vector.
```
```
DASCL( cccv )
Scales thei-th independent variablexiby the factora. Arguments are the DA,i; a;and the resulting
DA.
```
```
DATRN( cccccv )
Transforms independent variablesxiwithaixi+cifori=m 1 ; : : : ; m 2 :Arguments are the DA,ai
andcisupplied by arrays,m 1 ; m 2 ;and the resulting DA.
```
```
DASGN( ccvv )
Flips signs of coefficients of a DA vector by flipping the signs of independent variables to make the
firstNslinear coefficients positive. Arguments are the DA,Ns;then the array containing the signs
of original linear coefficients with the size at leastNs;and the resulting DA are returned.
```
```
DAPEE( ccv )
Returns a coefficient of a DA or CD vector. Arguments are the DA or CD vector, the id for the
coefficient in TRANSPORT notation (for example, the id for thex 1 x^23 term is 133), and the returning
real or complex number.
```


```
DAPEA( cccv )
Same as DAPEE, except the coefficient is specified by an array with each element denoting the
exponent. The third argument is the size of the array.
```
```
DACODE( ccv )
Decodes the DA internal monomial numbers to the exponents. The first argument is a vector
containing the DA parameters such as the order and the number of variables,v;and it is the
same vector as WRITEM returns. The supplied DA parameters are checked for the compatibility
against the current DAINI setup. For all the possible monomials under the current DAINI setup,
the corresponding exponents are returned to the third argument. The third argument is an array,
and theM-th array element contains the exponents of theM-th monomial, whereMis the COSY
DA internal number. Each array element is a number (ifv= 1), or a vector (ifv >1) consisting of
vcomponents. Supply the length of the array via the second argument.
```
```
DANORO( cccvv )
Computes the norms of power sorted parts of the DA. The power sorting is performed with respect
to thei-th variablexi:Arguments are the DA,i;the size of the array (the next argument), then the
norms⃗cstored in the array, and the maximum powerniofxiexisting in the DA are returned. The
maximum norms are computed for⃗c;andc(k+ 1) represents the norm of thek-th power part of the
DA. The number of returned elements of⃗cisni+1:If 0 is given fori;an order sorting is performed.
For weighted order DA computation,niandkdenote the weight divided power.
```
```
DANORS( cccvv )
Computes the summation norms of power sorted parts of the DA. The feature is the same with
DANORO except that DANORO computes maximum norms.
```
```
DACLIW( ccv )
Extracts \linear" coefficients of a DA. When order weighted DA is used, it extracts order weighted
coefficients. Arguments are the DA, the size of the array (the next argument), and the array
containing \linear" coefficients.
```
```
DACQLC( ccvvv )
Extracts coefficients up to second order of a DA. When order weighted DA is used, it extracts order
weighted coefficients. Arguments are the DA, and the size of arrays to store the Hessian matrix and
\linear" coefficients. The returning arguments are the two dimensional array for the Hessian matrix
H;the one dimensional array for the \linear" coefficientsL;and a real number for the constantc:
The quadratic part has the formxtHx=2 +Lx+c:
```
```
DAPEP( cccv )
Returns a parameter dependent component of a DA or CD vector. Arguments are the DA or CD
vector, the coefficient id in TRANSPORT notation for the firstmvariables,m;and the resulting
DA or CD vector. The order of resulting DA or CD is lowered by the amount indicated by id.
```
```
DANOW( ccv )
Computes the order weighted max norm of the DA vector in the first argument. The other arguments
are the weight and the result.
```
```
DAEST( cccv )
Estimates the size ofj-th order terms of the DA vector (with respect to thei-th variablexiifi >0).
Arguments are the DA,i;andj;then the estimated size as summation norm is returned.
```
```
MTREE( vvvvvvv )
Computes the tree representation of a DA array. Arguments: DA array, elements, coefficient array,
2 steering arrays, elements, length of tree.
```


```
CDF2 ( vvvvv )
Lets exp(:f 2 :)) act on first argument in Floquet variables. Other Arguments: 3 tunes (2), result.
```
```
CDNF( vvvvvvvv )
Lets 1=(1-exp(:f 2 :)) act on first argument in Floquet variables. Other Arguments: 3 tunes (2),
array of resonances with dimensions, result.
```
```
CDNFDA( vvvvvvv )
LetsCjact on the first argument. Other Arguments: moduli, arguments, coordinate number, total
number, epsilon, and result.
```
```
CDNFDS( vvvvvvv )
LetsSjact on the first argument. Other Arguments: moduli, arguments, spin argument, total
number, epsilon, and result.
```
```
LINV( cvccv )
Inverts a quadratic matrix. Arguments are the matrix, the inverse, the number of actual entries, the
allocation dimension, and an error flag (0: no error, 132: determinant is zero or very close to zero).
```
```
LDET( cccv )
Computes the determinant of a matrix. Arguments are the matrix, the number of actual entries,
the allocation dimension, and the determinant.
```
```
LEV( cvvvcc )
Computes the eigenvalues and eigenvectors of a matrix. Arguments are the matrix A, the real and
imaginary parts of eigenvalues, a matrix V containing eigenvectors as column vectors, the number of
actual entries, and the allocation dimension. If thei-th eigenvalue is complex with positive imaginary
part, thei-th and (i+ 1)-th columns of V contain the real and imaginary parts of its eigenvector.
```
```
MBLOCK( cvvcc )
Transforms a quadratic matrix to a blocks on diagonal. Arguments are matrix, the transformation
matrix and its inverse, allocation and actual dimension.
```
```
LSLINE( cccvv )
Computes the least square fit liney=ax+bfornpairs of values (x(i); y(i)):Arguments are the
arrayx(); y();and the number of pairsn;thenaandbare returned.
```
```
SUBSTR( cccv )
Returns a substring. Arguments are string, first and last numbers identifying substring, and sub-
string.
```
```
STCRE( cv )
Converts a string to a real. Argument are the string and the real.
```
```
RECST ( ccv )
Converts a real or a complex to a string using a Fortran format. Arguments are the real (or complex),
the format, and the string.
```
```
VELSET( vcc )
Sets a component of a vector of reals VE. Arguments are the vector, the number of the component,
and the real value for the component to be set.
```
```
VELGET( ccv )
Returns a component of a vector of reals VE. Arguments are the vector, the number of the compo-
nent, and on return the real value of the component.
```


```
VEDOT( ccv )
Computes the scalar (inner, dot) product of vectors. Arguments are the two vectors VEs, and on
return the scalar product.
```
```
VEUNIT( cv )
Normalizes the vector. Arguments are the vector VE to be normalized, and on return the normalized
unit vector VE.
```
```
VEZERO( vvv )
Sets any components of vectors in an array to zero if the component exceeds a threshold value.
Arguments are the array of real vectors VE, the number of VE array elements to be checked, and
the threshold value. VEZERO is used in repetitive tracking to prevent over
ow due to lost particle.
```
```
IMUNIT( v )
Returns the imaginary uniti:
LTRUE( v )
Returns the logical value true.
```
```
LFALSE( v )
Returns the logical value false.
```
```
INTPOL( vc )
Determines coefficients of Polynomial satisfyingP(1) =1,P(i)(1) = 0,i= 1; :::; n. Arguments:
coefficient array, n.
```
```
CLEAR ( v )
Clears a graphics object.
```
```
GRMOVE( cccv )
Appends one move to a graphics object. Arguments are the three coordinatesx; y; zand the graphics
object.
GRDRAW( cccv )
Appends one draw to a graphics object. Arguments are the three coordinatesx; y; zand the graphics
object.
```
```
GRDOT ( cccv )
Appends one move and one dot to a graphics object. Arguments are the three coordinatesx; y; z
and the graphics object.
```
```
GRTRI ( cccv )
Appends a triangle to a graphics object. The triangle is formed by the last two positions and the
given point, and updates the current position. Arguments are the three coordinatesx; y; zof the
newly given point and the graphics object.
```
```
GRPOLY( cccv )
Appends a polynomial curve or surface patch to a graphics object. The first argument specifies the
curve or surface by an array of DA vectors with three array elements forx; y; zdescribed in one or
two independent variable(s). The second argument specifies the color, either by GRCOLR style (the
color ID number (RE), or a vector (VE) of RGBA values, or the previously GRCOLR set color if -1),
or by color polynomials using an array of DA vectors with four array elements for RGBA. The third
argument describes the independent variable(s) of the position and color polynomials as type RE
for the curve case, or VE for the surface case. It is possible to specify the discretization number(s)
by using an array for the third argument. In this case, the second component of the array specifies
the discretization number(s) corresponding to the independent variable(s). The fourth argument
contains the graphics object.
```


```
GRCURV( cccccccccv )
Appends a cubic spline curve to a graphics object. Arguments are the three final coordinatesxf;
yf; zf;the three components of the initial tangent vectortix; tiy; tiz;the three components of the
final tangent vectortfx; tfy; tfz;and the graphics object.
```
```
GRCHAR( cv )
Adds a string of characters at the current position in a graphics object. Arguments are the string
and the graphics object.
```
```
GRCOLR( cv )
Adds a color change to a graphics object. Arguments are the new color ID number (RE) or a
vector (VE) of RGBA values, and the graphics object. RGBA describes red, green, blue and alpha
(opacity), and values are between 0 and 1. When the graphics driver supports alpha, A=1 is opaque
(default), and A=0 is transparent and thus invisible. A can be omitted.
color ID color R G B
1 black (default) 0 0 0
2 blue 0 0.2 1
3 red 1 0 0
4 yellow 1 1 0
5 green 0 1 0
6 yellowish green 0.6 0.9 0.2
7 cyan 0 1 1
8 magenta 1 0 1
9 navy 0 0.2 0.7
10 white 1 1 1
```
```
GRWDTH( cv )
Adds a width change to a graphics object. Arguments are the new width and the graphics object.
The default value is 1.
```
```
GRPROJ( ccv )
Sets the 3D projection angles of a graphics object. Arguments are phi and theta in degrees and the
graphics object.
```
```
GRZOOM( ccccccv )
Sets the 3D zooming area specified by two points (x 1 ; y 1 ; z 1 ) and (x 2 ; y 2 ; z 2 ) of a graphics object.
Arguments arex 1 ; x 2 ; y 1 ; y 2 ; z 1 ; z 2 ;and the graphics object.
GRMIMA( cvvvvvv )
Finds the minimal and the maximal coordinates in a graphics object. Arguments are the object and
xmin; xmax; ymin; ymax; zmin; zmax:
```
```
GREPS ( cv )
Sets drawing error tolerances for GRPOLY for the approximation of curves or surfaces by line
segments or quadrilateral meshes, respectively. The first argument specifies the absolute tolerance(s).
If 0, it resets to the default value. By giving a scalar value (RE), it specifies the space error tolerance.
To add the color error tolerance, use the second component of a vector VE. The default tolerance
is 0.005 (i.e., 0.5%) of the frame size of the graphics object for thex; y; zspace part, and 0.005 of
the full range 1 for the color part. Be aware that unreasonably small values may lead to exceedingly
large graphics objects. The second argument is the graphics object.
GRSTYL( cv )
Sets the drawing style. Arguments are the style option and the graphics object. The default option
value is 0. If the option is set to 1, the surface by GRPOLY or GRTRI is drawn by wire frame. The
default is fill painting.
```


```
GROUTF( cc )
Sets the prefix and the sequence starting number for the graphics output file name used in graphics
drivers outputting data to a file. The arguments are the prefix string and the sequence starting num-
ber. The default is `pic' and 1. This is useful to prevent parallel COSY processes from overwriting
each other's graphics output.
```
```
GUISET( ccc )
Updates the value of then-th input field in the given GUI window unit. Arguments are the GUI
window unit, the counting numbernof the input element to replace, and the new value.
```
```
RKCO( vvvvv )
Sets the coefficient arrays used in the COSY eighth order Runge Kutta integrator.
```
```
POLSET( c )
Sets the polynomial evaluation method used in POLVAL. 0: expanded, 1: Horner.
```
```
POLVAL( cccccvc )
Performs the POLVAL composition operation. See Section 2.5 for details.
```
