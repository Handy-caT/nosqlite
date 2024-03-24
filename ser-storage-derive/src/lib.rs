use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields};

#[proc_macro_derive(Storable)]
pub fn derive_storable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    impl_storable(&input)
}

fn impl_storable(input: &DeriveInput) -> TokenStream {
    if let syn::Data::Struct(ref data) = input.data {
        if let Fields::Named(ref fields) = data.fields {
            let field_vals = fields.named.iter().map(|field| {
                let name = &field.ident;
                quote!(__storable_data.push(Box::new(self.#name));)
            });

            let name = input.clone().ident;

            return TokenStream::from(quote!(
            impl serde_storage::ser::Storable for #name {
                fn encode(
                        &self,
                        encoder: serde_storage::ser::SingleItemEncoder
                    ) -> Result<(), serde_storage::ser::Error> {
                        type __Vec = Vec<Box<dyn serde_storage::ser::Storable>>;
                    let mut __storable_data: __Vec = Vec::new();
                    #(#field_vals)*
                    encoder.emit_struct(__storable_data)
                }
            }));
        }
    }

    TokenStream::from(
        syn::Error::new(
            input.ident.span(),
            "Only structs with named fields can derive `Storable`",
        )
        .to_compile_error(),
    )
}
