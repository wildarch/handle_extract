extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

fn impl_handle_extract(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let extracts: Vec<syn::Ident> = match &ast.data {
        syn::Data::Struct(strct) => match &strct.fields {
            syn::Fields::Named(fields) => {
                fields.named.iter().flat_map(|f| f.ident.clone()).collect()
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };

    let gen = quote! {
        impl HandleExtract for #name {
            fn extract(&mut self, handles: &mut Vec<u64>) {
                #(
                    self.#extracts.extract(handles);
                )*
            }

            fn inject(&mut self, handles: &mut Vec<u64>) {
                #(
                    self.#extracts.inject(handles);
                )*
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(HandleExtract)]
pub fn handle_extract(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    impl_handle_extract(&ast)
}
