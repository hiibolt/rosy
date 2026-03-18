## 1 Before Using COSY INFINITY Contents

### 1.1 How to Avoid Reading this Manual

This manual attempts to be a sufficiently complete description of the features of COSY INFINITY; but
as such much of it will be unnecessary for many users much of the time. The following is a road map that
may allow navigation of the information most efficiently.

```
1.New users of COSY INFINITY interested in its inner workings beyond merely using tools based on
COSY INFINITY, read the remainder of this chapter.
```
```
2.New users of COSY INFINITY, install it on the system of your choice. Follow instructions in Section
1.5 on page 8.
```
```
3.COSYScript language users, note the brief syntax table in Section 3.1 on page 32.
There is a also brief guide to quickly start COSYScript programming in Appendix B on page 93,
especially for performing Beam Physics computations.
```
```
4.COSYScript language users in particular disciplines, glance at the respective demo files. Beam
Physics: demo.fox, Rigorous computing: TM.fox, Others: contact us.
```
```
5.C++ Users, refer to Section 7 on page 55.
```
```
6.F90 Users, refer to Section 8 on page 62.
```
```
7.
```
### 1.2 What is COSY INFINITY

COSY INFINITY is an environment for the use of various advanced concepts of modern scientific com-
puting. COSY INFINITY is extensively verified, and currently has more than 2000 registered users. The
COSY INFINITY system consists of the following parts.

```
1.A collection of advanced data types for various aspects of scientific computing. The data types
include
```
```
(a)DA (as well as the related CD) for differential algebraic computations [3], as well as high-order,
multivariate automatic differentiation [1] [4] [2]
(b)TM, the Taylor model [18] [16] [17] data type. Allows rigorous verified computation under
suppression of dependencies. Includes the rigorous treatment of a remainder bound over a
given domain. (This data type is not supported in the current version of COSY INFINITY.)
(c)VE, the high-performance vector data type. Provides performance advantages in environments
supporting hyperthreading, multiple cores, or shared memory parallelism based on OpenMP,
and even leads to gains on conventional systems because of favorable memory localization.
Also supported are the conventional types Real (RE), String (ST), Logical (LO), as well as a
Graphics data type (GR).
```
```
The types are highly optimized for speed and performance, including extensive support of sparsity.
For details, refer to [1] [3]. Objects are stored in a built-in dynamic memory management system
such that an entire object is always consecutive for efficient memory access.
```


```
2.Libraries for C and F77 of highly optimized common operations for the types.
```
```
3.The COSYScript environment to use these data types with the following key features:
```
```
(a)Scripting language that is compiled and executed on the fly, highly optimized for turnaround.
No need for linking, and very low interpretative overhead. Geared towards simulation, control,
and algorithm prototyping.
```
```
(b)Compactness of syntax and resulting code.
```
```
(c)Object oriented with polymorphism (dynamic typing)
(d)Local and global optimization (non-verified) built in at the language level
```
```
4.A C++ Interface making the types and operations available as a class to be used within C++ user
code
```
```
5.A F90 Interface to utilize the types and operations as a module to be used within F90 user code
```
The environment is extensively verified, and currently has more than 2700 registered users. For purposes
of maintainability, there is only one source, which is automatically cross-translated to C, and there are
tools that automatically generate the C++ and F90 class and module.

```
The COSY INFINITY system is being used for the following tasks.
```
```
1.High-order multivariate Automatic Differentiation of functions written C++, F90, and COSYScript
with support for sparsity and checkpointing.
```
```
2.Solution of ODEs, single point and flow (dependence on initial conditions), as well as DAEs, based
on COSY INFINITY's differential algebraic tools [3].
```
```
3.Arithmetic with Levi-Civita Numbers, allowing rigorous arithmetic including infinitely small and
infinitely large numbers. Support for differentials, delta functions, etc.
```
```
4.The TM.fox package for rigorous and verified computation based on Taylor models with often sig-
nificantly reduced dependency problem. (Note: Taylor models are not supported in version 10.)
```
```
5.COSY-VI [20] [6] [19], a rigorous verified integrator based on approximate differential algebraic

ows and Taylor models. (Note: Taylor models are not supported in version 10.)
```
```
6.COSY-GO, a rigorous global optimizer based on Taylor models. (Note: Taylor models are not
supported in version 10.)
```
```
7.The cosy.fox package for advanced particle beam dynamics simulations. Applications include high-
order effects in storage rings, spectrographs, electron microscopes. Supports general arrangements
of electromagnetic fields, including fringe fields, time dependent fields, and measured field data (on
surface for stability). Support for normal form analysis, symplectic tracking, rigorous long-term
stability estimates [8], and various other applications. For more details, refer to [3]. Note that some
of low and high level utility tools can be found in cosy.fox (see the Beam Physics Manual of COSY
INFINITY).
```


### 1.3 User's Agreement

COSY INFINITY can be obtained from MSU under the following conditions.

Permitted Uses: Michigan State University (\MSU") grants you, as \End User," the right to use
COSY INFINITY for non-commercial purposes only. Registered users will automatically be given access
to updates of the code as they become available. Conversely, we encourage end users to make available
tools of sufficient generality they develop for the purpose of inclusion in the master version. Any errors
detected in COSY INFINITY should be reported; comments to improve its performance are appreciated.
If the code proves useful for work that is being published, a reference is expected.

Prohibited Uses:End User may not make copies of COSY INFINITY available to others, but rather refer
them to register for their own license. End User may not distribute, rent, lease, sub-license, decompile,
disassemble, or reverse-engineer the COSY INFINITY materials provided by MSU without the prior
express written consent of MSU; or remove or obscure the Board of Trustees of MSU copyright notices or
those of its licensors. The source files are provided for purposes of compilation only and should not be
modified. We advise against modification of the provided COSYScript libraries so as to maintain a clear
upgrade path, but rather to maintain derivative code in separate files.

Intellectual Property:COSY INFINITY is a proprietary product of MSU and is protected by copyright
laws and international treaty. This Agreement is a legal contract between you, as End User, and the Board
of Trustees of MSU governing your use of COSY INFINITY. MSU retains title to COSY INFINITY. You
agree to use reasonable efforts to protect the code from unauthorized use, reproduction, distribution, or
publication. All rights not specifically granted in this License Agreement are reserved by MSU.

Warranty:MSU MAKES NO WARRANTY, EXPRESS OR IMPLIED, TO END USER OR TO ANY
OTHER PERSON OR ENTITY. SPECIFICALLY, MSU MAKES NO WARRANTY OF FITNESS FOR
A PARTICULAR PURPOSE OF COSY INFINITY. MSU WILL NOT BE LIABLE FOR SPECIAL,
INCIDENTAL, CONSEQUENTIAL, INDIRECT OR OTHER SIMILAR DAMAGES, EVEN IF MSU
OR ITS EMPLOYEES HAVE BEEN ADVISED OF THE POSSIBILITY OF SUCH DAMAGES. IN NO
EVENT WILL MSU LIABILITY FOR ANY DAMAGES TO END USER OR ANY PERSON EVER
EXCEED THE FEE PAID FOR THE LICENSE TO USE THE SOFTWARE, REGARDLESS OF THE
FORM OF THE CLAIM.

### 1.4 How to Obtain Help and to Give Feedback

While this manual is intended to describe the use of the code as completely as possible, there will probably
arise questions that this manual cannot answer. Furthermore, we encourage users to contact us with any
suggestions, criticism, praise, or other feedback they may have. We also appreciate receiving COSY source
code for utilities users have written and find helpful. We can be contacted at support@cosyinfinity.org.

### 1.5 How to Install the Code

All the system files, manuals, and installation packages of COSY INFINITY are currently distributed
from the COSY INFINITY web site

```
cosyinfinity.org
```
Installation packages compiled with the Intel Fortran Compiler for Microsoft Windows PC, for macOS,
and for Linux are available. Please refer to Sections 1.5.1 and 1.5.2 for the details.


Instead of using the executables in the above installation packages, there may be some situations
necessary to install COSY INFINITY by compiling the COSY INFINITY Fortran source files. Typically,
such situations happen for Linux systems and UNIX-like systems. The instructions are provided for typical
Linux/UNIX-like installations in Section 1.5.5, and for parallel environments in Section 1.5.8. First please
review Section 1.5.3 that describes the COSY INFINITY Fortran source files and Fortran compilers.

The code for COSY INFINITY consists of the macro source files, which depend on the particular
application of COSY INFINITY. For use in Beam Physics, the code cosy.fox is needed. For applications to
rigorous computing, other files are needed. The respective macro files are written in COSY's own language
COSYScript and have to be compiled by the local executable program of COSY INFINITY as part of the
installation process. Please see Section 1.7 for running COSY INFINITY to compile COSYScript macro
source files.

#### 1.5.1 Installation Package for Microsoft Windows PC

An optimized 64-bit executable program for Microsoft Windows PC produced by the Intel Fortran Com-
piler is available, and its usage is recommended compared to user Fortran compilation. COSY INFINITY
has been verified on all recent flavors of Windows.

To install COSY INFINITY for Windows, download and run the COSY INFINITY installer package
for Microsoft Windows,wincosy10.2.exe. Ahead of running the COSY INFINITY installer, please make
sure that Java is installed with version 8 or higher. You can download Java free at

```
https://www.java.com
```
The file association of.jarhas to be assigned tojavaw.exeahead of time. To check it, double click a
.jarfile to see if Java starts. For example, tryCOSYGUI.jarthat is available at the COSY INFINITY
download site. This file is the same one that is to be included in the installation folder set up by the
installer. If Java does not start due to some existing file association con
icts, which typically is caused by
some zip/unzip programs such as WinRAR, the file association assignment has to be fixed. You may do
this manually, or reinstall Java, or use a fixing tool such as Jarfix (developed and provided by Johann N.
Lofflmann).

COSY INFINITY will be installed to the folder that you specify, by default called \COSY 10.2".
The folder contains the executable programcosy.exeand the platform independent Java COSY GUI
(graphical user interface) driver program fileCOSYGUI.jar; see Section 1.7.5 on page 25 to utilize the Java
COSY GUI features. The installer will also associate all COSYScript files with the \.fox" extension in
your system to allow running COSY INFINITY with the Java COSY GUI driver. When the installation
finishes, please restart your Windows PC to activate the changes in your system.

Notes

```
While the COSY executable allocates less than 2GB of memory, experience shows that one should
increase the size of the page file to at least the size of the physically available memory in order to
avoid unexpected program terminations. If you see an error message like \Windows could not run
the executable," this is most likely because the size of the page file is too small. Please refer to
Windows documentation or your system administrator for information on how to increase the size
of the page file.
```
```
Earlier COSY INFINITY installer packages for Microsoft Windows had included the GrWin graphics
driver as an interactive graphics driver. However, GrWin is not included anymore, because GrWin
now requires a license obtained by each end user with a license key installed on each individual
```


```
Windows PC. If a usage of GrWin is desired, please refer to Section 1.5.7 for the instructions
to install COSY INFINITY with linking to GrWin. Please also note, starting from version 10.
of COSY INFINITY, the platform independent Java COSY GUI driver offers outputting COSY
graphics interactively, so GrWin is not needed anymore.
```
```
Different versions of COSY INFINITY may be installed on the same Windows PC, though the
latest version should be used for execution. The Uninstaller included in the COSY INFINITY
installation package will erase the installed files and registry entries, but some items may remain,
especially so with evolving Windows flavors. To uninstall a version of COSY INFINITY more
thoroughly, we recommend to use one of the uninstaller programs available for Windows PCs, such
as Revo Uninstaller, which can clean up remaining items better than the Windows default method
to uninstall a program (app) from your Windows PC.
```
Running COSY INFINITY

There are several ways to run the COSY INFINITY system program. The installer sets up a few convenient
ways that offer the Java COSY GUI environment. See Section 1.7 for more information.

```
Double click any COSYScript file with the file extension \.fox", or
Right click any COSYScript file with the file extension \.fox", and select \Run" (or \Run with
COSY INFINITY" on earlier Windows), or
```
```
From the Start Menu, start the App (or program) by typing \Run COSY 10.2", which opens a
window to \Select FOX file to run". Use this window to navigate to specify a COSYScript file with
the file extension \.fox".
[To use this feature, at the time of the installation, please do not mark the \Do not create shortcuts"
button in the \Choose Start Menu Folder" page.]
```
For Beam Physics computations, make sure to execute the COSYScript filecosy.foxfirst so that the
binary fileCOSY.binis created in the work folder. See Section 1.7.6 on page 26 for more details on Beam
Physics computations.

Running COSY INFINITY from the Command Line

COSY INFINITY can be run from the command line. Please refer to Section 1.7 for more information.

WSL Terminal:If your PC has WSL (Windows Subsystem for Linux) set up, the simplest way is
first copyingcosy.exeandCOSYGUI.jarfrom the installation folder to your work folder. To execute a
COSYScript file, for exampleguidemo.foxavailable at the COSY INFINITY download site, utilizing the
Java COSY GUI features, type as follows in the terminal window.

```
java -jar COSYGUI.jar guidemo.fox
```
If the Java features are not needed, typing

```
./cosy.exe
```
will start the COSY INFINITY system program.

For Beam Physics computations, make sure to execute the COSYScript filecosy.foxfirst so that the
binary fileCOSY.binis created in the work folder. See Section 1.7.6 on page 26 for more details on Beam
Physics computations.


Command Prompt:A crude way to run COSY INFINITY on a Windows PC is to type a command
line in the command prompt of Windows. The simplest way is first copyingcosy.exeandCOSYGUI.jar
from the installation folder to your work folder. Start the command prompt from the start menu, then
change from the current folder to the work folder where you have COSYScript files with the file extension
\.fox". To execute a COSYScript file, for example guidemo.foxavailable at the COSY INFINITY
download site, utilizing the Java COSY GUI features, type as follows in the command prompt window.

```
java -jar COSYGUI.jar guidemo.fox
```
If the Java features are not needed, typing

```
cosy
```
(orcosy.exe) will start the COSY INFINITY system program.

For Beam Physics computations, make sure to execute the COSYScript filecosy.foxfirst so that the
binary fileCOSY.binis created in the work folder. See Section 1.7.6 on page 26 for more details on Beam
Physics computations.

#### 1.5.2 Linux/UNIX-like Systems and macOS

Optimized executable programs for Linux and macOS are available at the COSY INFINITY download
site. Download a package that may best fit to your system. A downloadable package typically contains
only one file \cosy", a COSY INFINITY executable program, and it is compressed typically into a tar-gzip
file with the name likelincosy102.tgz. If your system has a graphical file viewer, you may extract
the executable file \cosy" by double-clicking the package file. Alternatively you can type the following
command in the terminal, after moving to the directory/folder where you want to have the file \cosy"
extracted.

```
tar -xf lincosy102.tgz
```
Once you have the executable file \cosy" in the directory/folder, please check first if it runs in the terminal
mode. For this, have an example COSYScript program file in the same directory/folder. If this is your first
time running COSY INFINITY, please use a small example program filebriefdemo.foxthat is available
at the COSY INFINITY download site.

In the directory/folder where you have the executable program \cosy" and an example COSYScript
program file, type the following command in the terminal

```
./cosy
```
or, depending on the settings of your system,

```
cosy
```
If you see the error message \Permission denied" here, you need to add \execute" mode to the permis-
sions of the file \cosy". This can be done by the following command in the terminal, then try the above
action again.

```
chmod +x cosy
```
When the COSY INFINITY system program properly starts, the terminal screen displays the title of the
COSY INFINITY system, and it asks you for a file name with extension.fox.

```
GIVE SOURCE FILE NAME WITHOUT EXTENSION .FOX
```


At this point, you can terminate the executable program if you know how to do so safely. Otherwise
you type \briefdemo" (without the quotation marks, just nine characters). The programbriefdemo.fox
displays some lines and waits for your response; press the enter key to respond until the COSY INFINITY
system program ends.

If all you see when running the executable program \cosy" is the message \killed", this indicates
that you do not have enough memory available to run this COSY INFINITY executable program. COSY
INFINITY requires 2GB of system memory to be available to run successfully. Please increase the amount
of memory available to COSY INFINITY to run the executable program. Please refer to your system's
documentation or your system administrator for information on how to increase the available memory.

When the downloaded executable program is not compatible with your local systems, it is straight-
forward to install COSY INFINITY by compiling the COSY INFINITY Fortran source files. It also may
be possible to adjust your system environments to run the downloaded executable program if the cause
is something to do with software settings. To install COSY INFINITY by Fortran compiling, please
first review Section 1.5.3 about the COSY INFINITY Fortran source files and Fortran compilers, and
see Section 1.5.5 for the instructions. There are some other cases when the user needs to install COSY
INFINITY by Fortran compiling. For parallel environments, please see Section 1.5.8 for installing COSY
INFINITY. When linking to an additional interactive graphics library such as PGPLOT and AquaTerm is
desired, please review Section 5.2 about COSY graphics drivers, and see Section 1.5.6 for the instructions
of installing linked with PGPLOT. Please note, however, starting from version 10.2 of COSY INFINITY,
the platform independent Java COSY GUI driver offers outputting COSY graphics interactively, so an
additional interactive graphics driver is typically not needed.

To utilize the Java COSY GUI (graphical user interface) features, please make sure that Java is installed
with version 8 or higher. You can check if Java is installed on your local system and the version number
by typing the following command in the terminal.

```
java -help
```
You can download Java free at

```
https://www.java.com
```
Then, please download the platform independent Java COSY GUI driver program fileCOSYGUI.jarthat
is available at the COSY INFINITY download site. The interactive COSY graphics output feature of the
COSY GUI Java package is implemented for version 10.2 of COSY INFINITY, so anyCOSYGUI.jarfile
you may have from earlier versions of COSY INFINITY does not support the interactive COSY graphics
output feature. The latestCOSYGUI.jarfile is backward compatible, so any earlier user COSYScript file
written with COSY GUI features runs. Refer to Section 1.7.5 on page 25 to utilize the Java COSY GUI
features.

After checking that the executable program \cosy" runs in the terminal mode on your computer, you
may want to customize the setup of running COSY INFINITY. Path settings and file associations are
typical topics to be customized. Since they are local system dependent, please refer to your system's
documentation or your system administrator for information. Please refer to Section 1.7 on page 23 for
running COSY INFINITY for details and various ways.

Running COSY INFINITY in the Terminal

To utilize the Java COSY GUI features, the simplest way is to first place the executable programcosy
andCOSYGUI.jarin your work directory/folder. To execute a COSYScript file, for exampleguidemo.fox
available at the COSY INFINITY download site, type as follows in the terminal.


```
java -jar COSYGUI.jar guidemo.fox
```
If the Java features are not needed, typing

```
./cosy
```
or, depending on the settings of your system,

```
cosy
```
will start the COSY INFINITY system program.

For Beam Physics computations, make sure to execute the COSYScript filecosy.foxfirst so that the
binary fileCOSY.binis created in the work directory/folder. See Section 1.7.6 on page 26 for more details
on Beam Physics computations.

Running COSY INFINITY Using a Graphical File Viewer

When the file associations are properly set, a COSYScript file could run from a graphical file viewer.

```
Double clickCOSYGUI.jar, and in an opened file choosing widow, select a COSYScript file with the
file extension \.fox", or
```
```
Double click any COSYScript file with the file extension \.fox".
```
For Beam Physics computations, make sure to execute the COSYScript filecosy.foxfirst so that the
binary fileCOSY.binis created in the work directory/folder. See Section 1.7.6 on page 26 for more details
on Beam Physics computations.

#### 1.5.3 Source Files

If it is necessary to install a COSY INFINITY executable program by yourself without using the installation
packages explained in the previous subsections, the Fortran source files and some installation support files
such asMakefileare available at the COSY INFINITY download site.

Fortran Source Files

```
foxy.f
```
```
dafox.f
```
```
foxfit.f
```
```
foxgraf.f
```
The four files foxy.f, dafox.f, foxfit.f and foxgraf.f are written in standard Fortran 77 and have to be
compiled and linked.foxy.fis the compiler and executor of COSYScript.dafox.fcontains the routines
to perform operations with objects, in particular the differential algebraic routines.foxfit.fcontains the
package of nonlinear optimizers.foxgraf.fcontains the available graphics output drivers, which are listed
in Section 5.2. The foxgraf.f file available at the COSY INFINITY download site is prepared without
linking to PGPLOT, GrWin, AquaTerm libraries. If local PGPLOT, GrWin, AquaTerm libraries are
available, the desired libraries can be linked after modifying the source file foxgraf.f. See Section 1.5.


for modifying foxgraf.f, and see Section 5.2 regarding the graphics output drivers. Please note, however,
starting from version 10.2 of COSY INFINITY, the platform independent Java COSY GUI driver offers
outputting COSY graphics interactively, so an additional interactive graphics driver is typically not needed.

All the Fortran parts of COSY INFINITY are written in standard ANSI Fortran 77. However, certain
aspects are platform dependent; in particular, this concerns command line handling and the system time
measurement. The following compilers have been verified recently for compatibility with the COSY
INFINITY system.

```
Intel Fortran Compiler [14] (ifort, under Intel oneAPI) for Microsoft Windows, Linux, and macOS
```
```
This is our recommendation for Intel-based computers. Intel oneAPI is available for free as of
2023. We recommend to use the compiler option-fp-model strictfor value-safe floating-point
handling. For rigorous computations, this is required. SeeMakefileavailable at the COSY INFIN-
ITY download site.
```
```
GNU Fortran Compiler [12] (gfortran, under GCC, the GNU Compiler Collection [13]) for Linux/UNIX,
macOS, UNIX-like systems under Microsoft Windows such as WSL (Windows Subsystem for Linux)
and Cygwin
```
```
The compiler option-std=legacyis needed with the recent versions of GNU Fortran Compiler.
It is advised to check the documentation of the GNU Fortran Compiler about platform specific
options. In general, compiler optimization options are not recommended for the GNU Fortran
Compiler, because it sometimes causes inconsistent results as discussed in the next paragraph. See
Makefileavailable at the COSY INFINITY download site.
```
In general, default compiler optimization is recommended. According to our experiences and studies
related to speed and reliability particularly for verified computations [15], we have been recommending to
use the Intel Fortran Compiler rather than GNU Fortran Compilers. Recently conducted studies in 2023
show that the computational speed performance of the GNU Fortran generated code with-O1optimization
option is about three times faster than the one with no optimization, and the optimization option-O2and
higher does not gain much additional speed. This is widely observed over different platforms. As for the
computational result, it tends to get different with higher optimization level, and how much it gets different
depends on platform; in short, the computational reliability is relatively low. On Intel-based computers
where both Intel Fortran Compiler and GNU Fortran Compiler exist, the computational speed by the
GNU Fortran-O1optimization generated code is comparable to that of the Intel Fortran generated code
with default optimization. The Intel Fortran generated code with option-fp-model strictproduces
consistent result over different platforms. Note that such a compiler option does not exist for the GNU
Fortran Compiler, and it is because the accuracy of floating-point arithmetic is \unknown" for GCC [13].
Please refer to the report [15] for more ideas on suitable compiler options.

Should there be additional problems, a short message to us would be appreciated in order to facilitate
life for future users on the same system.

#### 1.5.4 Conversion of a Source File Using VERSION

There are some situations when some of COSY INFINITY Fortran source files have to be adjusted for
specific purposes, for example linking to the PGPLOT graphics library, or to the GrWin graphics package,
or installing the MPI version of COSY INFINITY for parallel computations. The necessary conversion
can be accomplished using the small program VERSION.


First, install the program VERSION using the Fortran source fileversion.f, which is available at the
COSY INFINITY download site. In Linux/UNIX, the following command will install VERSION using
the Intel Fortran:

```
ifort version.f -o VERSION
```
Example of VERSION to Convert foxgraf.f for PGPLOT:

This example shows how to convert the standardfoxgraf.ffile downloaded from the COSY INFINITY
download site to the PGPLOT linking version. In the terminal (shell, console) window, start the program
VERSION by typing \version", and supply the following as the program prompts for your input.

```
the original file name \foxgraf.f"
```
```
the new file name as a result of VERSION conversion (any name is OK, below foxgrafPGP.f is given
as a mere example)
```
```
the current ID name (nothing, because the file foxgraf.f is the original standard version)
the new/target ID name for the conversion \*PGP"
```
Below, you see what is displayed in the console screen. When the conversion is successfully completed,
the program ends with the message, \The VERSION change finished."

##### ****************************************************

##### * *

##### * UTILITY PROGRAM VERSION *

##### * *

```
* This program changes the type of machine/system. *
* The current COSY INFINITY system supports *
* NORM MPI FACE RND and PGP GRW AQT. *
* See the User's Guide and Reference Manual. *
* *
****************************************************
```
GIVE OLD FILENAME:
foxgraf.f

GIVE NEW FILENAME:
foxgrafPGP.f

```
SPECIFY ID OF CURRENT VERSION (MUST START WITH * OR C):
Examples: *PGP *GRW *AQT, and *NORM *MPI *FACE *RND
```
##### SPECIFY ID OF NEW VERSION (MUST START WITH * OR C):

##### *PGP

```
The VERSION change finished.
```


#### 1.5.5 Installation by Fortran Compilation

To install the C++ and the F90 Interface Packages, please refer to Section 7 and Section 8. Below, we
describe the procedures how to install COSY INFINITY by compiling the Fortran source files. Typi-
cally such cases may arise for Linux systems and UNIX-like systems, so the descriptions below assume
Linux/UNIX-like systems unless explicitly mentioned. Depending on the local platform, some details will
have to be adjusted.

Should there be any difficulties, we would appreciate hearing about them for a verification of the
master version. Should you plan to install COSY INFINITY system programs on yet another system
which requires changes, please send us a complete description about the changes for inclusion in the
master version.

Compiling COSY INFINITY without Linking to Special Packages such as PGPLOT

The four Fortran source files

```
foxy.f,dafox.f,foxfit.f,foxgraf.f
```
mentioned in Section 1.5.3 have to compiled and linked. A makefile \Makefile" for the Intel Fortran
compiler is available at the COSY INFINITY download site. When the executable programcosyis
successfully produced by themakeprocess, proceed to Section 1.7 for running COSY INFINITY.

Compiling COSY INFINITY with PGPLOT Linked

See the next section 1.5.6 about the graphics library PGPLOT, and have the PGPLOT library prepared.
The procedures to compile and link COSY INFINITY Fortran source files with PGPLOT below assume
that the X Window System (X-Windows,X11) is available on your local UNIX based machine, to which
PGPLOT graphics is going to be output.

```
1.Conversion of foxgraf.f: The standardfoxgraf.ffile, as downloaded from the COSY INFINITY
download site, is prepared without linking to PGPLOT. So, the filefoxgraf.fhas to be modified
using the program VERSION; please follow the instructions in Section 1.5.4.
```
```
2.Compiling COSY INFINITY Fortran source files with PGPLOT:Modify the makefile
\Makefile" available at the COSY INFINITY download site to activate the \LIBS=" description to
use PGPLOT.
```
When the executable programcosyis successfully produced by themakeprocess, proceed to Section
1.7 for running COSY INFINITY. The Beam Physics demo programdemo.fox, available at the COSY
INFINITY download site, is a good test case to check if the PGPLOT interactive graphics output works
well. Just before runningdemo.fox, the COSY Beam Physics library programcosy.foxhas to be run,
and the data fileSYSCA.DAThas to be placed in the executing directory; See Section 1.7.6 on page 26 for
running COSY INFINITY forcosy.foxanddemo.fox.

Compiling COSY INFINITY with GrWin Linked on Windows 10 PC

See Section 1.5.7 for the detailed instructions.


#### 1.5.6 Preparation of the PGPLOT Library

The PGPLOT Graphics Subroutine Library is a graphics package copyrighted by California Institute of
Technology, and is written mostly in standard Fortran 77. As of July 2013, the latest release of PGPLOT
is Version 5.2.2 of February 2001. Some Linux systems have a pre-installed PGPLOT library; in such a
case the installation of COSY INFINITY linking to PGPLOT is fairly easy. If it is not the case, one needs
to install the PGPLOT library using a Fortran compiler ahead of compiling and linking COSY INFINITY
Fortran source files. Even though this adds an extra step in the task of installing COSY INFINITY, due to
the high quality interactive graphics outputs, namely the crisp appearance and quick response, we consider
it worth the extra effort when no other interactive graphics packages such as GrWin and AquaTerm are
readily available. When there is no need to produce interactive graphics outputs for a particular machine,
the user may not want to be bothered to link PGPLOT to a COSY INFINITY executable program on
the machine, as there are various other graphics output alternatives in COSY INFINITY. Please refer to
Section 5.2 for other graphics output options; in particular, the PDF and the PS graphics drivers offer
high quality graphics output by producing small size files, and doing it fast.

Using a Pre-Installed PGPLOT Library

Some Linux systems have a pre-installed PGPLOT library. The availability seems to differ from time to
time and depends on each platform.

According to the information supplied by Ravi Jagasia and Alexander Wittig in 2009, on Ubuntu one
can check if a pre-installed PGPLOT library exists in your machine as follows. Using the Synaptic Package
Manager located under \System!Administration", search for PGPLOT to find the packagepgplot5as
a package either installed or to be installed. Alternatively, one can use the command:

```
sudo apt-get install pgplot
```
In either case, you will need root access. This will provide the library file/usr/lib/libpgplot.a.

If a PGPLOT library is not pre-installed in your machine, you may want to search it on the web with
keywords \pgplot5", \download", and a suitable name of the Linux flavor.

Please see item9)below in \PGPLOT Library Installation" for some necessary environment ad-
justments. Then, follow the instructions in Section 1.5.5, \Compiling COSY INFINITY with PGPLOT
Linked".

Compiling PGPLOT Source Files for a PGPLOT Library

The PGPLOT source package (pgplot5.2.tar.gz) is available at

```
https://www.astro.caltech.edu/~tjp/pgplot/
```
Please follow the information and the instructions provided there, especially \installation instructions"
for \UNIX (all varieties)" available currently as of 2020 at

```
https://www.astro.caltech.edu/~tjp/pgplot/install.html
```
which is the same instructions written in the fileinstall-unix.txtthat is included in the PGPLOT
source package. Even though some topics are outdated since the instructions are as of 1997, the instructions
supplied by the original PGPLOT distributor are basic.


Please refer to the short summary \PGPLOT Library Installation" below for the installation instruc-
tions and some necessary adjustments. When the PGPLOT library is successfully created, please proceed
to Section 1.5.5, \Compiling COSY INFINITY with PGPLOT Linked". If it cannot be accomplished,
it is still possible to link PGPLOT to COSY by compiling necessary PGPLOT source files and directly
linking together with COSY's compiled objective files. Please see \Compiling and Linking PGPLOT to
COSY Without Creating a PGPLOT Library" below (page 20).

PGPLOT Library Installation

This is a short summary on how to install the PGPLOT library on Linux. For the simplicity, specific names
are given below for the directories, which you may adjust depending on your local situation. Performing
the operations below as root (super-user,su) will simplify the task.

1) Download the PGPLOT source packagepgplot5.2.tar.gzfrom the web site of the PGPLOT dis-
tributor at

```
https://www.astro.caltech.edu/~tjp/pgplot/
```
If the above site is not reachable, the package may possibly be obtained from us.

2) Create a directory for the final PGPLOT library storage.

```
mkdir /usr/local/pgplot
```
Also, prepare a directory for the PGPLOT distribution source storage/usr/local/src/pgplot/. If the
directory/usr/local/src/does not exist, create it.

```
mkdir /usr/local/src
```
3) Unpack the PGPLOT source packagepgplot5.2.tar.gzin the directory/usr/local/src/so that
the contents are stored in the directory/usr/local/src/pgplot/. Some modern unpacking programs
may do this easily. Otherwise, type the following commands. The options \-xvzf" in thetarcommand
may need to be given as \xvzf" without \-" depending on your local system.

```
cp pgplot5.2.tar.gz /usr/local/src
cd /usr/local/src
tar -xvzf pgplot5.2.tar.gz
```
4) Copy the filedrivers.listfrom/usr/local/src/pgplot/to/usr/local/pgplot/, and edit the
file in/usr/local/pgplot/. Remove comments for all the necessary devices; choose Color PS (four of
PSDRIV), and X Windows (two ofXWDRIV) in addition toNULL(NUDRIV) of the default.

5) In/usr/local/pgplot/, create a makefile by typing the command for the \makemake" program as
follows.

```
../src/pgplot/makemake /usr/local/src/pgplot linux g77gcc
```
This creates the filemakefileforg77andgccin the directory/usr/local/pgplot/. The filemakefile
has to be modified to be used for the Intel Fortranifortor newer GNU Fortran likegfortranrather


thang77. As a general rule, the Fortran compiler to be used for the process described in Section 1.5.
should be used here.

6) Edit the filemakefileof the previous step5). The following instructions are based on the information
supplied by Markus Neher in 2009 and the PGPLOT installation guide filepgplot quick.txtwritten
by the team developing LORENE and available at

```
https://lorene.obspm.fr/prerequisites.html
```
under the topic regarding PGPLOT. Markus Neher had installed PGPLOT for COSY INFINITY on a
64bit SuSE 11.0 platform, testing to use bothifortandgfortran(gcc 4.3).

```
6.1) When usingifort, replace \FCOMPL=g77" in line 25 by \FCOMPL=ifort". Go to6.3).
```
6.2) When usinggfortran, replace \FCOMPL=g77" in line 25 by \FCOMPL=gfortran", and also replace
\FFLAGC=-u -Wall -fPIC -O" in line 26 by

```
\FFLAGC=-ffixed-form -ffixed-line-length-none -u -Wall -fPIC -O".
```
6.3) The information here is supplied by M. Neher, and further supplemented by Ravi Jagasia and
Alexander Wittig in 2009; some of the details may need to be adjusted to the specifics of your system.

Instead of linking PGPLOT to the sharedf2clibrary, PGPLOT must be linked to the static library.
Assuming that the static librarylibf2c.ais located in the directory/usr/lib64/, replace \-lf2c" in
lines 48-51 by \/usr/lib64/libf2c.a". If you are using a 32 bit system, you should locate the file in
/usr/lib32/or/usr/lib/. In some cases, you can opt to not change this line and instead install the
packagelibf2c2-devwith the package manager.

7) Type \make" in the directory/usr/local/pgplot/.

In the end, only the next four files have to be in the directory/usr/local/pgplot/. Even if themake
process does not complete according to the descriptions in the filemakefile, as far as these four files are
created, they are suffice for PGPLOT to be linked to COSY INFINITY.

```
libpgplot.a
```
```
grfont.dat
```
```
rgb.txt
```
```
pgxwin server(orpgxwin server.exe)
```
If \make" doesn't work to create some of the files above, try \make libpgplot.a" etc. individually.
rgb.txtexists in the directory/usr/local/pgplot/before executing \make".

R. Jagasia and A. Wittig commented for some cases such as compiling PGPLOT in Ubuntu in 2009:
Some additional packages may be needed, which are not installed by default, for example the package
libx11-dev; these can be installed via the package manager.

8) Clean up the directories by typing \make clean" in the directory/usr/local/pgplot/, and further
delete all unnecessary files.


9) Set the environment parameters. This differs a lot depending on the system. In general, each end
user has to make the necessary adjustments.

a)bashshell { this is to be added in~/.bashrc

export PGPLOTDIR="/usr/local/pgplot"
export LDLIBRARY PATH="/usr/local/pgplot":$LD LIBRARYPATH

b) CygWin

Add/usr/local/pgplotto thePATHlist, for example in the file/etc/profile.

Compiling and Linking PGPLOT to COSY Without Creating a PGPLOT Library

When a PGPLOT library cannot be created by compiling the PGPLOT source files, it is still possible to
compile PGPLOT source files to be linked directly together with objective files of COSY's Fortran source
files. This approach is based on a suggestion made by Shashikant Manikonda in 2006. Most of the steps
described above, \PGPLOT Library Installation", apply here, though there is no need to operate as root
(super-user,su) and specific directory names are not necessarily to be used.

Follow the above steps1)through5), though you don't have to use the same directory names. Compile
the following PGPLOT source files, then link the resulting objective files together with the objective files
of Fortran source files of COSY INFINITY.

```
PGPLOT source files to be compiled and linked
```
```
{All the Fortran source files in the directorypgplot/src/. Note that there are two include files
in the directory.
{All the Fortran source files in the directorypgplot/sys/
{Two C source filesgrdate.candgruser.cin the directorypgplot/sys/
{Two Fortran source filesnudriv.fandpsdriv.fin the directorypgplot/drivers/
{One C source filexwdriv.cin the directorypgplot/drivers/
{A Fortran source filegrexec.fin the directory that executes \makemake" in the step5), and
this file is a result of \makemake"
```
```
COSY INFINITY Fortran source files to be compiled and linked
foxy.f,dafox.f,foxfit.f,foxgraf.f
foxgraf.fhas to be converted for PGPLOT using the program VERSION following the instructions
in Section 1.5.4.
```
```
Other PGPLOT files necessary to have in the resultingcosyexecuting directory
```
```
{The ASCII database filergb.txtin the directorypgplot
{The binary PGPLOT font filegrfont.datas a result of \make" in the step6). See \makefile"
for PGPLOT. It may be possible to obtain the file from some other machines.
{pgxwin serverin the step7)may be needed. This can be created as a result of \make" in the
step6).
```


```
Compiler options and linking
Please see step6)and the descriptions in \makefile" for PGPLOT and in \Makefile" for COSY
INFINITY. In the \LIBS=" description in the COSY makefile \Makefile", the \pgplot" items are
not needed, but the \X11" items have to be kept.
```
#### 1.5.7 Compiling COSY INFINITY with GrWin Linked on Windows PC

GrWin is a graphics package for Microsoft Windows, and the package is available for download at

```
https://spdg1.sci.shizuoka.ac.jp/grwin/en
```
As of 2020, the current release GrWin Version 1.1.1 consists of the GrWin server and the GrWin library.
The GrWin server is a graphics window program to display GrWin graphics output contents, and the
GrWin library is a collection of graphics utilities. Currently the GrWin server requires to obtain a license
to install and run on each Windows PC; refer to the GrWin web site for the up-to-date details; a free
license can be obtained for non-commercial use.

It is possible to link the GrWin graphics library to create an executable program of COSY INFINITY
that displays graphics output interactively to the GrWin graphics window program. To create such an
executable, the matching GrWin license key has to be included during the compiling and linking process
of the Fortran source files of COSY INFINITY.

```
The following detailed instructions are based on the GrWin package as of 2020.
```
```
1.Installation of the GrWin server:
```
```
(a)Download the GrWin server installer from the GrWin web site, and install it. As of 2020,
a free license including a one-day trial license can be obtained instantaneously through the
installation wizard. Once the license expires, you may need to uninstall the GrWin server
before re-installing it.
(b)Upon a successful installation, the GrWin server program executable filegrwin.exeand the
license key source filegrkey.cwill be found in the GrWin installation folder. Note that a new
license key source file is produced together with a new GrWin server program executable for
each new GrWin server installation.
```
```
2.Preparation of COSY INFINITY Fortran source files:
```
```
(a)Download four Fortran source files of COSY INFINITY from the COSY INFINITY download
site as mentioned in Section 1.5.3:
foxy.f,dafox.f,foxfit.f,foxgraf.f
(b)The standardfoxgraf.ffile as downloaded is prepared without linking to GrWin. So, the file
foxgraf.fhas to be modified using the program VERSION. Please follow the instructions in
Section 1.5.4 with the new/target ID name for the conversion \*GRW".
```
```
3.Linking to a pre-compiled GrWin library:
```
```
(a)Pre-compiled GrWin libraries for various environments are available at the GrWin web site.
Download a suitable GrWin library installer package from the GrWin web site, and install it.
Note that, without the GrWin server successfully installed ahead of time, a GrWin library
installer will not install.
(b)It would be useful to try to create executables of some of GrWin example demo programs,
particularly those with a Fortran source file, and check if they run on your Windows PC to
display the GrWin graphics output contents to the GrWin server program.
```


```
(c)If the GrWin library is successful to create a functioning GrWin example demo program ex-
ecutable, the same environment can be used to create an executable of COSY INFINITY by
replacing the GrWin example demo program source file with the four COSY INFINITY Fortran
source files.
(d)If it is unsuccessful, any other pre-compiled GrWin library installer packages are likely to be
unsuccessful as well. Proceed to the next step.
```
```
4.If a pre-compiled GrWin library does not work, compile and link GrWin source files with the COSY
INFINITY Fortran source files:
```
```
(a)Download the GrWin ToolKit package from the GrWin web site, which contains c source files
of the GrWin library for GrWin graphics utilities.
(b)Prepare an environment that can compile and link Fortran source files and c source files to
create an executable. The explanation here uses the command prompt window of the Intel
Fortran compiler (64-bit) for Windows under Microsoft Visual Studio. A makefile \Makefile"
is available at the COSY INFINITY download site, which uses commands \ifort" (the Intel
Fortran compiler), \cl" (the c compiler of Microsoft Visual Studio), \link" (the linker of
Microsoft Visual Studio). In the Intel Fortran command prompt window, move to the work
directory that has all the necessary source files together with the makefile. Then type the
command for make as \nmake". Please adjust the makefile as needed for your environment.
(c)Have the four Fortran source files of COSY INFINITY as described above at 2.
(d)Have c source files of the GrWin library that are needed to link withfoxgraf.fof COSY
INFINITY. Copy the following GrWin source files from the GrWin package. Unless noted, the
files can be found in thesrc/directory of the GrWin ToolKit.
Brush.c, Core.c, Pen.c, RGB.c, Text.c, Tools.c,
CheckUI.c, Lib.c, LowLevel.c, Misc.c,
gwkey.c(in the GrWin server installation folder mentioned above at 1b { important!)
Globals.h, GrWinAll.h, gw.h, Messages.h,
grwin.h, Version.h (in the main directoryGrWinTk/of the GrWin ToolKit)
```
When the executable programcosy.exeis successfully produced, proceed to Section 1.7 for running COSY
INFINITY. The Beam Physics demo programdemo.fox, available at the COSY INFINITY download
site, is a good test case to check if the GrWin interactive graphics output works well. Just before running
demo.fox, the COSY Beam Physics library programcosy.foxhas to be run, and the data fileSYSCA.DAT
has to be placed in the executing folder; See Section 1.7.6 on page 26 for running COSY INFINITY for
cosy.foxanddemo.fox.

#### 1.5.8 Installation for Parallel Environments

COSY INFINITY provides native routines that interface with MPI for parallel processing. This is useful
for machines with multiple cores, or for computation on clusters. At this point, COSY INFINITY has
been successfully run on up to 2048 processors on the NERSC cluster in Berkeley, as well as various smaller
clusters at ANL and MSU.

There are different machine and cluster specific commands that can be run, but we will reference
OpenMPI calls. The user can use appropriate commands to replace their functionality.

```
For the MPI version of COSY INFINITY, prepare the four Fortran source files
```
```
foxy.f, dafox.f, foxfit.f, foxgraf.f
```


(see Section 1.5.3) as follows. Download the standard COSY INFINITY Fortran source files from the
COSY INFINITY download site. The MPI supports have to be activated by converting these files to the
MPI version.

The files foxy.f and dafox.f must be converted from*NORMto*MPIusing VERSION, while foxgraf.f
and foxfit.f can remain the same. See Section 1.5.4 on how to use VERSION. Specify*NORMand*MPI
as the current ID and the new ID, then VERSION un-comments all the lines that contain the string*MPI
in columns 1 to 4, and comments all the lines containing the string*NORMin columns 73 to 80. The
conversion of the files can be done on any machine. If done on a local machine, transfer the converted
files to the cluster machine.

On the cluster machine, compile the four Fortran source files with the appropriate compiler options.
This should be done with the compiler wrapper function \mpif77" which we recommend having made with
the Intel Fortran Compiler. If you plan to perform verified computations, we recommend you to contact
us first before proceeding. To compile to obtain an MPI version of COSY INFINITY executable program,
mpif77can be used in the Makefile as the Fortran compiler instead of the usual Fortran compiler.

When the executable programcosyis successfully produced by themakeprocess, proceed to Section
1.7 and Section 1.7.4 for running COSY INFINITY.

### 1.6 Memory Usage and Limitations

COSY INFINITY is written in such a way that with modern compilers, including those used for the
downloadable Windows, memory is allocated dynamically as needed, up to a certain maximum. At start-
up, COSY INFINITY requires approximately 200MB of physical memory, and the ultimate size of the
executable process depends on the amount of memory being allocated within COSY. The executables
come pre-configured for a maximum size of a little under 2GB. Should this be not enough for certain large
applications, the maximal memory available for allocation can be increased by changing the parameter
LMEM in all occurrences in foxy.f, dafox.f and foxgraf.f to a higher value, limited only by the underlying
computational environment. For purpose of estimating the final size, increasing LMEM by 1 increases the
maximally required memory by 12 bytes.

### 1.7 How to Run COSY INFINITY

Programs written in COSYScript with the file extension \.fox" can be compiled and executed by
the COSY INFINITY system executable program obtained above. First, we use a brief demo pro-
grambriefdemo.foxas an example case, which shows various COSY data types. The program file
briefdemo.foxis available at the COSY INFINITY download site.

There are several ways to execute the COSY INFINITY system program, also depending on the
platform.

#### 1.7.1 Windows Users

When using the installation package for Microsoft Windows to install the COSY INFINITY system ex-
ecutable program, the installer sets convenient ways to run COSY INFINITY. The installer is prepared
to be able to use the COSY GUI (graphical user interface) environment. Please refer to Section 1.5.1 on
page 9. For the details on COSY GUI execution, please refer to Section 1.7.5 on page 25.


#### 1.7.2 Execution with Input Query

This execution method applies to Linux/UNIX-like systems, including macOS.

In the terminal (shell, console) window, just type \cosy" to execute the COSY INFINITY system
program. Depending on how your program execution environment is set, you may need to type in a
different way such as \./cosy", \cosy.exe", or \a.out". When the COSY INFINITY system program
properly starts, the console screen displays the title of the COSY INFINITY system, and it asks you for
a file name with extension.fox:

##### GIVE SOURCE FILE NAME WITHOUT EXTENSION .FOX

At this point you type \briefdemo" (without the quotation marks, just nine characters). If you make a
mistake, it will prompt you again for a file name, and suggests the previous one. From now on the input
works like a line editor: You can replace any erroneous characters by typing the proper ones underneath.
After having entered the name successfully, you will see the following message.

##### --- BEGINNING COMPILATION

##### --- BEGINNING EXECUTION

After this, the program executes COSYScript inputs written inbriefdemo.fox.

Upon this execution, the COSYScript file name \briefdemo" (without the quotation marks, just nine
characters) is recorded in the filefoxyinp.dat. At the next execution of COSY INFINITY, the file name
\briefdemo" is suggested for the input source file. If you intend to run the same COSYScript file, in this
casebriefdemo.fox, just hit the return key to confirm the file name instead of typing the name again.

#### 1.7.3 Single Line Execution

This execution method applies to Linux/UNIX-like systems, including macOS.

In the terminal (shell, console) window, the COSY INFINITY system program can be executed by
one command line mode by giving the COSYScript file name:

```
cosy briefdemo.fox
```
The file extension \.fox" can be omitted in this mode, thus the following works as well:

```
cosy briefdemo
```
Regarding the program execution environment, the same caution applies as described in the beginning
of the previous section 1.7.2.

When the COSY INFINITY system program properly starts, the console screen displays the title of
the COSY INFINITY system, and you will see the following message.

##### --- BEGINNING COMPILATION

##### --- BEGINNING EXECUTION

After this, the program executes COSYScript inputs written inbriefdemo.fox.


#### 1.7.4 Running COSY INFINITY for Parallel Computations

Normally Linux systems are employed to operate parallel computation environments (high performance
computation systems), so the explanations below assume Linux systems and other conventional properties
of the system. Since a high performance system often has its specific ways to operate parallel computations,
please consult the system administrators for specific instructions.

Through the \mpirun" command, specify the MPI version of COSY INFINITY executable program
(see Section 1.5.8 for preparing such a COSY INFINITY executable program) to be run. Using the single
line execution mode described in the previous section 1.7.3, the typicalmpiruncommand to be typed in
the terminal window would be

```
mpirun -n <NP>./cosy <filename>
```
assuming the MPI version of COSY INFINITY executable programcosyis located in the current com-
mand operating directory. <NP>is the number of requested processes, and<filename>specifies the
COSYScript file (with the file extension \.fox").

On high performance systems with strict computation time management, thePWTIMEcommand
is useful to monitor CPU time being consumed.

When performing Beam Physics computations in parallel environments, executingcosy.foxto produce
the binary fileCOSY.binshould be operated using only one process. Please see Section 1.7.6 on page 26
for running COSY INFINITY for Beam Physics computations.

#### 1.7.5 COSY GUI Execution

To utilize the COSY GUI (graphical user interface) functionality, explained in Section 6, the platform
independent COSY GUI Java program fileCOSYGUI.jaris necessary, which is available at the COSY
INFINITY download site or included in COSY INFINITY installation packages. In order to run the Java
GUI program, you must have Java 8 or higher installed. If you do not have Java installed already, you
can get Java for free at

```
https://www.java.com
```
There are several COSY GUI example files available at the COSY INFINITY download site:

```
guidemo.fox: An example of how to use all COSY GUI facilities in a simple program. This program
uses the picture filecoffee.png, also available at the COSY INFINITY download site.
```
```
guielements.fox: An overview over all COSY GUI elements and what they look like
```
```
briefdemo basicgui.fox: A variation of briefdemo.fox, using basic COSY GUI facilities
```
```
briefdemo fullgui.fox: A variation of briefdemo.fox, using advanced COSY GUI facilities, with
full, manual adjustments
```
The COSY INFINITY installer for Microsoft Windows sets the COSY GUI execution environment. The
user using the installer, please refer to the instructions in Section 1.5.1 on page 9.

For Linux systems, you may install Java using the Linux distribution's package manager. Please refer to
your Linux documentation for further instructions on installing Java on your system. Once Java is properly
installed, run the COSY GUI Java program to execute a COSYScript file, for exampleguidemo.fox, by
typing as follows.


```
java -jar COSYGUI.jar guidemo.fox
```
Depending on your Linux desktop environment, you can either start the GUI by double clicking the
COSYGUI.jarfile, or using the command line.

The Java GUI tries to find the COSY INFINITY executable program \cosy.exe" (Windows) or
\cosy" (Linux/UNIX, macOS) to use by searching the following locations in the following ordering.

```
1.Location of the COSYScript file (with the file extension \.fox") to be executed
```
```
2.Location of theCOSYGUI.jarfile
```
In order to use a user self built COSY INFINITY executable program generated by Fortran compilation,
one can simply copy the executable program into the same directory as the COSYScript.foxfile to be
executed. Then COSY INFINITY can be executed by the methods provided by the installer, automatically
using the intended COSY INFINITY executable program.

#### 1.7.6 Running COSY INFINITY for Beam Physics Computations

There are the Beam Physics programs written in COSYScript calledcosy.foxanddemo.fox, available at
the COSY INFINITY download site.SYSCA.DAT, also available at the COSY INFINITY download site, is
a data file for the computation of fast fringe field approximations (fringe field mode 2). Some of example
programs indemo.foxuse this mode.

For Beam Physics computations, you first have to run the COSY INFINITY system executable program
for the COSYScript filecosy.fox. When the program starts properly forcosy.fox, following the console
screen displaying the title of the COSY INFINITY system, you will see the next message.

##### --- BEGINNING COMPILATION

##### --- BIN FILE WRITTEN: COSY

After this, the program terminates. There is now a binary fileCOSY.bin, which contains a compiled code
ofcosy.fox, and this is used via theINCLUDEcommand in all Beam Physics user input.

Whenever you start using a new COSY INFINITY executable program(due to a newer
version of COSY INFINITY or using a new computer or whatever the reason is!!),you have to run the
filecosy.foxfor the purpose of updating the binary fileCOSY.bin. Only then it will be compatible
with the new COSY INFINITY executable program.

The filedemo.foxcontains a set of user inputs written in COSYScript and also demonstrates most of
COSY INFINITY's Beam Physics features. As an example, let us executedemo.fox. The COSYScript
description of the file starts with theINCLUDEcommand:

```
INCLUDE 'COSY' ;
```
This reads the contents of the binary fileCOSY.binin. When the program starts properly fordemo.fox,
following the console screen displaying the title of the COSY INFINITY system, you will see the next
message.

##### --- BEGINNING COMPILATION

##### --- BIN FILE READ: COSY

##### --- BEGINNING EXECUTION


The display of the demo title \COSY INFINITY Beam Physics Demos" and the demo menu will follow,
which is the starting performance of the COSYScript inputs written indemo.fox.

For Beam Physics computations, beyond the description in this section, please refer to the Beam
Physics Manual of COSY INFINITY [7].

### 1.8 Syntax Changes

With very minor exceptions, version 10.2 is downward compatible to the previously released versions of
COSY INFINITY, and any user deck for version 6 and higher should run under versions 10.2 and higher.
However, Taylor models are not supported in version 10.

### 1.9 Future Developments

A variety of additional features are currently under development and/or alpha testing and are expected
to become available in a future version. Even before the official release, they may be available for use by
collaborators. Some of the features under development are

```
1.Arbitrary precision and rigorous data types and operations for DA, Taylor models
```
```
2.Enhanced non-verified optimization tools, primarily genetic algorithms
```
```
3.Direct language-level interface to the rigorous verified global optimizers COSY-GO
```
```
4.Direct language-level interface to a new hybrid differential algebraic ODE integrator as a further
development of COSY-VI
```
```
5.Fully rigorous tools for theorem proving in Dynamical Systems, including enclosures for attractors,
stable and unstable manifolds, homoclinic and heteroclinic points, Poincare sections, normal forms,
automatic bounds for topological entropy, and others.
```
