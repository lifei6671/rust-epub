use std::time::SystemTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "package")]
pub struct Package {

}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "metadata")]
pub struct Metadata {
    #[serde(rename = "@xmlns:opf")]
    xmlns_opf: String,
    #[serde(rename = "@xmlns:dc")]
    xmlns_dc :String,
    #[serde(rename = "@xmlns:xsi")]
    xmlns_xsi :String,

    #[serde(rename = "dc:creator")]
    creator : String,
    #[serde(rename = "dc:title")]
    title: String,
    #[serde(rename = "dc:subject")]
    subject : Option<String>,
    #[serde(rename = "dc:description")]
    description: Option<String>,
    #[serde(rename = "dc:date")]
    date : Option<String>,
    #[serde(rename = "dc:type")]
    category : Option<String>,
    #[serde(rename = "dc:publisher")]
    publisher:Option<String>,
    #[serde(rename = "dc:contributor")]
    contributor:Option<String>,
    #[serde(rename = "dc:format")]
    format : Option<String>,
    #[serde(rename = "dc:identifier", skip_serializing_if="skip_if_empty_identifier")]
    identifier : Option<Identifier>,
    #[serde(rename = "dc:source")]
    source : Option<String>,
    #[serde(rename = "dc:language")]
    language : Option<String>,
    #[serde(rename = "dc:relation")]
    relation : Option<String>,
    #[serde(rename = "dc:coverage")]
    coverage : Option<String>,
    #[serde(rename = "dc:rights")]
    rights : Option<String>,
    #[serde(rename = "dc:cover")]
    cover : Option<String>,


}

impl Metadata {
    pub fn new(title :String,creator :String) -> Metadata{
        Metadata{
            xmlns_opf: "http://www.idpf.org/2007/opf".to_string(),
            xmlns_dc: "http://purl.org/dc/elements/1.1/".to_string(),
            xmlns_xsi: "http://www.w3.org/2001/XMLSchema-instance".to_string(),
            creator: creator.to_string(),
            title: title.to_string(),
            subject: None,
            description: None,
            date: None,
            category: None,
            publisher: None,
            contributor: None,
            format: None,
            identifier: None,
            source: None,
            language: None,
            relation: None,
            coverage: None,
            rights: None,
            cover: None,
        }
    }
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct MetaItem {
    #[serde(rename = "@content")]
    content: String,
    #[serde(rename = "@name")]
    name: String,
}

// 自定义结构体的序列化条件
fn skip_if_empty_identifier(identifier: &Option<Identifier>) -> bool {
    if let Some(id) = identifier {
        id.text.is_empty()
    } else {
        true
    }
}

/// 出版社信息
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "identifier")]
pub struct Identifier {
    #[serde(rename = "@id", skip_serializing_if = "String::is_empty")]
    id : String,
    #[serde(rename = "opf:scheme",skip_serializing_if = "String::is_empty")]
    scheme :String,
    #[serde(rename = "$text")]
    text : String,
}


#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "manifest")]
pub struct  Manifest {
    #[serde(rename = "item")]
    items: Vec<Metadata>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "metadata")]
pub struct Item {
    #[serde(rename = "@id")]
    id:String,
    #[serde(rename = "@href")]
    href:String,
    #[serde(rename = "@media-type")]
    media_type:String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "spine")]
pub struct Spine {
    #[serde(rename = "@toc")]
    toc :String,
    #[serde(rename = "itemref")]
    items :Vec<Item>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "spine")]
pub struct SpineItem {
    #[serde(rename = "@idref")]
    idref:String,
    #[serde(rename = "@properties")]
    properties:String,
}