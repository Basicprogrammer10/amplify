Once Jeremiah leaves, you unlock and open the door again.
You look inside and see a metal box.
You reach in and grab it, it contains a signal sheet of paper.
You start reading it but just at that moment the captain of the sub comes walking down the hallway and sees you.
They have a _little talk_ and they end up fireing you for _know too much_.
Since you can't really get off the sub the captain gives you some busywork.

They task with simulating 50 steps of the [Rule 30 cellular automaton](https://en.wikipedia.org/wiki/Rule_30).
In this cellular automaton you have a 1D array of cells, each with the possible state of alive or dead.
Starting with cell #50 being alive, your program should print and simulate the next 49 steps.
The simulation is governed by these simple rules:

| 111 | 110 | 101 | 100 | 011 | 010 | 001 | 000 |
| :-: | :-: | :-: | :-: | :-: | :-: | :-: | :-: |
|  0  |  0  |  0  |  1  |  1  |  1  |  1  |  0  |

This can also be expressed at the following boolean expression: `left_cell ^ (central_cell || right_cell)`.
Honestly Wikipedia explains this much bettor so just check that.
When printing the world use a space for dead and `█` for live also make sure to trim the extra spaces at the end of each line.
Example output [here](https://paste.connorcode.com/b/b3cc1705-9eae-4539-a4f4-cdb320967799).
