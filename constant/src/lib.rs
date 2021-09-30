#[macro_use]
mod macros;
mod constdef_impls;
// re-export derives
pub use const_internals::*;

/// # Constant defaults
///
/// The [`Constdef`] trait is the heart of constant, compile-time default values. This trait
/// is automatically implemented for several types in the standard library.
///
/// ## Implementing this trait
///
/// Usually, for implementing this trait -- you'll simply need to use `#[derive(constant::Constdef)]`
/// and the macro will do the magic for you. In other cases, you'll need to implement it yourself,
/// like this for example:
///
/// ```
/// use constant::Constdef;
///
/// struct MyWeirdBool(bool);
///
/// impl Constdef for MyWeirdBool {
///     const DEF: MyWeirdBool = MyWeirdBool(false);
/// }
///
/// ```
///
pub trait Constdef {
    const DEFAULT: Self;
}
