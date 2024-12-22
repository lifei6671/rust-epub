use std::env;
use rust_epub::epub::EpubBuilder;

#[test]
fn test_add_css() {
    let mut epub = EpubBuilder::new("大语言模型：辩论");
    let current_dir = env::current_dir().
        unwrap().join("tests").
        join("testdata").
        join("cover.jpg").
        to_str().unwrap().to_string();

    let image_1 = epub.add_image(&current_dir, None);
    println!("{:?}", image_1);
    assert!(image_1.is_ok());
    assert_eq!(image_1.unwrap(), "../images/cover.jpg");

    let ret = epub.add_image(&current_dir, None);

    assert!(ret.is_ok());
    let image_str_2 = ret.unwrap();
    assert_eq!(image_str_2, "../images/image_2.jpg");

    println!("{:?}", image_str_2);
}