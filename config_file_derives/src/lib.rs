use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Error};

#[proc_macro_derive(ConfigFile, attributes(config_file_ext))]
pub fn derive_config_file(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match impl_config_file(&input) {
        Ok(output) => output.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn impl_config_file(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;

    if !input.generics.params.is_empty() {
        return Err(Error::new_spanned(
            input,
            "ConfigFile cannot be derived for generic types",
        ));
    }

    let config_file_ext = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("config_file_ext"))
        .map(|attr| attr.parse_args::<syn::LitStr>())
        .transpose()?
        .map(|lit| lit.value())
        .unwrap_or_else(|| "toml".to_string());

    let wrapper_type = match config_file_ext.as_str() {
        "json" => quote! { config_file_types::json::JsonConfigFileWrapper },
        "toml" => quote! { config_file_types::toml::TomlConfigFileWrapper },
        _ => {
            return Err(Error::new_spanned(
                input,
                "Unsupported format. Use either 'json' or 'toml'",
            ))
        }
    };

    Ok(quote! {
        impl #name {
            // read from file
            pub fn load(path: &str, ignore_error: bool) -> Option<Self> {
                match #wrapper_type::<Self>::load(path, ignore_error) {
                    None => {
                        if !ignore_error {
                            None
                        } else {
                            let mut c = Self::default();
                            c.path = path.to_string();
                            Some(c)
                        }
                    },
                    Some(mut c) => {
                        c.path = path.to_string();
                        Some(c)
                    }
                }
            }

            // read from str
            pub fn loads(text: &str, ignore_error: bool) -> Option<Self> {
                #wrapper_type::<Self>::loads(text, ignore_error)
            }

            // write to file
            pub fn dump(&self, ignore_error: bool) -> bool {
                #wrapper_type::<Self>::dump_data(self, &self.path, ignore_error)
            }

            // write to str
            pub fn dumps(&self, ignore_error: bool) -> String {
                #wrapper_type::<Self>::dumps_data(self, ignore_error)
            }
        }
    })
}
