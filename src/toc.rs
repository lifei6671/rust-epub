use serde::{Deserialize, Serialize};
use crate::Error;
use serde::ser::{SerializeSeq, Serializer};


#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "ncx")]
pub struct TocElement {
    #[serde(rename = "head")]
    head : Metadata,
    #[serde(rename = "docTitle")]
    doc_title : Text,
    /// This holds the body XML for the EPUB v2 TOC file (nav.xhtml).
    /// http://www.idpf.org/epub/20/spec/OPF_2.0.1_draft.htm#Section2.4.1
    #[serde(rename = "navMap")]
    nav_map: NavMap,
    #[serde(rename = "@xmlns")]
    xmlns : String,
    #[serde(rename = "@version")]
    version : String,
    #[serde(rename = "@xml:lang")]
    lang : String,
}

trait TocEncode {
    fn encode_xml(&mut self) -> Result<String,Error>;
}

/// epub文件的toc.ncx文件处理方法
impl TocElement {
    pub fn new<S:Into<String>>(title :S) -> TocElement {
        TocElement{
            head: Metadata{ meta: Vec::new()},
            doc_title: Text{ text: title.into() },
            nav_map: NavMap::default(),
            xmlns: "http://www.daisy.org/z3986/2005/ncx/".to_string(),
            version: "2005-1".to_string(),
            lang: "zh".to_string(),
        }
    }

    /// 设置语言
    pub fn lang<S:Into<String>>(&mut self, lang :S) -> &mut Self {
        self.lang = lang.into();
        self
    }

    /// 追加元数据
    pub fn add_metadata<T :Into<String>>(&mut self, content :T,name : T) -> &mut Self {
        self.head.meta.push( MetaItem{content : content.into(), name : name.into()});
        self
    }

    /// 追加子节点到当前节点
    pub fn add_nav_point(&mut self,mut sub :NavPoint) -> &mut Self {
        self.nav_map.nav_point.push(sub.into());
        self
    }

    /// 将toc文件格式化为xml文件。
    pub fn encode_xml(&mut self) -> Result<String,Error> {
        let mut play_order = 0u64;
        for nav in self.nav_map.nav_point.iter_mut() {
            nav.play_order = Some(play_order);
            play_order += 1;

            nav.traverse(|node| {
                node.play_order = Some(play_order);
                play_order += 1;
            });
        }
        let ret = quick_xml::se::to_string(self);
        match ret {
            Ok(xml) => {
                Ok(format!("<?xml version='1.0' encoding='utf-8'?>{}", xml))
            }
            Err(e) => {
                Err(Error::NonEncodable(e.to_string()))
            }
        }
    }
}

#[derive(Debug, PartialEq, Default,Serialize, Deserialize,)]
#[serde(default)]
struct Metadata {
    #[serde(rename = "meta")]
    meta :Vec<MetaItem>,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct MetaItem {
    #[serde(rename = "@content")]
    content: String,
    #[serde(rename = "@name")]
    name: String,
}

#[derive(Debug, Clone,Default, Serialize, Deserialize)]
#[serde(rename = "navMap")]
pub struct NavMap {
    nav_point: Vec<NavPoint>,
}

#[derive(Debug, Clone,Default, Serialize, Deserialize)]
#[serde(rename = "navPoint")]
pub struct NavPoint {
    #[serde(rename = "@id")]
    id : String,
    nav_label : Option<Text>,
    content: Option<Content>,
    #[serde(rename = "navPoint")]
    nav_element : Vec<NavPoint>,
    #[serde(rename = "@playOrder")]
    play_order : Option<u64>,
}

impl NavPoint {
    pub fn new<S : Into<String>>(id: S, nav_label: String, content: String) -> NavPoint {
        NavPoint{
            id: id.into(),
            nav_label: Option::from(Text { text: nav_label }),
            content:  Option::from(Content{ src: content}),
            nav_element: Vec::new(),
            play_order: None,
        }
    }
    /// 追加一个子节点
    pub fn add_nav_point(&mut self,nav_point: NavPoint) -> &mut Self {
        self.nav_element.push(nav_point);
        self
    }
    /// 设置content节点的src属性
    pub fn content(&mut self, src : String) -> &mut Self {
        self.content =  Option::from(Content{ src });
        self
    }

    pub fn nav_label(&mut self, label : String) -> &mut Self {
        self.nav_label = Option::from(Text { text : label });
        self
    }
    /// 设置节点的id属性
    pub fn id(&mut self, id : String) -> &mut Self {
        self.id =  id;
        self
    }

    /// 递归所有子节点
    fn traverse<F> (&mut self,mut callback:F)
    where F : FnMut(&mut NavPoint) {
        let mut stack = vec![self];
        while let Some(mut node) = stack.pop() {
            callback(node);
            for child in node.nav_element.iter_mut() {
                stack.push(child);
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct Text {
    #[serde(rename = "text",skip_serializing_if = "String::is_empty")]
    text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Content {
    #[serde(rename = "@src",skip_serializing_if = "String::is_empty")]
    src : String,
}

