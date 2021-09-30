# `constant`: Constant, compile-time evaluation tools for Rust 🦀

![Rust Stable](https://img.shields.io/badge/rust-stable-50C878?style=for-the-badge) ![GitHub Workflow Status](https://img.shields.io/github/workflow/status/ohsayan/constant/Test?style=for-the-badge) [![Crates.io](https://img.shields.io/crates/v/constant?style=for-the-badge)](https://crates.io/crates/constant) [![docs.rs](https://img.shields.io/docsrs/constant?style=for-the-badge)](https://docs.rs/constant) [![GitHub](https://img.shields.io/github/license/ohsayan/constant?style=for-the-badge)](./LICENSE)

The `constant` crate aims to provide tools for safely _working around_ the limits imposed by constant evaluation in Rust.

## Features

- Create [constant default implementations](#constant-default-implementations)
- Created [nested constant default implementations](#nested-constant-default-implementations)
- Full support for generics, lifetimes and generics in `Constdef` impls
- Almost all types provided by the `std` are supported by this crate. If you need any other external type, [just open an issue and ask!](https://github.com/ohsayan/constant/issues/new)
- More coming soon!

## Constant default implementations

```rust
#[derive(constant::Constdef)]
pub struct SpookyFriend {
    name: String,
    email: String,
    friend_names: Vec<String>,
    userid: u64,
}

const SPOOKY: SpookyFriend = SpookyFriend::default();

#[test]
fn test_struct_with_heap_fields() {
    // spooky name; it's empty!
    assert_eq!(SPOOKY.name, "");
    // spooky email; it's empty!
    assert_eq!(SPOOKY.email, "");
    // spooky friend has no friends!
    assert_eq!(SPOOKY.friend_names, Vec::<String>::new());
    // spooky userid; it's 0!
    assert_eq!(SPOOKY.userid, 0);
}
```

## Nested constant default implementations

```rust
use constant::Constdef;

#[derive(Constdef)]
pub struct SystemLoad {
    disk: DiskLoad,
    net: NetLoad,
}

#[derive(Constdef)]
pub struct DiskLoad {
    disk_count: usize,
    media_list: Vec<String>,
}

#[derive(Constdef)]
pub struct NetLoad {
    interface_list: Vec<String>,
    portload: [usize; 65536],
}

static mut SYSLOAD: SystemLoad = SystemLoad::default();

#[test]
fn test_system_load_nested_struct() {
    unsafe {
        // check the number of disks
        assert_eq!(SYSLOAD.disk.disk_count, 0);
        // increment the number of disks
        SYSLOAD.disk.disk_count += 1;
        assert_eq!(SYSLOAD.disk.disk_count, 1);
        // check port 80 load
        assert_eq!(SYSLOAD.net.portload[80], 0);
        // increment the load
        SYSLOAD.net.portload[80] += 1;
        assert_eq!(SYSLOAD.net.portload[80], 1);

        // now let's add a disk
        SYSLOAD.disk.media_list.push("/dev/sda1".to_string());
        // now let's add a network interface
        SYSLOAD.net.interface_list.push("eth01".to_string());
    }
}
```

## License

This library is distributed under the [Apache-2.0 License](./LICENSE).
