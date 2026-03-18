## 6 Graphical User Interface

Starting with version 9.1, COSY INFINITY has built in support for building graphical user interfaces
(GUIs) from within COSYScript [25]. The GUI feature requires COSY INFINITY to be run with a GUI
program, such as the platform independent Java GUI program \COSYGUI.jar". Please refer to Section
1.7.5 for running the COSY GUI Java program for COSYScript files.

The programming of GUI interfaces from within COSY fits in naturally with the classic way of handling
input and output, while providing a wide range of commonly used GUI elements. COSY provides the
special GUI unit numbers -201... -210, each of which represents one window in the user's graphical
environment.

Existing programs can be easily converted to use a GUI (see Section 6.1) with only minimal modifica-
tions to existing code. For more sophisticated GUIs, a variety of special GUI commands can be written
to the GUI unit numbers to define the elements in each window and to interact with them. Available
commands are described in detail in Section 6.3 below.

The GUI also allows input from and output to the traditional console units 5 and 6 in a GUI program.
These calls are automatically routed to a separate terminal window if COSY is run in a GUI environment.
Similarly, a simple ASCII based GUI is shown instead of a GUI window when COSY is run in a non-GUI
environment (e.g. from the command line).

### 6.1 Basic GUIs

The main conceptual difference between a GUI and the traditional console based I/O is the concept of
a delayed read. In a GUI window, the user can enter values into various fields and modify them in any
order before pushing a button, which then causes all values to be read in at once.

This concept is integrated into COSY by makingREADcommands to the GUI window units delayed.
That means that COSY will not immediately read a value and place it into the variable passed to the
READcommand. Instead, COSY will associate each variable with a GUI input field, and only place
values in them once a delayed read is initiated. At what point in the code such a delayed read is to be
performed, is up to the programmer to specify.

To convert an existing program using traditional console based I/O to a GUI program, the developer
generally has to perform the following steps:

```
1.Change \WRITE 6" to \WRITE -201" to output to a GUI window instead of the terminal.
```
```
2.Similarly, change \READ 5" to \READ -201" to read input from a GUI window instead of the
terminal. Note that the GUI unit number is the same for bothREADandWRITEcommands.
```
```
3.Insert the call \GUIIO -201 ;" at the correct places where you want to initiate a delayed read
from the window. In this form, the command will automatically add an OK button at the end of
the window, show it to the user, wait for the button to be pushed, and then fill in the values from
each input field into the variables specified in theREADcalls.
```
TheWRITEcommands to the GUI units will output each string as a line of simple text in the GUI.
A graphics object will be rendered in an area within the GUI window. Additional graphics objects will
overwrite the previously shown graphics object in the same GUI window, enabling an effect of animations.
(The GUI commandnGRAppendallows multiple graphics objects shown in the same GUI window { see
Section 6.3.) All other data types will appear in an embedded console in the GUI window the same way


they would appear in a terminal. The user can select and copy content out of an embedded console, and
scroll if the content is too long. Consecutive output into a console is appended to an existing console until
a string is written to the window.

For eachREADfrom a GUI window unit, COSY will insert an input field in a separate line in the
GUI. The variable to be read is associated with this input field, and its value is placed in the variable
once the window is shown to the user by callingGUIIO.

When converting programs to use the GUI, developers must make sure that their code is ready for the
delayed read concept. In particular, the variable being read cannot be used in the code before the call
toGUIIO. Furthermore, allREADcommands must read into different variables to be useful, otherwise
the variable will only contain the value of the lastREADcommand.

### 6.2 Advanced GUIs

For more fine grained control over the appearance of the GUI, the full GUI interface can be controlled
through special GUI commands written to the GUI window units. The COSY GUI operates with double
buffered windows, that is for each window number there is the currently displayed window and a second
hidden window. Most GUI commands act on the hidden window, but some can be issued to manipulate
the currently displayed window (if any).

In general, the code structure to define a GUI window looks very much like the traditional console
based I/O code, where the user is prompted for some input through aWRITEand the input is then
read from the user by aREAD. In COSY's GUI model, the GUI window is still constructed by issuing
WRITEcommands to prompt the user for input, immediately followed byREADcommands to read
back the actual input. TheREADs are automatically delayed by COSY until a delayed read is initiated.

GUI commands are issued by writing to the corresponding GUI window output unit usingWRITE.
GUI commands are strings starting with the backslash character (n), e.g. nReadField. Each GUI com-
mand can take a number of arguments. Those are specified as additional arguments to theWRITEcall.
Their type can be anything COSY can convert into a string using theSTfunction. A singleWRITE
command may contain several GUI commands.

To read back a value from a GUI element, aREADcommand is issued to a GUI window unit. This
associates the variable given to theREADcommand with the most recently written GUI field that can
return a value, provided it has not been associated yet. If there is no such field, either because no GUI
field that returns a value has been written yet or because the last GUI field has been associated (\read")
already, theREADcommand will instead insert a newnReadFieldfield on its own line into the GUI
window, and associate the variable with that field.

To initiate the delayed read into these associated variables, the commandGUIIOis used. It can be
used in two different ways, depending on how it is called:

GUIIO<unit>;

If called with only one argument,<unit>specifies the GUI window unit to read from. The command
adds an OK button at the end of the window if no button was defined yet, shows the window, waits for a
button to be pushed, reads all values from the window, and then closes the window.

GUIIO<unit> <button>;

If called with two arguments,<unit>specifies the GUI window unit, and<button>must be a variable
to receive the name of the button that was pushed. In this more advanced form,GUIIOonly waits
for a button to be pushed in the currently displayed window, and then reads the values of all associated


variables. The text on the button that was pushed is stored in<button>(note that this string is subject
to COSY's usualREADprocessing). It does not modify the window in any way (e.g. showing it,
adding buttons, or closing it). If there is no window currently displayed, all variables are filled with zeros
immediately and the number-1 is returned in<button>. If there is a window displayed, but it does not
have a button, all variables are read immediately and the number 0 is returned in<button>.

The commandGUISETis used to update the value of a component in the currently displayed window
without closing and reopening the window.

GUISET<unit> < n > <value>;

< n >is the counting number of GUI input elements (GUI command names starting with \Read") that
were added to the window.<value>is the new updating value.

### 6.3 GUI Command Reference

Tables 1, 2 list all available GUI commands currently implemented in COSY. The first column gives
the name of the command. Commands are case insensitive, the spelling used here is by convention but
not required. Commands starting with \Read" insert a GUI element that can be read by a subsequent
READcall. The second column specifies which of the two windows the command acts on (either hidden
or currently displayed). The third column indicates whether a command returns a value when the GUI
is read from. The last column lists the arguments the command takes. Optional arguments are indicated
by a default value in parenthesis, if they are omitted, this value is used. Optional arguments can only be
omitted beginning with the last argument. Following Tables 1 and 2, we give some further remarks on
specific GUI commands.


```
Command Window Value Arguments
nConsole hidden No Any number of arguments of any type
Write to embedded console
nText hidden No String to be inserted
Static text
nImage hidden No Image filename
Static image
nLine hidden No None
Vertical line
nSpacer hidden No Width in pixels
Transparent element Height in pixels (0)
nButton hidden No Text on button
Push button 1 - default button, 0 - otherwise (0)
Tooltip (none)
nReadCheckbox hidden Yes Text next to checkbox (none)
Checkbox 1 - selected, 0 - not selected (0)
Tooltip (none)
nReadOption hidden Yes Text next to radio button (none)
Radio button 1 - selected, 0 - not selected (0)
name of button group (none)
Tooltip (none)
nReadField hidden Yes Initial value (none)
Unformatted input field Tooltip (none)
nReadNumber hidden Yes Current value
Numerical input Minimum value
Maximum value
Increment ((Max-Min)/100)
1 - editable, 0 - not editable (1)
Tooltip (none)
nReadList hidden Yes List of entries separated byj
Selection from list Initially selected value
1 - combobox, 0 - dropdown,
L - list, M - multiselect (0)
Tooltip (none)
nReadFileName hidden Yes Initial value (none)
File selector
nReadProgress hidden Yes Progress in % or -1 (-1)
Progress bar
```
```
Table 1: Available GUI commands in COSY
```


```
Command Window Value Arguments
nNewLine hidden No None
Jump to next line
nNewCell hidden No Width of cell (1)
Jump to next cell
nLeft hidden No None
Set current cell's alignment to left
nCenter hidden No None
Set current cell's alignment to center
nRight hidden No None
Set current cell's alignment to right
nJust hidden No None
Set current cell's alignment to justified
nTitle hidden No Window title
Set window title
nGRScale hidden No Height (0.35)
Scale COSY graphics object
nGRAppend hidden No 1 - append, 0 - overwrite (0)
Append COSY graphics object
nCanvas hidden No None
COSY graphics object window
nDeactivate displayed N/A None
Make non-interactive
nActivate displayed N/A None
Make interactive
nShow hidden N/A x coordinate (center)
Display window y coordinate (center)
nClose displayed N/A None
Close and destroy window
nSet displayed N/A Number of element
Set value of interactive element Value to be set
nFocus displayed N/A None
Make this the active window
nDebug all N/A Debug level between 0 { 3
Set GUI debug level
nFinish hidden No Text on the button ('OK')
Add a button if none is there yet
```
```
Table 2: Available GUI commands in COSY (continued)
```


nConsole
All arguments are output in the same form as on a regular terminal. An embedded console is
inserted into the GUI window on a separate line. Output is appended to this console until another
GUI component is added, or one ofnNewLine,nNewCell, ornShoware called. The user can select
and copy text in an embedded console, scroll if the text is too long, but cannot change the content.

nImage
Image file names are specified with forward slashes (/) as path separators or asfile:///URLs for
full paths. Any fully qualified URL can be given to load images over the internet (if the computer has
an internet connection). The Java GUI shipped with COSY INFINITY comes with some commonly
used icons built in which can be accessed using URLs of the formcosy://yes.png, where instead
of \yes.png" any one of the built in icons (\ask.png", \clock.png", \cosy.png", \info.png",
\msu.png", \msupa.png", \no.png", \star.png", \warn.png", \wrench.png", \yes.png") can be
used.

nReadOption
Options are a group of GUI elements of which only one can be selected at a time (typically displayed
as round buttons). In order to designate which option belongs to which group, the name of an option
group can be specified. Of all options in a group with the same name, at most one is selected at
each time.

nReadNumber
When editable, this will display an input field with adjoining up and down buttons. Only numeric
input is allowed in this field. When not editable, a slider is shown which can be dragged by the user
to indicate a numeric value. When read, this field always returns a number in COSY.

nReadList
Presents the user with a list of options from which to select one. If the list is set to editable, the
user is allowed to enter a value that is not on the list, otherwise the user must select a value on the
list.

nReadProgress
This element can either display a progress bar with the given percentage of completion, or a bar with
an indeterminate state to indicate that a computation is ongoing but the total time is not known.
When read, this element will simply return the value it is currently set to. This is mostly so that it
can have its value changed while the window is displayed usingnSet.

nNewLine,nNewCell,nLeft,nCenter,nRight,nJust
See Section 6.4 for information about the layout of GUI elements.

nGRScale
This command sets the height of a COSY graphics object in the window. The height is relative to
the screen height, and the initial height is 0.35.

nGRAppend
When a COSY graphics object is output to a GUI window, any additional COSY graphics object to
the same window overwrites the previously displayed one by default (nGRAppend 0), and it enables
an effect of animations. Call this command with 1 (nGRAppend 1) to append additional COSY
graphics objects displayed in the same window.

nCanvas
This prepares the GUI window to render a COSY graphics object. The same action of this command
happens internally by aWRITEcommand of a COSY graphics object to a GUI unit, so it is not
necessary for the user to call this command. However, it can be used to \reserve" a GUI window
for outputting COSY graphics objects at a later time.


```
Note: AWRITEcommand of a COSY graphics object to a GUI unit displays the contents right
away even without using anShowcommand. This feature is to be compatible with the other graphics
drivers (see Section 5.2). GUI windows with any COSY graphics object follow the positioning rules
as described below fornShow.
```
nDeactivate,nActivate
These commands make the currently displayed window either inactive or active. In an inactive
window, the user cannot interact with the elements of the window any more, they are often shown
in gray. This command does not change window visibility, the window remains visible all the time.
By default, windows are active, i.e. the user can interact with the elements in the window.

nShow
This command displays the hidden window by making it visible to the user. At the same time, the
previously displayed window, if any, is closed and destroyed.

```
If both coordinates are given, the window's top left corner is positioned accordingly, with (0,0)
being the top left corner of the screen, and (1,1) the bottom right corner.
```
```
If no arguments are given, the new window is initially shown as follows. If the window includes
a COSY graphics object, the first four GUI windows are positioned at the left top, the left bottom,
the right top, and the right bottom. Otherwise, the GUI window is centered on the screen. After-
ward, the new window is shown where the previously displayed window was, such as after the user
relocated the window on the screen.
```
```
All subsequent calls to create GUI elements will act on a newly created, initially empty hidden
window. This command returns immediately, to wait for user input viaGUIIO.
```
nSet
It is not recommended to use this command directly. Use GUISET procedure instead.

nDebug
Set the debug level for the GUI. Integer between 0 and 3, with 0 (the default) meaning no debug
output, 1 meaning errors are logged to the Terminal, 2 also outputting diagnostic messages, and 3
echoing the entire GUI protocol read from COSY. Can be called several times to turn debugging of
certain parts of the GUI on or off.

### 6.4 GUI Layout

Components in the GUI are arranged based on the order in which they are added and a natural size for
each element as determined by the GUI program. Each element is added at the end of the current line
in the GUI. A line can be ended using thenNewLinecommand, which causes all further elements to be
added at the beginning of the next line.

The size of the window is determined by the width of the longest line, and the total number of lines. If
a line is shorter than the width of the resulting window, it is aligned according to the alignment specified
by one of the commandsnLeft,nCenter,nRight, ornJust. nJustwill cause the elements in the line
to be resized such that they fill up the entire line. By default, if none of the alignment commands was
issued, lines are left justified.

Alignment commands can be called at any time, before or after writing elements to a line. It always
applies to the current line and if called multiple times within the same line, the last call carries.


For more sophisticated layouts, the COSY GUI specification supports thenNewCellcommand. With
this command it is possible to lay out elements in a tabular grid, where each cell behaves much like the
lines described above. Each cell can have its own alignment, and the size of each row and column in
the table is determined by the largest cell in the row or column. A row of cells is ended by calling the
nNewLinecommand.

By providing an integer argument tonNewCell, the current cell can be made to span multiple cells.
The last cell in each row is automatically expanded to the end of the window, so it is not necessary to
provide a cell span for the last cell. If this behavior is not desired, an empty cell can be inserted after the
last occupied cell by simply callingnNewCell.

### 6.5 Examples

```
WRITE -201 'nNewLine' 'nNewLine' 'nNewLine' ;
Inserts three empty lines in window number -201.
```
```
WRITE -201 'nReadNumber' tax 0 100 ;
Inserts a slider with its initial value taken from variable tax and minimum value 0 and maximum
value 100 in window number -201.
```
```
WRITE -201 'nReadField' name 'nNewLine' ;
Inserts an input box with initial text taken from variable name followed by a new line in window
number -201.
```
```
WRITE -201 'nReadList' 'RajZeusjJupiterjOther' 'Zeus' 0 'Select yours!' ;
Inserts a non-editable list with the options \Ra", \Zeus", \Jupiter", and \Other", with \Zeus"
initially selected and a tooltip of \Select yours!" in window number -201.
```
```
WRITE -201 'nShow' ; GUIIO -201 button ;
Show window number -201 and wait for a button to be pushed in this window. The name of the
button is stored in variable button.
```
There are several examples of COSY GUI COSYScript program files, available at the COSY INFINITY
download site. Please refer to Section 1.7.5 for the details and how to execute them.
