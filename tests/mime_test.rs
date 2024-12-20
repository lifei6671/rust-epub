use std::ffi::OsStr;
use std::path::Path;
use rust_epub::mime::{from_path, get_mime_type};

#[test]
fn test_from_path() {
    let path =Path::new("tests/resources/mimes.mp4");
    let ext_str =    path.extension().and_then(OsStr::to_str);
    assert!(ext_str.is_some());
    assert_eq!(ext_str.unwrap(), "mp4");
    let ext = from_path(path);
     assert!(ext.is_some());
    assert_eq!(ext.unwrap(), "video/mp4");
}

#[test]
fn test_get_mime_type() {
    let path =Path::new("tests/resources/mimes.mp4");
    let ext_str =    path.extension().and_then(OsStr::to_str);
    assert!(ext_str.is_some());
    assert_eq!(ext_str.unwrap(), "mp4");
    let ext = get_mime_type(ext_str.unwrap().as_ref());
    assert!(ext.is_some());

    let ext_str = get_mime_type("mmmm");
    println!("{:?}",ext_str);
    assert!(ext_str.is_none());
}