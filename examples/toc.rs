use std::ptr::write;
use rust_epub::epub::EpubVersion::V20;
use rust_epub::toc;
fn main() {
   let mut nav = toc::TocNav::new("梦回大明春","zh");

   let mut el1 = toc::TocElement::new("Text/cover.xhtml","封面");
   let mut el2 = toc::TocElement::new("Text/Chapter0002.xhtml","第一卷 蛮家子");
   el2.add_child(toc::TocElement::new("Text/Chapter0003.xhtml","第001章 工程狗是什么品种？")).
   add_child(toc::TocElement::new("Text/Chapter0004.xhtml","第002章 土匪式拜师"));

   nav.add_element(el1).add_element(el2);

   let ret =  nav.encode_file(V20);
   match ret {
      Ok(s) => println!("{}", s),
      Err(e) => println!("{:?}", e),
   }

}