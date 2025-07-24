use derive_new::new;
use quote::{ToTokens, quote};

pub type BuilderResult<T> = Result<T, BuilderError>;

#[derive(new)]
pub struct BuilderError {
    message: String,
}

impl ToTokens for BuilderError {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self { message } = self;

        quote! {
            compile_error! { #message }
        }
        .to_tokens(tokens);
    }
}

#[macro_export(local_inner_macros)]
macro_rules! builder_error {
    ($($tt:tt)*) => {
        BuilderError::new(std::format!($($tt)*))
    };
}
