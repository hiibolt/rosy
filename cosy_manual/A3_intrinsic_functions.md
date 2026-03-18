### A.3 Intrinsic Functions

The following is a list of all available intrinsic functions. Each function has a single argument. Also shown
are all allowed incoming types and the resulting types of the function.

```
 RE Converts various types to Real (RE)
```
```
Argument Result Comment
RE RE (no effect)
ST RE Converts a String to Real
CM RE Extracts the Real part
VE RE Determines the average
DA RE Extracts constant part of DA
```
```
 ST Converts various types to String (ST)
```
```
Argument Result Comment
RE ST Formatted Conversion
ST ST (no effect)
LO ST Text of the logical values True or False
CM ST Formatted Conversion
```
```
 LO Converts various types to Logical (LO)
```
```
Argument Result Comment
RE LO 1: True, 0: False
LO LO (no effect)
```
```
 CM Converts various types to Complex (CM)
```
```
Argument Result Comment
RE CM Converts real number to complex
CM CM (no effect)
VE CM Converts two-vector with real and imaginary parts
CD CM Extracts constant part from Complex DA Vector
```
```
 VE Converts various types to Vector (VE)
```
```
Argument Result Comment
RE RE (no effect)
CM VE Extracts real and imaginary parts in two-vector
VE VE (no effect)
```


```
 DA Converts various types to DA Vector
```
```
Argument Result Comment
RE DA Generates the i-th component of identity DA vector
DA DA (no effect)
CD DA Extracts the Real part
```
```
 CD Converts various types to Complex DA Vector (CD)
```
```
Argument Result Comment
RE CD Generates the i-th component of identity CD vector
DA CD Converts DA to CD
CD CD (no effect)
```
```
 LREDetermines allocation size of Real (RE)
```
```
Argument Result Comment
RE RE
```
```
 LSTDetermines allocation size of String (ST)
```
```
Argument Result Comment
RE RE Input: length of string
```
```
 LLODetermines allocation size of Logical (LO)
```
```
Argument Result Comment
RE RE Input: arbitrary
```
```
 LCM Determines allocation size of Complex (CM)
```
```
Argument Result Comment
RE RE Input: arbitrary
```
```
 LVEDetermines allocation size of Vector (VE)
```
```
Argument Result Comment
RE RE Input: number of components
```
```
 LDADetermines allocation size of DA
```
```
Argument Result Comment
VE RE Input: two-vector consisting of order, variables
```
```
 LCD Determines allocation size of Complex DA Vector (CD)
```
```
Argument Result Comment
VE RE Input: two-vector consisting of order, variables
```


```
 LGRDetermines allocation size of Graphics (GR)
```
```
Argument Result Comment
RE RE Input: number of GR elements (Output: approximate length)
```
```
 TYPE Returns the type of an object as a number in internal order
```
```
Argument Result Comment
RE RE
ST RE
LO RE
CM RE
VE RE
DA RE
CD RE
GR RE
```
```
 LENGTH Returns the currently used memory of an object (8 byte blocks)
```
```
Argument Result Comment
RE RE
ST RE
LO RE
CM RE
VE RE
DA RE
CD RE
GR RE
```
```
 VARMEMReturns the current memory address of an object
```
```
Argument Result Comment
RE RE
ST RE
LO RE
CM RE
VE RE
DA RE
CD RE
GR RE
```


```
 VARPOIReturns the current pointer address of an object
```
```
Argument Result Comment
RE RE
ST RE
LO RE
CM RE
VE RE
DA RE
CD RE
GR RE
```
```
 EXP Computes the exponential function
```
```
Argument Result Comment
RE RE
CM CM
VE VE
DA DA
```
```
 LOG Computes the natural logarithm
```
```
Argument Result Comment
RE RE
CM CM
VE VE
DA DA
```
```
 SIN Computes the sine
```
```
Argument Result Comment
RE RE
CM CM
VE VE
DA DA
```
```
 COS Computes the cosine
```
```
Argument Result Comment
RE RE
CM CM
VE VE
DA DA
```


```
 TAN Computes the tangent
```
```
Argument Result Comment
RE RE
VE VE
DA DA
```
```
 ASINComputes the arc sine
```
```
Argument Result Comment
RE RE
VE VE
DA DA
```
```
 ACOSComputes the arc cosine
```
```
Argument Result Comment
RE RE
VE VE
DA DA
```
```
 ATANComputes the arc tangent
```
```
Argument Result Comment
RE RE
VE VE
DA DA
```
```
 SINHComputes the hyperbolic sine
```
```
Argument Result Comment
RE RE
CM CM
VE VE
DA DA
```
```
 COSH Computes the hyperbolic cosine
```
```
Argument Result Comment
RE RE
CM CM
VE VE
DA DA
```


```
 TANHComputes the hyperbolic tangent
```
```
Argument Result Comment
RE RE
VE VE
DA DA
```
```
 SQRT Computes the square root
```
```
Argument Result Comment
RE RE
CM CM
VE VE
DA DA
```
```
 ISRTComputes the reciprocal of the square root,x-^1 =^2
```
```
Argument Result Comment
RE RE
VE VE
DA DA
```
```
 ISRT3 Computes the reciprocal to the power 3/2,x-^3 =^2
```
```
Argument Result Comment
RE RE
VE VE
DA DA
```
```
 SQRComputes the square
```
```
Argument Result Comment
RE RE
CM CM
VE VE
DA DA
CD CD
```
```
 ERFComputes the real error function erf
```
```
Argument Result Comment
RE RE
DA DA
```


```
 WERFComputes the complex error function w
```
```
Argument Result Comment
CM CM
CD CD
```
```
 VMIN Computes the minimum of vector elements
```
```
Argument Result Comment
VE RE
```
```
 VMAX Computes the maximum of vector elements
```
```
Argument Result Comment
VE RE
```
```
 ABS Computes the absolute value
```
```
Argument Result Comment
RE RE
CM RE
VE RE Determines the sum of absolute values of components
DA RE Determines the max norm of coefficients
CD RE Determines the max of the max norms of real and imag. parts
```
```
 NORM Computes the norm of a vector
```
```
Argument Result Comment
VE VE same as ABS
DA RE same as ABS
CD RE same as ABS
```
```
 CONS Determines the constant part of certain types
```
```
Argument Result Comment
RE RE
CM CM
VE RE Determines the largest absolute value of components
DA RE
CD CM
```
```
 REAL Determines the real part of certain types
```
```
Argument Result Comment
RE RE
CM RE
DA DA
CD DA
```


```
 IMAGDetermines the imaginary part of certain types
```
```
Argument Result Comment
RE RE
CM RE
DA DA
CD DA
```
```
 CMPLXConverts types to complex
```
```
Argument Result Comment
RE CM
CM CM
DA CD
CD CD
```
```
 CONJ Determines the complex conjugate of certain types
```
```
Argument Result Comment
RE RE
CM CM
DA DA
CD CD
```
```
 INT Determines the integer part
```
```
Argument Result Comment
RE RE
VE VE
```
```
 NINTDetermines the nearest integer
```
```
Argument Result Comment
RE RE
VE VE
```
```
 NOT Returns the negation of a logical
```
```
Argument Result Comment
LO LO
```
```
 TRIM Removes the space characters from the end of a string
```
```
Argument Result Comment
ST ST
```


```
 LTRIMRemoves the space characters from the beginning of a string
```
```
Argument Result Comment
ST ST
```
```
 GRIUReturns the internally allocated graphics output unit number
```
```
Argument Result Comment
RE RE
```
