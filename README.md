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

## Known issues
Somehow, the proc macro causes a strange issue that reports "range end index 4 out of range for slice of length 3".
I have no idea what this is referring to, and it's not a true compile error, as it compiles and runs fine. It appears 
that only rust-analyzer has a problem. If this is something I can fix, please let me know (open an issue on github)
and I will fix it, but as it stands I think this is just a bug in rust-analyzer.