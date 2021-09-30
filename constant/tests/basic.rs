use constant::Constdef;

#[derive(Constdef, Debug)]
pub struct EmptyStruct {}

#[test]
fn test_empty_struct() {
    const ES: EmptyStruct = EmptyStruct::default();
    // just for avoiding the unused warning
    let _debug_view = format!("{:?}", ES);
}

#[derive(Constdef)]
pub struct BasicStruct {
    userid: u64,
    verified: bool,
}

#[test]
fn test_basic_struct() {
    const BS: BasicStruct = BasicStruct::default();
    assert_eq!(BS.userid, 0);
    assert!(!BS.verified);
}

#[derive(Constdef)]
pub struct SpookyFriend {
    name: String,
    email: String,
    friend_names: Vec<String>,
    userid: u64,
}

#[test]
fn test_struct_with_heap_fields() {
    const SPOOKY: SpookyFriend = SpookyFriend::default();
    // spooky name; it's empty!
    assert_eq!(SPOOKY.name, "");
    // spooky email; it's empty!
    assert_eq!(SPOOKY.email, "");
    // spooky friend has no friends!
    assert_eq!(SPOOKY.friend_names, Vec::<String>::new());
    // spooky userid; it's 0!
    assert_eq!(SPOOKY.userid, 0);
}
