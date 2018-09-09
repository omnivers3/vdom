#![recursion_limit = "128"]
#![feature(proc_macro_diagnostic)]
#![feature(box_syntax)]

// #[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate proc_macro2;

// use proc_macro::TokenStream;
use proc_macro2::{ Span, TokenStream };
use syn::parse::{Parse, ParseStream, Result};
// use syn::spanned::Spanned;
// use syn::{Expr, Ident, Lit, Type, Visibility};
use syn::{Expr, Ident};
use std::collections::VecDeque;

// struct Node<'a> {
//     parent: Option<&'a Node<'a>>,
//     kind_ident: Ident,
//     attributes_expr: Option<Expr>,
//     children_expr: Option<Expr>,
// }

struct Context {
    // nodes: VecDeque<&'a Node<'a>>,
    // output: Vec<&'a Node<'a>>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            // input,
            // nodes: VecDeque::<&'a Node<'a>>::new(),
            // output: Vec::new(),
        }
    }
}

impl Parse for Context {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut ctx = Context::new();
        parse_kind(input).map(|_| ctx)
        // parse_node();
        // parse_kind(input).and_then(|kind_ident| {
        //     parse_attributes(kind_ident.to_owned(), input).map(|_attributes| {
        //         Node {
        //             parent: None,
        //             kind_ident,
        //             attributes_expr: None,
        //             children_expr: None,
        //         }
        //     })
        // })
        // let children: Expr = input.parse()?;
        // println!("attr: {:#?}", attributes);
        
        // input.parse::<Token![static]>().map(|_| Node {
        //     kind: "html".to_owned(),
        // })

        // let count: Expr = input.parse()?;
        // Ok(VDom{
        //     count,
        // })
        // Ok (ctx)
    }
}

fn parse_kind(input: ParseStream) -> Result<Ident> {
    let kind: Result<Ident> = input.parse();
    match kind {
        Err (_) => {
            let call_site = Span::call_site();
            call_site.unstable().error("Expected a valid html dom node type").emit();
        },
        Ok (ref _kind) => {
            check_kind(_kind);
        }
    }
    kind
}

fn check_kind(kind: &Ident) {
    if kind == "html" {
        return
    }
    kind.span()
        .unstable()
        .warning("Not a recognized html dom node type")
        .emit();
}

fn parse_attributes(kind: Ident, input: ParseStream) -> Result<Expr> {
    let attributes: Result<Expr> = input.parse();
    if let Err (_) = attributes {
        let message = format!("Missing attributes after vdom node tag");
        kind.span()
            .unstable()
            .error(message.to_owned())
            .emit();
        Err(syn::parse::Error::new(kind.span(), message))
    } else {
        attributes
    }
}

#[proc_macro]
pub fn vdom(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Span::call_site().unstable().warning("VDOM").emit();
    // let input: TokenStream = input.into();
    // let parse_result = syn::parse2(input);
    syn::parse2(input.into())
        // .map_err(|_| proc_macro::TokenStream::new())
        .map(|ctx: Context| {
            Span::call_site().unstable().warning("VDOM").emit();
            TokenStream::from(quote! {
                // var a = 10;
                pub fn blah() -> u64 {
                    10
                }
            })
        })
        .map(|ts| ts.into())
        .map_err(|err| {
            Span::call_site().unstable().error(format!("{:?}", err)).emit();
            err
        })
        .unwrap_or(proc_macro::TokenStream::new())
        // .unwrap()
        
}

// TokenStream::from_iter()
    // .map(|)

// let VDom {
//     count,
// } = 
// let count: Lit = syn::parse(input).unwrap();

// if let Lit::Int(ref _count) = count {
//     if _count.value() == 10 {
//         count.span().unstable().warning("Ten!").emit();
//     }
// }
// if let Expr::Lit(ref count) = count {
//     // if let Lit { ref count } = count {
//         // let count = box count.expr;
//     // if let Lit ({})
//         let count: u32 = count.lit.into();
//         // if let Lit(count.lit == 10 {
//             count.span()
//                 .unstable()
//                 .warning("Ten!")
//                 .emit();
//             return TokenStream::new();
//         // }
//     // }
// }

// let count_ptr = quote_spanned! {count.span() =>
//     #count
// };

// let parse_result = syn::parse(input.into());
// match parse_result {
//     // Err (_) => parse_result,
//     Err (_) => {
//         TokenStream::new()
//     },
//     Ok (node) => {

//         let Node {
//             kind,
//         } = node;//parse_result.unwrap();

//         // let node_ptr = quote_spanned! { node.span() =>

//         // }

//         let output = quote! {
//             #[derive(Debug)]
//             pub struct #kind;

//             pub fn blah() -> u32 {
//                 println!("{:?}", #kind);
//                 42
//             }
//         };

//         TokenStream::from(output)
//     }
// }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn should_() {
//         let dom = vdom! {
//             html [][]
//         };

//         println!("dom: {:?}", dom);
//     }
// }