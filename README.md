# Robinfish - Now Oxidized!
A deadfish interpreter that is expanded slightly, written in Rust 1.87.0!

| Operator | Explanation |
| -------- | ----------- |
| i | Increment the accumulator. |
| d | Decrement the accumulator. |
| s | Sets the accumulator to the square root of itself. |
| o | Outputs the accumulator as a number. |
| c | Outputs the accumulator as an ASCII character *unless* the value is outside 0-255. |
| h | Halts program execution. |
| w | Outputs "Hello, World!"
| ; | Resets the accumulator *and* the stack to their initial states. |
| # | Resets the accumulator to 0. |
| $ | Empties the stack. |
| q | Output the source code to the program *only once* during the program's execution. |
| ^ | Pop first element in the stack. Do nothing with it. |
| > | Push the accumulator into the stack. *Does not* reset the accumulator. |
| < | Pull the value from the front of the stack and set the accumulator to it. |
| r | Reverse the stack order. |
| R | Toggles Robinfish math. By default it is off. |
| = | Sets the accumulator to ``stack[accumulator]``. It does nothing if out of bounds. |
| Space | Do nothing. |
| Newline | Do nothing. |
