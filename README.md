Tic-tac-toe (CLI)
=================

Learning Rust via implementing simple games.

Description
-----------
The goal of building this project was to summarize knowledge acquired from reading Rust books and documentation 
by implementing a simple, though not completely trivial, exercise. 

The implementation was done with code completion disabled, using compiler hints and Rust docs (mostly.) 
Each of the language features used here was added only in case when it helped to improve code quality, like 
decoupling user's input from game state. Therefore, it was a nice learning experiment helping to find a 
"method to Rust's madness", and its not-so-obvious for a non-Rust developer aspects like `dyn` keyword, 
lifetimes, and borrow checker rules.

Finally, it is coded by a hobbist learning Rust in spare time. So, I believe that the implementation provided here
might not follow best Rust's coding practicies, or doesn't use all of it's features that might have helped to
make the implementation easier/better/cleaner.

Improvements are welcome!

Features
--------
1. Provides a basic CLI interface that reads turns from `stdin` and prints game's state to the terminal.
2. Implements traits like `BufRead` and `Iterator`.
3. Creates custom traits to make easily-configurable I/O devices.
4. Testable code that can be executed both interactively (keyboard input) and programmatically (via encoded
sequence of turns.)

Running the code
----------------
```bash
git clone https://github.com/devforfu/tictactoe
cd tictactoe
cargo run
```

Example
-------
```
Welcome to the Tic-Tac-Toe game!
Commands:
(1) put mark at x row and y column: x,y
(2) stop the game: [s]top (or Ctrl-C)

+---+
|...|
|...|
|...|
+---+

1,1
+---+
|...|
|.x.|
|...|
+---+

0,1
+---+
|.o.|
|.x.|
|...|
+---+

1,1
Impossible turn [1, 1]: cell is not empty; try again!
2,2
+---+
|.o.|
|.x.|
|..x|
+---+

0,2
+---+
|.oo|
|.x.|
|..x|
+---+

0,0
+---+
|xoo|
|.x.|
|..x|
+---+

The game is over: the player X wins!
The final board's state:
+---+
|xoo|
|.x.|
|..x|
+---+
```