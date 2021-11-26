/*!

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

*/

extern crate proc_macro;
use proc_macro::{Literal, TokenStream, TokenTree};

#[proc_macro]
pub fn charify(item: TokenStream) -> TokenStream {
    let mut iter = item.into_iter();
    let tok = match iter.next() {
        None => panic!("Must supply one token to charify!"),
        Some(tok) => tok,
    };

    if let Some(_) = iter.next() {
        panic!("Must supply exactly one token to charify!");
    };

    match tok {
        TokenTree::Ident(ident) => {
            let tok_str = ident.to_string();
            if tok_str.len() != 1 {
                panic!("Token length must be exactly one!");
            }
    
            let c = tok_str.chars().next().unwrap();
            let lit = TokenTree::Literal(Literal::character(c));
            let mut new_stream = TokenStream::new();
            new_stream.extend([lit]);
            return new_stream;
        },
        TokenTree::Punct(punct) => {
            let c = punct.as_char();
            let lit = TokenTree::Literal(Literal::character(c));
            let mut new_stream = TokenStream::new();
            new_stream.extend([lit]);
            return new_stream;
        },
        TokenTree::Group(_) => {
            panic!("Token must not be a group!")
        },
        TokenTree::Literal(lit) => {
            if let Ok(digit) = lit.to_string().parse::<i32>() {
                if digit < 0 || digit > 9 { panic!("Number literals must be 0-9!") };

                let c = digit.to_string().chars().next().unwrap();
                let lit = TokenTree::Literal(Literal::character(c));
                let mut new_stream = TokenStream::new();
                new_stream.extend([lit]);
                return new_stream;
            } 
            else { panic!("Literals must be numbers!") }
        },
    }
}