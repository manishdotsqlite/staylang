### staylang

staylang is a lightweight interpreter for a simple scripting language written in Rust.

### features

- familiar control structures:`LET, WHILE, INPUT, PRINT`

- easy-to-read syntax for teaching or scripting

- written in performant, safe Rust

### sample staylang File

bash

```
        LET a = 0
        WHILE a < 1 REPEAT
        PRINT "Enter number of scores: "
        INPUT a
        ENDWHILE

        LET b = 0
        LET s = 0
        PRINT "Enter one value at a time: "
        WHILE b < a REPEAT
        INPUT c
        LET s = s + c
        LET b = b + 1
        ENDWHILE

        PRINT "Average: "
        PRINT s / a
```

### usage

# command: `staylang yourfile.txt`
