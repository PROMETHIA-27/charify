# Charify

A simple proc macro to turn its token input into a char, similar to the built-in stringify! macro.

## Examples

```
println!("Hell{}, w{}rld!", charify!(o), charify!(o));
```
Result: 
"Hello, world!"

```
println!("What{} Outrageous{}", charify!(?), charify!(!))
```
Result:
"What? Outrageous!"
