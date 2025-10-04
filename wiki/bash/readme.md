# Bash

## Piping

```bash
one | two  # Pipe stdout of `one` to `two`
one > two  # Pipe stdout of `one` to file named `two` (will remove all previously present content)
one >> two # Pipe stdout of `one` to file named `two` (will append to the end of file)
one >(two) # Spawn `two` and pass stdout file descriptor to `one`
# & and > can be swapped
one 2>&1   # Pass stderr (fd 2) of `one` to stdout (fd 1)
one &>2    # Pass stdout (fd 1) of `one` to stderr (fd 2)
# & and < can be swapped
one <&2    # Pass file descriptor 2 to `one`
```

## Variable expansions

Variable expansions cannot be nested.

```bash

```

## Arrays

Bash arrays are 0-indexed.

```bash
array=(a,b,c)      # Allocating an array
"${array[0]}"      # Obtain first element of the array
cmd "${array[@]}"  # Expand the array, each element will be its own argument to `cmd`
${#array[@]}       # Get array length, in this case 3
```

## File descriptors

```bash
exec 3<>"file.txt"    # Open `file.txt` on descriptor 3
exec {x}<>"file.txt"  # Open `file.txt` and save file descriptor to var `x`
exec 3<&-             # Close file descriptor 3
exec {x}<&-           # Close file descriptor stored in var `x`
```

## Sources
- <https://bash.cyberciti.biz/guide/Opening_the_file_descriptors_for_reading_and_writing>
- <https://www.linuxbash.sh/post/use-exec-fdfile-to-dynamically-assign-file-descriptors>
