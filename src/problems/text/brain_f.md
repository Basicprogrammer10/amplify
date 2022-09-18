With your base 64 decoding program the navagation is going very smoothly.
You are completing your tasks quicker than expected leaving time for more amazing programming problems!

You start to learn about [Esoteric programming languages](https://en.wikipedia.org/wiki/Esoteric_programming_language).
This _inspires_ you to make your own [brain,,,_fun_](https://en.wikipedia.org/wiki/Brainfuck) interpreter.
In this language there you are on an infinite array of cells and there are only 8 instructions (7 for this problem):

| Instruction | Description (C code)                                  |
| ----------- | ----------------------------------------------------- |
| >           | Increment data pointer (`++ptr`)                      |
| <           | Decrement data pointer (`--ptr`)                      |
| +           | Increment data value (`++*prt`)                       |
| -           | Decrement data value (`--*prt`)                       |
| .           | Print data as ASCII (`putchar(*ptr)`)                 |
| [           | If data value is 0 jump 1 past `]` (`while (*ptr) {`) |
| ]           | If the data value is not 0 jump 1 past `[` (`}`)      |

So make a program to take in _brainfun_ programs from argv and run them.
Example input / output [here](https://paste.connorcode.com/b/b663b4b4-6250-4979-9125-02ed5c22844a).
