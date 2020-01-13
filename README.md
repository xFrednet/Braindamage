# Braindamage
 The goal of this project is to implement all functionality from "Brainfuck" and then continue to add several features in the brainfuck fashion to actually make it a "viable" option.

Some planed features are:
  * Multithreading... YES MULTITHREADING
  * File IO
  * Maybe network access
  * Maybe library support

## Functions
### Default Brainfuck operations
| Operator | Function | C-Equivalent | Status |
|---|---|---|---|
| `>` | increment the data pointer (to point to the next cell to the right). | `++ptr;` | Working |
| `<` | decrement the data pointer (to point to the next cell to the left).  | `--ptr;` | Working |
| `+` | increment (increase by one) the byte at the data pointer. | `++*ptr;` | Working |
| `-` | decrement (decrease by one) the byte at the data pointer. | `--*ptr;` | Working |
| `.` | output the byte at the data pointer.  | `putchar(*ptr);` | Working |
| `,` | accept one byte of input, storing its value in the byte at the data pointer.  | `*ptr=getchar();` | Working |
| `[` | if the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command | `while (*ptr) {` | Working |
| `]` | if the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command. | `}` | Working |

Source: https://en.wikipedia.org/wiki/Brainfuck

### New Braindamage operations
| Operator | Function | Pseudocode | Status |
|---|---|---|---|
| `:` | This writes the the current buffer value into a file buffer. The buffer will be filled until a `0` is submitted, this will save the current buffer to a file. | `if (cell == 0) {`<br>`__buffer.save();`<br>`__buffer.clear();`<br>`} else {`<br>`__buffer.push(cell);`<br>`}` | Working |
| `;` | Loads a file and reads a cell from the file into the value at the data pointer. The read command will continue reading the file cell by cell until the end is reached. The end is indicated by a `0` and the next reading instruction will reload the file. <br><br> Example: The operation `;;;;` on the file `xF` would read: `xF0x` | `if (index > buffer.size()) {`<br>`__file.read(buffer);`<br>`__index = 0;`<br>`}`<br><br>`if (index < buffer.size()) {`<br>`__return buffer[index++];`<br>`} else {`<br>`__index++;`<br>`__return 0;`<br>`}`| Working |
|`{` & `}`| This starts a new thread that will execute the operations in the brackets. The current thread will jump over the operations. The started thread terminates when the closing curly bracket is reached.<br> Example: `>+{[+]}>>[+]` this will result in two threads with one incrementing the second value and one incrementing the forth value of the data buffer. The program would never terminate. | `TODO` | Planning |
|`@`| Connect to a network | `TODO` | Planning |
|`°`| This reads any send data from the network buffer byte by byte. The reading is implemented as a queue that is drained. Messages are separated by 0. This operation will also return a `0` when the queue is empty. | `TODO` | Planning |
|`^`| This writes one byte to the network buffer. The message buffer will be send when a ´0´ is written to the stream. | `TODO` | Planning |

Source: My damaged brain... Feel free to make any suggestions :D

## Examples
### Hello World
This writes `"Hello World!"` into a file. Then it reads the file and writes the content to the console. This is just a Braindamage `Hello World!` program.
```
++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>++:>+:+++++++::+++:<<++:>+++++++++++++++:>:+++:------:--------:<<+:<:[-]--<:;[.;]
```

### Write a file to console
```
;[.;]
```