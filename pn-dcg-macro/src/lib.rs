extern crate core;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Literal};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::__private::TokenStream2;
use syn::parse::Parser;
use syn::{ItemStruct, Lit, Meta, NestedMeta, Type};

// #[proc_macro_derive(derefmutHead)]
// pub fn impl_derefmut_head_derive(input: TokenStream) -> TokenStream {
//     // 构建 Rust 代码所代表的语法树
//     // 以便可以进行操作
//     let ast = syn::parse(input).unwrap();
//     // 构建 trait 实现
//     impl_derefmut_head_macro(&ast)
// }
fn impl_derefmut(attr_ty: AttrTy, ty: Type, name: Ident, item: ItemStruct) -> TokenStream {
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
    impl_derefmut(attr_ty, ty, item.ident.clone(), item)
}

// #[proc_macro_attribute]
// pub fn derefmut_zero(attr: TokenStream, item: TokenStream) -> TokenStream {
//     impl_derefmut_zero_derive2(attr.into(), item.into()).into()
// }
// fn impl_derefmut_zero_derive2(
//     attr: proc_macro2::TokenStream,
//     item: proc_macro2::TokenStream,
// ) -> proc_macro2::TokenStream {
//     let ast: syn::DeriveInput = syn::parse2(item).unwrap();
//     let name = &ast.ident;
//     let a = quote! {
//         impl Deref for #name {
//             type Target = #attr;
//             fn deref(&self) -> &Self::Target {
//                 &self.head
//             }
//         }
//         impl DerefMut for #name {
//             fn deref_mut(&mut self) -> &mut Self::Target {
//                 &mut self.head
//             }
//         }
//     };
//     a
// }

#[test]
fn test_parse() {
    let input = quote! {
        #[abc(0, abc)]
        struct A;
    };
    let a: ItemStruct = syn::parse2(input).unwrap();
    println!("{:?}", a);
} //
  //
#[test]
fn test_derive_packet_mid() {
    let input = quote! {
        struct A {
            b: i32
        }
    };
    let a: ItemStruct = syn::parse2(input).unwrap();
    for field in a.fields {
        let c: Ident = field.ident.unwrap();
        let b = Ident::new("b", c.span());
        if c == b {
            if let Type::Path(ty) = field.ty {
                println!("{:?}", ty.path);
                // let path = ty.path;
                let input = quote! {
                    #ty
                };
                println!("{}", input.to_string());
            }
        }
    }
    // println!("{:?}", a);
} //

#[test]
fn test_derive_packet_mid2() {
    let input = quote! {
        #[abc(0)]
        #[abc(head)]
        struct A;
    };
    let a: ItemStruct = syn::parse2(input).unwrap();
    println!("{:?}", a);
    for field in a.fields {
        let c: Ident = field.ident.unwrap();
        let b = Ident::new("b", c.span());
        if c == b {
            if let Type::Path(ty) = field.ty {
                println!("{:?}", ty.path);
                // let path = ty.path;
                let input = quote! {
                    #ty
                };
                println!("{}", input.to_string());
            }
        }
    }
    // println!("{:?}", a);
} //

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

// #[test]
// fn test_resolve_struct_field() {
//     let struct_stream = quote! {
//         struct A {
//             a: usize
//         }
//     };
//     let item: ItemStruct = syn::parse2(struct_stream).unwrap();
//     let field_ident = get_ident_tmp();
//     get_field_ty(item, field_ident);
// }
// #[test]
// fn test_resolve_struct_0() {
//     let struct_stream = quote! {
//         struct A(usize, i32);
//     };
//     let item: ItemStruct = syn::parse2(struct_stream).unwrap();
//     println!("{:?}", item);
//     let field_ident = get_ident_tmp();
//     get_field_ty(item, field_ident);
// }
//
// fn get_ident_tmp() -> Ident {
//     let attr_stream = quote! {
//         a
//     };
//     let attr_vals = AttrAlisa::parse_terminated.parse2(attr_stream).unwrap();
//     for attr_val in attr_vals.iter() {
//         match attr_val {
//             NestedMeta::Meta(Meta::Path(meta)) => {
//                 return meta.get_ident().unwrap().clone();
//             }
//             a => {
//                 panic!("");
//             }
//         }
//     }
//     panic!("");
// }
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

// 解析属性宏中属性
#[test]
fn test_resolve_attr() {
    let attr_stream = quote! {
        0,abc
    };
    let attr_vals = AttrAlisa::parse_terminated.parse2(attr_stream).unwrap();
    // println!("{:?}", attr_vals);
    for attr_val in attr_vals.iter() {
        match attr_val {
            NestedMeta::Meta(Meta::Path(meta)) => {
                println!("Meta {:?}", meta);
                println!("Meta {:?}", meta.get_ident().unwrap())
            }
            NestedMeta::Lit(Lit::Int(lit)) => {
                println!("Lit::Int {:?}", lit.base10_parse::<usize>().unwrap());
                println!("Lit::Int {:?}", lit)
            }
            a => {
                println!("other {:?}", a)
            }
        }
    }
}

#[test]
fn test_resolve_fn() {
    use syn::File;
    let attr_stream = quote! {
        impl Deref for PacketIdentReq {
            type Target = ();
            fn deref(&self) -> &Self::Target {
                todo!()
            }
        }
    };
    let file: File = syn::parse2(attr_stream).unwrap();
    println!("{:?}", file);
    // let attr_vals = AttrAlisa::parse_terminated.parse2(attr_stream).unwrap();
    // // println!("{:?}", attr_vals);
    // for attr_val in attr_vals.iter() {
    //     match attr_val {
    //         NestedMeta::Meta(Meta::Path(meta)) => {
    //             println!("Meta {:?}", meta);
    //             println!("Meta {:?}", meta.get_ident().unwrap())
    //         }
    //         NestedMeta::Lit(Lit::Int(lit)) => {
    //             println!("Lit::Int {:?}", lit.base10_parse::<usize>().unwrap());
    //             println!("Lit::Int {:?}", lit)
    //         }
    //         a => {
    //             println!("other {:?}", a)
    //         }
    //     }
    // }
}
