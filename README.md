# progress bars
This contains three files that give different approaches to building a command-line progress bar in python.

#### `carriage-return.py`
This utilizes the "carriage return" approach, which amounts to rewriting a single, formatted string, and prepending the string with the `\r` character. This is great if you only need to update a single progress bar

#### `ansi.py`
This utilizes ANSI escape codes, specifically those around cursor position. The practice can be simplified to "for every update to the line, send the cursor to the left by the number of columns present in the current tty/terminal".

#### `ansi-multiple.py`
This also utilizes ANSI escape codes, but also includes moving the cursor _up_ and to the left. This enables multiple progress bars, for use in multithreaded applications.