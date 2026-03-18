### A.2 Operators

Now follows a list of all operators available for various combinations of objects. Allowed types of the left
and the right operands are shown as well as the resulting types of the operation.

For each operation, a relative priority is given which determines the hierarchy of the operations in
expressions if there are no parentheses. An operation with a larger priority number has higher priority.


```
 + (Addition) (Priority: 3)
```
```
Left Right Result Comment
RE RE RE
RE CM CM
RE VE VE Add Real componentwise
RE DA DA
RE CD CD
LO LO LO Logical OR
CM RE CM
CM CM CM
CM DA CD
CM CD CD
VE RE VE Add Real componentwise
VE VE VE Add componentwise
DA RE DA
DA CM CD
DA DA DA
DA CD CD
CD RE CD
CD CM CD
CD DA CD
CD CD CD
```
```
 -(Subtraction) (Priority: 3)
```
```
Left Right Result Comment
RE RE RE
RE CM CM
RE VE VE Subtract componentwise from Real
RE DA DA
RE CD CD
CM RE CM
CM CM CM
CM DA CD
CM CD CD
VE RE VE Subtract Real componentwise
VE VE VE Subtract componentwise
DA RE DA
DA CM CD
DA DA DA
DA CD CD
CD RE CD
CD CM CD
CD DA CD
CD CD CD
```


```
 * (Multiplication) (Priority: 4)
```
```
Left Right Result Comment
RE RE RE
RE CM CM
RE VE VE Multiply with Real componentwise
RE DA DA
RE CD CD
LO LO LO Logical AND
CM RE CM
CM CM CM
CM DA CD
CM CD CD
VE RE VE Multiply with Real componentwise
VE VE VE Multiply componentwise
DA RE DA
DA CM CD
DA DA DA
DA CD CD
CD RE CD
CD CM CD
CD DA CD
CD CD CD
```
```
 / (Division) (Priority: 4)
```
```
Left Right Result Comment
RE RE RE
RE CM CM
RE VE VE Divide Real componentwise
RE DA DA
RE CD CD
CM RE CM
CM CM CM
CM DA CD
CM CD CD
VE RE VE Divide by Real componentwise
VE VE VE Divide componentwise
DA RE DA
DA CM CD
DA DA DA
DA CD CD
CD RE CD
CD CM CD
CD DA CD
CD CD CD
```


```
 ^ (Exponentiation) (Priority: 5)
```
```
Left Right Result Comment
RE RE RE
VE RE VE Raise to Real power componentwise
```
```
 <(Less Than) (Priority: 2)
```
```
Left Right Result Comment
RE RE LO
ST ST LO Lexicographic Ordering
```
```
 >(Greater Than) (Priority: 2)
```
```
Left Right Result Comment
RE RE LO
ST ST LO Lexicographic Ordering
```
```
 = (Equal) (Priority: 2)
```
```
Left Right Result Comment
RE RE LO
ST ST LO
```
```
 # (Not Equal) (Priority: 2)
```
```
Left Right Result Comment
RE RE LO
ST ST LO
```
```
 & (Concatenation) (Priority: 2)
```
```
Left Right Result Comment
RE RE VE Concatenate two Reals to a Vector
RE VE VE Append a Real to the left of a Vector
ST ST ST Concatenate two Strings
VE RE VE Append a Real to the right of a Vector
VE VE VE Concatenate two Vectors
GR GR GR Concatenate two Graphics Objects
```


```
 j(Extraction) (Priority: 6)
```
```
Left Right Result Comment
RE RE RE (no effect when the 1st component is requested)
RE VE RE (no effect when the 1st component is requested)
ST RE ST Extract the i-th component
ST VE ST Extract component range in two-vector
CM RE RE Input 1: real part, 2: imaginary part
VE RE RE Extract the i-th component
VE VE VE Extract component range in two-vector
DA RE RE Extract coefficient of 1D DA for supplied exponent
DA VE RE Extract coefficient for exponents in vector
CD RE CM Extract coefficient of 1D CD for supplied exponent
CD VE CM Extract coefficient for exponents in vector
```
```
 % (Derivation) (Priority: 7)
```
```
Left Right Result Comment
RE RE DA Diff. (i > 0 ;Result=0) or Integ. (i <0) w.r.t.xjij
CM RE CD Diff. (i > 0 ;Result=0) or Integ. (i <0) w.r.t.xjij
DA RE DA Differentiate (i >0) or Integrate (i <0) w.r.t.xjij
CD RE CD Differentiate (i >0) or Integrate (i <0) w.r.t.xjij
```
