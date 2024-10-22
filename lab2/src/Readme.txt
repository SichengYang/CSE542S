1. Sicheng Yang sicheng@wustl.edu
Qinzhou Song qinzhounick@wustl.edu

Modules:
We added declarations and script_gen into our mod.rs and then we added use statement like use lab2::declarations
  to use the public types, constants, and functions in main.rs.
We changed the print statements under whinge mode so that they only print to the standard error stream.
We had challenges where our script_gen function was private and cannot be used in main.rs so we made it a public function.
We also had error from not including the atomic library in the script_gen file so we added that.


