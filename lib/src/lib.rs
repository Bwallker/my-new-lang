use proc_macro;
use syn;
use proc_macro::TokenStream;
    #[proc_macro_derive(EnumStr, attributes(strum))]
    pub fn make_func(token_stream: TokenStream) -> TokenStream {
        let ast: syn::DeriveInput = syn::parse(token_stream).unwrap();
        return "fn func() -> std::string::String { std::string::String::from(\"it works!\") }".parse().unwrap();
    }
