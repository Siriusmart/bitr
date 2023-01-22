# Casting

Since there is only one datatype in BitScript, casting isn't really a thing.

However, what *is* a possible is accepting inputs of octal, denary, and hexidecimal numbers. And opposite to that - displaying variables in that format.

## Input casting

```sh
dclr store 0 # decalre a dummy variable

inpt store # now accepting only binary numbers
inpt o store # now accepting only octal numbers
inpt d store # now accepting only denary numbers
inpt h store # now accepting only hexidecimal numbers
```

## Output casting

```sh
dbg 1100 # 1100
dbg OCT(1100) # 14
dbg DEN(1100) # 12
dbg HEX(1100) # C
```

## Assign casting

```sh
dclr store 0

store = 1100 # 1100
store =o 14 # 1100
store =d 12 # 1100
store =h C # 1100
```
