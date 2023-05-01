use proc_macro::TokenStream;

#[proc_macro_derive(Database)]
pub fn derive_database(_input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(_input as syn::DeriveInput);
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
            let ty = &field.ty;
            names.push(name);

            let index = syn::Index::from(i);
            values.push(quote::quote! {
                #ty::custom_parse(input[#index])?
            });
        }

        let output = quote::quote! {
            impl Deserializer for #name {
                fn deserialize_raw(input: Vec<&str>) -> anyhow::Result<Self> {
                    use planetscale_driver::Parser;

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

        TokenStream::from(output)
    } else {
        panic!("Only structs are supported")
    }
}

#[proc_macro_derive(DatabaseJSON)]
pub fn derive_database_json(_input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(_input as syn::DeriveInput);
    let name = &input.ident;

    if let syn::Data::Struct(_) = &input.data {
        let output = quote::quote! {
            impl planetscale_driver::Parser for #name {
                fn custom_parse(input: &str) -> anyhow::Result<Self> {
                    serde_json::from_str(input).map_err(|e| e.into())
                }
            }

            impl ToString for TestJSON {
                fn to_string(&self) -> String {
                    serde_json::to_string(self).unwrap()
                }
            }
        };

        TokenStream::from(output)
    } else {
        panic!("Only structs are supported")
    }
}
