# TODO List for the Linger Programming Language

## URGENT PROBLEMS THAT NEED FIXING

Nothing so far, let's keep it that way.

## Language Features

- [x] add row and column number to Token struct
- [x] check that variables and user-defined procedure names are not keywords
- [x] empty return statements
- [x] boolean expression short-circuiting
- [ ] string literals
- [ ] lambda expressions
- [ ] use statements
- [ ] while loops
- [ ] for loops
- [ ] bitwise operations
- [ ] more robust builtin print function with formatting

## Bugs

- [ ] token row and column data breaks when parsing ID tokens

## Language Optimizations

- [ ] implement for loops as syntactic sugar on top of while loops
- [ ] immediately return error the moment we parse two main procedures

## Potential Code Improvements

- [ ] use Option<Value> in place of an explicit Void type
- [ ] refactor Token struct to separate TokenValue from its associated data
  - This would allow for a "mode-based" schema system
  - This has the drawback of dealing with A Token's value and data separately
    - could maybe get around this by storing the value and data both as enum
      variants, but that is super redundant

## Testing

## Miscellaneous

- [ ] Address TODO comments in code files
