use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(ImplDerefMutHead)]
pub fn impl_derefmut_head_derive(input: TokenStream) -> TokenStream {
    // 构建 Rust 代码所代表的语法树
    // 以便可以进行操作
    let ast = syn::parse(input).unwrap();
    // 构建 trait 实现
    impl_derefmut_head_macro(&ast)
}
fn impl_derefmut_head_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Deref for #name {
            type Target = DcgHead;

            fn deref(&self) -> &Self::Target {
                &self.head
            }
        }
        impl DerefMut for #name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.head
            }
        }
    };
    gen.into()
}
#[proc_macro_attribute]
pub fn derefmut_zero(attr: TokenStream, item: TokenStream) -> TokenStream {
    impl_derefmut_zero_derive2(attr.into(), item.into())
}
pub fn impl_derefmut_zero_derive2(
    attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> TokenStream {
    // 构建 Rust 代码所代表的语法树
    // 以便可以进行操作
    let ast = syn::parse(input).unwrap();
    // 构建 trait 实现
    impl_derefmut_head_macro(&ast)
}
fn impl_derefmut_zero_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Deref for #name {
            type Target = DcgHead;

            fn deref(&self) -> &Self::Target {
                &self.head
            }
        }
        impl DerefMut for #name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.head
            }
        }
    };
    gen.into()
}
