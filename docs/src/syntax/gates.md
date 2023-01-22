# Gates

Logic gates are the only way to process variables in BitScript, and that's the entire point.

```sh
dbg NOT(0) # 1
dbg AND(1, 0) # 0
dbg OR(1, 0) # 1
dbg NOR(1, 0) # 0
dbg NAND(1, 0) # 1
dbg XOR(1, 0) # 1
dbg XNOR(1, 0) # 0
```

## Short hand expression

```sh
dbg NOT(1101) # 0010, same as NOT(1, 1, 0, 1)
dbg AND(1101) # 0, same as AND(1, AND(1, AND(0, 1)))
dbg OR(1101) # 1, same as OR(1, OR(1, OR(0, 1)))
dbg NOR(1101) # 0, same as NOR(1, NOR(1, NOR(0, 1)))
dbg NAND(1101) # 1, same as NAND(1, NAND(1, NAND(0, 1)))
dbg XOR(1101) # 1, same as XOR(1, XOR(1, XOR(0, 1)))
dbg XNOR(1101) # 0, same as XNOR(1, XNOR(1, XNOR(0, 1)))
```

> Single values, whole arrays, and ranges can be used in gates.

In conclusion, this is how each gate behaves.

|Gate|Behavior|
|---|---|
|`NOT`|Flips all bits to its opposite.|
|`AND`|Only returns `1` if all bits are `1`.|
|`OR`|Returns `1` if any of the bit is `1`.|
|`NOR`|`NOT(OR)`|
|`NAND`|`NOT(AND)`|
|`XOR`|Returns `1` if there is an *odd number* of `1`s.|
|`XNOR`|Returns `1` if there is an *even number* of `1`s.|
