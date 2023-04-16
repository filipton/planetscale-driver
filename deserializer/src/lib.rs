use proc_macro::TokenStream;

#[proc_macro_derive(Database)]
pub fn derive_database(_input: TokenStream) -> TokenStream {
    let input = _input.clone();
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;

    if let syn::Data::Struct(data) = &input.data {
        let fields = match &data.fields {
            syn::Fields::Named(fields) => &fields.named,
            _ => panic!("Only named fields are supported"),
        };

        let mut field_names = Vec::new();
        let mut field_values = Vec::new();

        for i in 0..fields.len() {
            let field = &fields[i];

            let name = &field.ident;
            field_names.push(name);

            let index = syn::Index::from(i);
            field_values.push(quote::quote! {
                input.get(#index)
                    .map_or_else(|| anyhow::bail!("Index out of range"), |v| Ok(v))?.parse()?
            });
        }

        let output = quote::quote! {
            impl Deserializer for #name {
                fn deserialize_raw(input: Vec<&str>) -> anyhow::Result<Self> {
                    Ok(Self {
                        #(
                            #field_names: #field_values,
                        )*
                    })
                }
            }
        };

        return TokenStream::from(output);
    } else {
        panic!("Only structs are supported")
    }
}
