//! This crate provides derive macros for the [AbsDiffEq] and [RelativeEq] traits of the
//! [approx](https://docs.rs/approx/latest/approx/) crate.
//!
//! ```
//! use approx_derive::AbsDiffEq;
//!
//! // Define a new type and derive the AbsDiffEq trait
//! #[derive(AbsDiffEq, PartialEq, Debug)]
//! struct Position {
//!     x: f64,
//!     y: f64
//! }
//!
//! // Compare if two given positions match
//! // with respect to geiven epsilon.
//! let p1 = Position { x: 1.01, y: 2.36 };
//! let p2 = Position { x: 0.99, y: 2.38 };
//! approx::assert_abs_diff_eq!(p1, p2, epsilon = 0.021);
//! ```
//!
//! # General Usage
//! The macros infer the `EPSILON` type of the [AbsDiffEq] trait by looking
//! at the type of the first struct field or any type specified by the user.
//!
//! ## Field Attributes
//! ### Skipping Fields
//!
//! Sometimes, we only want to compare certain fields and omit others completely.
//! ```
//! # use approx_derive::*;
//! #[derive(AbsDiffEq, PartialEq, Debug)]
//! struct Player {
//!     hit_points: f32,
//!     pos_x: f32,
//!     pos_y: f32,
//!     #[approx(skip)]
//!     id: (usize, usize),
//! }
//!
//! let player1 = Player {
//!     hit_points: 100.0,
//!     pos_x: 2.0,
//!     pos_y: -650.345,
//!     id: (0, 1),
//! };
//!
//! let player2 = Player {
//!     hit_points: 99.9,
//!     pos_x: 2.001,
//!     pos_y: -649.898,
//!     id: (22, 0),
//! };
//!
//! approx::assert_abs_diff_eq!(player1, player2, epsilon = 0.5);
//! ```
//!
//! ### Casting Fields
//!
//! Structs which consist of multiple fields with different
//! numeric types, can not be derived without additional hints.
//! After all, we should specify how this type mismatch will be handled.
//!
//! ```compile_fail
//! # use approx_derive::*;
//! #[derive(AbsDiffEq, PartialEq, Debug)]
//! struct MyStruct {
//!     v1: f32,
//!     v2: f64,
//! }
//! ```
//!
//! We can use the `#[approx(cast_field)]` and `#[approx(cast_value)]`
//! attributes to achieve this goal.
//! ```
//! # use approx_derive::*;
//! #[derive(AbsDiffEq, PartialEq, Debug)]
//! struct MyStruct {
//!     v1: f32,
//!     #[approx(cast_field)]
//!     v2: f64,
//! }
//! ```
//! Now the second field will be casted to the type of the inferred epsilon value (`f32`).
//! We can check this by testing if a change in the size of `f64::MIN_POSITIVE` would get lost by
//! this procedure.
//! ```
//! # use approx_derive::*;
//! # #[derive(RelativeEq, PartialEq, Debug)]
//! # struct MyStruct {
//! #   v1: f32,
//! #   #[approx(cast_field)]
//! #   v2: f64,
//! # }
//! let ms1 = MyStruct {
//!     v1: 1.0,
//!     v2: 3.0,
//! };
//! let ms2 = MyStruct {
//!     v1: 1.0,
//!     v2: 3.0 + f64::MIN_POSITIVE,
//! };
//! approx::assert_relative_eq!(ms1, ms2);
//! ```
//!
//! ### Static Values
//! We can force a static `EPSILON` or `max_relative` value for individual fields.
//! ```
//! # use approx_derive::*;
//! #[derive(AbsDiffEq, PartialEq, Debug)]
//! struct Rectangle {
//!     #[approx(static_epsilon = 5e-2)]
//!     a: f64,
//!     b: f64,
//!     #[approx(static_epsilon = 7e-2)]
//!     c: f64,
//! }
//!
//! let r1 = Rectangle {
//!     a: 100.01,
//!     b: 40.0001,
//!     c: 30.055,
//! };
//! let r2 = Rectangle {
//!     a: 99.97,
//!     b: 40.0005,
//!     c: 30.049,
//! };
//!
//! // This is always true although the epsilon is smaller than the
//! // difference between fields a and b respectively.
//! approx::assert_abs_diff_eq!(r1, r2, epsilon = 1e-1);
//! approx::assert_abs_diff_eq!(r1, r2, epsilon = 1e-2);
//! approx::assert_abs_diff_eq!(r1, r2, epsilon = 1e-3);
//!
//! // Here, the epsilon value has become larger than the difference between the
//! // b field values.
//! approx::assert_abs_diff_ne!(r1, r2, epsilon = 1e-4);
//! ```
//! ## Struct Attributes
//! ### Default Epsilon
//! The [AbsDiffEq] trait allows to specify a default value for its `EPSILON` associated type.
//! We can control this value by specifying it on a struct level.
//!
//! ```
//! # use approx_derive::*;
//! #[derive(AbsDiffEq, PartialEq, Debug)]
//! #[approx(default_epsilon = 10)]
//! struct Benchmark {
//!     cycles: u64,
//!     warm_up: u64,
//! }
//!
//! let benchmark1 = Benchmark {
//!     cycles: 248,
//!     warm_up: 36,
//! };
//! let benchmark2 = Benchmark {
//!     cycles: 239,
//!     warm_up: 28,
//! };
//!
//! // When testing with not additional arguments, the results match
//! approx::assert_abs_diff_eq!(benchmark1, benchmark2);
//! // Once we specify a lower epsilon, the values do not agree anymore.
//! approx::assert_abs_diff_ne!(benchmark1, benchmark2, epsilon = 5);
//! ```
//!
//! ### Default Max Relative
//! Similarly to [Default Epsilon], we can also choose a default max_relative devaition.
//! ```
//! # use approx_derive::*;
//! #[derive(RelativeEq, PartialEq, Debug)]
//! #[approx(default_max_relative = 0.1)]
//! struct Benchmark {
//!     time: f32,
//!     warm_up: f32,
//! }
//!
//! let bench1 = Benchmark {
//!     time: 3.502785781,
//!     warm_up: 0.58039458,
//! };
//! let bench2 = Benchmark {
//!     time: 3.7023458,
//!     warm_up: 0.59015897,
//! };
//!
//! approx::assert_relative_eq!(bench1, bench2);
//! approx::assert_relative_ne!(bench1, bench2, max_relative = 0.05);
//! ```
//! ### Epsilon Type
//! When specifying nothing, the macros will infer the `EPSILON` type from the type of the
//! first struct field.
//! This can be problematic in certain scenarios which is why we can also manually specify this
//! type.
//!
//! ```
//! # use approx_derive::*;
//! #[derive(RelativeEq, PartialEq, Debug)]
//! #[approx(epsilon_type = f32)]
//! struct Car {
//!     #[approx(cast_field)]
//!     produced_year: u32,
//!     horse_power: f32,
//! }
//!
//! let car1 = Car {
//!     produced_year: 1992,
//!     horse_power: 122.87,
//! };
//! let car2 = Car {
//!     produced_year: 2000,
//!     horse_power: 117.45,
//! };
//!
//! approx::assert_relative_eq!(car1, car2, max_relative = 0.05);
//! approx::assert_relative_ne!(car1, car2, max_relative = 0.01);
//! ```

mod args_parsing;
use args_parsing::*;

struct AbsDiffEqParser {
    item_struct: syn::ItemStruct,
    fields_with_args: Vec<FieldWithArgs>,
    struct_args: StructArgs,
}

impl syn::parse::Parse for AbsDiffEqParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let item_struct: syn::ItemStruct = input.parse()?;
        let struct_args = StructArgs::from_attrs(&item_struct.attrs)?;
        let fields_with_args = item_struct
            .fields
            .iter()
            .map(|field| FieldWithArgs::from_field(field))
            .collect::<syn::Result<Vec<_>>>()?;
        Ok(Self {
            item_struct,
            fields_with_args,
            struct_args,
        })
    }
}

struct FieldFormatted {
    base_type: proc_macro2::TokenStream,
    own_field: proc_macro2::TokenStream,
    other_field: proc_macro2::TokenStream,
    epsilon: proc_macro2::TokenStream,
    max_relative: proc_macro2::TokenStream,
}

impl AbsDiffEqParser {
    fn get_epsilon_type(&self) -> proc_macro2::TokenStream {
        self.struct_args
            .epsilon_type
            .clone()
            .and_then(|x| Some(quote::quote!(#x)))
            .or_else(|| {
                self.fields_with_args.first().and_then(|field| {
                    let eps_type = &field.field.ty;
                    Some(quote::quote!(#eps_type))
                })
            })
            .or_else(|| Some(quote::quote!(f64)))
            .unwrap()
    }

    fn get_epsilon_type_and_default_value(
        &self,
    ) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
        let epsilon_type = self.get_epsilon_type();
        let epsilon_default_value = self
            .struct_args
            .default_epsilon_value
            .clone()
            .and_then(|x| Some(quote::quote!(#x)))
            .or_else(|| Some(quote::quote!(#epsilon_type::EPSILON)))
            .unwrap();
        (epsilon_type, epsilon_default_value)
    }

    fn get_max_relative_default_value(&self) -> proc_macro2::TokenStream {
        let epsilon_type = self.get_epsilon_type();
        self.struct_args
            .default_max_relative_value
            .clone()
            .and_then(|x| Some(quote::quote!(#x)))
            .or_else(|| Some(quote::quote!(#epsilon_type::EPSILON)))
            .unwrap()
    }

    fn format_field(&self, field_with_args: &FieldWithArgs) -> Option<FieldFormatted> {
        // Determine if this field will be skipped and exit early
        if field_with_args.args.skip {
            return None;
        }

        // Get types for epsilon and max_relative
        let epsilon_type = self.get_epsilon_type();

        // Save field name and type in variables for easy access
        let field_name = &field_with_args.field.ident;
        let field_type = &field_with_args.field.ty;

        // Determine if the field or the value will be casted in any way
        let cast_strategy = &field_with_args.args.cast_strategy;

        // Get static values (if present) for epsilon and max_relative
        let epsilon = &field_with_args
            .args
            .epsilon_static_value
            .clone()
            .and_then(|x| Some(quote::quote!(#x)))
            .or_else(|| Some(quote::quote!(epsilon)))
            .unwrap();
        let max_relative = field_with_args
            .args
            .max_relative_static_value
            .clone()
            .and_then(|x| Some(quote::quote!(#x)))
            .or_else(|| Some(quote::quote!(max_relative)))
            .unwrap();

        // Use the casting strategy
        let (base_type, own_field, other_field, epsilon, max_relative) = match cast_strategy {
            Some(TypeCast::CastField) => (
                quote::quote!(#epsilon_type),
                quote::quote!(&(self.#field_name as #epsilon_type)),
                quote::quote!(&(other.#field_name as #epsilon_type)),
                quote::quote!(#epsilon),
                quote::quote!(#max_relative),
            ),
            Some(TypeCast::CastValue) => (
                quote::quote!(#field_type),
                quote::quote!(&self.#field_name),
                quote::quote!(&other.#field_name),
                quote::quote!(#epsilon as #field_type),
                quote::quote!(#max_relative as #field_type),
            ),
            None => (
                quote::quote!(#epsilon_type),
                quote::quote!(&self.#field_name),
                quote::quote!(&other.#field_name),
                quote::quote!(#epsilon),
                quote::quote!(#max_relative),
            ),
        };

        // Return the fully formatted field
        Some(FieldFormatted {
            base_type,
            own_field,
            other_field,
            epsilon,
            max_relative,
        })
    }

    fn get_abs_diff_eq_fields(&self) -> Vec<proc_macro2::TokenStream> {
        // We need to extend the where clause for all generics
        let fields = self.fields_with_args.iter().filter_map(|field_with_args| {
            if let Some(FieldFormatted {
                base_type,
                own_field,
                other_field,
                epsilon,
                #[allow(unused)]
                max_relative,
            }) = self.format_field(field_with_args)
            {
                Some(quote::quote!(
                    <#base_type as approx::AbsDiffEq>::abs_diff_eq(
                        #own_field,
                        #other_field,
                        #epsilon
                    ) &&
                ))
            } else {
                None
            }
        });
        fields.collect()
    }

    fn get_rel_eq_fields(&self) -> Vec<proc_macro2::TokenStream> {
        let fields = self.fields_with_args.iter().filter_map(|field_with_args| {
            if let Some(FieldFormatted {
                base_type,
                own_field,
                other_field,
                epsilon,
                max_relative,
            }) = self.format_field(field_with_args)
            {
                Some(quote::quote!(
                    <#base_type as approx::RelativeEq>::relative_eq(
                        #own_field,
                        #other_field,
                        #epsilon,
                        #max_relative
                    ) &&
                ))
            } else {
                None
            }
        });
        fields.collect()
    }

    fn implement_derive_abs_diff_eq(&self) -> proc_macro2::TokenStream {
        let struct_name = &self.item_struct.ident;
        let (epsilon_type, epsilon_default_value) = self.get_epsilon_type_and_default_value();
        let fields = self.get_abs_diff_eq_fields();
        let (impl_generics, ty_generics, where_clause) = self.item_struct.generics.split_for_impl();

        quote::quote!(
            const _ : () = {
                #[automatically_derived]
                impl #impl_generics approx::AbsDiffEq for #struct_name #ty_generics
                #where_clause
                {
                    type Epsilon = #epsilon_type;

                    fn default_epsilon() -> Self::Epsilon {
                        #epsilon_default_value
                    }

                    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                        #(#fields)*
                        true
                    }
                }
            };
        )
    }

    fn implement_derive_rel_diff_eq(&self) -> proc_macro2::TokenStream {
        let struct_name = &self.item_struct.ident;
        let max_relative_default_value = self.get_max_relative_default_value();
        let fields = self.get_rel_eq_fields();
        let (impl_generics, ty_generics, where_clause) = self.item_struct.generics.split_for_impl();

        quote::quote!(
            const _ : () = {
                #[automatically_derived]
                impl #impl_generics approx::RelativeEq for #struct_name #ty_generics
                #where_clause
                {
                    fn default_max_relative() -> Self::Epsilon {
                        #max_relative_default_value
                    }

                    fn relative_eq(
                        &self,
                        other: &Self,
                        epsilon: Self::Epsilon,
                        max_relative: Self::Epsilon
                    ) -> bool {
                        #(#fields)*
                        true
                    }
                }
            };
        )
    }
}

/// See the [crate] level documentation for a guide.
#[proc_macro_derive(AbsDiffEq, attributes(approx))]
pub fn derive_abs_diff_eq(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed = syn::parse_macro_input!(input as AbsDiffEqParser);
    parsed.implement_derive_abs_diff_eq().into()
}

/// See the [crate] level documentation for a guide.
#[proc_macro_derive(RelativeEq, attributes(approx))]
pub fn derive_rel_diff_eq(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed = syn::parse_macro_input!(input as AbsDiffEqParser);
    let mut output = quote::quote!();
    output.extend(parsed.implement_derive_abs_diff_eq());
    output.extend(parsed.implement_derive_rel_diff_eq());
    output.into()
}
