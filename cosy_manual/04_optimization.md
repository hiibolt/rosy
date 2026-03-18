## 4 Optimization

Many design problems require the use of nonlinear optimization algorithms. COSY INFINITY supports
the use of nonlinear optimizers at its language level using the commandsFITandENDFIT(see page
36). The optimizers for this purpose are given as Fortran subroutines. For a list of currently available
optimizers, see Section 4.1. Because of a relatively simple interface, it is also possible to include new
optimizers relatively easily. Details can be found in Section 4.2.

Besides the Fortran algorithms for nonlinear optimization, COSYScript allows the user to design his
own problem-dependent optimization strategies because of the availability of the FIT command as a
language element and the ability to nest with other control elements of the COSYScript language.

### 4.1 Optimizers

TheFITandENDFITcommands of COSY allow the use of various different optimizers supplied in
Fortran. The optimizers attempt to find optimal solutions to the problem

```
fi(⃗x) = 0;
```
where⃗xis a vector ofNvvariables listed in the FIT command, and thefiareNfobjectives listed in
the ENDFIT command. For details on the syntax of the commands, including termination criteria and
control parameters for selection of algorithms, we refer to page 36.

At the present time, COSY internally supports three different optimizers with different features and
strengths and weaknesses to attempt to find optimal solutions offi= 0:In addition, there is the rather
sophisticated rigorous global optimizer COSY-GO, but this tool can currently not be called from within
the FIT - ENDFIT structure, but has as a standalone interface. In the following we present a list of the
various currently supported optimizers with a short description of their strengths and weaknesses. Each
number is followed by the optimizer it identifies.

```
1.The Simplex Algorithm
This optimizer is suitable for rather general objective functions that do not have to satisfy any
smoothness criteria. In particular, it tolerates well the use of non-smooth penalty functions, for
example to restrict the search domain. It is quite rugged and finds local (and often global) minima
in a rather large class of cases. In simple smooth cases, it often requires more execution time than
the LMDIF algorithm. However, because of its generality at reasonable execution cost, it is often
the algorithm of choice.
```
```
2.Not currently available; rerouted to \4. The LMDIF optimizer".
```
```
3.The Simulated Annealing Algorithm
This algorithm, a special type of the wide class of stochastic methods, attempts to find the global
optimum, and often succeeds even for cases where other optimizers fail. This comes at the ex-
pense of a frequently very high and sometimes prohibitive number of function evaluations. Often
this algorithm is also helpful for finding promising starting values for the subsequent use of other
algorithms.
```
```
4.The LMDIF optimizer
This optimizer is a generalized least squares Newton method with various stability enhancements,
and is very efficient in the proximity of the solution and if the objectives are smooth, but it is not
as robust as the either the simplex or simulated annealing algorithms. For most cases, it should be
the first optimizer to try.
```


It should be stressed that the success or failure of non-verified optimization tasks often rests on the clever
use of strategies combining different optimizers, random search, or structured search. The COSY approach
of offering the FIT - ENDFIT environment at the language level attempts to give the demanding user
far-reaching freedom to tailor his own optimization strategy. This can be achieved by properly nested
structures involving loops, while blocks, and if blocks in combination with the fit blocks.

### 4.2 Adding an Optimizer

COSY INFINITY has a relatively simple interface that allows the addition of other Fortran optimizers. All
optimizers that can be used in COSY must use \reverse communication". This means that the optimizer
does not control the program flow, but rather acts as an oracle which is called repeatedly. Each time
it returns a point and requests that the objective function be evaluated at this new point, after which
the optimizer is to be called again. This continues until the optimum is found, at which time a control
variable is set to a certain value.

All optimizers are interfaced to COSY INFINITY via the routine FIT at the beginning of the file
foxfit.f, which is the routine that is called from the code executor in foxy.f. The arguments for the routine
are as follows:

```
IFIT! identification number of optimizer
XV $ current array of variables
NV! number of variables
EPS! desired accuracy of function value
ITER! maximum allowed iteration number
IEND status identifier
```
The last argument, the status identifier, communicates the status of the optimization process to the
executor of COSY. As long as it is nonzero, the optimizer requests evaluation of the objective function at
the returned point XV. If it is zero, the optimum has been found up to the abilities of the optimizer, and
XV contains the point where the minimum occurs.

The subroutine FIT branches to the various supported optimizers according to the value IFIT. It also
supplies the various parameters required by the local optimizers. To include a new optimizer merely
requires to put another statement label into the computed GOTO statement and to call the routine with
the proper parameters.

We note that when writing an optimizer for reverse communication, it is very important to have the
optimizer remember the variables describing the optimization status from one call to the next. This can
be achieved using the Fortran statement SAVE. If the optimizer can return at several different positions,
it is also important to retain the information from where the return occurred.

In case the user interfaces an optimizer of his own into COSY, we would appreciate receiving a copy
of the amended file foxfit.f in order to be able to distribute the optimizer to other users as well.
