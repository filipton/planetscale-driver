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
        let fields_len = syn::Index::from(fields.len());

        let mut names = Vec::new();
        let mut values = Vec::new();
        for i in 0..fields.len() {
            let field = &fields[i];

            let name = &field.ident;
            names.push(name);

            let index = syn::Index::from(i);
            values.push(quote::quote! {
                input[#index].parse()?
            });
        }

        let output = quote::quote! {
            impl Deserializer for #name {
                fn deserialize_raw(input: Vec<&str>) -> anyhow::Result<Self> {
                    if input.len() != #fields_len {
                        anyhow::bail!("Invalid number of fields");
                    }

                    Ok(Self {
                        #(
                            #names: #values,
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
