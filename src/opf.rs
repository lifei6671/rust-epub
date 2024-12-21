use serde::{Deserialize, Serialize};
use crate::epub::EpubVersion;

/// A struct representing an EPUB Package Document.

#[allow(dead_code)]
pub struct Package {
    metadata: Metadata,
    manifest: Vec<ManifestItem>,
    spine: Vec<SpineItemRef>,
    guide: Vec<GuideReference>,
    bindings: Vec<BindingItem>,
}

/// A struct representing an EPUB Package Document.
impl Package {
    pub fn new() -> Self {
        Package {
            metadata: Metadata::default(),
            manifest: Vec::new(),
            spine: Vec::new(),
            guide: Vec::new(),
            bindings: Vec::new(),
        }
    }
    /// Add a metadata item to the package
    pub fn add_other_metadata(&mut self, metadata: MetaItem) -> &mut Self {
        self.metadata.meta.push(metadata);
        self
    }
    /// Set the metadata for the package
    pub fn set_metadata(&mut self, metadata: Metadata) -> &mut Self {
        self.metadata = metadata;
        self
    }
    /// Add a manifest item to the package
    pub fn add_manifest(&mut self, manifest: ManifestItem) -> &mut Self {
        self.manifest.push(manifest);
        self
    }
    /// Add a spine item reference to the package
    pub fn add_spine(&mut self, spine: SpineItemRef) -> &mut Self {
        self.spine.push(spine);
        self
    }
    /// Add a guide reference to the package
    pub fn add_guide(&mut self, guide: GuideReference) -> &mut Self {
        self.guide.push(guide);
        self
    }
    /// Add a binding item to the package
    pub fn add_binding(&mut self, binding: BindingItem) -> &mut Self {
        self.bindings.push(binding);
        self
    }

    pub fn encode_xml(&self,ver : EpubVersion) -> Result<String, super::Error> {
        match ver {
            EpubVersion::V20 => self.encode_v2_xml(),
            EpubVersion::V30 => self.encode_v3_xml(),
        }
    }

    fn encode_v2_xml(&self) -> Result<String, super::Error> {
        let creator = self.metadata.creator.first().unwrap_or(&"".to_string()).clone();
        let mut xml = PackageOpf::new(EpubVersion::V20, self.metadata.title.clone(), creator);
        self.convert_metadata(&mut xml);
        self.convert_manifest(&mut xml);

       quick_xml::se::to_string(&xml).map_err(|e| super::Error::NonEncodable(e.to_string()))
    }
    fn encode_v3_xml(&self) -> Result<String, super::Error> {
        Ok("".to_string())
    }

    fn convert_metadata<'a>(&self, xml: &'a mut PackageOpf) -> &'a mut PackageOpf {
        xml.metadata.title = self.metadata.title.clone();
        xml.metadata.creator = self.metadata.creator.join(",");
        xml.metadata.subject = Some(self.metadata.subject.join(","));
        xml.metadata.description = self.metadata.description.clone();
        xml.metadata.date = self.metadata.date_published.map(|d| d.format("%Y-%m-%dT%H:%M:%S %z").to_string());
        if let Some(ref date) = self.metadata.date_modified {
            xml.metadata.meta.push(MetaItemOpf{
                name:None,
                text: date.format("%Y-%m-%dT%H:%M:%S %z").to_string(),
                property: Some(String::from("dcterms:modified")),
            });
        }
        xml.metadata.category = self.metadata.category.clone();
        xml.metadata.publisher = self.metadata.publisher.clone();
        xml.metadata.contributor = self.metadata.contributor.clone();
        xml.metadata.rights = self.metadata.rights.clone();
        xml.metadata.format = self.metadata.format.clone();
        xml.metadata.source = self.metadata.source.clone();
        xml.metadata.language = Some(self.metadata.language.clone());
        xml.metadata.relation = self.metadata.relation.clone();
        xml.metadata.coverage = self.metadata.coverage.clone();

        self.metadata.meta.iter().for_each(|m| {
            xml.metadata.meta.push(MetaItemOpf{
                name: Some(m.name.clone()),
                text: m.data.clone(),
                property: Some(m.property.clone()),
            });
        });
        xml.metadata.meta.push(MetaItemOpf{
            name: Some(String::from("generator")),
            text: self.metadata.generator.clone(),
            property: None,
        });
        xml.metadata.meta.push(MetaItemOpf{
            name : Some(String::from("generator-name")),
            text: self.metadata.generator_name.clone(),
        property: None,
        });

        xml
    }

    fn convert_manifest<'a>(&self, xml: &'a mut PackageOpf) -> &'a mut PackageOpf {
        self.manifest
            .iter()
            .for_each(|m| xml.manifest.items.push(ManifestItemOpf{
                id: m.id.clone(),
                href: m.href.clone(),
                media_type: m.media_type.clone(),
            }));
        xml
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Metadata {
    pub title: String,
    pub creator: Vec<String>,
    pub subject: Vec<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub publisher: Option<String>,
    pub contributor: Option<String>,
    pub format: Option<String>,
    pub identifier: Option<Identifier>,
    pub source: Option<String>,
    pub language: String,
    pub relation: Option<String>,
    pub coverage: Option<String>,
    pub rights: Option<String>,
    pub cover: Option<String>,
    pub date_published: Option<chrono::DateTime<chrono::Utc>>,
    pub date_modified: Option<chrono::DateTime<chrono::Utc>>,

    generator: String,
    generator_name: String,

    meta: Vec<MetaItem>,
}

#[allow(dead_code)]
impl Default for Metadata {
    fn default() -> Self {
        Metadata {
            title: String::new(),
            creator: vec![],
            subject: vec![],
            description: None,
            category: None,
            publisher: None,
            contributor: None,
            format: None,
            identifier: None,
            source: None,
            language: String::from("zh"),
            relation: None,
            coverage: None,
            rights: None,
            cover: None,
            date_published: None,
            date_modified: None,
            generator: String::from("Rust EPUB library"),
            generator_name: String::from("Table Of Contents"),
            meta: Vec::new(),
        }
    }
}

impl Metadata {
    /// 增加自定义元数据
    pub fn add_meta(&mut self, meta_item: MetaItem) -> &mut Self {
        self.meta.push(meta_item);
        self
    }
    /// set meta title
    pub fn set_title<S: Into<String>>(&mut self, title: S) -> &mut Self {
        self.title = title.into();
        self
    }
    /// set meta creator
    pub fn set_creator<S: Into<String>>(&mut self, creator: S) -> &mut Self {
        self.creator.push(creator.into());
        self
    }
    /// set meta subject
    pub fn set_subject<S: Into<String>>(&mut self, subject: S) -> &mut Self {
        self.subject.push(subject.into());
        self
    }
    /// set meta description
    pub fn set_description<S: Into<String>>(&mut self, description: S) -> &mut Self {
        self.description = Some(description.into());
        self
    }
    /// set meta category
    pub fn set_category<S: Into<String>>(&mut self, category: S) -> &mut Self {
        self.category = Some(category.into());
        self
    }
    /// set meta publisher
    pub fn set_publisher<S: Into<String>>(&mut self, publisher: S) -> &mut Self {
        self.publisher = Some(publisher.into());
        self
    }
    /// set meta contributor
    pub fn set_contributor<S: Into<String>>(&mut self, contributor: S) -> &mut Self {
        self.contributor = Some(contributor.into());
        self
    }
    /// set meta format
    pub fn set_format<S: Into<String>>(&mut self, format: S) -> &mut Self {
        self.format = Some(format.into());
        self
    }
    /// set meta identifier
    pub fn set_identifier(&mut self, identifier: Identifier) -> &mut Self {
        self.identifier = Some(identifier);
        self
    }
    /// set meta source
    pub fn set_source<S: Into<String>>(&mut self, source: S) -> &mut Self {
        self.source = Some(source.into());
        self
    }
    /// set meta language
    pub fn set_language<S: Into<String>>(&mut self, language: S) -> &mut Self {
        self.language = language.into();
        self
    }
    /// set meta relation
    pub fn set_relation<S: Into<String>>(&mut self, relation: S) -> &mut Self {
        self.relation = Some(relation.into());
        self
    }
    /// set meta coverage
    pub fn set_cover<S: Into<String>>(&mut self, cover: S) -> &mut Self {
        self.coverage = Some(cover.into());
        self
    }
    /// set meta rights
    pub fn set_rights<S: Into<String>>(&mut self, rights: S) -> &mut Self {
        self.rights = Some(rights.into());
        self
    }
    /// set published date
    pub fn set_date_published<D: Into<chrono::DateTime<chrono::Utc>>>(&mut self, date_published: D) -> &mut Self {
        self.date_published = Some(date_published.into());
        self
    }
    /// set modified date
    pub fn set_date_modified<D: Into<chrono::DateTime<chrono::Utc>>>(&mut self, date_modified: D) -> &mut Self {
        self.date_modified = Some(date_modified.into());
        self
    }
}

/// 其他自定义元数据
#[derive(Debug)]
pub struct MetaItem {
    pub refines: String,
    pub property: String,
    pub scheme: String,
    pub id: String,
    pub data: String,
    pub name: String,
    pub content: String,
}

impl Default for MetaItem {
    fn default() -> Self {
        MetaItem {
            refines: String::new(),
            property: String::new(),
            scheme: String::new(),
            id: String::new(),
            data: String::new(),
            name: String::new(),
            content: String::new(),
        }
    }
}

/// epub meta item
impl MetaItem {
    /// Create a new meta item
    pub fn new<S: Into<String>>(content:S) -> MetaItem {
        MetaItem {
            refines: String::new(),
            property: String::new(),
            scheme: String::new(),
            id: String::new(),
            data: String::new(),
            name: String::new(),
            content: content.into(),
        }
    }
}

/// epub manifest
#[derive(Debug)]
#[allow(dead_code)]
pub struct Manifest {
    items : Vec<ManifestItem>,
}
/// 资源列表
#[derive(Debug)]
#[allow(dead_code)]
pub struct ManifestItem {
    pub id: String,
    pub href: String,
    pub media_type: String,
    pub properties: String,
}

/// epub manifest item
impl Default for ManifestItem {
    fn default() -> Self {
        ManifestItem {
            id: String::new(),
            href: String::new(),
            media_type: String::new(),
            properties: String::new(),
        }
    }
}

/// epub manifest item
impl ManifestItem {
    /// Create a new manifest item
    pub fn new<S: Into<String>>(id: S, href: S, media_type :S) -> ManifestItem {
        ManifestItem {
            id: id.into(),
            href: href.into(),
            media_type: media_type.into(),
            properties: String::new(),
        }
    }
}

/// epub identifier
#[derive(Debug)]
#[allow(dead_code)]
pub struct Identifier {
    pub id: String,
    pub scheme: String,
    pub text: String,
}

/// epub spine item ref
#[derive(Debug)]
#[allow(dead_code)]
pub struct SpineItemRef {
    pub idref: String,
}

/// epub spine item ref
impl Default for SpineItemRef {
    /// Create a new spine item ref
    fn default() -> Self {
        SpineItemRef { idref: String::new() }
    }
}
/// epub spine item ref
impl SpineItemRef {
    /// Create a new spine item ref
    pub fn new<S: Into<String>>(idref: S) -> SpineItemRef {
        SpineItemRef { idref: idref.into() }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct GuideReference {
    ref_type: String,
    title: String,
    href: String,
}

/// epub guide reference
impl Default for GuideReference {
    /// Create a new guide reference
    fn default() -> Self {
        GuideReference {
            ref_type: String::new(),
            title: String::new(),
            href: String::new(),
        }
    }
}

/// epub guide reference
impl GuideReference {
    /// Create a new guide reference
    pub fn new<S1: Into<String>, S2: Into<String>, S3: Into<String>>(
        ref_type: S1,
        title: S2,
        href: S3,
    ) -> GuideReference {
        GuideReference {
            ref_type: ref_type.into(),
            title: title.into(),
            href: href.into(),
        }
    }
}

#[derive(Debug,)]
#[allow(dead_code)]
pub struct BindingItem {
    media_type: String,
    href: String,
}

/// epub binding item
impl BindingItem {
    /// Create a new binding item
    pub fn new<S1: Into<String>, S2: Into<String>>(media_type: S1, href: S2) -> BindingItem {
        BindingItem {
            media_type: media_type.into(),
            href: href.into(),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "package")]
struct PackageOpf {
    #[serde(rename = "metadata")]
    metadata: MetadataOpf,
    #[serde(rename = "manifest")]
    manifest: ManifestOpf,
    #[serde(rename = "spine")]
    spine: Vec<SpineOpf>,
    #[serde(rename = "guide")]
    guide: Vec<GuideReferenceOpf>,

    #[serde(rename = "@version")]
    version : String,
    #[serde(rename = "@xmlns:opf",skip_serializing_if = "String::is_empty")]
    xmlns_opf: String,
    #[serde(rename = "@xmlns:dc",skip_serializing_if = "String::is_empty")]
    xmlns_dc: String,
    #[serde(rename = "@xmlns:xsi" ,skip_serializing_if = "String::is_empty")]
    xmlns_xsi: String,
}

impl PackageOpf {
    fn new<S:Into<String>>(ver : EpubVersion, title:S, creator: S) -> PackageOpf {
        //            xmlns_opf: "http://www.idpf.org/2007/opf".to_string(),
        //             xmlns_dc: "http://purl.org/dc/elements/1.1/".to_string(),
        //             xmlns_xsi: "http://www.w3.org/2001/XMLSchema-instance".to_string(),
        let xmlns_opf = match ver { EpubVersion::V20=>"http://www.idpf.org/2007/opf".to_string(),EpubVersion::V30=>"http://www.idpf.org/2007/opf".to_string() ,};
        let xmlns_dc = match ver { EpubVersion::V20=>"".to_string(),EpubVersion::V30=>"http://purl.org/dc/elements/1.1/".to_string() ,};
        let xmlns_xsi = match ver { EpubVersion::V20=>"".to_string(),EpubVersion::V30=>"http://www.w3.org/2001/XMLSchema-instance".to_string() ,};
        PackageOpf {
            metadata: MetadataOpf::new(title,creator),
            manifest: ManifestOpf::default(),
            spine: Vec::new(),
            guide: Vec::new(),
            version:  match ver { EpubVersion::V20=> String::from("v2.0"),EpubVersion::V30=>String::from("v3.0") ,},
            xmlns_opf,
            xmlns_dc,
            xmlns_xsi,
        }
    }
}


#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "metadata")]
struct MetadataOpf {
    #[serde(rename = "dc:creator",skip_serializing_if = "String::is_empty")]
    creator: String,
    #[serde(rename = "dc:title")]
    title: String,
    #[serde(rename = "dc:subject",skip_serializing_if = "Option::is_none")]
    subject: Option<String>,
    #[serde(rename = "dc:description",skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(rename = "dc:date",skip_serializing_if = "Option::is_none")]
    date: Option<String>,
    #[serde(rename = "dc:type", skip_serializing_if = "Option::is_none")]
    category: Option<String>,
    #[serde(rename = "dc:publisher", skip_serializing_if = "Option::is_none")]
    publisher: Option<String>,
    #[serde(rename = "dc:contributor", skip_serializing_if = "Option::is_none")]
    contributor: Option<String>,
    #[serde(rename = "dc:format", skip_serializing_if = "Option::is_none")]
    format: Option<String>,
    #[serde(rename = "dc:identifier", skip_serializing_if = "skip_if_empty_identifier")]
    identifier: Option<IdentifierOpf>,
    #[serde(rename = "dc:source", skip_serializing_if = "Option::is_none")]
    source: Option<String>,
    #[serde(rename = "dc:language", skip_serializing_if = "Option::is_none")]
    language: Option<String>,
    #[serde(rename = "dc:relation", skip_serializing_if = "Option::is_none")]
    relation: Option<String>,
    #[serde(rename = "dc:coverage", skip_serializing_if = "Option::is_none")]
    coverage: Option<String>,
    #[serde(rename = "dc:rights", skip_serializing_if = "Option::is_none")]
    rights: Option<String>,
    #[serde(rename = "dc:cover", skip_serializing_if = "Option::is_none")]
    cover: Option<String>,


    #[serde(rename = "meta", skip_serializing_if = "Vec::is_empty")]
    meta: Vec<MetaItemOpf>,
}



impl MetadataOpf {
    fn new<S:Into<String>>(title: S, creator: S) -> MetadataOpf {
        MetadataOpf {
            creator: creator.into(),
            title: title.into(),
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
            meta: Vec::new(),
        }
    }
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "meta")]
struct MetaItemOpf {
    #[serde(rename = "$text", skip_serializing_if = "String::is_empty")]
    text: String,
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(rename = "@property",skip_serializing_if = "Option::is_none")]
    property: Option<String>,
}

/// 自定义结构体的序列化条件
fn skip_if_empty_identifier(identifier: &Option<IdentifierOpf>) -> bool {
    // *identifier.unwrap()?.is_some_and(|id| id.text.is_empty())
    match identifier {
        Some(id) => id.text.is_empty(),
        None => true,
    }
}

/// 出版社信息
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "identifier")]
struct IdentifierOpf {
    #[serde(rename = "@id", skip_serializing_if = "String::is_empty")]
    id: String,
    #[serde(rename = "opf:scheme", skip_serializing_if = "String::is_empty")]
    scheme: String,
    #[serde(rename = "$text")]
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ManifestOpf {
    #[serde(rename = "item", skip_serializing_if = "Vec::is_empty")]
    items: Vec<ManifestItemOpf>,
}

impl Default for  ManifestOpf {
    fn default() -> Self {
        ManifestOpf { items: Vec::new() }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "item")]
struct ManifestItemOpf {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@href")]
    href: String,
    #[serde(rename = "@media-type")]
    media_type: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "spine")]
struct SpineOpf {
    #[serde(rename = "@toc")]
    toc: String,
    #[serde(rename = "itemref")]
    items: Vec<SpineItemOpf>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "spine")]
struct SpineItemOpf {
    #[serde(rename = "@idref")]
    idref: String,
    #[serde(rename = "@properties")]
    properties: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "guide ")]
struct GuideReferenceOpf {
    #[serde(rename = "reference")]
    items: Vec<GuideReferenceItemOpf>,
}
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "reference ")]
struct GuideReferenceItemOpf {
    #[serde(rename = "@type")]
    ref_type: String,
    #[serde(rename = "@title")]
    title: String,
    #[serde(rename = "@href")]
    href: String,
}

