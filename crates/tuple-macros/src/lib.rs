//! Provides macros to work with tuples
#![warn(missing_docs)]

/// calls a macro provided as argument for tuples of all sizes
///
/// # Example
/// ```
/// trait Foo {}
/// macro_rules! impl_for_tuple {
///     ($($a:ident),*) => {
///         impl<$($a),*> Foo for ($($a,)*) {}
///     }
/// }
/// batbox_tuple_macros::call_for_tuples!(impl_for_tuple);
/// ```
#[macro_export]
macro_rules! call_for_tuples {
    ($macro:ident) => {
        $macro!();
        $macro!(a0);
        $macro!(a0, a1);
        $macro!(a0, a1, a2);
        $macro!(a0, a1, a2, a3);
        $macro!(a0, a1, a2, a3, a4);
        $macro!(a0, a1, a2, a3, a4, a5);
        $macro!(a0, a1, a2, a3, a4, a5, a6);
        $macro!(a0, a1, a2, a3, a4, a5, a6, a7);
        $macro!(a0, a1, a2, a3, a4, a5, a6, a7, a8);
        $macro!(a0, a1, a2, a3, a4, a5, a6, a7, a8, a9);
    };
}

#[macro_export]
macro_rules! call_for_tuples_2 {
    ($macro:ident) => {
        $macro!(;);
        $macro!(a0; b0);
        $macro!(a0, a1; b0, b1);
        $macro!(a0, a1, a2; b0, b1, b2);
        $macro!(a0, a1, a2, a3; b0, b1, b2, b3);
        $macro!(a0, a1, a2, a3, a4; b0, b1, b2, b3, b4);
        $macro!(a0, a1, a2, a3, a4, a5; b0, b1, b2, b3, b4, b5);
        $macro!(a0, a1, a2, a3, a4, a5, a6; b0, b1, b2, b3, b4, b5, b6);
        $macro!(a0, a1, a2, a3, a4, a5, a6, a7; b0, b1, b2, b3, b4, b5, b6, b7);
        $macro!(a0, a1, a2, a3, a4, a5, a6, a7, a8; b0, b1, b2, b3, b4, b5, b6, b7, b8);
        $macro!(a0, a1, a2, a3, a4, a5, a6, a7, a8, a9; b0, b1, b2, b3, b4, b5, b6, b7, b8, b9);
    };
}

pub trait TupleZip<RHS> {
    type Output;
    fn tuple_zip(self, rhs: RHS) -> Self::Output;
}

macro_rules! impl_tuple_zip {
    ($($a:ident),*; $($b:ident),*) => {
        impl <$($a,)* $($b,)*> TupleZip<($($b,)*)> for ($($a,)*) {
            type Output = ($(($a, $b),)*);
            fn tuple_zip(self, ($($b,)*): ($($b,)*)) -> Self::Output {
                #![allow(clippy::unused_unit)]
                let ($($a,)*) = self;
                ($(($a, $b),)*)
            }
        }
    };
}

call_for_tuples_2!(impl_tuple_zip);

#[test]
fn test_tuple_zip() {
    let zipped = (1, 2).tuple_zip(("hello", "world"));
    assert_eq!(zipped, ((1, "hello"), (2, "world")));
}
