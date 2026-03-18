## 3 COSYScript

The COSYScript language is based on aminimal and compact syntax. Experience shows that the
COSY Syntax Table combined with some examples usually allow users to work with COSYScript within
minutes.

COSYScript isobject orientedwithparametric polymorphism(dynamical type assignment). The
language is compiled and linked to a meta-format on the fly and immediately executed. Combined with
the ability to include pre-compiled code, this leads to avery rapid turnaroundfrom input completion
to execution. Combined with built-in tools foroptimization, this makes the tool particularly suitable
forsimulation, as a control language, and forfast prototyping.

Great emphasis is put onperformance, evidenced by negligible overhead to the cost of the operations
on the types. COSYScript usually outperforms code based on the C++ and F90 interfaces discussed in
further sections.

### 3.1 COSYScript Syntax Table

##### BEGIN; END;

```
VARIABLE<name> <length>;
PROCEDURE<arguments>; ENDPROCEDURE;
FUNCTION<arguments>; ENDFUNCTION;
```
```
<name> := <expression>; (Assignment)
```
```
IF<expression>; ELSEIF<expression>; ENDIF;
WHILE<expression>; ENDWHILE;
LOOP<name> <beg> <end>; ENDLOOP;
PLOOP<name> <beg> <end>; ENDPLOOP<comm. rules>;
FIT<variables>; ENDFIT<parameters, objectives>;
```
```
WRITE<unit> <expressions>; READ<unit> <names>;
SAVE<filename>; INCLUDE<filename>;
```
### 3.2 General Aspects of COSYScript

Most commands of COSYScript consist of a keyword, followed by expressions and names of variables,
and terminated by a semicolon. The individual entries are separated by blanks. The exceptions are the
assignment statement, which does not have a keyword but is identified by the assignment identifier :=,
and the call to a procedure, in which case the procedure name is used instead of the keyword.

Line breaks are not significant; commands can extend over several lines, and several commands can
be placed in one line. To facilitate readability of the code, it is possible to include comments. Everything
contained within a pair of curly brackets \f" and \g" is ignored.

Each keyword and each name consist of up to 32 characters, of which the first has to be a letter and
the subsequent ones can be letters, numbers, or the underscore character \". The case of the letters is
not significant.


### 3.3 Program Segments and Structuring

COSYScript consists of a tree-structured arrangement of nested program segments. There are three types
of program segments. The first is the main program, of which there has to be exactly one, and which has
to begin at the top of the input files and ends at their end. It is denoted by the keywords

BEGIN;

and

END;

The other two types of program segments are procedures and functions. Their beginning and ending
are denoted by the commands

PROCEDURE<name>f<name>g;

and

ENDPROCEDURE;

as well as

FUNCTION<name>f<name>g;

ENDFUNCTION;

The first name identifies the procedure and function for the purpose of calling it. The optional names
define the local names of variables that are passed into the routine. Like in other languages, the name of
the function can be used in arithmetic expressions, whereas the call to a procedure is a separate statement.
Procedures and functions must contain at least one executable statement.

Inside each program segment, there are three sections. The first section contains the declaration of local
variables, the second section contains the local procedures and functions, and the third section contains
the executable code. A variable is declared with the command

VARIABLE<name> <expression>f<expression>g;

Here the name denotes the identifier of the variable to be declared. As mentioned above, the types of
variables are free at declaration time. The next expression contains the amount of memory that has to be
allocated when the variable is used. The amount of memory has to be sufficient to hold the various types
that the variable can assume. Various convenience functions to determine these for the COSY types are
available; but if the information is provided directly, a real or double precision number requires a length of
1, a complex double precision number a length of 2. A DA vector requires a length of at least the number
of partial derivatives (n+v)!=(n!v!) invvariables to ordernto be stored, a CD vector requires twice
that, and a TM requires that plus 2n+ 2v:Note that during allocation, the type is initialized to Real,
and the value set to zero.

If the variable is to be used with indices as an array, the next expressions have to specify the different
dimensions. Different elements of an array can have different types, and in this manner it is possible to
emulate user-defined objects. As an example, the command

VARIABLE X 100 5 7 ;

declares X to be a two dimensional array with 5 respectively 7 entries, each of which has room for 100
memory locations. Note that names of variables that are being passed into a function or procedure do
not have to be declared.


All variables are visible inside the program segment in which they are declared as well as in all other
program segments inside it. In case a variable has the same name as one that is visible from a higher
level routine, its name and dimension override the name and properties of the higher level variable of the
same name for the remainder of the procedure and all local procedures. The next section of the program
segment contains the declaration of local procedures and functions. Any such program segment is visible
in the segment in which it was declared and in all program segments inside the segment in which it was
declared, as long as the reference is physically located below the declaration of the local procedure.

The third and final section of the program segment contains executable statements. Among the
permissible executable statements is the assignment statement, which has the form

```
<variable or array element>:=<expression>;
```
The assignment statement does not require a keyword. It is characterized by the assignment identifier
:=. The expression is a combination of variables and array elements visible in the routine, combined with
operands and grouped by parentheses, following common practice. Note that due to the object oriented
features, various operands can be loaded for various data types, and default hierarchies for the operands
are given in Appendix A. Parentheses are allowed to override default hierarchies. The indices of array
elements can themselves be expressions.

Another executable statement is the call to a procedure. This statement does not require a keyword
either. It has the form

<procedure name>f<expression>g;

The name is the identifier of the procedure to be called which has to be visible at the current position.
The rest are the arguments passed into the procedure. The number of arguments has to match the number
of arguments in the declaration of the procedure.

```
Finally, function calls have the form
```
<function name>(<expression>f<, expression>g) ;

The name is the identifier of the procedure to be called which has to be visible at the current position.
The arguments to be passed into the function are surrounded by parenthesis and separated by commas.
The number of arguments has to match the number of arguments in the declaration of the function and
the number of arguments has to be at least one.

### 3.4 Flow Control Statements

Besides the assignment statement and the procedure statement, there are statements that control the
program flow. These statements consist of matching pairs denoting the beginning and ending of a control
structure and sometimes of a third statement that can occur between such beginning and ending state-
ments. Control statements can be nested as long as the beginning and ending of the lower level control
structure is completely contained inside the same section of the higher level control structure.

```
The first such control structure begins with
```
IF<expression>;

which later has to be matched by the command

ENDIF;

If desired, there can be an arbitrary number of statements of the form


ELSEIF<expression>;

between the matchingIFandENDIFstatements.

If there is a structure involvingIF,ELSEIF, andENDIF, the first expression in theIForELSEIFis
evaluated. If it is not of Logical type, an error message will be issued. If the value is Logical True, execution
will continue after the current line and until the nextELSEIF, at which point execution continues after
theENDIF.

If the value is Logical False, the same procedure is followed with the logical expression in the next
ELSEIF, until all of them have been reached, at which point execution continues after theENDIF. At
most one of the sections of code separated byIFand the matching optionalELSEIFand theENDIF
statements is executed.

There is nothing equivalent of a Fortran ELSE statement in the COSYScript, but the same effect can
be achieved with the statement ELSEIF LO(1) ; where LO is a convenience function that returns True
and False for arguments 1 and 0, respectively.

```
The next such control structure consists of the pair
```
WHILE<expression>;

and

ENDWHILE;

If the expression is not of type logical, an error message will be issued. Otherwise, if it has the value true,
execution is continued after theWHILEstatement; otherwise, it is continued after theENDWHILE
statement. In the former case, execution continues until theENDWHILEstatement is reached. After
this, it continues at the matchingWHILE, where again the expression is checked. Thus, the block is run
through over and over again as long as the expression has the proper value.

```
Another such control structure is the familiar loop, consisting of the pair
```
LOOP<name> <expression> <expression>f<expression>g;

and

ENDLOOP;

Here the first entry is the name of a visible variable which will act as the loop variable, the first and second
expressions are the first and second bounds of the loop variable. If a third expression is present, this is
the step size; otherwise, the step size is set to 1. Initially the loop variable is set to the first bound.

If the step size is positive or zero and the loop variable is not greater than the second bound, or the step
size is negative and the loop variable is not smaller than the second bound, execution is continued at the
next statement, otherwise after the matchingENDLOOPstatement. When the matchingENDLOOP
statement is reached after execution of the statements inside the loop, the step size is added to the loop
variable. Then, the value of the loop variable is compared to the second bound in the same way as above,
and execution is continued after theLOOPor theENDLOOPstatement, depending on the outcome
of the comparison. While it is allowed to alter the value of the loop variable inside the loop, this has no
effect on the number of iterations (the loop variable is reset before the next iteration). Hence, it is not
possible to terminate execution of a loop prematurely.

The final control structure in the syntax of COSYScript allows nonlinear optimization as part of the
syntax of the language. This is an unusual feature not found in other languages, and it could also be
expressed in other ways using procedure calls. But the great importance of nonlinear optimization in


applications of the language and the clarity in the code that can be achieved with it seemed to justify
such a step. The structure consists of the pair

FIT<name>f<name>g;

and

ENDFIT< ε > < Nmax> < Nalgorithm> <Objective(s)>;

Here the names denote the visible variables that are being adjusted. εis the tolerance to which the
minimum is requested.Nmaxis the maximum number of evaluations of the objective function permitted.
If this number is set to zero, no optimization is performed and the commands in the fit block are executed
only once. Nalgorithmgives the number of the optimizing algorithm that is being used. For the various
optimizing algorithms, see Section 4 (page 40).<Objective(s)>are of real or integer type and denote
the objective quantities, the quantities that have to be minimized. Currently only the LMDIF optimizer
(Nalgorithm= 4) accepts multiple objectives.

This structure is run through over and over again, where for each pass the optimization algorithm
changes the values of the variables listed in theFITstatement and attempts to minimize the objective
quantity. This continues until the algorithm does not succeed in decreasing the objective quantity anymore
by more than the tolerance or the allowed number of iterations has been exhausted. After the optimization
terminates, the variables contain the values corresponding to the lowest value of the objective quantity
encountered by the algorithm.

Note that it is possible to terminate execution of the program at any time by calling the intrinsic
procedureQUIT. The procedure has one argument which determines if system information is provided.
If this is not desired, the value 0 should be used.

### 3.5 Input and Output

COSYScript has provisions for formatted or unformatted I/O. All input and output is performed using
the two fundamental routines

READ<expression> <name>;

and

WRITE<expression>f<expression>g;

The first expression stands for a unit number, where using common notation, unit 5 denotes the keyboard
and unit 6 denotes the screen. Special unit numbers are provided for input and output to the Graphical
User Interface (see Section 6). Unit numbers can be associated with particular file names by using the
OPENFandCLOSEFprocedures, which can be found in the index.

A user contacted us in 2017 to report an incidence of a system issued error \severe: write to READ-
ONLY file,..." regarding a log output file. This turned out to be caused falsely by an antivirus program.
Please refer to the description on the page found in the index under \RKLOG.DAT" in the Beam Physics
Manual of COSY INFINITY.

It is also possible to have binary input and output. The syntax of real number binary input and output
is similar to the syntax ofREADandWRITE. UseREADBandWRITEBinstead.

READB<expression> <name>;

WRITEB<expression>f<expression>g;


Files for binary input and output have to be opened and closed by using theOPENFBandCLOSEF
procedures. The syntax ofOPENFBis the same asOPENF.

In theREADcommand, the name denotes the variable to be read. If the information that is read is
a legal format free number, the variable will be of real type and contain the value of the number. In any
other case, the variable will be of type string and contain the text just read.

For the case of formatted input of multiple numbers, this resulting string can be broken into sub strings
with the operator \j" via

<string variable>j(<I1>&<I2>)

which returns the substring from position I1 to position I2, as well as the function

R(<string variable>,<I1>,<I2>)

which converts the string representation of the real number contained in the substring from position I1 to
I2 to the real number.

There are also dedicated read commands for other data types. For example, DA vectors can be read
with the procedureDAREA(see index).

In theWRITEcommand, the expressions following the unit are the output quantities. Each quantity
will be printed in a separate line. As described a few lines below, by using the utilities to convert Reals
or complex numbers to stringsSFandSand the concatenation of strings, full formatted output is also
possible.

Depending on the momentary type of the expression, the form of the output will be as follows. Strings
are printed character by character, if necessary over several lines with 132 characters per line, followed by
a line feed.

Real numbers are printed in the Fortran format G23.16E3, followed by a line feed. Complex numbers
will be printed in the form (R,I), where R and I are the real and imaginary parts which are printed in the
Fortran format G17.9E3; the number is followed by a line feed.

Differential Algebraic numbers will be output in several lines. Each line contains the expansion co-
efficient, the order, and the exponents of the independent variables that describe the term. Vanishing
coefficients are not printed. Complex Differential Algebraic variables are printed in a similar way, except
instead of one real coefficient, the real and imaginary parts of the complex coefficient is shown. We note
that it is also possible to print several DA vectors simultaneously such that the coefficients of each vector
correspond to one column. This can be achieved with the intrinsic procedureDAPRV(see index) and is
used for example for the output of transfer maps in the procedurePM(see index).

Taylor models will be output in several lines, too. In addition to the first part, which has the same
format as Differential Algebraic numbers, the information about the reference point and the domain, and
the remainder bound are output.

Vectors are printed component-wise such that five components appear per line in the format G14.7E3.
As discussed above, this can be used to output several Reals in one line.

Logicals are output as TRUE or FALSE followed by a line feed. Graphics objects are output in the
way described in Section 5.2.

As described above, each quantity in theWRITEcommand is output in a new line. To obtain for-
matted output, there are utilities to convert real numbers to strings, several of which can be concatenated
into one string and hence output in one line. The concatenation is performed with the string operator


\&" described in Appendix A. The conversion of a real number or a complex number pair to a string can
be performed with the procedureRECSTdescribed in Appendix A, as well as with the more convenient
COSY function

SF(<real variable>,<format string>)

which returns the string representation of the real variable using the Fortran format specified in the format
string. There is also a simplified version of this function

ST(<real variable>)

which uses the Fortran format G23.16.

BothSFandScan be used for a complex number pair, too. In this case, the format string should
specify only one Fortran number output format, which is applied to both numbers in the pair.

Besides the input and output of variables at execution, there are also commands that allow to save
and include code in compiled form. This allows later inclusion in another program without recompiling,
and thus achieves a similar function as linking. The command

SAVE<name>;

saves the compiled code in a file with the extension \.bin";<name>is a string containing the name of
root of the file, including paths and disks. The command

INCLUDE<name>;

includes the previously compiled code. The name follows the same syntax as in theSAVEcommand.

Each code may contain only oneINCLUDEstatement, and it has to be located at the very top of the
file. TheSAVEandINCLUDEstatements allow breaking the code into a chain of easily manageable
pieces and decrease compilation times considerably.

### 3.6 Parallel Computations

To utilize parallel computation environments, the tasks can be distributed to parallel processes using the
PLOOP{ENDPROOPcontrol structure.

PLOOP<name> <expression> <expression>;

and

ENDPLOOP<name>;

Much like theLOOPconstruct, the first entry is the name of a visible variable which will act as the
loop variable, and the first and second expressions are the first and second bounds of the loop variable.
This loop construct requires that the user run through all of the processes that were asked for; i.e. if
the user requestsNpprocesses for the parallel computations, the loop must traverse each of thoseNp
processes. In almost every case the first expression will be 1 and the second expression will beNp. Note
that it is recommended to avoid nesting this construct. In the situation to run only a single process,
PLOOPbehaves like theLOOPconstruct.

TheENDPLOOPconstruct takes the name associated with an array variable which can be used to
share information between processes. In the next example code, the processes share the information via
the array X.


##### VARIABLE X 1 NP ;

##### PLOOP I 1 NP ;

```
fuser code ;g
ENDPLOOP X ;
```
There are several utility procedures for parallel computations.PNPROreturns the total number of
concurrent processesNpin parallel execution, which enables to write a general purpose code instead of
hard-coding any specific number of processes.PROOTidentifies the root process. To monitor the CPU
time,PWTIMEcan be called to obtain the elapsed wall-clock time, and it is useful to keep track of the
execution time on machines and clusters with time allocations. See Appendix A for more explanations.

### 3.7 Error Messages

COSY distinguishes between five different kinds of error messages which have different meanings and
require different actions to correct the underlying problem. The five types of error messages are identified
by the symbols###,$$$,!!!,@@@and***. In addition, there are informational messages, denoted by
---. The meaning of the error messages is as follows:

###: This error message denotes errors in the syntax of the user input. Usually a short message describing
the problem is given, including the command in error. If this is not enough information to remedy the
problem, the file<inputfile>.lis can be consulted. It contains an element-by-element listing of the user
input, including the error messages at the appropriate positions.

$$$: This error message denotes runtime errors in a syntactically correct user input. Circumstances
under which it is issued include array bound violations, type violations, missing initialization of variables,
exhaustion of the memory of a variable, and illegal operations such as division by zero.

!!!: This error message denotes exhaustion of certain internal arrays in the compiler. Since the basis of
COSY is Fortran which is not recursive and requires a fixed memory allocation, all arrays used in the
compiler have to be previously declared. This entails that in certain cases of big programs etc., the upper
limits of the arrays can be reached. In such a case the user is told which parameter has to be increased.
The problem can be remedied by replacing the value of the parameter by a larger value and re-compiling.
Note that all occurrences of the parameter in question have to be changed globally inallFortran files.

@@@: This message describes a catastrophic error, and should never occur with any kind of user input,
erroneous or not. It means that COSY has found an internal error in its code by using certain self checks.
In the hopefully rare case that such an error message is encountered, the user is kindly asked to contact
us and submit the respective user program.

***: This error message denotes errors in the use of COSY INFINITY library procedures. It includes
messages about improper sequences and improper values for parameters.

In case execution cannot be continued successfully, a system error exit is produced by deliberately
attempting to compute the square root of -1.D0. Depending on the system COSY is run on, this will
produce information about the status at the time of error. In order to be system independent, this is done
by attempting to execute the computation of the root of a negative number.
