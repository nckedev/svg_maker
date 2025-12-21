use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(TextElement)]
pub fn derive_text_element(tokens: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(tokens).unwrap();
    let ident = ast.ident;

    quote! {
        impl TextElement for #ident {}
    }
    .into()
}

#[proc_macro_derive(BaseStyle)]
pub fn derive_base_style(tokens: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(tokens).unwrap();
    let ident = ast.ident;

    quote! {
        impl BaseStyle for #ident {}
    }
    .into()
}

#[proc_macro_derive(ClosedShape)]
pub fn derive_closed_shape(tokens: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(tokens).unwrap();
    let ident = ast.ident;

    quote! {
        impl ClosedShape for #ident {}
    }
    .into()
}

#[proc_macro_derive(OpenEndedShape)]
pub fn derive_open_shape(tokens: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(tokens).unwrap();
    let ident = ast.ident;

    quote! {
        impl OpenEndedShape for #ident {}
    }
    .into()
}

#[proc_macro_derive(BaseElement)]
pub fn derive_base_element(tokens: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(tokens).unwrap();
    let ident = ast.ident;

    quote! {
        impl BaseElement for #ident {}
    }
    .into()
}

#[proc_macro_derive(Animate)]
pub fn derive_animate(tokens: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(tokens).unwrap();
    let ident = ast.ident;

    quote! {
        impl Animate for #ident {}
    }
    .into()
}

#[proc_macro_derive(Hx)]
pub fn derive_hx(tokens: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(tokens).unwrap();
    let ident = ast.ident;

    quote! {
        impl Hx for #ident {}
    }
    .into()
}
