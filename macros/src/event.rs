use proc_macro::TokenStream;
use quote::*;
use syn::{parse_macro_input, ItemStruct};

///
/// Convert struct to abci::event
///
pub fn event(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as ItemStruct);

    let struct_name = parsed.ident.clone();
    let name = struct_name.to_string();

    let mut index_is_false_str = String::new();
    let mut count = 0;

    let mut key_vec = vec![];
    let mut key_str_vec = vec![];
    let mut index_vec = vec![];

    parsed.fields.iter().for_each(|f| {
        let mut index = false;
        f.attrs.iter().for_each(|a| {
            a.path
                .segments
                .iter()
                .for_each(|s| match s.ident.to_string().as_str() {
                    "abcf" => {
                        index_is_false_str += &*(count.to_string() + ",");
                        index = true;
                    }
                    _ => {}
                });
        });
        key_vec.push(f.ident.as_ref());
        key_str_vec.push(f.ident.clone().unwrap().to_string());
        index_vec.insert(count, index);
        count += 1;
    });

    let expanded = quote! {

        impl abcf::Event for #struct_name {
            fn to_abci_event(&self) -> abcf::Result<abcf::tm_protos::abci::Event> {

                let mut attributes = Vec::new();

                #(
                    let key_byte = #key_str_vec.as_bytes().to_vec();

                    let value_byte = self.#key_vec.to_value_bytes()?;
                    let index = #index_vec;

                    let a = abcf::tm_protos::abci::EventAttribute{
                        key: key_byte,
                        value: value_byte,
                        index,
                    };
                    attributes.push(a);

                )*

                Ok(abcf::tm_protos::abci::Event {
                    r#type: self.name().to_string(),
                    attributes,
                })
            }

            fn name(&self) -> &str {
                #name
            }

            fn from_abci_event(&mut self, e: abcf::tm_protos::abci::Event) -> abcf::Result<()> {
                Ok(())
            }

            fn from_abci_event_string(&mut self, str: String) -> abcf::Result<()> {
                let event = serde_json::from_str::<#struct_name>(&str)?;
                *self = event;
                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}
