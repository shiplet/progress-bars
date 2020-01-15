# progress bars
This contains three files that give different approaches to building a command-line progress bar in python.

#### `carriage-return.py`
This utilizes the "carriage return" approach, which amounts to rewriting a single, formatted string and prepending the string with the `\r` character. 
This is great if you only need to update a single progress bar.

#### `ansi.py`
This utilizes ANSI escape codes, specifically those around cursor position. 
The practice can be simplified to "for every update to the line, send the cursor to the left by the number of columns present in the current tty/terminal".

#### `ansi-multiple.py`
This also utilizes ANSI escape codes, but also includes moving the cursor _up_ and to the left. 
This enables multiple progress bars, for use in multithreaded applications.


### When running on Windows CMD or Powershell
[This StackOverflow question](https://stackoverflow.com/questions/16755142/how-to-make-win32-console-recognize-ansi-vt100-escape-sequences) sheds some light on why native Windows CMD & Powershell prompts don't work with ANSI escape sequences.
To get around this, I use the [Windows Terminal (Preview)](https://www.microsoft.com/en-us/p/windows-terminal-preview/9n0dx20hk701?activetab=pivot:overviewtab), which provides more modern features for all Windows-native command prompts.
The ANSI solutions will also work on Windows Subsystem for Linux terminals.