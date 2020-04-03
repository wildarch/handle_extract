extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(HandleExtract)]
pub fn handle_extract(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = &ast.ident;
    let extracts: Vec<syn::Ident> = match &ast.data {
        // TODO(daagra): Support enums
        syn::Data::Struct(strct) => match &strct.fields {
            // prost should only generate structs with named fields, so we only consider this case.
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
                // Loop over all fields and extract the handles
                #(
                    self.#extracts.extract(handles);
                )*
            }

            fn inject(&mut self, handles: &mut Vec<u64>) {
                // Loop over all fields and inject the handles
                #(
                    self.#extracts.inject(handles);
                )*
            }
        }
    };
    gen.into()
}
