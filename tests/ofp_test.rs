use rust_epub;
use rust_epub::epub::EpubVersion;
use rust_epub::opf::{BindingItem, GuideReference, ManifestItem, Metadata, SpineItemRef};

#[test]
fn test_encode_v2_xml() {
    let mut opf = rust_epub::opf::Package::new();
    let mut metadata = Metadata::default();
    metadata.title = String::from("北宋小厨师");
    metadata.set_date_published(chrono::Utc::now());
    metadata.set_category("历史穿越");
    metadata.set_creator("南希北庆");
    metadata.set_description("身为一家超五星级酒店首席大厨的李奇，因为喝了点小酒，竟然奇迹般的穿越到了北宋末年。在北宋末年那个风雨飘摇的年代，身为现代人的他究竟会带来那些奇迹呢？");
    metadata.set_publisher("掌上书苑");
    metadata.set_format("application/epub+zip");

    opf.set_metadata(metadata);

    let mut manifest = ManifestItem::new("stylesheet", "styles.css", "text/css");
    manifest.id = String::from("content");
    manifest.href = String::from("text.xhtml");
    manifest.media_type = String::from("application/xhtml+xml");

    opf.add_manifest(manifest).add_manifest(ManifestItem::new(
        "stylesheet",
        "styles.css",
        "text/css",
    ));

    opf.add_spine(SpineItemRef::new("cover.xhtml"));
    opf.add_spine(SpineItemRef::new("text.xhtml"));
    opf.add_spine(SpineItemRef::new("cover.xhtml"));

    opf.add_guide(GuideReference::new("Cover", "cover", "cover.xhtml"));

    opf.add_binding(BindingItem::new("application/x-dtbncx+xml", "toc.ncx"));

    let ret = opf.encode_xml(EpubVersion::V20);
    println!("{}", ret.unwrap_or_default());
}

#[test]
fn test_encode_v3_xml() {
    let mut opf = rust_epub::opf::Package::new();
    let mut metadata = Metadata::default();
    metadata.title = String::from("北宋小厨师");
    metadata.set_date_published(chrono::Utc::now());
    metadata.set_category("历史穿越");
    metadata.set_creator("南希北庆");
    metadata.set_description("身为一家超五星级酒店首席大厨的李奇，因为喝了点小酒，竟然奇迹般的穿越到了北宋末年。在北宋末年那个风雨飘摇的年代，身为现代人的他究竟会带来那些奇迹呢？");
    metadata.set_publisher("掌上书苑");
    metadata.set_format("application/epub+zip");

    opf.set_metadata(metadata);

    let mut manifest = ManifestItem::new("stylesheet", "styles.css", "text/css");
    manifest.id = String::from("content");
    manifest.href = String::from("text.xhtml");
    manifest.media_type = String::from(
        "
        application/xhtml+xml",
    );
    opf.add_manifest(manifest)
        .add_spine(SpineItemRef::new("cover.xhtml"));
    opf.add_spine(SpineItemRef::new("text.xhtml"));
    opf.add_spine(SpineItemRef::new("cover.xhtml"));
    opf.add_guide(GuideReference::new("Cover", "cover", "cover.xhtml"));

    opf.add_binding(BindingItem::new("application/x-dtbncx+xml", "toc.ncx"));

    let ret = opf.encode_xml(EpubVersion::V30);
    println!("{}", ret.unwrap_or_default());
}
