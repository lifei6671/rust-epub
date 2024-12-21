use quick_xml::DeError;
use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;
use serde_with::{serde_as, NoneAsEmptyString, SerializeAs};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "html")]
pub struct XHtmlRoot {
    #[serde(rename = "@xmlns")]
    xmlns: String,
    #[serde(rename = "@xmlns:epub")]
    xmlns_epub: String,
    #[serde(rename = "head")]
    head: XHtmlHead,
    #[serde(rename = "body")]
    body: XHtmlBody,
}

impl Default for XHtmlRoot {
    fn default() -> Self {
        XHtmlRoot {
            xmlns: String::from("http://www.w3.org/1999/xhtml"),
            xmlns_epub: String::from("http://www.idpf.org/2007/ops"),
            head: XHtmlHead::default(),
            body: XHtmlBody::default(),
        }
    }
}

impl XHtmlRoot {
    pub fn set_body<S: Into<String>>(&mut self, body: S) -> &mut Self {
        let ret = quick_xml::de::from_str(&*body.into().clone());

        self.body.content = body.into();
        self
    }
    pub fn set_title<S: Into<String>>(&mut self, title: S) -> &mut Self {
        self.head.title = XHtmlTitle::new(title);
        self
    }
    pub fn add_link(&mut self, link: XHtmlLinkItem) -> &mut Self {
        self.head.add_link(link);
        self
    }
    pub fn add_style(&mut self, style: StyleContent) -> &mut Self {
        self.head.add_style(style);
        self
    }
    pub fn add_style_content<S: Into<String>>(&mut self, style: S) -> &mut Self {
        self.head
            .style_content
            .push(StyleContent::new(style, String::from("text/css")));
        self
    }

    pub fn encode_xml(&mut self) -> Result<String, super::Error> {
        let ret = quick_xml::se::to_string(self);

        match ret {
            Ok(s) => Ok(format!(
                "<?xml version=\"1.0\" encoding=\"UTF-8\" ?><!DOCTYPE html>{}",
                s
            )),
            Err(e) => Err(super::Error::NonEncodable(e.to_string())),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct XHtmlHead {
    #[serde(rename = "@lang", skip_serializing_if = "String::is_empty")]
    lang: String,
    #[serde(rename = "title")]
    title: XHtmlTitle,
    #[serde(rename = "link", skip_serializing_if = "Vec::is_empty")]
    link: Vec<XHtmlLinkItem>,
    #[serde(rename = "style", skip_serializing_if = "Vec::is_empty")]
    style_content: Vec<StyleContent>,
}

impl Default for XHtmlHead {
    fn default() -> Self {
        XHtmlHead {
            lang: String::new(),
            title: XHtmlTitle {
                text: String::new(),
                dir: Some(String::from("auto")),
            },
            link: Vec::new(),
            style_content: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StyleContent {
    #[serde(rename = "@type", skip_serializing_if = "is_none_or_empty")]
    style_type: Option<String>,
    #[serde(rename = "$value", skip_serializing_if = "is_none_or_empty")]
    value: Option<String>,
}

impl Default for StyleContent {
    fn default() -> Self {
        StyleContent {
            style_type: None,
            value: None,
        }
    }
}

impl StyleContent {
    pub fn new<S1: Into<String>,S2:Into<String>>(value: S1,style_type :S2) -> Self {
        StyleContent {
            style_type: Some(style_type.into()),
            value: Some(value.into()),
        }
    }
}

impl XHtmlHead {
    /// sets the language of the head.
    pub fn set_lang<S: Into<String>>(mut self, lang: S) -> Self {
        self.lang = lang.into();
        self
    }
    /// Sets the title of the head.
    pub fn set_title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = XHtmlTitle::new(title);
        self
    }
    /// Adds a style to the head.
    pub fn add_style(&mut self, style: StyleContent) -> &mut Self {
        self.style_content.push(style);
        self
    }
    /// Adds a style content to the head.
    pub fn add_style_content<S: Into<String>>(mut self, style: S) -> Self {
        self.style_content.push(StyleContent::new(style, String::from("text/css")));
        self
    }
    /// Adds a link to the head.
    pub fn add_link(&mut self, link: XHtmlLinkItem) -> &mut Self {
        self.link.push(link);
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct XHtmlTitle {
    #[serde(rename = "$text")]
    text: String,
    #[serde(rename = "@dir", skip_serializing_if = "Option::is_none")]
    dir: Option<String>,
}

impl Default for XHtmlTitle {
    fn default() -> Self {
        XHtmlTitle::new(String::new())
    }
}

impl XHtmlTitle {
    fn new<S: Into<String>>(text: S) -> Self {
        XHtmlTitle {
            text: text.into(),
            dir: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct XHtmlLink {
    #[serde(rename = "link", skip_serializing_if = "Vec::is_empty")]
    link: Vec<XHtmlLinkItem>,
}

impl Default for XHtmlLink {
    fn default() -> Self {
        XHtmlLink::new()
    }
}

impl XHtmlLink {
    fn new() -> Self {
        XHtmlLink { link: Vec::new() }
    }
    fn add_link(mut self, link: XHtmlLinkItem) -> Self {
        self.link.push(link);
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XHtmlLinkItem {
    #[serde(rename = "@href")]
    href: String,
    #[serde(rename = "@rel", skip_serializing_if = "is_none_or_empty")]
    rel: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "is_none_or_empty")]
    link_type: Option<String>,
}

impl Default for XHtmlLinkItem {
    fn default() -> Self {
        XHtmlLinkItem::new(String::new(), String::new(), String::from("stylesheet"))
    }
}

impl XHtmlLinkItem {
    pub fn new<S1: Into<String>,S2:Into<String>,S3:Into<String>>(href: S1, link_type: S2, rel: S3) -> Self {
        XHtmlLinkItem {
            href: href.into(),
            rel: Some(rel.into()),
            link_type: Some(link_type.into()),
        }
    }
}

#[derive(Debug, Serialize,Deserialize)]
#[serde_as]
struct XHtmlBody{
    #[serde(rename = "$text", serialize_with = "serialize_raw_xml")]
    content: String,
    #[serde(rename = "@dir")]
    #[serde_as(as = "NoneAsEmptyString")]
    dir: Option<String>,
}

impl Default for XHtmlBody {
    fn default() -> Self {
        XHtmlBody::new(String::new())
    }
}
impl XHtmlBody{
    pub fn new<S: Into<String>>(content : S) -> Self {
        XHtmlBody {
            content: content.into(),
            dir: Some(String::from("auto")),
        }
    }
}

fn partial_escape<S>(s: &str, ser: S) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    ser.serialize_str(&quick_xml::escape::partial_escape(s),)
}

fn serialize_raw_xml<S>(value: &String, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // serializer.serialize_str(value)
    serializer.collect_str(value)
}

// 自定义函数：判断 Option<String> 是否应跳过序列化
fn is_none_or_empty(value: &Option<String>) -> bool {
    match value {
        None => true,                           // 如果是 None，跳过序列化
        Some(s) if s.trim().is_empty() => true, // 如果字符串为空白，跳过序列化
        _ => false,                             // 否则不跳过
    }
}
