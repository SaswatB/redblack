use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    visit_mut::VisitMut,
    Lifetime, Token, Type,
};

#[allow(dead_code)]
struct TypeWithLifetime {
    ty: Type,
    comma: Token![,],
    lifetime: Lifetime,
}

impl Parse for TypeWithLifetime {
    fn parse(input: ParseStream) -> syn::Result<Self> { Ok(TypeWithLifetime { ty: input.parse()?, comma: input.parse()?, lifetime: input.parse()? }) }
}

struct ReplaceStatic {
    new_lifetime: Lifetime,
}

impl VisitMut for ReplaceStatic {
    fn visit_lifetime_mut(&mut self, lt: &mut Lifetime) {
        if lt.ident == "static" {
            *lt = self.new_lifetime.clone();
        }
        syn::visit_mut::visit_lifetime_mut(self, lt);
    }
}

#[proc_macro]
pub fn replace_lifetime(input: TokenStream) -> TokenStream {
    let TypeWithLifetime { mut ty, lifetime, .. } = parse_macro_input!(input as TypeWithLifetime);
    let mut replacer = ReplaceStatic { new_lifetime: lifetime };
    replacer.visit_type_mut(&mut ty);
    TokenStream::from(quote! { #ty })
}
