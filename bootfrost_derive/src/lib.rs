extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;
use proc_macro::TokenStream;

use core::mem::size_of_val;


#[proc_macro_derive(TotalSize)]
pub fn total_size(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    // Build the impl
    let gen = impl_total_size(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_total_size(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl TotalSize for #name {
            fn total_size(&self) -> usize {
                return core::mem::size_of_val(self);
            }
        }
    }
}