use rust_epub::xhtml::{StyleContent, XHtmlLinkItem, XHtmlRoot};
#[test]
fn test_encode_xml() {
    let mut html = XHtmlRoot::default();

    html.set_title("第1章 大语言模型：辩论、争议与未来发展方向");
    html.add_link(XHtmlLinkItem::new(
        "https://www.epubit.com/book/25106",
        "text/html",
        "",
    ));

    html.add_style_content("body {font-size: 14px;}");
    html.add_style_content("h1 {font-size: 20px;}");
    html.add_style(StyleContent::new("p {font-size: 14px;}", "text/css"));

    html.set_body("<h1>第1章 大语言模型：辩论、争议与未来发展方向</h1>");

    let ret = html.encode_xml();

    println!("ret: {:?}", ret);
}
