extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn py(tokens: TokenStream) -> TokenStream {
    let mut buff = String::from("");
    for token in tokens.into_iter() {
        match token {
            proc_macro::TokenTree::Group(g) => {
                println!("group {}", g);
            },

            proc_macro::TokenTree::Ident(i) => {
                println!("identification {}", i);
                let id = i.to_string();
                if id == "def" {
                    buff.push_str("fn"); // maybe something with recursion; do recursive macros work?
                    
                }
                buff.push_str(format!("let {}", i.to_string().as_str()).as_str());
            },

            proc_macro::TokenTree::Punct(p) => {
                println!("punctuation {}", p);
                let punc = p.as_char();
                buff.push(punc);
            },

            proc_macro::TokenTree::Literal(l) => {
                println!("literal {}", l);
                buff.push_str(l.to_string().as_str());
            },
        }
    }
    buff.push(';');
    println!("buff: {}", buff);
    // buff.parse().unwrap()
    "let i = 0;".parse().unwrap()
}