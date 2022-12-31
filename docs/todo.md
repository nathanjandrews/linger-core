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
- [ ] support for "else if" statements
- [ ] better error message when there are multiple "else" statements after an
      "if" statement (current message is that "else" is a keyword being used as
      a variable)
- [x] lambda expressions
- [ ] use statements
- [ ] while loops
- [ ] for loops
- [ ] bitwise operations
- [ ] more robust builtin print function with formatting
- [x] escape sequences
- [x] closures (static-scope)

## Bugs

- [x] token row and column data breaks when parsing ID tokens

## Language Optimizations

- [ ] implement for loops as syntactic sugar on top of while loops
- [ ] immediately return error the moment we parse two main procedures

## Potential Code Improvements

- [ ] make keywords and enum
- [ ] use Option<Value> in place of an explicit Void type
- [ ] refactor Token struct to separate TokenValue from its associated data
  - This would allow for a "mode-based" schema system
  - This has the drawback of dealing with A Token's value and data separately
    - could maybe get around this by storing the value and data both as enum
      variants, but that is super redundant

## Testing

## Miscellaneous

- [ ] Address TODO comments in code files
