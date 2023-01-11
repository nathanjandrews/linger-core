# TODO List for the Linger Programming Language

## URGENT PROBLEMS THAT NEED FIXING

Nothing so far, let's keep it that way.

## Language Features

- [x] add row and column number to Token struct
- [x] check that variables and user-defined procedure names are not keywords
- [x] empty return statements
- [x] boolean expression short-circuiting
- [x] string literals
- [x] string concatenation
- [x] distinction between assignment and initialization
  - [x] variable shadowing
  - [x] variable reassignment
- [ ] type cohesion for concatenation of strings and other types
- [x] support for "else if" statements
  - implement as syntactic sugar on top of simpler "if-else" statements
- [ ] better error message when there are multiple "else" statements after an
      "if" statement (current message is that "else" is a keyword being used as
      a variable)
- [x] lambda expressions
- [ ] use statements
- [x] while loops
  - [x] break keyword
  - [x] continue keyword
- [x] for-loops
- [ ] bitwise operations
- [ ] more robust builtin print function with formatting
- [x] escape sequences
- [x] decimal numbers
- [x] closures (static-scope)
- [x] comments
- [x] increment/decrement unary operators
- [x] assignment operators (+=, -=, etc.)
- [x] consecutive invocation of functions (i.e "sum(x)(y)" should work if "sum"
      returns a function)
- [x] const values
- [ ] lists

## Bugs

- [x] token row and column data breaks when parsing ID tokens

## Language Optimizations

- [x] implement for-loops as syntactic sugar on top of while loops
- [x] immediately return error the moment we parse two main procedures
- [x] implement assignment operators (+=, -=) as syntactic sugar on top of assignment statements

## Potential Code Improvements

- [x] make keywords and enum
- [x] have a separate "top-level procedures" field in the environment
  - this could provide better error handling when trying to reassign top-level procedures
- [ ] make binary operators and unary operators separate enums
- [ ] use Option<Value> in place of an explicit Void type
- [ ] better error messages
  - currently some errors cannot be reached due to the nature of the
      the parser. For example, the expression "*10" will result in an
      "unexpected token" error and not a "unary used as binary" error (However
      this does seem to be the normal behavior in other languages).

## Repository Hygiene

- [ ] document code

## Testing

## Miscellaneous

- [ ] Address TODO comments in code files
