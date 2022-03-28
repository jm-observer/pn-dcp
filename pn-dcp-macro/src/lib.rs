use proc_macro::TokenStream;
use proc_macro2::{Ident, Literal};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::__private::TokenStream2;
use syn::parse::Parser;
use syn::{ItemStruct, Lit, Meta, NestedMeta, Type};

fn impl_derefmut(attr_ty: AttrTy, name: Ident, item: ItemStruct) -> TokenStream {
    let gen = quote! {
        #item
        impl DerefMut for #name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.#attr_ty
            }
        }
    };
    gen.into()
}

fn impl_deref(attr_ty: AttrTy, ty: Type, name: Ident, item: ItemStruct) -> TokenStream {
    let gen = quote! {
        #item
        impl Deref for #name {
            type Target = #ty;

            fn deref(&self) -> &Self::Target {
                &self.#attr_ty
            }
        }
        impl DerefMut for #name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.#attr_ty
            }
        }
    };
    gen.into()
}

type AttrAlisa = syn::punctuated::Punctuated<syn::NestedMeta, syn::Token![,]>;
#[proc_macro_attribute]
pub fn derefmut(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_ty = resolve_attr(attr.into());
    let item: ItemStruct = syn::parse(item).unwrap();
    let ty = get_field_ty(&item, attr_ty.clone());
    match &attr_ty {
        AttrTy::Ident(_) => {
            impl_derefmut(attr_ty, item.ident.clone(), item)
        }
        AttrTy::Index(_) => {
            impl_deref(attr_ty, ty, item.ident.clone(), item)
        }
    }

}

fn get_field_ty(a: &ItemStruct, right: AttrTy) -> Type {
    match right {
        AttrTy::Index(index_right) => {
            for (index, field) in a.fields.iter().enumerate() {
                if index_right == index {
                    return field.ty.clone();
                }
            }
            panic!("字段索引越界！")
        }
        AttrTy::Ident(ref ident) => {
            for field in a.fields.iter() {
                if let Some(c) = field.ident.as_ref() {
                    if c == ident {
                        return field.ty.clone();
                    }
                }
            }
            panic!("无法找到字段！")
        }
    }
}
#[derive(Clone)]
enum AttrTy {
    Index(usize),
    Ident(Ident),
}

impl ToTokens for AttrTy {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            AttrTy::Ident(ident) => ident.to_tokens(tokens),
            AttrTy::Index(index) => {
                let lit = Literal::usize_unsuffixed(*index);
                tokens.append(lit)
            }
        }
    }
}

fn resolve_attr(attr: TokenStream2) -> AttrTy {
    let attr_vals = AttrAlisa::parse_terminated.parse2(attr).unwrap();
    // println!("{:?}", attr_vals);
    for attr_val in attr_vals.iter() {
        match attr_val {
            NestedMeta::Meta(Meta::Path(meta)) => {
                return AttrTy::Ident(meta.get_ident().unwrap().clone());
            }
            NestedMeta::Lit(Lit::Int(lit)) => {
                return AttrTy::Index(lit.base10_parse::<usize>().unwrap());
            }
            _ => {
                panic!("无法解析属性值：只能为字段索引或者字段名称")
            }
        }
    }
    panic!("必须设置属性值（字段索引或者字段名称）")
}
