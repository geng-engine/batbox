//! Diffing structs
#![warn(missing_docs)]

pub use batbox_diff_derive::*;

/// A diffable type
///
/// [Can be derived](::batbox_derive::Diff)
///
/// For [Copy] types implementation just uses the type itself as delta.
pub trait Diff {
    /// Object representing the difference between two states of Self
    type Delta;

    /// Calculate the difference between two states
    fn diff(&self, to: &Self) -> Self::Delta;

    /// Update the state using the delta
    ///
    /// ```
    /// # use batbox_diff::*;
    /// let a = 0_i32;
    /// let b = 1_i32;
    /// let delta = Diff::diff(&a, &b);
    ///
    /// let mut a = a;
    /// a.update(&delta);
    /// assert_eq!(a, b);
    /// ```
    fn update(&mut self, delta: &Self::Delta);
}

impl<T: Copy> Diff for T {
    type Delta = Self;
    fn diff(&self, to: &Self) -> Self {
        *to
    }
    fn update(&mut self, new_value: &Self) {
        *self = *new_value;
    }
}
