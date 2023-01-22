# Hello world

To display the text "Hello world", create a file `hello_world.bs`.

```sh
# hello_world.bs

# this is a comment

disp Hello world\n # disp [string]
```

Save the file and run `bitr display`. Alternatively, you can run it in the repl, start the repl with `bitr`.

## Debug

`dbg` display values with no formatting.

```sh
dbg 1 # 1
dbg AND(1, 0) # 0
dbg NOT(11001101) # 00110010
dbg DEN(1111) # 15, DEN for denary (base 10)
```

## Display

`disp` display strings with formatting arguments. Use singe quotes (`''`) to preserve leading/trailing spaces, and curly braces (`{}`) for values.

```sh
disp This is a string.\n # This is a string.
disp 'Enter a value: ' # Enter a value: 
disp 1 and 1 is {AND(1, 1)} # 1 and 1 is 1
```
