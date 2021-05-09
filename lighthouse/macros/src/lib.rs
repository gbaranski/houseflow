use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn decoder(_args: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = syn::parse_macro_input!(item as ItemFn);
    let attrs = &input.attrs;
    let sig = &mut input.sig;
    let vis = &input.vis;
    let body = &input.block;

    (quote! {
        #(#attrs)*
        #vis #sig {
            if buf.remaining() < Self::MIN_SIZE {
                return Err(DecodeError::InvalidSize {
                    expected: Self::MIN_SIZE,
                    received: buf.remaining(),
                });
            } else {
                #body
            }
        }
    })
    .into()
}
