use rust_epub;
use rust_epub::epub::EpubVersion::*;
use rust_epub::toc::*;

#[test]
fn encode_ncx_file() {
    let mut nav = TocNav::new("梦回大明春", "zh");

    nav.add_metadata("dtb:uid", "easypub-8a83e708")
        .add_metadata("dtb:depth", "2");

    let el1 = TocElement::new("Text/cover.xhtml", "封面");
    let mut el2 = TocElement::new("Text/Chapter0002.xhtml", "第一卷 蛮家子");
    el2.add_child(TocElement::new(
        "Text/Chapter0003.xhtml",
        "第001章 工程狗是什么品种？",
    ))
    .add_child(TocElement::new(
        "Text/Chapter0004.xhtml",
        "第002章 土匪式拜师",
    ));

    nav.add_element(el1).add_element(el2);

    let ret = nav.encode_file(V20);
    assert!(ret.is_ok());
    println!("{}", ret.unwrap());
}

#[test]
fn encode_nav_file() {
    let mut nav = TocNav::new("梦回大明春", "zh");

    nav.add_metadata("dtb:uid", "easypub-8a83e708")
        .add_metadata("dtb:depth", "2");

    let el1 = TocElement::new("Text/cover.xhtml", "封面");
    let mut el2 = TocElement::new("Text/Chapter0002.xhtml", "第一卷 蛮家子");
    el2.add_child(TocElement::new(
        "Text/Chapter0003.xhtml",
        "第001章 工程狗是什么品种？",
    ))
    .add_child(TocElement::new(
        "Text/Chapter0004.xhtml",
        "第002章 土匪式拜师",
    ));

    nav.add_element(el1).add_element(el2);

    let ret = nav.encode_file(V30);

    assert!(ret.is_ok());
    println!("{}", ret.unwrap());
}
