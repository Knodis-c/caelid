use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn persistent_entity(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut output = TokenStream::from(quote! {
        #[skip_serializing_none]
        #[derive(Identifiable, Insertable, Queryable, Serialize, Deserialize, PartialEq, Debug)]
        #[serde(rename_all = "camelCase")]
    });

    output.extend(input);

    output
}
