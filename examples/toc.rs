use rust_epub::toc;
fn main() {
    let mut nav1 = toc::NavPoint::new("np_1", "总目录".parse().unwrap(), "Text/part0000.xhtml#aid-1".parse().unwrap());

    let mut nav2 =nav1.clone();
    nav2.id("mp_2".parse().unwrap()).nav_label("第一章".parse().unwrap());

    nav1.add_nav_point(nav2);

    // 手动创建实例
    let mut toc_link = toc::TocElement::new("测试");
    toc_link.lang("zh")
        .add_metadata("3516245767","dtb:uid")
        .add_metadata("6","dtb:depth").
    add_nav_point(nav1);

     let sd = toc_link.encode_xml();
    match sd{
        Ok(s) => println!("{}", s),
        Err(e) => println!("{:?}", e),
    }

}