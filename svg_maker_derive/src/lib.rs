use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

macro_rules! impl_derive {
    ($tr:ident, $t:ident) => {
        if let Ok(DeriveInput { ident, .. }) = syn::parse::<DeriveInput>($t) {
            quote! {
                impl crate::marker_traits::$tr for #ident {}
            }
            .into()
        } else {
            quote! {}.into()
        }
    };
}
#[proc_macro_derive(TextElement)]
pub fn derive_text_element(tokens: TokenStream) -> TokenStream {
    impl_derive!(TextElement, tokens)
}

#[proc_macro_derive(BaseStyle)]
pub fn derive_base_style(tokens: TokenStream) -> TokenStream {
    impl_derive!(BaseStyle, tokens)
}

#[proc_macro_derive(ClosedShape)]
pub fn derive_closed_shape(tokens: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(tokens).unwrap();
    let ident = ast.ident;

    quote! {
        impl crate::marker_traits::ClosedShape for #ident {}
        impl crate::marker_traits::Shape for #ident {}
    }
    .into()
}

#[proc_macro_derive(OpenEndedShape)]
pub fn derive_open_shape(tokens: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(tokens).unwrap();
    let ident = ast.ident;

    quote! {
        impl crate::marker_traits::OpenEndedShape for #ident {}
        impl crate::marker_traits::Shape for #ident {}
    }
    .into()
}

#[proc_macro_derive(BaseElement)]
pub fn derive_base_element(tokens: TokenStream) -> TokenStream {
    impl_derive!(BaseElement, tokens)
}

#[proc_macro_derive(Animate)]
pub fn derive_animate(tokens: TokenStream) -> TokenStream {
    impl_derive!(Animate, tokens)
}

#[proc_macro_derive(Hx)]
pub fn derive_hx(tokens: TokenStream) -> TokenStream {
    impl_derive!(Hx, tokens)
}

#[proc_macro_derive(Shape)]
pub fn derive_shape(tokens: TokenStream) -> TokenStream {
    impl_derive!(Shape, tokens)
}

#[proc_macro_derive(ContainerElement)]
pub fn derive_container_element(tokens: TokenStream) -> TokenStream {
    impl_derive!(ContainerElement, tokens)
}

#[proc_macro_derive(ElementKind)]
pub fn derive_element_kind(tokens: TokenStream) -> TokenStream {
    impl_derive!(ElementKind, tokens)
}
