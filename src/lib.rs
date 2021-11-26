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