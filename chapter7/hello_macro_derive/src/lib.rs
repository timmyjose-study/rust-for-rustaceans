extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(SomeTrait)]
pub fn some_trait_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse::<syn::DeriveInput>(input).expect("could not parse input");
    let name = &ast.ident;
    let gen = quote! {
        impl SomeTrait for #name {
            fn do_something() {
                println!("{} is doing something!", stringify!(#name));
            }
        }
    };
    gen.into()
}
