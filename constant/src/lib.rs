//! # `constant`: Constant, compile-time evaluation tools for Rust 🦀
//!
//! ![Rust Stable](https://img.shields.io/badge/rust-stable-50C878?style=for-the-badge) ![GitHub Workflow Status](https://img.shields.io/github/workflow/status/ohsayan/constant/Test?style=for-the-badge) [![Crates.io](https://img.shields.io/crates/v/constant?style=for-the-badge)](https://crates.io/crates/constant) [![docs.rs](https://img.shields.io/docsrs/constant?style=for-the-badge)](https://docs.rs/constant) [![GitHub](https://img.shields.io/github/license/ohsayan/constant?style=for-the-badge)](./LICENSE)
//! The `constant` crate aims to provide tools for safely _working around_ the limits imposed by constant evaluation in Rust.
//!
//! ## Features
//!
//! - Create [constant default implementations](#constant-default-implementations)
//! - Created [nested constant default implementations](#nested-constant-default-implementations)
//! - Full support for generics, lifetimes and generics in `Constdef` impls
//! - Almost all types provided by the `std` are supported by this crate. If you need any other external type, [just open an issue and ask!](https://github.com/ohsayan/constant/issues/new)
//! - More coming soon!
//!
//! ## Constant default implementations
//!
//! ```rust
//! #[derive(constant::Constdef)]
//! pub struct SpookyFriend {
//!     name: String,
//!     email: String,
//!     friend_names: Vec<String>,
//!     userid: u64,
//! }
//! const SPOOKY: SpookyFriend = SpookyFriend::default();
//! #[test]
//! fn test_struct_with_heap_fields() {
//!     // spooky name; it's empty!
//!     assert_eq!(SPOOKY.name, "");
//!     // spooky email; it's empty!
//!     assert_eq!(SPOOKY.email, "");
//!     // spooky friend has no friends!
//!     assert_eq!(SPOOKY.friend_names, Vec::<String>::new());
//!     // spooky userid; it's 0!
//!     assert_eq!(SPOOKY.userid, 0);
//! }
//! ```
//!
//! ## Nested constant default implementations
//!
//! ```rust
//! use constant::Constdef;
//! #[derive(Constdef)]
//! pub struct SystemLoad {
//!     disk: DiskLoad,
//!     net: NetLoad,
//! }
//! #[derive(Constdef)]
//! pub struct DiskLoad {
//!     disk_count: usize,
//!     media_list: Vec<String>,
//! }
//! #[derive(Constdef)]
//! pub struct NetLoad {
//!     interface_list: Vec<String>,
//!     portload: [usize; 65536],
//! }
//! static mut SYSLOAD: SystemLoad = SystemLoad::default();
//! #[test]
//! fn test_system_load_nested_struct() {
//!     unsafe {
//!         // check the number of disks
//!         assert_eq!(SYSLOAD.disk.disk_count, 0);
//!         // increment the number of disks
//!         SYSLOAD.disk.disk_count += 1;
//!         assert_eq!(SYSLOAD.disk.disk_count, 1);
//!         // check port 80 load
//!         assert_eq!(SYSLOAD.net.portload[80], 0);
//!         // increment the load
//!         SYSLOAD.net.portload[80] += 1;
//!         assert_eq!(SYSLOAD.net.portload[80], 1);
//!         // now let's add a disk
//!         SYSLOAD.disk.media_list.push("/dev/sda1".to_string());
//!         // now let's add a network interface
//!         SYSLOAD.net.interface_list.push("eth01".to_string());
//!     }
//! }
//! ```
//!
//! ## License
//!
//! This library is distributed under the [Apache-2.0 License](./LICENSE).
//!

#[macro_use]
mod macros;
mod constdef_impls;
// re-export derives
/// # The `Constdef` macro
/// Overcome the limits of the [`Default`] trait to get constant, compile-time default implementations.
///
/// The [`Constdef`](derive.Constdef.html) derive macro enables you to create constant,
/// compile-time [`Default`] implementations, in the simplest possible way. With restrictions
/// imposed by [RFC 911 on `const` functions](https://rust-lang.github.io/rfcs/0911-const-fn.html#detailed-design),
/// trait methods cannot currently be called in `const` contexts. To work around this, this crate
/// provides you with the [`Constdef`](derive.Constdef.html) macro that peeks into the AST and substitutes
/// the default value at compile-time. This enables you to call the `default` function in constant
/// contexts.
///
/// ## Example
/// ```
/// use constant::Constdef;
///
/// type MyType = u16;
/// #[derive(Constdef)]
/// pub struct SpookyFriend {
///     name: String,
///     email: String,
///     friend_names: Vec<String>,
///     userid: u64,
///     my_tag: MyType,
/// }
///
/// const SPOOKY: SpookyFriend = SpookyFriend::default();
///
/// #[test]
/// fn test_struct_with_heap_fields() {
///     // spooky name; it's empty!
///     assert_eq!(SPOOKY.name, "");
///     // spooky email; it's empty!
///     assert_eq!(SPOOKY.email, "");
///     // spooky friend has no friends!
///     assert!(SPOOKY.friend_names.is_empty());
///     // spooky userid; it's 0!
///     assert_eq!(SPOOKY.userid, 0);
///     // spooky tag; it's 0!
///     assert_eq!(SPOOKY.mytag, 0);
/// }
/// ```
/// Even more complex types are supported. See [crate level docs](crate) for more information.
///
pub use const_internals::Constdef;

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
///     const DEFAULT: MyWeirdBool = MyWeirdBool(false);
/// }
///
/// ```
///
pub trait Constdef {
    /// The default value for Self
    const DEFAULT: Self;
}
