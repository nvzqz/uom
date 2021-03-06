/// Macro to implement a [system of quantities](http://jcgm.bipm.org/vim/en/1.3.html). `@...` match
/// arms are considered private.
///
/// * `$quantities_attr`: System of quantities attributes. Generally used to set documentation
///   comments for the system of quantities.
/// * `$quantities`: Name of the system of quantities (e.g. `ISQ`).
/// * `$name`: Name of the base quantities for the system of quantities (e.g. `length`, `mass`,
///   ...). Note that this name must match the module name of the quantity.
/// * `$unit`: Base unit of the quantity (e.g. `meter`, `kilogram`).
/// * `$symbol`: Dimension symbol of the quantity.
/// * `$units_attr`: System of units attributes. Generally used to set documentation comments for
///   the system of units.
/// * `$units`: Name of the system of units (e.g. `SI`).
/// * `$module`: Module name of the quantity. When prefixed by the `mod` keyword the module must
///   already be defined with the `#[macro_use]` attribute. A `#[macro_use] pub mod $module;`
///   statement is generated if this variable is not prefixed by the `mod` keyword.
/// * `$quantity`: Quantity name (e.g. `Length`, `Mass`, ...).
///
/// An example invocation is given below for a meter-kilogram-second system. The `#[macro_use]`
/// attribute must be used when including the `uom` crate to make the `system!` macro available.
///
/// ```
/// #[macro_use]
/// extern crate uom;
///
/// # fn main() { }
/// # mod mks {
/// #     #[macro_use]
/// #     mod length {
/// #         quantity! {
/// #             /// Length (base unit meter, m<sup>1</sup>).
/// #             quantity: Length; "length";
/// #             /// Length dimension, m<sup>1</sup>.
/// #             dimension: Q<P1 /*length*/, Z0 /*mass*/, Z0 /*time*/>;
/// #             units {
/// #                 @meter: 1.0E0; "m", "meter", "meters";
/// #                 @foot: 3.048E-1; "ft", "foot", "feet";
/// #             }
/// #         }
/// #     }
/// #     #[macro_use]
/// #     mod mass {
/// #         quantity! {
/// #             /// Mass (base unit kilogram, kg<sup>1</sup>).
/// #             quantity: Mass; "mass";
/// #             /// Mass dimension, kg<sup>1</sup>.
/// #             dimension: Q<Z0 /*length*/, P1 /*mass*/, Z0 /*time*/>;
/// #             units {
/// #                 @kilogram: 1.0; "kg", "kilogram", "kilograms";
/// #             }
/// #         }
/// #     }
/// #     #[macro_use]
/// #     mod time {
/// #         quantity! {
/// #             /// Time (base unit second, s<sup>1</sup>).
/// #             quantity: Time; "time";
/// #             /// Time dimension, s<sup>1</sup>.
/// #             dimension: Q<Z0 /*length*/, Z0 /*mass*/, P1 /*time*/>;
/// #             units {
/// #                 @second: 1.0; "s", "second", "seconds";
/// #             }
/// #         }
/// #     }
/// system! {
///     /// System of quantities, Q.
///     quantities: Q {
///         length: meter, L;
///         mass: kilogram, M;
///         time: second, T;
///     }
///     /// System of units, U.
///     units: U {
///         mod length::Length,
///         mod mass::Mass,
///         mod time::Time,
///     }
/// }
/// #     mod f32 {
/// #         Q!(mks, f32/*, (centimeter, gram, second)*/);
/// #     }
/// # }
/// ```
#[macro_export]
macro_rules! system {
    (
        $(#[$quantities_attr:meta])* quantities: $quantities:ident {
            $($name:ident: $unit:ident, $symbol:ident;)+
        }
        $(#[$units_attr:meta])* units: $units:ident {
            $($module:ident::$quantity:ident,)+
        }
    ) => {
        $(#[macro_use]
        pub mod $module;)+

        system! {
            $(#[$quantities_attr])*
            quantities: $quantities {
                $($name: $unit, $symbol;)+
            }
            $(#[$units_attr])*
            units: $units {
                $(mod $module::$quantity,)+
            }
        }
    };
    (
        $(#[$quantities_attr:meta])* quantities: $quantities:ident {
            $($name:ident: $unit:ident, $symbol:ident;)+
        }
        $(#[$units_attr:meta])* units: $units:ident {
            $(mod $module:ident::$quantity:ident,)+
        }
    ) => {
        /// Marker trait to express the dependence of a [quantity][quantity] on the
        /// [base quantities][base] of a [system of quantities][quantities] as a product of powers
        /// of factors corresponding to the base quantities, omitting any numerical factor.
        ///
        /// * <http://jcgm.bipm.org/vim/en/1.7.html>
        ///
        /// [quantity]: http://jcgm.bipm.org/vim/en/1.1.html
        /// [base]: http://jcgm.bipm.org/vim/en/1.4.html
        /// [quantities]: http://jcgm.bipm.org/vim/en/1.3.html
        pub trait Dimension: Send + Sync {
            $(/// Quantity dimension.
            type $symbol: $crate::typenum::Integer;)+
        }

        /// Marker trait to identify a [system of units][units] based on a set of [base units][base]
        /// of a [system of quantities][quantities].
        ///
        /// [units]: http://jcgm.bipm.org/vim/en/1.13.html
        /// [base]: http://jcgm.bipm.org/vim/en/1.10.html
        /// [quantities]: http://jcgm.bipm.org/vim/en/1.3.html
        pub trait Units<V>: Send + Sync
        where
            V: $crate::Conversion<V>,
        {
            $(/// Base unit.
            type $name: Unit + $crate::Conversion<V, T = V::T>;)+
        }

        /// Trait to identify [measurement units][measurement] of individual
        /// [quantities][quantity].
        ///
        /// [measurement]: http://jcgm.bipm.org/vim/en/1.9.html
        /// [quantity]: http://jcgm.bipm.org/vim/en/1.1.html
        pub trait Unit: Copy {
            /// Unit abbreviation.
            fn abbreviation() -> &'static str;

            /// Unit singular description.
            fn singular() -> &'static str;

            /// Unit plural description.
            fn plural() -> &'static str;
        }

        /// Property of a phenomenon, body or substance, where the property has a magnitude that
        /// can be expressed as a number and a reference.
        ///
        /// The preferred method of creating a `Quantity` instance is to use the `new` constructor
        /// which is generic over the input unit and accepts the input value as it's only
        /// parameter.
        ///
        #[cfg_attr(all(feature = "si", feature = "f32"), doc = " ```rust")]
        #[cfg_attr(not(all(feature = "si", feature = "f32")), doc = " ```rust,ignore")]
        /// # use uom::si::f32::*;
        /// # use uom::si::length::meter;
        /// // Create a length of 1 meter.
        /// let L = Length::new::<meter>(1.0);
        /// ```
        ///
        /// `Quantity` fields are public to allow for the creation of `const` values and instances
        /// of non-named `Quantity`s. This functionality will be deprecated and subsequently removed
        /// once the [`const fn`](https://github.com/rust-lang/rust/issues/24111) feature is
        /// stabilized.
        ///
        #[cfg_attr(all(feature = "si", feature = "f32"), doc = " ```rust")]
        #[cfg_attr(not(all(feature = "si", feature = "f32")), doc = " ```rust,ignore")]
        /// # use uom::si::{Quantity, ISQ, SI};
        /// # use uom::si::f32::*;
        /// # use uom::lib::marker::PhantomData;
        /// # use uom::typenum::{P2, Z0};
        /// // Create a `const` length of 1 meter.
        /// const L: Length = Length { dimension: PhantomData, units: PhantomData, value: 1.0, };
        /// // Create a `const` area of 1 square meter explicitly without using the `Area` alias.
        /// const A: Quantity<ISQ<P2, Z0, Z0, Z0, Z0, Z0, Z0>, SI<f32>, f32> =
        ///     Quantity { dimension: PhantomData, units: PhantomData, value: 1.0, };
        /// ```
        ///
        /// Using units for the wrong quantity will cause a compile error:
        ///
        #[cfg_attr(all(feature = "si", feature = "f32"), doc = " ```rust,compile_fail")]
        #[cfg_attr(not(all(feature = "si", feature = "f32")), doc = " ```rust,ignore")]
        /// # use uom::si::f32::*;
        /// # use uom::si::time::second;
        /// // error[E0277]: the trait bound `second: length::Unit` is not satisfied
        /// let l = Length::new::<second>(1.0);
        /// ```
        ///
        /// Mixing quantities will also cause a compile error:
        ///
        #[cfg_attr(all(feature = "si", feature = "f32"), doc = " ```rust,compile_fail")]
        #[cfg_attr(not(all(feature = "si", feature = "f32")), doc = " ```rust,ignore")]
        /// # use uom::si::f32::*;
        /// # use uom::si::length::meter;
        /// # use uom::si::time::second;
        /// // error[E0308]: mismatched types
        /// let r = Length::new::<meter>(1.0) + Time::new::<second>(1.0);
        /// ```
        ///
        #[cfg_attr(all(feature = "si", feature = "f32"), doc = " ```rust,compile_fail")]
        #[cfg_attr(not(all(feature = "si", feature = "f32")), doc = " ```rust,ignore")]
        /// # use uom::si::f32::*;
        /// # use uom::si::length::meter;
        /// # use uom::si::time::second;
        /// // error[E0308]: mismatched types
        /// let v: Velocity = Length::new::<meter>(1.0) * Time::new::<second>(1.0);
        /// ```
        ///
        /// * http://jcgm.bipm.org/vim/en/1.1.html
        pub struct Quantity<D, U, V>
        where
            D: Dimension + ?Sized,
            U: Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V>,
        {
            /// Quantity dimension. See [`Dimension`](./trait.Dimension.html).
            pub dimension: $crate::lib::marker::PhantomData<D>,
            /// Quantity base units. See [`Units`](./trait.Units.html).
            pub units: $crate::lib::marker::PhantomData<U>,
            /// Quantity value stored in the base units for the quantity.
            pub value: V,
        }

        // Type alias for dimensions where all exponents of the factors are the given value.
        type DN<N> = Dimension<$($symbol = system!(@replace $symbol N)),+>;

        /// Type alias for [dimension one][one] for which all the exponents of the factors
        /// corresponding to the [base quantities][base] are zero.
        ///
        /// [one]: http://jcgm.bipm.org/vim/en/1.8.html
        /// [base]: http://jcgm.bipm.org/vim/en/1.4.html
        #[allow(dead_code)]
        pub type DimensionOne = DN<$crate::typenum::Z0>;

        $(#[$quantities_attr])*
        pub type $quantities<$($symbol),+> = Dimension<$($symbol = $symbol),+>;

        $(#[$units_attr])*
        #[allow(unused_qualifications)]
        pub type $units<V> = Units<V, $($name = $name::$unit),+>;

        /// Convert a value from base units to the given unit.
        #[inline(always)]
        fn from_base<D, U, V, N>(v: &V) -> V
        where
            D: Dimension + ?Sized,
            U: Units<V> + ?Sized,
            V: $crate::Conversion<V> + $crate::lib::ops::Mul<V, Output = V>,
            N: $crate::Conversion<V, T = V::T>,
        {
            use $crate::typenum::Integer;
            use $crate::Conversion;
            use $crate::ConversionFactor;

            (v.into_conversion() $(* U::$name::conversion().powi(D::$symbol::to_i32()))+
                    / N::conversion())
                .value()
        }

        /// Convert a value from the given unit to base units.
        #[inline(always)]
        fn to_base<D, U, V, N>(v: &V) -> V
        where
            D: Dimension + ?Sized,
            U: Units<V> + ?Sized,
            V: $crate::Conversion<V> + $crate::lib::ops::Mul<V, Output = V>,
            N: $crate::Conversion<V, T = V::T>,
        {
            use $crate::typenum::Integer;
            use $crate::Conversion;
            use $crate::ConversionFactor;

            (v.into_conversion() * N::conversion()
                    / (V::conversion() $(* U::$name::conversion().powi(D::$symbol::to_i32()))+))
                .value()
        }

        /// Convert a value from one set of base units to a second.
        #[inline(always)]
        fn change_base<D, Ul, Ur, V>(v: &V) -> V
        where
            D: Dimension + ?Sized,
            Ul: Units<V> + ?Sized,
            Ur: Units<V> + ?Sized,
            V: $crate::Conversion<V> + $crate::lib::ops::Mul<V, Output = V>,
        {
            use $crate::typenum::Integer;
            use $crate::Conversion;
            use $crate::ConversionFactor;

            (v.into_conversion() $(* Ur::$name::conversion().powi(D::$symbol::to_i32())
                    / Ul::$name::conversion().powi(D::$symbol::to_i32()))+)
                .value()
        }

        impl<D, U, V> $crate::lib::clone::Clone for Quantity<D, U, V>
        where
            D: Dimension + ?Sized,
            U: Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V> + $crate::lib::clone::Clone,
        {
            #[inline(always)]
            fn clone(&self) -> Self {
                match *self {
                    Quantity { ref value, .. } => {
                        Quantity {
                            dimension: $crate::lib::marker::PhantomData,
                            units: $crate::lib::marker::PhantomData,
                            value: $crate::lib::clone::Clone::clone(&(*value)),
                        }
                    }
                }
            }
        }

        impl<D, U, V> $crate::lib::marker::Copy for Quantity<D, U, V>
        where
            D: Dimension + ?Sized,
            U: Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V> + $crate::lib::marker::Copy,
        {
        }

        #[allow(non_camel_case_types)]
        impl<D, U, V> $crate::lib::fmt::Debug for Quantity<D, U, V>
        where
            D: Dimension + ?Sized,
            U: Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V> + $crate::lib::fmt::Debug,
        {
            fn fmt(&self, f: &mut $crate::lib::fmt::Formatter) -> $crate::lib::fmt::Result {
                self.value.fmt(f)
                $(.and_then(|_| {
                    let d = <D::$symbol as $crate::typenum::Integer>::to_i32();

                    if 0 != d {
                        write!(f, " {}^{}", U::$name::abbreviation(), d)
                    }
                    else {
                        Ok(())
                    }
                }))+
            }
        }

        impl<D, U, V> $crate::lib::hash::Hash for Quantity<D, U, V>
        where
            D: Dimension + ?Sized,
            U: Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V> + $crate::lib::hash::Hash,
        {
            fn hash<H: $crate::lib::hash::Hasher>(&self, state: &mut H) {
                self.value.hash(state);
            }
        }

        #[doc(hidden)]
        macro_rules! impl_ops {
            (
                $AddSubTrait:ident, $addsub_fun:ident, $addsub_op:tt,
                $AddSubAssignTrait:ident, $addsubassign_fun:ident, $addsubassign_op:tt,
                $AddSubAlias:ident,
                $MulDivTrait:ident, $muldiv_fun:ident, $muldiv_op:tt,
                $MulDivAssignTrait:ident, $muldivassign_fun:ident, $muldivassign_op:tt,
                $Mod:ident
            ) => {
                impl<D, Ul, Ur, V> $crate::lib::ops::$AddSubTrait<Quantity<D, Ur, V>>
                    for Quantity<D, Ul, V>
                where
                    D: Dimension + ?Sized,
                    Ul: Units<V> + ?Sized,
                    Ur: Units<V> + ?Sized,
                    V: $crate::num::Num + $crate::Conversion<V>,
                {
                    type Output = Quantity<D, Ul, V>;

                    #[inline(always)]
                    fn $addsub_fun(self, rhs: Quantity<D, Ur, V>) -> Self::Output {
                        Quantity {
                            dimension: $crate::lib::marker::PhantomData,
                            units: $crate::lib::marker::PhantomData,
                            value: self.value $addsub_op change_base::<D, Ul, Ur, V>(&rhs.value),
                        }
                    }
                }

                impl<D, Ul, Ur, V> $crate::lib::ops::$AddSubAssignTrait<Quantity<D, Ur, V>>
                    for Quantity<D, Ul, V>
                where
                    D: Dimension + ?Sized,
                    Ul: Units<V> + ?Sized,
                    Ur: Units<V> + ?Sized,
                    V: $crate::num::Num + $crate::Conversion<V>
                        + $crate::lib::ops::$AddSubAssignTrait<V>,
                {
                    #[inline(always)]
                    fn $addsubassign_fun(&mut self, rhs: Quantity<D, Ur, V>) {
                        self.value $addsubassign_op change_base::<D, Ul, Ur, V>(&rhs.value);
                    }
                }

                impl<Dl, Dr, Ul, Ur, V> $crate::lib::ops::$MulDivTrait<Quantity<Dr, Ur, V>>
                    for Quantity<Dl, Ul, V>
                where
                    Dl: Dimension + ?Sized,
                    $(Dl::$symbol: $crate::lib::ops::$AddSubTrait<Dr::$symbol>,)+
                    Dr: Dimension + ?Sized,
                    Ul: Units<V> + ?Sized,
                    Ur: Units<V> + ?Sized,
                    V: $crate::num::Num + $crate::Conversion<V> + $crate::lib::ops::$MulDivTrait<V>,
                {
                    type Output = Quantity<
                        $quantities<$($crate::typenum::$AddSubAlias<Dl::$symbol, Dr::$symbol>,)+>,
                        Ul, V>;

                    #[inline(always)]
                    fn $muldiv_fun(self, rhs: Quantity<Dr, Ur, V>) -> Self::Output {
                        Quantity {
                            dimension: $crate::lib::marker::PhantomData,
                            units: $crate::lib::marker::PhantomData,
                            value: self.value
                                $muldiv_op change_base::<Dr, Ul, Ur, V>(&rhs.value),
                        }
                    }
                }

                impl<D, U, V> $crate::lib::ops::$MulDivTrait<V> for Quantity<D, U, V>
                where
                    D: Dimension + ?Sized,
                    U: Units<V> + ?Sized,
                    V: $crate::num::Num + $crate::Conversion<V>,
                {
                    type Output = Quantity<D, U, V>;

                    #[inline(always)]
                    fn $muldiv_fun(self, rhs: V) -> Self::Output {
                        Quantity {
                            dimension: $crate::lib::marker::PhantomData,
                            units: $crate::lib::marker::PhantomData,
                            value: self.value $muldiv_op rhs,
                        }
                    }
                }

                impl<D, U, V> $crate::lib::ops::$MulDivAssignTrait<V> for Quantity<D, U, V>
                where
                    D: Dimension + ?Sized,
                    U: Units<V> + ?Sized,
                    V: $crate::num::Num + $crate::Conversion<V>
                        + $crate::lib::ops::$MulDivAssignTrait<V>,
                {
                    #[inline(always)]
                    fn $muldivassign_fun(&mut self, rhs: V) {
                        self.value $muldivassign_op rhs;
                    }
                }

                #[doc(hidden)]
                mod $Mod {
                    storage_types! {
                        use super::super::*;

                        impl<D, U> $crate::lib::ops::$MulDivTrait<Quantity<D, U, V>> for V
                        where
                            D: Dimension + ?Sized,
                            U: Units<V> + ?Sized,
                            $($crate::typenum::Z0: $crate::lib::ops::$AddSubTrait<D::$symbol>,)+
                        {
                            type Output = Quantity<
                                $quantities<
                                    $($crate::typenum::$AddSubAlias<
                                      $crate::typenum::Z0,
                                      D::$symbol>,)+>,
                                U, V>;

                            #[inline(always)]
                            fn $muldiv_fun(self, rhs: Quantity<D, U, V>) -> Self::Output {
                                Quantity {
                                    dimension: $crate::lib::marker::PhantomData,
                                    units: $crate::lib::marker::PhantomData,
                                    value: self $muldiv_op rhs.value,
                                }
                            }
                        }
                    }
                }
            };
        }

        impl_ops!(Add, add, +, AddAssign, add_assign, +=, Sum,
            Mul, mul, *, MulAssign, mul_assign, *=, add_mul);
        impl_ops!(Sub, sub, -, SubAssign, sub_assign, -=, Diff,
            Div, div, /, DivAssign, div_assign, /=, sub_div);

        impl<D, U, V> Quantity<D, U, V>
        where
            D: Dimension + ?Sized,
            U: Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V>,
        {
            /// Returns `true` if this value is `NAN` and `false` otherwise.
            #[cfg_attr(feature = "cargo-clippy", allow(wrong_self_convention))]
            #[inline(always)]
            pub fn is_nan(self) -> bool
            where
                V: $crate::num::Float,
            {
                self.value.is_nan()
            }

            /// Returns `true` if this value is positive infinity or negative infinity and
            /// `false` otherwise.
            #[cfg_attr(feature = "cargo-clippy", allow(wrong_self_convention))]
            #[inline(always)]
            pub fn is_infinite(self) -> bool
            where
                V: $crate::num::Float,
            {
                self.value.is_infinite()
            }

            /// Returns `true` if this number is neither infinite nor `NAN`.
            #[cfg_attr(feature = "cargo-clippy", allow(wrong_self_convention))]
            #[inline(always)]
            pub fn is_finite(self) -> bool
            where
                V: $crate::num::Float,
            {
                self.value.is_finite()
            }

            /// Returns `true` if the number is neither zero, infinite, subnormal, or `NAN`.
            #[cfg_attr(feature = "cargo-clippy", allow(wrong_self_convention))]
            #[inline(always)]
            pub fn is_normal(self) -> bool
            where
                V: $crate::num::Float,
            {
                self.value.is_normal()
            }

            /// Returns the floating point category of the number. If only one property is
            /// going to be tested, it is generally faster to use the specific predicate
            /// instead.
            #[inline(always)]
            pub fn classify(self) -> $crate::lib::num::FpCategory
            where
                V: $crate::num::Float,
            {
                self.value.classify()
            }

            /// Takes the cubic root of a number.
            ///
            #[cfg_attr(all(feature = "si", feature = "f32"), doc = " ```rust")]
            #[cfg_attr(not(all(feature = "si", feature = "f32")), doc = " ```rust,ignore")]
            /// # use uom::si::f32::*;
            /// # use uom::si::volume::cubic_meter;
            /// let l: Length = Volume::new::<cubic_meter>(8.0).cbrt();
            /// ```
            ///
            /// The input type must have dimensions divisible by three:
            ///
            #[cfg_attr(all(feature = "si", feature = "f32"), doc = " ```rust,compile_fail")]
            #[cfg_attr(not(all(feature = "si", feature = "f32")), doc = " ```rust,ignore")]
            /// # use uom::si::f32::*;
            /// # use uom::si::area::square_meter;
            /// // error[E0271]: type mismatch resolving ...
            /// let r = Area::new::<square_meter>(8.0).cbrt();
            /// ```
            #[inline(always)]
            pub fn cbrt(
                self
            ) -> Quantity<
                $quantities<$($crate::typenum::PartialQuot<D::$symbol, $crate::typenum::P3>),+>,
                U, V>
            where
                $(D::$symbol: $crate::lib::ops::PartialDiv<$crate::typenum::P3>,)+
                V: $crate::num::Float,
            {
                Quantity {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value: self.value.cbrt(),
                }
            }

            /// Computes the absolute value of `self`. Returns `NAN` if the quantity is
            /// `NAN`.
            #[inline(always)]
            pub fn abs(self) -> Self
            where
                V: $crate::num::Signed,
            {
                Quantity {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value: self.value.abs(),
                }
            }

            /// Returns a quantity that represents the sign of `self`.
            ///
            /// * `1.0` of the base unit if the number is positive, `+0.0`, or `INFINITY`.
            /// * `-1.0` of the base unit if the number is negative, `-0.0`, or
            ///   `NEG_INFINITY`.
            /// * `NAN` if the number is `NAN`.
            #[inline(always)]
            pub fn signum(self) -> Self
            where
                V: $crate::num::Signed,
            {
                Quantity {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value: self.value.signum(),
                }
            }

            /// Returns `true` if `self`'s sign bit is positive, including `+0.0` and
            /// `INFINITY`.
            #[cfg_attr(feature = "cargo-clippy", allow(wrong_self_convention))]
            #[inline(always)]
            pub fn is_sign_positive(self) -> bool
            where
                V: $crate::num::Float,
            {
                self.value.is_sign_positive()
            }

            /// Returns `true` if `self`'s sign is negative, including `-0.0` and
            /// `NEG_INFINITY`.
            #[cfg_attr(feature = "cargo-clippy", allow(wrong_self_convention))]
            #[inline(always)]
            pub fn is_sign_negative(self) -> bool
            where
                V: $crate::num::Float,
            {
                self.value.is_sign_negative()
            }

            /// Fused multiply-add. Computes `(self * a) + b` with only one rounding error.
            /// This produces a more accurate result with better performance than a separate
            /// multiplication operation followed by an add.
            #[inline(always)]
            pub fn mul_add<Da, Ua, Ub>(
                self,
                a: Quantity<Da, Ua, V>,
                b: Quantity<$quantities<$($crate::typenum::Sum<D::$symbol, Da::$symbol>),+>, Ub, V>,
            ) -> Quantity<$quantities<$($crate::typenum::Sum<D::$symbol, Da::$symbol>),+>, U, V>
            where
                $(D::$symbol: $crate::lib::ops::Add<Da::$symbol>,)+
                V: $crate::num::Float,
                Da: Dimension + ?Sized,
                Ua: Units<V> + ?Sized,
                Ub: Units<V> + ?Sized,
            {
                // (self * a) + b
                Quantity {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value: self.value.mul_add(a.value, b.value),
                }
            }

            /// Takes the reciprocal (inverse) of a number, `1/x`.
            ///
            #[cfg_attr(all(feature = "si", feature = "f32"), doc = " ```rust")]
            #[cfg_attr(not(all(feature = "si", feature = "f32")), doc = " ```rust,ignore")]
            /// # use uom::si::f32::*;
            /// # use uom::si::time::second;
            /// let f: Frequency = Time::new::<second>(1.0).recip();
            /// ```
            #[inline(always)]
            pub fn recip(
                self
            ) -> Quantity<$quantities<$($crate::typenum::Negate<D::$symbol>),+>, U, V>
            where
                $(D::$symbol: $crate::lib::ops::Neg,)+
                V: $crate::num::Float,
            {
                Quantity {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value: self.value.recip(),
                }
            }

            /// Raises a quantity to an integer power.
            ///
            #[cfg_attr(all(feature = "si", feature = "f32"), doc = " ```rust")]
            #[cfg_attr(not(all(feature = "si", feature = "f32")), doc = " ```rust,ignore")]
            /// # use uom::si::f32::*;
            /// # use uom::si::length::meter;
            /// let a: Area = Length::new::<meter>(3.0).powi(::uom::typenum::P2::new());
            /// ```
            #[inline(always)]
            pub fn powi<E>(
                self, e: E
            ) -> Quantity<$quantities<$($crate::typenum::Prod<D::$symbol, E>),+>, U, V>
            where
                $(D::$symbol: $crate::lib::ops::Mul<E>,)+
                E: $crate::typenum::Integer,
                V: $crate::typenum::Pow<E, Output = V> + $crate::Conversion<V>,
            {
                Quantity {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value: $crate::typenum::Pow::powi(self.value, e),
                }
            }

            /// Takes the square root of a number. Returns `NAN` if `self` is a negative
            /// number.
            ///
            #[cfg_attr(all(feature = "si", feature = "f32"), doc = " ```rust")]
            #[cfg_attr(not(all(feature = "si", feature = "f32")), doc = " ```rust,ignore")]
            /// # use uom::si::f32::*;
            /// # use uom::si::area::square_meter;
            /// let l: Length = Area::new::<square_meter>(4.0).sqrt();
            /// ```
            ///
            /// The input type must have dimensions divisible by two:
            ///
            #[cfg_attr(all(feature = "si", feature = "f32"), doc = " ```rust,compile_fail")]
            #[cfg_attr(not(all(feature = "si", feature = "f32")), doc = " ```rust,ignore")]
            /// # use uom::si::f32::*;
            /// # use uom::si::length::meter;
            /// // error[E0271]: type mismatch resolving ...
            /// let r = Length::new::<meter>(4.0).sqrt();
            /// ```
            #[inline(always)]
            pub fn sqrt(
                self
            ) -> Quantity<
                $quantities<$($crate::typenum::PartialQuot<D::$symbol, $crate::typenum::P2>),+>,
                U, V>
            where
                $(D::$symbol: $crate::typenum::PartialDiv<$crate::typenum::P2>,)+
                V: $crate::num::Float,
            {
                Quantity {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value: self.value.sqrt(),
                }
            }

            /// Returns the maximum of the two quantities.
            #[inline(always)]
            pub fn max(self, other: Self) -> Self
            where
                V: $crate::num::Float,
            {
                Quantity {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value: self.value.max(other.value),
                }
            }

            /// Returns the minimum of the two quantities.
            #[inline(always)]
            pub fn min(self, other: Self) -> Self
            where
                V: $crate::num::Float,
            {
                Quantity {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value: self.value.min(other.value),
                }
            }
        }

        impl<D, U, V> $crate::lib::ops::Neg for Quantity<D, U, V>
        where
            D: Dimension + ?Sized,
            U: Units<V> + ?Sized,
            V: $crate::num::Signed + $crate::Conversion<V>,
        {
            type Output = Quantity<D, U, V>;

            #[inline(always)]
            fn neg(self) -> Self::Output {
                Quantity {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value: -self.value,
                }
            }
        }

        impl<D, Ul, Ur, V> $crate::lib::ops::Rem<Quantity<D, Ur, V>>
            for Quantity<D, Ul, V>
        where
            D: Dimension + ?Sized,
            Ul: Units<V> + ?Sized,
            Ur: Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V>,
        {
            type Output = Quantity<D, Ul, V>;

            #[inline(always)]
            fn rem(self, rhs: Quantity<D, Ur, V>) -> Self::Output {
                Quantity {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value: self.value % change_base::<D, Ul, Ur, V>(&rhs.value)
                }
            }
        }

        impl<D, Ul, Ur, V> $crate::lib::ops::RemAssign<Quantity<D, Ur, V>>
            for Quantity<D, Ul, V>
        where
            D: Dimension + ?Sized,
            Ul: Units<V> + ?Sized,
            Ur: Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V> + $crate::lib::ops::RemAssign,
        {
            #[inline(always)]
            fn rem_assign(&mut self, rhs: Quantity<D, Ur, V>) {
                self.value %= change_base::<D, Ul, Ur, V>(&rhs.value)
            }
        }

        impl<D, U, V> $crate::num::Zero for Quantity<D, U, V>
        where
            D: Dimension + ?Sized,
            U: Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V>,
        {
            fn zero() -> Self {
                Quantity {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value: V::zero(),
                }
            }

            fn is_zero(&self) -> bool {
                self.value.is_zero()
            }
        }

        /// Macro to implement [`quantity`](si/struct.Quantity.html) type aliases for a specific
        /// [system of units][units] and value storage type.
        ///
        /// * `$path`: Path to the module where the [`system!`](macro.system.html) macro was run
        ///   (e.g. `::uom::si`).
        /// * `$V`: Underlying value storage type (e.g. `f32`).
        /// * `$U`: Optional. Base units. Pass as a tuple with the desired units: `(meter, kilogram,
        ///   second, ampere, kelvin, mole, candela)`. The system's base units will be used if no
        ///   value is provided.
        ///
        /// An example invocation is given below for a meter-kilogram-second system setup in the
        /// module `mks` with a system of quantities name `Q`. The `#[macro_use]` attribute must be
        /// used when including the `uom` crate to make macros for predefined systems available.
        /// The optional units parameter to change the base units is included commented out.
        ///
        /// ```
        /// #[macro_use]
        /// extern crate uom;
        ///
        /// # fn main() { }
        /// # mod mks {
        /// #     #[macro_use]
        /// #     mod length {
        /// #         quantity! {
        /// #             /// Length (base unit meter, m<sup>1</sup>).
        /// #             quantity: Length; "length";
        /// #             /// Length dimension, m<sup>1</sup>.
        /// #             dimension: Q<P1 /*length*/, Z0 /*mass*/, Z0 /*time*/>;
        /// #             units {
        /// #                 @meter: 1.0E0; "m", "meter", "meters";
        /// #                 @foot: 3.048E-1; "ft", "foot", "feet";
        /// #             }
        /// #         }
        /// #     }
        /// #     #[macro_use]
        /// #     mod mass {
        /// #         quantity! {
        /// #             /// Mass (base unit kilogram, kg<sup>1</sup>).
        /// #             quantity: Mass; "mass";
        /// #             /// Mass dimension, kg<sup>1</sup>.
        /// #             dimension: Q<Z0 /*length*/, P1 /*mass*/, Z0 /*time*/>;
        /// #             units {
        /// #                 @kilogram: 1.0; "kg", "kilogram", "kilograms";
        /// #             }
        /// #         }
        /// #     }
        /// #     #[macro_use]
        /// #     mod time {
        /// #         quantity! {
        /// #             /// Time (base unit second, s<sup>1</sup>).
        /// #             quantity: Time; "time";
        /// #             /// Time dimension, s<sup>1</sup>.
        /// #             dimension: Q<Z0 /*length*/, Z0 /*mass*/, P1 /*time*/>;
        /// #             units {
        /// #                 @second: 1.0; "s", "second", "seconds";
        /// #             }
        /// #         }
        /// #     }
        /// #     system! {
        /// #         /// System of quantities, Q.
        /// #         quantities: Q {
        /// #             length: meter, L;
        /// #             mass: kilogram, M;
        /// #             time: second, T;
        /// #         }
        /// #         /// System of units, U.
        /// #         units: U {
        /// #             mod length::Length,
        /// #             mod mass::Mass,
        /// #             mod time::Time,
        /// #         }
        /// #     }
        /// mod f32 {
        ///     Q!(mks, f32/*, (centimeter, gram, second)*/);
        /// }
        /// # }
        /// ```
        ///
        /// [units]: http://jcgm.bipm.org/vim/en/1.13.html
        #[macro_export]
        macro_rules! $quantities {
            ($path:path) => {
                use $path as system;

                $(/// [`Quantity`](struct.Quantity.html) type alias using the default base units
                /// parameterized on the underlying storage type.
                #[allow(dead_code)]
                #[allow(unused_qualifications)]
                pub type $quantity<V> = system::$module::$quantity<system::$units<V>, V>;)+
            };
            ($path:path, $V:ty) => {
                use $path as system;

                $(/// [`Quantity`](struct.Quantity.html) type alias using the default base units.
                #[allow(dead_code)]
                #[allow(unused_qualifications)]
                pub type $quantity = system::$module::$quantity<system::$units<$V>, $V>;)+
            };
            ($path:path, $V:ty, $U:tt) => {
                system!(@quantities $path, $V; $($name),+; $U; $($module::$quantity),+);
            };
        }
    };
    (
        @quantities $path:path,
        $V:ty;
        $($name:ident),+;
        ($($U:ident),+);
        $($module:ident::$quantity:ident),+
    ) => {
        use $path as system;

        type Units = system::Units<$V, $($name = system::$name::$U,)+>;

        $(/// [`Quantity`](struct.Quantity.html) type alias using the given base units.
        #[allow(dead_code)]
        #[allow(unused_qualifications)]
        pub type $quantity = system::$module::$quantity<Units, $V>;)+
    };
    (@replace $_t:tt $sub:ty) => { $sub };
}
