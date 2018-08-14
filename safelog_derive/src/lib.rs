extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(Loggable)]
pub fn loggable_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let gen = impl_loggable(&ast);

    //gen.parse().unwrap()
    gen
}

fn impl_loggable(ast: &DeriveInput) -> TokenStream {
    println!("{:#?}", ast);
    //ast.parse().unwrap();
    let expanded = quote!{};
    expanded.into()
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test() {
//         #[derive(Loggable)]
//         struct TestLoggable {
//             n_i32: i32,
//             #[safelog(redact)]
//             n_string: String,
//         }
//     }
// }
