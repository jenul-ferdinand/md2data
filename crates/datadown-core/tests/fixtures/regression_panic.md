# Regression Test: Parser Panic Bug

This file contains the markdown that previously caused a parser panic.

# Authors

* Nate Vack
* **Vendor Packages**
  * docopt
  * CommonMark-py

The bug was caused by inline formatting (bold) within list items, which would cause the stack to become unbalanced and panic.

This should now parse successfully.
