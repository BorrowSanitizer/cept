use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn intercept(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    todo!()
}
