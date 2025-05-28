use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_quote, Data, DeriveInput, Fields, Type};

#[cfg(test)]
mod tests;

/// A derive macro to emit a Godot-compatible resource struct + impls for a pure Rust struct.
#[proc_macro_derive(AsGdRes, attributes(export, init))]
pub fn as_gd_res_derive(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as DeriveInput);
    TokenStream::from(expand_as_gd_res(derive_input))
}

fn expand_as_gd_res(input: DeriveInput) -> proc_macro2::TokenStream {
    let name = input.ident;
    let res_name = format_ident!("{}Resource", name);

    match input.data {
        Data::Struct(data) => {
            let mut defs = Vec::new();
            let mut extracts = Vec::new();
            for field in data.fields.iter() {
                if let Some(ident) = &field.ident {
                    // clone export/init attrs or inject #[export]
                    let mut attrs = field
                        .attrs
                        .iter()
                        .filter(|a| a.path().is_ident("export") || a.path().is_ident("init"))
                        .cloned()
                        .collect::<Vec<_>>();
                    if attrs.is_empty() {
                        attrs.push(parse_quote!(#[export]));
                    }
                    let ty = &field.ty;
                    defs.push(quote! {
                        #(#attrs)*
                        pub #ident: <#ty as ::as_gd_res::AsGdRes>::ResType,
                    });
                    extracts.push(quote! {
                        #ident: self.#ident.extract(),
                    });
                }
            }

            quote! {
                impl ::as_gd_res::AsGdRes for #name {
                    type ResType = ::godot::prelude::OnEditor<::godot::obj::Gd<#res_name>>;
                }
                impl ::as_gd_res::AsGdResOpt for #name {
                    type GdOption = Option<::godot::obj::Gd<#res_name>>;
                }
                impl ::as_gd_res::AsGdResArray for #name {
                    type GdArray = ::godot::prelude::Array<::godot::obj::Gd<#res_name>>;
                }

                #[derive(::godot::prelude::GodotClass)]
                #[class(tool,init,base=Resource)]
                pub struct #res_name {
                    #[base]
                    base: ::godot::obj::Base<::godot::classes::Resource>,
                    #(#defs)*
                }

                impl ::as_gd_res::ExtractGd for #res_name {
                    type Extracted = #name;
                    fn extract(&self) -> Self::Extracted {
                        Self::Extracted {
                            #(#extracts)*
                        }
                    }
                }
            }
        }
        Data::Enum(data) => {
            // check enum variant shapes
            let all_unit = data
                .variants
                .iter()
                .all(|v| matches!(&v.fields, Fields::Unit));
            let all_tuple1 = data
                .variants
                .iter()
                .all(|v| matches!(&v.fields, Fields::Unnamed(u) if u.unnamed.len()==1));

            if all_unit {
                quote! {


                compile_error!(
                    "`derive(AsGdRes)` only or enums with single-tuple variants, not unit variants. Did you mean to use `derive(AsSimpleGdEnum)`?"
                );
                        // impl ::as_gd_res::AsGdRes for #name {
                        //     type ResType = #name;
                        // }
                        // impl ::as_gd_res::AsGdResArray for #name {
                        //     type GdArray = ::godot::prelude::Array<::godot::obj::Gd<#name>>;
                        // }
                        // impl ExtractGd for #name {
                        //     type Extracted = #name;
                        //     fn extract(&self) -> Self::Extracted {
                        //         self.clone()
                        //     }
                        // }
                    }
            } else if all_tuple1 {
                let dyn_trait = format_ident!("{}ResourceExtractVariant", name);
                let mut variant_impls = Vec::new();
                for var in &data.variants {
                    if let Fields::Unnamed(fields) = &var.fields {
                        let var_ident = &var.ident;
                        let ty = &fields.unnamed[0].ty;
                        let variant_res = match ty {
                            Type::Path(tp) => {
                                let seg = tp.path.segments.last().unwrap().ident.clone();
                                format_ident!("{}Resource", seg)
                            }
                            _ => format_ident!("{}Resource", var_ident),
                        };
                        variant_impls.push(quote! {
                            #[godot_dyn]
                            impl #dyn_trait for #variant_res {
                                fn extract_enum_variant(&self) -> #name {
                                    #name::#var_ident(self.extract())
                                }
                            }
                        });
                    }
                }

                quote! {
                    pub trait #dyn_trait {
                        fn extract_enum_variant(&self) -> #name;
                    }

                    type #res_name = ::godot::obj::DynGd<::godot::classes::Resource, dyn #dyn_trait>;

                    impl ::as_gd_res::AsGdRes for #name {
                        type ResType = ::godot::prelude::OnEditor<#res_name>;
                    }
                    impl ::as_gd_res::AsGdResOpt for #name {
                        type GdOption = Option<#res_name>;
                    }
                    impl ::as_gd_res::AsGdResArray for #name {
                        type GdArray = ::godot::prelude::Array<#res_name>;
                    }

                    impl ::as_gd_res::ExtractGd for dyn #dyn_trait {
                        type Extracted = #name;
                        fn extract(&self) -> Self::Extracted {
                            self.extract_enum_variant()
                        }
                    }

                    #(#variant_impls)*
                }
            } else {
                let invalid = data
                    .variants
                    .iter()
                    .filter_map(|v| match &v.fields {
                        Fields::Unnamed(u) if u.unnamed.len() == 1 => None,
                        Fields::Unit => None,
                        _ => Some(v.ident.to_string()),
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                let msg = format!(
                    "`derive(AsGdRes)` only supports unit enums or single-tuple enums. Unsupported variants: {}",
                    invalid
                );
                quote! { compile_error!(#msg); }
            }
        }
        _ => quote! {
            compile_error!(
                "`derive(AsGdRes)` only supports structs with named fields, enums with unit variants, or enums with single-tuple variants"
            );
        },
    }
}
