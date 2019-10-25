# A simple diff and mask format for color terminals

Usage:
```
use fmt_diff::*;

println!("{:08b}", diff(&0x0F, &0x07));
```

The only two public functions, `diff` and `mask` take two functions, and return a format object. The format object takes the parameters supplied by the format macro, and formats the provided objects.

The diff format will format every character that is the same with a green color, and different characters with a red color. The diff only prints the first object.

The mask format will format every character of the first, colored green if the second is an (ascii) zero, or space, or red otherwise.
