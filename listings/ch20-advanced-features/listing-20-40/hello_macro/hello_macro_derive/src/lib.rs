use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Constrói uma representação do código Rust como uma árvore sintática
    // que podemos manipular.
    let ast = syn::parse(input).unwrap();

    // Constrói a implementação da trait.
    impl_hello_macro(&ast)
}
