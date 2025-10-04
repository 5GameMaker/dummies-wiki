# Bytedoc format

Bytedoc is the format used in this wiki to display ABIs without confusion.

Possible extensions: `.bytedoc` (albeit it's not meant to be used on its own).

Only ASCII characters (no control characters, excluding tab, carriage return and line feed)
are allowed in `.bytedoc` files (except in comments).

All bytedoc documents must be valid UTF-8.

In codeblocks bytedoc uses `bytedoc` syntax.

## Whitespace

In most cases all whitespace (except for spaces in-between visible characters) is ignored:
- `Hello!` and `Hello !` represent ASCII strings `"Hello!"` and `"Hello !"` respectively.
- `Hello!`, `Hello! ` and ` Hello!` represent the same string.

Tabs in-between visible characters are invalid.

## Root strings

Root strings are strings that aren't placed in a wrapper:
```bytedoc
This is a raw string

And this is a raw string
```

Since all newlines are ignored, this bytedoc is the same as:
```bytedoc
This is a raw stringAnd this is a raw string
```

To not have those collapse into a single line, newlines must be inserted explicitly:
```bytedoc
This is a raw string\n
\n
And this is a raw string
```

All supported escape sequences in raw strings:
- `'\n'`: line feed character
- `'\r'`: carriage return character
- `'\t'`: tab character
- `'\x('[([Uu]-)?[0-9a-fA-F]+]')'`: arbitrary unicode character
- `'\\'`: backslash character

## Objects

An object is denoted by parenthesies: `(<type>)`

A human-comment may be added: `(<type>: <comment>)`

Object may be named: `(<type>(<name>))`

Object may be an array: `(<type>[])`, in which case value for it may be specified with: `(<type>[] {1,2,3..})`.
Array length may be specified as well: `(<type>[<length>])`

For non-array objects value is specified via: `(<type> = <value>)`. You may still add an `=` when setting the value
of an array, but it's optional there.

If object may not be present in the stream, it may be made optional with: `(..)?`

Parts go in the following order: `(<type>[<length>](<name>) {}: <comment>)`

## Types

There are several built-in types:
- `?`: an unknown type. Size is determined contextually.
- `byte`: a singular byte. Signess is contextual.
- `(i,u)(8,16,32,64,128,..)(be,le)?`: either signed or unsigned integer of specified size. Size must be a multiple of 8, but not necessarily a power of 2. Endiannes may be specified for integer types larger than 8
- `ack`: a special 0-sized type for sockets. Used contextually to indicate acknowledgement or start of a connection.
- `dyn ..`: a dynamically sized object with unknown size. `dyn` is the same as `dyn ?`.
- `pad`: padding. Acts the same as `byte`. Padding bytes are considered arbitrary. Specifying the value of `pad` is an error.

Other types can be defined via: `Type:` on a separate line.

If this type extends other types, options may be specified via: `Type: (Parent.field = value)`.
Multiple parameters are separated by commas `Type: (Parent1.field1 = value1, Parent2.field2 = value2)`.
If `Parent1` and `Parent2` are the same, it can be written in a shorter form: `Type: (Parent.{field1 = value1,field2 = value2})`.
If values are the same too, it becomes even shorter: `Type: (Parent.{field1,field2} = value)`.

Type parameters may be specified with (i.e. for type `Type` with a generic parameter `T`): `Type<T>:`.

## Comments

Comments start with a `#` at the beginning of the line or are enclosed in `<< >>` inside of root strings.

## Conditions

A condition is represented via `{if <cond1>, <then1>, if <cond2>, <then2>, else <otherwise>}`.

`else` and extra `if` parts may be dropped.

Conditions are checked sequentually until `else` is encountered or one of the conditions is true.

## Network transmissions

Network transmissions are included at the top of bytedoc files:

```
<host1> -> <host2> (<socket-type>): <data>
<host1> <- <host2> (<socket-type>): <data>
```

`...` line can be used to indicate an arbitrarily long wait.
