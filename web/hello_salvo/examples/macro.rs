use proc_macro::TokenStream;

#[proc_macro]
pub fn my_macro(tokens: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(tokens as syn::Expr);
    let output = format!("The input expression is: {:?}", input);

    output.into()
}

fn main() {
    println!("{}", my_macro!(123 + 456));
}
