use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Constrói uma representação do código Rust como uma árvore sintática
    // que podemos manipular
    let ast = syn::parse(input).unwrap();

    // Constrói a implementação da trait
    impl_hello_macro(&ast)
}

// ANCHOR: here
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generated = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    generated.into()
}
// ANCHOR_END: here
