use quote::quote;

use crate::types::Required;
use crate::utils::{quote_code, quote_message};

pub fn required_tokens(
    required: Required,
    field_name: &proc_macro2::TokenStream,
    field_name_str: &str,
) -> proc_macro2::TokenStream {
    let message = quote_message(required.message);
    let code = quote_code(required.code, "required");

    quote! {
        if !#field_name.validate_required() {
            #code
            #message
            err.add_param(::std::borrow::Cow::from("value"), &#field_name);
            errors.add(#field_name_str, err);
        }
    }
}
