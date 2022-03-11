# Scheme interpreter

This is a small interpreter for the subset of Scheme language.
It supports some simple operations (see [scm_builtins.rs](src/engine/scm_builtins.rs)), conditional operator (`if`), lambdas with variables capturing, defining new variables and functions. Examples of these capabilities are shown in [test.scm](test.scm) file.

## Usage

Interpreter can be built and ran by using cargo as shown below.

```
cargo run -- <file>
```

## Internal structure

This iterpreter uses stack machines for expressions evaluation.
Each expression and called procedure instance has it's own stack.

After parsing, scheme expressions are converted into an intermediate instructions sequence.
When some procedure is stored as a value, it also holds such sequence.
During parsing instructions are emitted in reverse order (parser puts instructions into the stack and evaluator extracts them from it).

There are following kinds of instructions:

- `VALUE(val)` - push value into the stack.
- `VARIABLE(name)` - find variable by name and push it's value into the stack.
- `PROC_CALL(proc_name, args_cnt)` - pop `args_cnt` values from the stack and call procedure with these values as arguments.
- `LAMBDA(args, size)` - create new procedure and push it as a value into the stack. In this instruction `args` is a strings array of parameters names and `size` is a count of instructions, that will be moved into the new procedure.
- `TRUE_BRANCH(size)` - pop a value from the stack and skip next `size` instructions if the extracted value is false.
- `FALSE_BRANCH(size)` - skip next `size` instructions.
- `ASSIGN(name)` - pop a value from the stack and assign it to the variable with given name.

Following data types are supported:

- Integer (`i64`)
- Number (`f64`)
- Boolean
- Character
- String
- Symbol
- Pair (can be used for creating lists)
- Nil (empty list)
- Procedure (custom defined or builtin)
