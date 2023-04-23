# Functions

All BitScript files are valid functions, each file represents a single function.

## Syntax

```sh
reg [component name] [file name] # registers the component
[conponent name](params) # uses the component
```

## Parameters

The parameters are passed into the *input buffer*, which can be read by the component using `inpt`.

```sh
# echo.bs

dclr A 8 0
inpt A # takes 1 params

disp You entered {A}!
```

Use the function

```sh
# run-function.bs

reg echo echo
echo(11111111)
```

## Return values

Values can be returned using the `exit` keyword.

```sh
# add-function.bs

# just a simple 8 bit adder
dclr A 8 0
dclr B 8 0

inpt A
inpt B

dclr U 7 0
dclr C 8 0

C[0] = XOR(A[0], B[0])
U[0] = AND(A[0], B[0])
# snippet
C[7] = XOR(A[7], B[7], U[6])

exit C # returns the value of C
```

Use the function in an 8 bit adder program.

```sh
# run-function.bs
reg add add-function

# create two 8 bit variables
dclr A 8 0
dclr B 8 0

# assign them with values
A =d 11
B =d 23

# use the functon `add`
disp add(A, B)
```
