# Funx
A programmable programming language written in Rust

---
# Introduction
In Funx everything is a call. Those calls are composed of a head value and argument values
written like this for example:`(+ 1 2)`.
The head value in this case is a native function (function in rust) called `+` and the 1st and 2nd
arguments are `int` values. This call will return `3` as the native function `+` sums up all it's
arguments.

## Grammar
The language's grammar is pretty simple as it nearly only consists of calls and values.

| name   | example                                        |
|--------|------------------------------------------------|
| call   | `(...)`                                        |
| vector | `[...]`                                        |
| body   | `{...; ...}`                                   |
| value  | `1`, `true`, `null`, ... _see [types](#types)_ |
_**Note:**_ White space is important between values.

## Types
| name            | example                                                     |
|-----------------|-------------------------------------------------------------|
| undefined       | `null`                                                      |
| any             | `_`                                                         |
| int             | `0`, `1`, ...                                               |
| float           | `1.5`, `0.25`, ...                                          |
| bool            | `true` or `false`                                           |
| str             | `"..."`, `'...'`                                            |
| addr            | `@...`                                                      |
| closure         | `#...`                                                      |
| pattern         | `<...>`                                                     |
| native function | _a rust function_                                           |
| function        | _a closure with a pattern that has to be matched_           |
| type            | `undefined`, `any`, `int`, ..._all the other type names_... |
| union           | _a set of types_                                            |
| exclusion       | _a set of types which are excluded_                         |


### Wildcard
The wild card value written as `_` is a value that matches with any other value.
That means if you call `(= _ anything)` it'll always return true.
### Null
A value with nothing in it, just to represent the absence of anything.
Other languages like Python may call it `None` or Lua may call it `nil`.
### Address
Address values are pretty much names. They can be used for functions to define a variable for example.
Note that writing `(var a 1)` does not assign the value `1` to `a`, because `a` will be evaluated
before the function call which, if not defined, will return `null`. That means you have to write
`(var @a 1)`.
### Closure
Closure values are basically call node trees as a value which can be used e.g. for functions. Without them
the language would not be programmable.
```
    var @double #(* %0 2);
    print (double 4);
```
This will print out the int value `8`. The `%` is an argument getter which means that, if you call
a closure, the arguments passed to the call will be stored in order from 0 to how many arguments have been
passed in. So `%0` will get the first argument passed to the closure's call, which in this case is `4`.
### Pattern
Patterns are only really used for functions to check the arguments types. If a wrong type is passed to
the function, it'll throw an error.
### Union
Unions are collections of types. When matching against other types, the Funx interpreter will check if the
other type is contained within the union, if so the match will succeed.
### Exclusion
The opposite of an union. When matching against other types, the Funx interpreter will check if the
other type is **NOT** contained within the exclusion, if so the match will succeed.

## Functions
Functions are what makes this language roll. You define them by putting the type `function` in the call head,
an argument pattern as the 1st argument and then a closure as the 2nd. In the following Funx code there
is a function defined under the name `double` which takes in an `int` value and returns the value times 2.
```
    def @double (function <int> #(* %0 2));
    double "strings can't be passed in";
```
But for examples sake, I have put a `str` value in as the 1st argument which will make the program throw
and error:

`ERROR: expected type int but got type str`

