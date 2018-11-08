#![recursion_limit = "512"]

#[macro_use]
extern crate quote;

#[macro_use]
extern crate syn;

extern crate genet_abi;
extern crate inflector;
extern crate proc_macro;

use inflector::cases::camelcase::to_camel_case;
use proc_macro::TokenStream;
use syn::{Data, DataStruct, DeriveInput, Fields, Ident};

mod meta;
use meta::AttrMetadata;

mod initialisms;
use initialisms::to_title_case;

#[proc_macro_derive(Attr, attributes(genet))]
pub fn derive_attr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Struct(s) => parse_struct(&input, &s),
        _ => panic!("Attr derive supports struct types only"),
    }
}

fn normalize_ident(ident: &Ident) -> String {
    let ident = ident.to_string();
    ident.trim_start_matches("r#").into()
}

fn parse_struct(input: &DeriveInput, s: &DataStruct) -> TokenStream {
    let mut fields_ident = Vec::new();
    let mut fields_ctx = Vec::new();
    let mut fields_aliases = Vec::new();

    if let Fields::Named(f) = &s.fields {
        for field in &f.named {
            if let Some(ident) = &field.ident {
                let meta = AttrMetadata::parse(&field.attrs);
                let id = normalize_ident(&ident);
                let id = format!(".{}", to_camel_case(&id));
                let typ = meta.typ;
                let name = if meta.name.is_empty() {
                    to_title_case(&id)
                } else {
                    meta.name
                };
                let desc = meta.description;
                fields_ctx.push(quote!{
                    AttrContext{
                        path: format!("{}{}", ctx.path, #id)
                            .trim_matches('.').into(),
                        typ: #typ.into(),
                        name: #name.into(),
                        description: #desc.into(),
                        ..Default::default()
                    }
                });
                fields_ident.push(ident);

                for name in meta.aliases {
                    fields_aliases.push(quote!{
                        (format!("{}", #name), format!("{}{}", ctx.path, #id))
                    });
                }
            }
        }
    }

    let self_attrs = AttrMetadata::parse(&input.attrs);
    let self_name = self_attrs.name;
    let self_desc = self_attrs.description;

    let ident = &input.ident;
    let fields_ident2 = fields_ident.clone();
    let tokens = quote!{
        impl genet_sdk::attr::AttrField for #ident {
            fn init(&mut self, ctx: &::genet_sdk::attr::AttrContext)
                -> genet_sdk::attr::AttrList {
                use genet_sdk::attr::{Attr, SizedAttrField, AttrList, AttrContext, AttrClass};
                use genet_sdk::cast;
                use genet_sdk::fixed::Fixed;

                let mut class = None;
                let mut bit_offset = ctx.bit_offset;
                let mut attrs = Vec::new();
                let mut children = Vec::new();
                let mut aliases = vec![
                    #( #fields_aliases ),*
                ];

                #(
                    {
                        let mut subctx = #fields_ctx;
                        subctx.bit_offset = bit_offset;
                        let attr : &mut SizedAttrField = &mut self.#fields_ident;
                        let size = attr.bit_size();
                        if ctx.detached || size == 0 {
                            subctx.detached = true
                        }
                        let mut child = attr.init(&subctx);
                        if size > 0 {
                            attrs.push(
                                Attr::builder(child.class.clone())
                                    .bit_range(0, bit_offset..(bit_offset + size))
                                    .build()
                            );
                        }
                        bit_offset += size;

                        children.push(child.class.clone());
                        children.append(&mut child.children);
                        if (!subctx.detached) {
                            attrs.append(&mut child.attrs);
                        }
                        aliases.append(&mut child.aliases);
                    }
                )*

                AttrList {
                    class: class.unwrap_or_else(|| Fixed::new(
                        AttrClass::builder(ctx.path.clone())
                        .cast(cast::ByteSlice())
                        .typ(ctx.typ.clone())
                        .name(if ctx.name.is_empty() {
                            #self_name
                        } else {
                            ctx.name
                        })
                        .description(if ctx.description.is_empty() {
                            #self_desc
                        } else {
                            ctx.description
                        })
                        .build()
                    )),
                    children,
                    attrs,
                    aliases,
                }
            }
        }

        impl genet_sdk::attr::SizedAttrField for #ident {
            fn bit_size(&self) -> usize {
                use genet_sdk::attr::{SizedAttrField, AttrField};
                let mut size = 0;

                #(
                    {
                        let attr : &SizedAttrField = &self.#fields_ident2;
                        size += attr.bit_size();
                    }
                )*

                size
            }
        }
    };

    tokens.into()
}
