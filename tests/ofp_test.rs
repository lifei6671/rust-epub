use rust_epub;
use rust_epub::epub::EpubVersion;


#[test]
fn test_encode_v2_xml() {
    let opf = rust_epub::opf::Package::new();

    let ret = opf.encode_xml(EpubVersion::V20);
    println!("{:?}", ret);

}