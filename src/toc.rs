use crate::epub::Version;
use serde::ser::{SerializeSeq, Serializer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct TocNav {
    title: String,
    lang: String,
    metadata: Vec<(String, String)>,
    elements: Vec<TocElement>,
}

impl TocNav {
    pub fn new<S1: Into<String>, S2: Into<String>>(title: S1, lang: S2) -> Self {
        TocNav {
            title: title.into(),
            lang: lang.into(),
            metadata: Vec::new(),
            elements: Vec::new(),
        }
    }
    /// 增加元数据
    pub fn add_metadata<S1: Into<String>, S2: Into<String>>(
        &mut self,
        name: S1,
        value: S2,
    ) -> &mut TocNav {
        self.metadata.push((name.into(), value.into()));
        self
    }
    pub fn add_element(&mut self, elem: TocElement) -> &mut TocNav {
        self.elements.push(elem);
        self
    }
    pub fn encode_file(&mut self, ver: Version) -> Result<String, super::Error> {
        match ver {
            Version::V2 => self.encode_ncx_file(),
            Version::V3 => self.encode_nav_file(),
        }
    }

    fn encode_ncx_file(&mut self) -> Result<String, super::Error> {
        let mut ncx = TocNCX::new(self.title.clone(), self.lang.clone());
        for (_, (name, content)) in self.metadata.iter().enumerate() {
            ncx.head.meta.push(MetaItem {
                name: String::from(name),
                content: String::from(content),
            });
        }
        for el in self.elements.iter_mut() {
            ncx.nav_map.nav_point.push(NavPoint::from_toc_element(el));
        }
        let mut ret = quick_xml::se::to_string(&ncx);
        match ret {
            Ok(s) => {
                let xml_str = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>
                    <!DOCTYPE PUBLIC \"-//NISO//DTD ncx 2005-1//EN\"\n\"http://www.daisy.org/z3986/2005/ncx-2005-1.dtd\">{}",
                    s
                );
                Ok(xml_str)
            }
            Err(e) => Err(super::Error::NonEncodable(e.to_string())),
        }
    }

    fn encode_nav_file(&mut self) -> Result<String, super::Error> {
        let mut html = Html::new(self.title.clone());
        for (_, (name, content)) in self.metadata.iter().enumerate() {
            html.head.meta.push(MetaItem {
                name: String::from(name),
                content: String::from(content),
            });
        }

        for el in self.elements.iter() {
            html.body.nav_toc.add_list(List::from_toc_element(el));
        }
        let mut ret = quick_xml::se::to_string(&html);
        match ret {
            Ok(s) => {
                let xml_str = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>
                    <!DOCTYPE html>{}",
                                      s
                );
                Ok(xml_str)
            }
            Err(e) => Err(super::Error::NonEncodable(e.to_string())),
        }
    }
}

#[derive(Debug, Default)]
pub struct TocElement {
    /// 排序
    pub level: i32,
    /// 链接地址
    pub url: String,
    /// 标题
    pub title: String,
    /// 子节点
    pub childs: Vec<TocElement>,
}

/// epub文件的toc.ncx文件处理方法
impl TocElement {
    pub fn new<S1: Into<String>, S2: Into<String>>(url: S1, title: S2) -> Self {
        TocElement {
            level: 1,
            url: url.into(),
            title: title.into(),
            childs: vec![],
        }
    }
    /// 调整每个子节点的level
    fn level_up(&mut self, level: i32) {
        let mut stack = vec![(self, level)];
        while let Some((el, current_level)) = stack.pop() {
            el.level = current_level;
            let children_to_process: Vec<_> = el
                .childs
                .iter_mut()
                .filter(|child| child.level <= el.level)
                .collect();

            for child in children_to_process {
                stack.push((child, current_level + 1));
            }
        }
    }
    pub fn traverse<F>(&mut self, mut callback: F) -> bool
    where
        F: FnMut(&mut TocElement) -> bool,
    {
        let mut stack = vec![self];
        while let Some(top) = stack.pop() {
            if !callback(top) {
                return false;
            }
            stack.extend(top.childs.iter_mut());
        }
        true
    }
    /// 增加一个子节点
    pub fn add_child(&mut self, mut child: TocElement) -> &mut TocElement {
        if child.level <= self.level {
            child.level_up(self.level + 1); // 调整子节点及其所有子节点的层级
        }
        self.childs.push(child);
        self
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "ncx")]
pub struct TocNCX {
    #[serde(rename = "head")]
    head: Metadata,
    #[serde(rename = "docTitle")]
    doc_title: Text,
    /// This holds the body XML for the EPUB v2 TOC file (nav.xhtml).
    /// http://www.idpf.org/epub/20/spec/OPF_2.0.1_draft.htm#Section2.4.1
    #[serde(rename = "navMap")]
    nav_map: NavMap,
    #[serde(rename = "@xmlns")]
    xmlns: String,
    #[serde(rename = "@version")]
    version: String,
    #[serde(rename = "@xml:lang")]
    lang: String,
}

impl TocNCX {
    pub fn new(title: String, lang: String) -> TocNCX {
        TocNCX {
            head: Metadata { meta: Vec::new() },
            doc_title: Text { text: title },
            nav_map: NavMap {
                nav_point: Vec::new(),
            },
            xmlns: String::from("http://www.daisy.org/z3986/2005/ncx/"),
            version: String::from("2005-1"),
            lang: String::from(lang),
        }
    }
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
struct Metadata {
    #[serde(rename = "meta")]
    meta: Vec<MetaItem>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct MetaItem {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@content")]
    content: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename = "navMap")]
pub struct NavMap {
    nav_point: Vec<NavPoint>,
}

impl NavMap {
    /// 将 TocElement 集合转换为epub v2版本的navMap结构
    fn from_toc_elements(elements: &[TocElement]) -> Self {
        NavMap {
            nav_point: elements.iter().map(NavPoint::from_toc_element).collect(),
        }
    }
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename = "navPoint")]
pub struct NavPoint {
    #[serde(rename = "@id")]
    id: String,
    nav_label: Option<Text>,
    content: Option<Content>,
    #[serde(rename = "navPoint")]
    nav_element: Vec<NavPoint>,
    #[serde(rename = "@playOrder")]
    play_order: Option<i32>,
}

impl NavPoint {
    pub fn new<S: Into<String>>(id: S, nav_label: String, content: String) -> NavPoint {
        NavPoint {
            id: id.into(),
            nav_label: Option::from(Text { text: nav_label }),
            content: Option::from(Content { src: content }),
            nav_element: Vec::new(),
            play_order: None,
        }
    }
    /// 追加一个子节点
    pub fn add_nav_point(&mut self, nav_point: NavPoint) -> &mut Self {
        self.nav_element.push(nav_point);
        self
    }
    /// 设置content节点的src属性
    pub fn content(&mut self, src: String) -> &mut Self {
        self.content = Option::from(Content { src });
        self
    }

    pub fn nav_label(&mut self, label: String) -> &mut Self {
        self.nav_label = Option::from(Text { text: label });
        self
    }
    /// 设置节点的id属性
    pub fn id(&mut self, id: String) -> &mut Self {
        self.id = id;
        self
    }

    /// 递归所有子节点
    fn traverse<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut NavPoint),
    {
        let mut stack = vec![self];
        while let Some(mut node) = stack.pop() {
            callback(node);
            for child in node.nav_element.iter_mut() {
                stack.push(child);
            }
        }
    }

    /// 将TocElement转换为epub v2版本的navPoint结构
    fn from_toc_element(element: &TocElement) -> Self {
        NavPoint {
            id: format!("navPoint-{}", element.level),
            nav_label: Option::from(Text {
                text: element.title.clone(),
            }),
            content: Option::from(Content {
                src: element.url.clone(),
            }),
            nav_element: element
                .childs
                .iter()
                .map(NavPoint::from_toc_element)
                .collect(),
            play_order: Option::from(element.level),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct Text {
    #[serde(rename = "text", skip_serializing_if = "String::is_empty")]
    text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Content {
    #[serde(rename = "@src", skip_serializing_if = "String::is_empty")]
    src: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "html")]
struct Html {
    #[serde(rename = "@xmlns")]
    xmlns: String,
    #[serde(rename = "@xmlns:epub")]
    xmlns_epub: String,
    #[serde(rename = "head")]
    head: Head,
    #[serde(rename = "body")]
    body : Body,
}

impl Html {
    pub fn new<S1 :Into<String>>(title : S1) -> Self {
        let title_str = title.into();
        Html{
            xmlns: "http://www.w3.org/1999/xhtml".to_string(),
            xmlns_epub: "http://www.idpf.org/2007/ops".to_string(),
            head: Head::new(title_str.clone()),
            body: Body::new(title_str),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "head")]
struct Head {
    #[serde(rename = "meta")]
    meta: Vec<MetaItem>,
    #[serde(rename = "title")]
    title :String,
}

impl Head {
    fn new<S1 :Into<String>>(title : S1) -> Self {
        Head{
            meta: Vec::new(),
            title: title.into(),
        }
    }
}
#[derive(Debug,Default, Clone, Serialize, Deserialize)]
#[serde(rename = "body")]
struct Body {
    #[serde(rename = "nav")]
    nav_toc: NavToc
}

impl Body {
    fn new<S1 :Into<String>>(title : S1) -> Self {
        Body{
            nav_toc: NavToc::new(title.into()),
        }
    }
}
#[derive(Debug,Default, Clone, Serialize, Deserialize)]
#[serde(rename = "nav")]
struct NavToc {
    #[serde(rename = "@epub:type")]
    epub_type: String,
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "h1")]
    h1: H1,
    #[serde(rename = "ol")]
    order_list: OrderList,
}

impl NavToc {
    fn new<S1 :Into<String>>(title : S1) -> Self {
        NavToc{
            epub_type: String::from("toc"),
            id:String::from("toc"),
            h1: H1{id:None, text: title.into() },
            order_list: Default::default(),
        }
    }

    fn add_list(&mut self, list: List) -> &mut Self {
        self.order_list.list.push(list);
        self
    }
}
#[derive(Debug,Default, Clone, Serialize, Deserialize)]
struct H1 {
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(rename = "$text")]
    text: String,
}

#[derive(Debug,Default, Clone, Serialize, Deserialize)]
#[serde(rename = "ol")]
struct OrderList {
    #[serde(rename = "li")]
    list: Vec<List>,
}

impl OrderList {
    fn from_toc_elements(elements: &[TocElement]) -> Self {
        OrderList{
            list: elements.iter().map(|e|List::from_toc_element(e)).collect(),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "li")]
struct List {
    #[serde(rename = "a")]
    anchor: Anchor,
    #[serde(rename = "ol",skip_serializing_if="Option::is_none")]
    order_list: Option<OrderList>,
}

impl List {
    fn new<S1 :Into<String>,S2:Into<String>>(title : S1,href: S2) -> Self {
        List{
            anchor: Anchor{
                href: href.into(),
                text: title.into(),
            },
            order_list: None,
        }
    }

    fn from_toc_element(element: &TocElement) ->Self{
        List {
            anchor:Anchor{
                href:element.url.clone(),
                text:element.title.clone(),
            },
            order_list : if element.childs.is_empty() {
                None
            }   else {
                Some(OrderList::from_toc_elements(&element.childs))
            }
        }
    }

}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "a")]
struct Anchor {
    #[serde(rename = "@href")]
    href: String,
    #[serde(rename = "$text")]
    text: String,
}
