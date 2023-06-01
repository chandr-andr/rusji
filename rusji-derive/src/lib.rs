use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(ViewWrapper)]
pub fn view_wrapper_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_view_weapper(&ast)
}

fn impl_view_weapper(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl ViewWrapper for #name {
            type V = NamedView<Dialog>;

            fn with_view<F, R>(&self, f: F) -> Option<R>
            where
                F: FnOnce(&Self::V) -> R,
            {
                Some(f(&self.inner_view))
            }

            fn with_view_mut<F, R>(&mut self, f: F) -> Option<R>
            where
                F: FnOnce(&mut Self::V) -> R,
            {
                Some(f(&mut self.inner_view))
            }

            fn wrap_call_on_any<'a>(
                &mut self,
                selector: &cursive::view::Selector<'_>,
                callback: cursive::event::AnyCb<'a>,
            ) {
                self.with_view_mut(|v| v.call_on_any(selector, callback));
            }
        }
    };
    gen.into()
}
