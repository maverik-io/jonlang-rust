# jonlang
# What is it?
jonlang is an easy programming language built as a joke.
# How do I use it?
jonlang has pretty simple rules -:

- All programs must begin with `hi, jon !` and end with `bye, jon !`
- All lines must start with `jon,` and end with an exclamation point `!`
## Commands

### `say` - output and input
| Command  | Description | Usage Examples |
|-|-|-|
|say \<msg>      | prints messages onto the console| `jon, say "Hello, World" !`|
say \<msg> and read into \<varname>| prints a prompt onto the console and gets input into the specified variable| `jon, say "Name: " and read into name as text!`|
say \<msg> and read aloud \<varname> | prints a message and the value of a variable | `jon, say "Your name is :" and read aloud name !`|
---
> [!TIP]
> The read into operation needs a type that can be `number` or `text` 

### `remember` - variable control
| Command | Description | Usage Examples |
|-|-|-|
|remember that \<varname> is \<value> | sets variables | `jon, remember that pi is 3.14 !`|
|remember that \<varname> will be \<expression>| do math with variables| `jon, remember that s_square will be s times s`|
----

> [!TIP]
> The above expression takes 4 keywords for math:
> |operation|keyword|
> |-|-|
> | Addition | plus|
> | Subtraction | minus |
> | Multiplication | times |
> | Division | by |




