# Variables

There are no data types in BitScript, because all data are stored in *1d array of bits*.

## Declare

`dclr` is used declare variables, the size (length) of variables cannot be changed once created, and variable names *can only contain alphabet and underscores*.

```sh
dclr first_var 4 # creates a variable of length 4
dclr another_var 4 1 # creates a variable of length 4, initialise it with 1s
```

## Assigning to variables

Variables can be displayed *as a whole*, *individual cells* or *ranges*.

```sh
first_var = another_var # sets first_var to 1111
first_var[0] = 0 # sets the least significant bit to 0, first_var is now 1110
first_var[2..4] = 00 # sets the 2-3rd least significant bit to 00, first_var is now 0010
first_var[..] = 0 # ranges can be unbounded, in this case it sets first_var to 0000
```

Variables can also be used as index for another variable, for example.

```sh
first_var[..DEN(11)] = another_var[1..] # denary of `11` is 3
first_var[AND(1, 0)] = 2
```

## Displaying variables

In the same sense that it is assigned, it can be displayed the same way with `dbg` and `disp`.

```sh
dbg another_var # 0000
dbg another_var[0] # 0
dbg another_var[NOT(0)..] # 000

disp another_var is {another_var}
disp the least significant bit of another_var is {another_var[0]}
disp the other 3 bits are {another_var[NOT(0)..]}
```
