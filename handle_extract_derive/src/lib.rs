extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(HandleExtract)]
pub fn handle_extract(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = &ast.ident;
    match &ast.data {
        syn::Data::Struct(data) => struct_impls(name, data),
        syn::Data::Enum(data) => enum_impls(name, data),
        _ => unimplemented!(),
    }
}

fn struct_impls(name: &syn::Ident, data: &syn::DataStruct) -> TokenStream {
    let accessors: Vec<syn::Ident> = match &data.fields {
        // prost should only generate structs with named fields, so we only consider this case.
        syn::Fields::Named(fields) => fields.named.iter().flat_map(|f| f.ident.clone()).collect(),
        _ => unimplemented!(),
    };

    let gen = quote! {
        impl ::handle_extract::HandleExtract for #name {
            fn extract(&mut self, handles: &mut Vec<u64>) {
                // Loop over all fields and extract the handles
                #(
                    self.#accessors.extract(handles);
                )*
            }

            fn inject(&mut self, handles: &mut Vec<u64>) {
                // Loop over all fields and inject the handles
                #(
                    self.#accessors.inject(handles);
                )*
            }
        }
    };
    gen.into()
}

fn enum_impls(name: &syn::Ident, data: &syn::DataEnum) -> TokenStream {
    let variants: Vec<syn::Ident> = data
        .variants
        .iter()
        .map(|variant| {
            match &variant.fields {
                // Prost should only generate a single unnamed field, check to make sure.
                syn::Fields::Unnamed(fields) => assert_eq!(fields.unnamed.len(), 1),
                _ => unimplemented!(),
            }
            variant.ident.clone()
        })
        .collect();

    let gen = quote! {
        impl ::handle_extract::HandleExtract for #name {
            fn extract(&mut self, handles: &mut Vec<u64>) {
                match self {
                    #(
                        #name::#variants(v) => v.extract(handles),
                    )*
                }
            }

            fn inject(&mut self, handles: &mut Vec<u64>) {
                match self {
                    #(
                        #name::#variants(v) => v.inject(handles),
                    )*
                }
            }
        }
    };
    gen.into()
}
