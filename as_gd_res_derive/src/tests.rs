use super::expand_as_gd_res;
use pretty_assertions::assert_eq;
use quote::quote;
use syn::parse_quote;

// # STRUCTS
// For each field in a composite struct, if it includes any `#[export(...)]` or `#[init(...)]` attributes,
// the generated `ResType` should include those attributes on the generated struct.
// If the field has no attributes, we must add the `#[export]` attribute to the generated struct.
// ## Notes:
// - The derive macro should not support generics, so the input struct should not have any generic parameters.
//
// # ENUMS
// `#[derive(::as_gd_res::AsGdRes)]` only works on enums where either:
// 1. all variants have a single, unnamed associated data type
// -or-
// 2. all variants are unit variants (i.e. no associated data)
//
// In any other case, the macro should emit an error saying that these conditions have not been met
//
// # NOTES:
// There are limitations upstream in *godot-rust* (or really: in Godot itself) that prevent the representation of certain types. You'll need work arounds in at least these cases:
// - `Option<{enum types}>`: If you want an "optional" enum, include a `None` variant in the enum itself, and set that as the default value.
// - `Array<{enum types}>` are also not supported

#[test]
fn test_simple() {
    let input: syn::DeriveInput = parse_quote! {
        pub struct SimpleStructParams {
            a: f32,
            b: f32,
        }
    };
    let actual = expand_as_gd_res(input);
    let expected = quote! {
      impl ::as_gd_res::AsGdRes for SimpleStructParams {
          type ResType = ::godot::prelude::OnEditor<::godot::obj::Gd<SimpleStructParamsResource>>;
      }

      impl ::as_gd_res::AsGdResOpt for SimpleStructParams {
          type GdOption = Option<::godot::obj::Gd<SimpleStructParamsResource>>;
      }

      impl ::as_gd_res::AsGdResArray for SimpleStructParams {
          type GdArray = ::godot::prelude::Array<::godot::obj::Gd<SimpleStructParamsResource>>;
      }

      #[derive(::godot::prelude::GodotClass)]
      #[class(tool,init,base=Resource)]
      pub struct SimpleStructParamsResource {
          #[base]
          base: ::godot::obj::Base<::godot::classes::Resource>,
          #[export]
          pub a: <f32 as ::as_gd_res::AsGdRes>::ResType,
          #[export]
          pub b: <f32 as ::as_gd_res::AsGdRes>::ResType,
      }

      impl ::as_gd_res::ExtractGd for SimpleStructParamsResource {
          type Extracted = SimpleStructParams;
          fn extract(&self) -> Self::Extracted {
              Self::Extracted {
                  a: self.a.extract(),
                  b: self.b.extract(),
              }
          }
      }
    };
    assert_eq!(actual.to_string(), expected.to_string());
}

#[test]
fn test_2() {
    let input: syn::DeriveInput = parse_quote! {
        pub struct DropParams2 {
            pub total_value: f32,
            pub max_value_per_coin: f32,
            pub coin_scene_1: Option<PackedScenePath>,
            pub coin_scene_2: OnEditorInit<PackedScenePath>,
        }
    };
    let actual = expand_as_gd_res(input);
    let expected = quote! {

            impl ::as_gd_res::AsGdRes for DropParams2 {
                type ResType = ::godot::prelude::OnEditor<::godot::obj::Gd<DropParams2Resource>>;
            }

            impl ::as_gd_res::AsGdResOpt for DropParams2 {
                type GdOption = Option<::godot::obj::Gd<DropParams2Resource>>;
            }

            impl ::as_gd_res::AsGdResArray for DropParams2 {
                type GdArray = ::godot::prelude::Array<::godot::obj::Gd<DropParams2Resource>>;
            }

            #[derive(::godot::prelude::GodotClass)]
            #[class(tool,init,base=Resource)]

            pub struct DropParams2Resource {
                #[base]
                base: ::godot::obj::Base<::godot::classes::Resource>,
                #[export]
                pub total_value: <f32 as ::as_gd_res::AsGdRes>::ResType,
                #[export]
                pub max_value_per_coin: <f32 as ::as_gd_res::AsGdRes>::ResType,
                #[export]
                pub coin_scene_1: <Option<PackedScenePath> as ::as_gd_res::AsGdRes>::ResType,
                #[export]
                pub coin_scene_2: <OnEditorInit<PackedScenePath> as ::as_gd_res::AsGdRes>::ResType,
            }

            impl ::as_gd_res::ExtractGd for DropParams2Resource {
                type Extracted = DropParams2;
                fn extract(&self) -> Self::Extracted {
                    Self::Extracted {
                        total_value: self.total_value.extract(),
                        max_value_per_coin: self.max_value_per_coin.extract(),
                        coin_scene_1: self.coin_scene_1.extract(),
                        coin_scene_2: self.coin_scene_2.extract(),
                    }
                }
            }

    };
    assert_eq!(actual.to_string(), expected.to_string());
}

#[test]
fn test_attr_pass_through() {
    let input: syn::DeriveInput = parse_quote! {
        pub struct DropParams2 {
          #[export(range = (100.0, 500.0))]
          #[init(val = 200.0)]
          pub total_value: f32,

          #[export(range = (0.0, 5.0))]
          #[init(val = 3.0)]
          pub max_value_per_coin: f32,
          pub coin_scene_1: Option<PackedScenePath>,
          pub coin_scene_2: OnEditorInit<PackedScenePath>,
      }
    };

    let expected = quote! {
        impl ::as_gd_res::AsGdRes for DropParams2 {
            type ResType = ::godot::prelude::OnEditor<::godot::obj::Gd<DropParams2Resource>>;
        }

        impl ::as_gd_res::AsGdResOpt for DropParams2 {
            type GdOption = Option<::godot::obj::Gd<DropParams2Resource>>;
        }

        impl ::as_gd_res::AsGdResArray for DropParams2 {
            type GdArray = ::godot::prelude::Array<::godot::obj::Gd<DropParams2Resource>>;
        }

      #[derive(::godot::prelude::GodotClass)]
      #[class(tool,init,base=Resource)]
      pub struct DropParams2Resource {
          #[base]
          base: ::godot::obj::Base<::godot::classes::Resource>,
          #[export(range = (100.0, 500.0))]
          #[init(val = 200.0)]
          pub total_value: <f32 as ::as_gd_res::AsGdRes>::ResType,
          #[export(range = (0.0, 5.0))]
          #[init(val = 3.0)]
          pub max_value_per_coin: <f32 as ::as_gd_res::AsGdRes>::ResType,
          #[export]
          pub coin_scene_1: <Option<PackedScenePath> as ::as_gd_res::AsGdRes>::ResType,
          #[export]
          pub coin_scene_2: <OnEditorInit<PackedScenePath> as ::as_gd_res::AsGdRes>::ResType,
      }

      impl ::as_gd_res::ExtractGd for DropParams2Resource {
          type Extracted = DropParams2;
          fn extract(&self) -> Self::Extracted {
              Self::Extracted {
                  total_value: self.total_value.extract(),
                  max_value_per_coin: self.max_value_per_coin.extract(),
                  coin_scene_1: self.coin_scene_1.extract(),
                  coin_scene_2: self.coin_scene_2.extract(),
              }
          }
      }

    };

    assert_eq!(expand_as_gd_res(input).to_string(), expected.to_string());
}

// NOTE: Option<{enum types}> is not supported, ::as_gd_res::AsGdRes not impled for that
#[test]
fn test_simple_enum() {
    let input: syn::DeriveInput = parse_quote! {
            #[derive(Default, Clone, Copy, GodotConvert, Var, Export)]
            #[godot(via = ::godot::builtin::GString)]
            pub enum DamageTeam {
                #[default]
                Player,
                Enemy,
                Environment,
            }

    };

    let expected = quote! {
        compile_error!(
                    "`derive(AsGdRes)` only or enums with single-tuple variants, not unit variants. Did you mean to use `derive(AsSimpleGdEnum)`?"
                );
            // #[derive(Default, Clone, Copy, GodotConvert, Var, Export)]
            // #[godot(via = ::godot::builtin::GString)]
            // pub enum DamageTeam {
            //     #[default]
            //     Player,
            //     Enemy,
            //     Environment,
            // }

            // impl ::as_gd_res::AsGdRes for DamageTeam {
            //     type ResType = DamageTeam;
            // }

            // impl ::as_gd_res::AsGdResArray for DamageTeam {
            //     type GdArray = ::godot::prelude::Array<::godot::obj::Gd<DamageTeam>>;
            // }

            // impl ExtractGd for DamageTeam {
            //     type Extracted = DamageTeam;
            //     fn extract(&self) -> Self::Extracted {
            //         self.clone()
            //     }
            // }

    };

    assert_eq!(expand_as_gd_res(input).to_string(), expected.to_string());
}

/// For enums with data variants, we do the following:
/// - Create a new trait called `{EnumName}ResourceExtractVariant` that has a method `extract_enum_variant`
/// - Create a new type for the enum resource called `{EnumName}Resource`, which aliases `DynGd<Resource, dyn {EnumName}ResourceExtractVariant>`
/// - Implement `::as_gd_res::AsGdRes` for the enum, which returns the new resource type
/// - Implement `ExtractGd` for the new resource type, which extracts the resource back to the input enum
/// - For each enum variant, implement the `{EnumName}ResourceExtractVariant>` trait for the resource corresponding to the type in within the variant. It is up to the user to derive `::as_gd_res::AsGdRes` on the type inside each variant, which will create the resource type for that variant. (For example, if the enum has a variant `Money(MoneyData)`, the user must derive `::as_gd_res::AsGdRes` on `MoneyData` to create the resource type `MoneyDataResource`.). Each impl must be annotated with `#[godot_dyn]` for compatibility with `DynGd`.
///
/// Note that having
#[test]
fn test_enum_with_data_variants() {
    let input: syn::DeriveInput = parse_quote! {
        pub enum Pickup {
            Money(MoneyData),
            PowerUp(PowerUpData),
            Heal(HealData),
        }
    };

    let expected = quote! {
        pub trait PickupResourceExtractVariant {
            fn extract_enum_variant(&self) -> Pickup;
        }

        type PickupResource =
            ::godot::obj::DynGd<::godot::classes::Resource, dyn PickupResourceExtractVariant>;

        impl ::as_gd_res::AsGdRes for Pickup {
            type ResType = ::godot::prelude::OnEditor<PickupResource>;
        }
        impl ::as_gd_res::AsGdResOpt for Pickup {
            type GdOption = Option<PickupResource>;
        }
        impl ::as_gd_res::AsGdResArray for Pickup {
            type GdArray = ::godot::prelude::Array<PickupResource>;
        }

        impl ::as_gd_res::ExtractGd for dyn PickupResourceExtractVariant {
            type Extracted = Pickup;
            fn extract(&self) -> Self::Extracted {
                self.extract_enum_variant()
            }
        }

        #[godot_dyn]
        impl PickupResourceExtractVariant for MoneyDataResource {
            fn extract_enum_variant(&self) -> Pickup {
                Pickup::Money(self.extract())
            }
        }
        #[godot_dyn]
        impl PickupResourceExtractVariant for PowerUpDataResource {
            fn extract_enum_variant(&self) -> Pickup {
                Pickup::PowerUp(self.extract())
            }
        }

        #[godot_dyn]
        impl PickupResourceExtractVariant for HealDataResource {
            fn extract_enum_variant(&self) -> Pickup {
                Pickup::Heal(self.extract())
            }
        }
    };

    assert_eq!(expand_as_gd_res(input).to_string(), expected.to_string());
}

#[test]
fn test_complex_nested_struct() {
    let input: syn::DeriveInput = parse_quote! {
      pub struct EnemyParams {
          pub brain_params_required: OnEditorInit<BrainParams>,
          pub brain_params_optional: Option<BrainParams>,
          pub brains_vec: Vec<BrainParams>,
          pub drop_params: Option<DropParams2>,
          pub damage_team: DamageTeam,
      }
    };

    let actual = expand_as_gd_res(input);
    let expected = quote! {

        impl ::as_gd_res::AsGdRes for EnemyParams {
            type ResType = ::godot::prelude::OnEditor<::godot::obj::Gd<EnemyParamsResource>>;
        }

        impl ::as_gd_res::AsGdResOpt for EnemyParams {
            type GdOption = Option<::godot::obj::Gd<EnemyParamsResource>>;
        }

        impl ::as_gd_res::AsGdResArray for EnemyParams {
            type GdArray = ::godot::prelude::Array<::godot::obj::Gd<EnemyParamsResource>>;
        }


        #[derive(::godot::prelude::GodotClass)]
        #[class(tool,init,base=Resource)]

        pub struct EnemyParamsResource {
            #[base]
            base: ::godot::obj::Base<::godot::classes::Resource>,

            #[export]
            pub brain_params_required: <OnEditorInit<BrainParams> as ::as_gd_res::AsGdRes>::ResType,
            #[export]
            pub brain_params_optional: <Option<BrainParams> as ::as_gd_res::AsGdRes>::ResType,
            #[export]
            pub brains_vec: <Vec<BrainParams> as ::as_gd_res::AsGdRes>::ResType,

            #[export]
            pub drop_params: <Option<DropParams2> as ::as_gd_res::AsGdRes>::ResType,
            #[export]
            pub damage_team: <DamageTeam as ::as_gd_res::AsGdRes>::ResType,
        }

        impl ::as_gd_res::ExtractGd for EnemyParamsResource {
            type Extracted = EnemyParams;
            fn extract(&self) -> Self::Extracted {
                Self::Extracted {
                    brain_params_required: self.brain_params_required.extract(),
                    brain_params_optional: self.brain_params_optional.extract(),
                    brains_vec: self.brains_vec.extract(),
                    drop_params: self.drop_params.extract(),
                    damage_team: self.damage_team.extract(),
                }
            }
        }

    };

    assert_eq!(actual.to_string(), expected.to_string());
}
