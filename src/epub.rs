use crate::mime::first;
use crate::toc::{TocElement, TocNav};
use crate::xhtml::{XHtmlLinkItem, XHtmlRoot};
use crate::{write, Error};
use dashmap::{DashMap, DashSet};
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

#[allow(dead_code)]
pub(crate) const CSS_FOLDER_NAME: &str = "css";
#[allow(dead_code)]
pub(crate) const FONT_FOLDER_NAME: &str = "fonts";
#[allow(dead_code)]
pub(crate) const IMAGE_FOLDER_NAME: &str = "images";
#[allow(dead_code)]
pub(crate) const VIDEO_FOLDER_NAME: &str = "videos";
#[allow(dead_code)]
pub(crate) const AUDIO_FOLDER_NAME: &str = "audios";

#[allow(dead_code)]
pub(crate) const XHTML_FOLDER_NAME: &str = "xhtml";

#[allow(dead_code)]
pub(crate) const CONTENT_FOLDER_NAME: &str = "EPUB";
#[allow(dead_code)]
pub(crate) const MEDIA_TYPE_CSS: &str = "text/css";
#[allow(dead_code)]
pub(crate) const MEDIA_TYPE_EPUB: &str = "application/epub+zip";
#[allow(dead_code)]
pub(crate) const MEDIA_TYPE_JPEG: &str = "image/jpeg";
#[allow(dead_code)]
pub(crate) const MEDIA_TYPE_NCX: &str = "application/x-dtbncx+xml";
#[allow(dead_code)]
pub(crate) const MEDIA_TYPE_XHTML: &str = "application/xhtml+xml";
#[allow(dead_code)]
pub(crate) const META_INF_FOLDER_NAME: &str = "META-INF";

#[allow(dead_code)]
pub(crate) const CONTAINER_FILENAME: &str = "container.xml";
#[allow(dead_code)]
pub(crate) const PKG_FILENAME: &str = "content.opf";

#[allow(dead_code)]
pub(crate) const COVER_CSS_CONTENT: &str = "\
body {
  background-color: #FFFFFF;
  margin-bottom: 0px;
  margin-left: 0px;
  margin-right: 0px;
  margin-top: 0px;
  text-align: center;
}
img {
  max-height: 100%;
  max-width: 100%;
}";
#[allow(dead_code)]
pub(crate) const COVER_FILE_NAME: &str = "cover.xhtml";
#[allow(dead_code)]
pub(crate) const COVER_CSS_FILE: &str = "cover.css";

/// epub规范版本
#[derive(Debug, Copy)]
#[allow(dead_code)]
pub enum EpubVersion {
    V20,
    V30,
}

impl Clone for EpubVersion {
    fn clone(&self) -> Self {
        match self {
            EpubVersion::V20 => EpubVersion::V20,
            EpubVersion::V30 => EpubVersion::V30,
        }
    }
}
/// An epub file structure instance
#[derive(Debug)]
#[allow(dead_code)]
pub struct EpubBuilder {
    /// Book title
    title: String,
    /// Book creator
    creator: Vec<String>,
    /// Book subject
    subject: Option<String>,
    /// Book description
    description: Option<String>,
    /// Book date
    date: Option<SystemTime>,
    /// Book category
    category: Option<String>,
    /// Book Publishers
    publisher: Option<String>,
    /// Book contributor
    contributor: Option<String>,
    /// Book format
    format: Option<String>,
    /// Book identifier
    identifier: Option<String>,
    /// Book source
    source: Option<String>,
    /// Book language
    language: Option<String>,
    /// Book relation
    relation: Option<String>,
    /// Book coverage
    coverage: Option<String>,
    /// Book rights
    rights: Option<String>,

    /// Book cover
    cover: Option<Arc<Mutex<Cover>>>,

    /// Book other metadata
    metadata: Option<DashMap<String, String>>,

    /// Custom style sheet collection
    stylesheet: DashMap<String, String>,
    /// Custom font collection
    fonts: DashMap<String, String>,
    /// Books Picture Collection
    images: DashMap<String, String>,
    /// Books Video Collection
    videos: DashMap<String, String>,
    /// Books Audio Collection
    audios: DashMap<String, String>,

    /// Book section collection
    sections: Vec<Section>,

    /// Internal file name collection
    filenames: DashSet<String>,

    /// Epub version
    version: EpubVersion,
}

impl Default for EpubBuilder {
    fn default() -> Self {
        EpubBuilder::new(String::from(""), EpubVersion::V20)
    }
}

impl EpubBuilder {
    #[allow(dead_code)]
    pub fn new<S: Into<String>>(title: S, ver: EpubVersion) -> EpubBuilder {
        EpubBuilder {
            title: title.into(),
            creator: Vec::new(),
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
            metadata: None,
            cover: None,
            stylesheet: DashMap::new(),
            fonts: DashMap::new(),
            images: DashMap::new(),
            videos: DashMap::new(),
            audios: DashMap::new(),
            sections: Vec::new(),
            filenames: DashSet::new(),
            version: ver,
        }
    }

    /// Set the epub identifier
    pub fn set_id<S: Into<String>>(&mut self, id: S) -> &mut Self {
        self.identifier = Some(id.into());
        self
    }
    /// Set the epub format
    pub fn set_format<S: Into<String>>(&mut self, format: S) -> &mut Self {
        self.format = Some(format.into());
        self
    }
    /// Set the epub publisher
    pub fn set_publisher<S: Into<String>>(&mut self, publisher: S) -> &mut Self {
        self.publisher = Some(publisher.into());
        self
    }
    /// Set the epub category
    pub fn set_category<S: Into<String>>(&mut self, category: S) -> &mut Self {
        self.category = Some(category.into());
        self
    }
    /// Set the epub date
    pub fn set_date(&mut self, date: SystemTime) -> &mut Self {
        self.date = Some(date);
        self
    }
    /// Set the epub description
    pub fn set_description<S: Into<String>>(&mut self, description: S) -> &mut Self {
        self.description = Some(description.into());
        self
    }
    /// Set the epub subject
    pub fn set_subject<S: Into<String>>(&mut self, subtitle: S) -> &mut Self {
        self.title = subtitle.into();
        self
    }
    /// Set the epub title
    pub fn set_title<S: Into<String>>(&mut self, title: S) -> &mut Self {
        self.title = title.into();
        self
    }
    /// Set the epub source
    pub fn set_source<S: Into<String>>(&mut self, source: S) -> &mut Self {
        self.source = Some(source.into());
        self
    }

    /// Set the epub language
    pub fn set_language<S: Into<String>>(&mut self, language: S) -> &mut Self {
        self.language = Some(language.into());
        self
    }

    /// Set the epub relation
    pub fn set_relation<S: Into<String>>(&mut self, relation: S) -> &mut Self {
        self.relation = Some(relation.into());
        self
    }

    /// Set the epub right
    pub fn set_right<S: Into<String>>(&mut self, rights: S) -> &mut Self {
        self.rights = Some(rights.into());
        self
    }

    /// Add a metadata key-value pair to the epub
    pub fn add_metadata<S: Into<String>>(&mut self, key: S, value: S) -> &mut Self {
        if self.metadata.is_none() {
            self.metadata = Some(DashMap::new());
        }
        let metadata = &mut self.metadata;
        metadata.as_mut().unwrap().insert(key.into(), value.into());
        self
    }

    /// Add an image file to the epub
    pub fn add_image<S1: Into<String>>(
        &mut self,
        source: S1,
        internal_filename: Option<String>,
    ) -> Result<String, Error> {
        let images = &mut self.images;
        super::add_media(
            source.into(),
            internal_filename.map(|s| s.to_string()),
            String::from("image"),
            String::from(IMAGE_FOLDER_NAME),
            images,
        )
    }

    /// Add a video file to the epub
    pub fn add_video<S1: Into<String>>(
        &mut self,
        source: S1,
        internal_filename: Option<String>,
    ) -> Result<String, Error> {
        let videos = &mut self.videos;
        super::add_media(
            source.into(),
            internal_filename.map(|s| s.to_string()),
            String::from("video"),
            String::from(VIDEO_FOLDER_NAME),
            videos,
        )
    }

    /// Add an audio file to the epub
    pub fn add_audio<S1: Into<String>>(
        &mut self,
        source: S1,
        internal_filename: Option<String>,
    ) -> Result<String, Error> {
        let audios = &mut self.audios;
        super::add_media(
            source.into(),
            internal_filename.map(|s| s.to_string()),
            String::from("audio"),
            String::from(AUDIO_FOLDER_NAME),
            audios,
        )
    }
    /// Add a stylesheet file to the epub
    pub fn add_stylesheet<S1: Into<String>>(
        &mut self,
        source: S1,
        internal_filename: Option<String>,
    ) -> Result<String, Error> {
        let stylesheet = &mut self.stylesheet;
        super::add_media(
            source.into(),
            internal_filename.map(|s| s.to_string()),
            String::from("style"),
            String::from(CSS_FOLDER_NAME),
            stylesheet,
        )
    }

    /// Add a font file to the epub
    pub fn add_font<S1: Into<String>>(
        &mut self,
        source: S1,
        internal_filename: Option<String>,
    ) -> Result<String, Error> {
        let fonts = &mut self.fonts;
        super::add_media(
            source.into(),
            internal_filename.map(|s| s.to_string()),
            String::from("font"),
            String::from(FONT_FOLDER_NAME),
            fonts,
        )
    }

    /// Set the epub cover
    pub fn set_cover<S1: Into<String>>(
        &mut self,
        internal_image_path: S1,
        internal_css_path: Option<String>,
    ) -> Result<String, Error> {
        let raw_image_path = internal_image_path.into();
        let image_path = Path::new(&raw_image_path);

        if !image_path.exists() {
            return Err(Error::FileNotFoundErr(format!(
                "file not found:{}",
                raw_image_path.clone()
            )));
        }

        first(raw_image_path.clone()).ok_or_else(|| {
            Error::MediaError(format!("file mime err:{}", raw_image_path.clone()))
        })?;

        // 移除之前的封面
        self.remove_cover_resources()?;

        // 添加封面图片到资源列表中
        let cover_image_filename = self.add_image(raw_image_path.clone(), None)?;

        let body = format!("<img src=\"{}\" alt=\"cover\"/>", cover_image_filename);
        let cover_xhtml_filename = self.add_section(
            body,
            "封面",
            Some(String::from(COVER_FILE_NAME)),
            internal_css_path,
        )?;

        // 添加新封面
        let cover = self
            .cover
            .get_or_insert_with(|| Arc::new(Mutex::new(Cover::default())));
        let mut cover = cover.lock().unwrap();

        // 设置封面文件名
        cover.filename = image_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default()
            .to_string();

        Ok(cover_xhtml_filename)
    }

    /// Add a section to the epub
    pub fn add_section<S1: Into<String>, S2: Into<String>>(
        &mut self,
        body: S1,
        section_title: S2,
        internal_filename: Option<String>,
        internal_css_path: Option<String>,
    ) -> Result<String, Error> {
        self.add_sub_section(
            None,
            body,
            section_title,
            internal_filename,
            internal_css_path,
        )
    }
    /// Add a section to the epub
    pub fn add_sub_section<S1: Into<String>, S2: Into<String>>(
        &mut self,
        parent_filename: Option<String>,
        body: S1,
        section_title: S2,
        internal_filename: Option<String>,
        internal_css_path: Option<String>,
    ) -> Result<String, Error> {
        let mut base_filename = String::new();
        if let Some(mut filename) = internal_filename {
            let ext = Path::new(&filename)
                .extension()
                .and_then(|osstr| osstr.to_str())
                .unwrap_or("");
            if ext != "xhtml" {
                filename = format!("{}.xhtml", filename);
            }
            if self.filenames.contains(&filename) {
                return Err(Error::FilenameExistedErr(format!(
                    "file already exists:{}",
                    filename
                )));
            }
            base_filename = filename.clone();
        };
        let mut index = self.filenames.len();
        loop {
            if base_filename.is_empty() || self.filenames.contains(&base_filename) {
                base_filename = format!("section_{}.xhtml", index + 1);
                index += 1;
                continue;
            };
            break;
        }
        let mut parent_current_filename = String::new();
        if let Some(p_filename) = parent_filename {
            if !p_filename.is_empty() {
                if !self.filenames.contains(&p_filename) {
                    return Err(Error::ParentExistedErr(format!(
                        "Parent file already exists:{}",
                        p_filename
                    )));
                }
                parent_current_filename = p_filename;
            }
        }

        let mut section = Section::new(base_filename.clone());
        let title = section_title.into();
        section.title = title.clone();
        section.xhtml.set_body(body.into());
        section.xhtml.set_title(title.clone());

        if let Some(css_path) = internal_css_path {
            let base_css_path = self.add_stylesheet(css_path, None)?;
            section
                .xhtml
                .add_link(XHtmlLinkItem::new(base_css_path, "text/css", None));
        }
        if !parent_current_filename.is_empty() {
            let mut target_section = None;
            for item in self.sections.iter_mut() {
                let mut stack = vec![item];
                while let Some(parent) = stack.pop() {
                    if parent.filename == parent_current_filename {
                        target_section = Some(parent); // 保存引用
                        break;
                    } else {
                        stack.extend(parent.childs.iter_mut());
                    }
                }
                if target_section.is_some() {
                    break;
                }
            }
            // 延迟将 section 添加到目标 section 的子节点
            if let Some(parent) = target_section {
                parent.childs.push(section);
            }
        } else {
            self.sections.push(section);
        }

        self.filenames.insert(base_filename.clone());

        Ok(String::from(base_filename))
    }

    /// 移除旧封面资源
    fn remove_cover_resources(&mut self) -> Result<(), Error> {
        if let Some(cover) = &self.cover {
            let cover = cover.lock().unwrap();
            // 如果封面不存在，则直接返回
            if cover.filename.is_empty() {
                return Ok(());
            }
            let parent_current_filename = &cover.xhtml_filename;

            // 移除封面章节
            let mut indices_to_remove = vec![];
            for (index, item) in self.sections.iter_mut().enumerate() {
                let mut stack = vec![item];
                while let Some(parent) = stack.pop() {
                    if parent.filename == *parent_current_filename {
                        indices_to_remove.push(index);
                        break;
                    } else {
                        stack.extend(parent.childs.iter_mut());
                    }
                }
            }
            for index in indices_to_remove.into_iter().rev() {
                self.sections.remove(index);
            }

            // 移除文件名
            self.filenames.remove(parent_current_filename);

            // 移除封面样式
            if let Some(css_filename) = &cover.css_filename {
                self.stylesheet.remove(css_filename);
            }

            // 移除封面图片
            self.images.remove(&cover.image_filename);
        }
        Ok(())
    }

    pub fn output(&mut self, output_path: &Path) -> Result<(), Error> {
        println!("Output: {}", output_path.display());
        self.create_folder(output_path)?;


        let ret = self.encode_toc_xml()?;
        self.write_file(output_path.join("toc.ncx").as_ref(), &ret)?;



        Ok(())
    }

    fn write_sections(&mut self, root_path:&Path, item: &mut Section) -> Result<(), Error> {
        let path = root_path.join(CONTENT_FOLDER_NAME).join(XHTML_FOLDER_NAME).join(&item.filename);
        self.write_file(path.as_ref(), &item.xhtml.encode_xml()?)?;
        for child in &mut item.childs {
            self.write_sections(root_path,child)?
        }
        Ok(())
    }

    fn encode_toc_xml(&mut self) -> Result<String, Error> {
        let lang = self.language.clone().unwrap_or(String::from("zh-CN"));

        let mut toc = TocNav::new(self.title.clone(), lang);
        toc.add_metadata("dtb:uid", self.identifier.clone().unwrap_or(String::new()));
        toc.add_metadata("dtb:totalPageCount", "0");
        toc.add_metadata("dtb:maxPageNumber", "0");

        let mut depth = 0;
        let index = 0;
        self.sections
            .iter()
            .map(|item| Self::convert_section(item, index, 0, &mut depth))
            .for_each(|element| {
                toc.add_element(element);
            });

        toc.add_metadata("dtb:depth", format!("{}", depth + 1));
        let toc_xml = toc.encode_file(self.version);

        toc_xml
    }

    /// 递归将 Section 转换为 TocElement
    fn convert_section(section: &Section, index: i32, depth: usize, max_depth: &mut usize)
        -> TocElement {
        if depth > *max_depth {
            *max_depth = depth;
        }
        TocElement {
            level: index,
            url: format!("{}/{}", XHTML_FOLDER_NAME, section.filename.clone()),
            title: section.title.clone(),
            childs: section
                .childs
                .iter()
                .map(|item| Self::convert_section(item, index + 1, depth + 1, max_depth)) // 递归转换子节点
                .collect(),
        }
    }

    /// 创建文件夹
    fn create_folder(&mut self,output_path: &Path) -> Result<(), Error> {
        let folder_path = Path::new(output_path);
        write::create_epub_folders(folder_path)?;
        write::create_media_folder(folder_path,CSS_FOLDER_NAME,&mut self.stylesheet)?;
        write::create_media_folder(folder_path,IMAGE_FOLDER_NAME,&mut self.images)?;
        write::create_media_folder(folder_path,FONT_FOLDER_NAME,&mut self.fonts)?;
        write::create_media_folder(folder_path,VIDEO_FOLDER_NAME,&mut self.videos)?;
        write::create_media_folder(folder_path,AUDIO_FOLDER_NAME,&mut self.audios)?;

        if !self.filenames.is_empty() {
            let media_folder_path = folder_path.join(CONTENT_FOLDER_NAME).join(XHTML_FOLDER_NAME);
            if let Err(e) = fs::create_dir_all(&media_folder_path) {
                return Err(Error::PathCreateErr(format!("Could not create media folder:{}", e)));
            }
        }
        Ok(())
    }
    /// 写入文件
    fn write_file(&mut self, output_path: &Path,content:&str) -> Result<(), Error> {
        if let Err(e) = fs::write(output_path, content) {
           return  Err(Error::PathCreateErr(format!("Could not create file:{}", e)))
        }
        Ok(())
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Cover {
    filename: String,
    css_filename: Option<String>,
    image_filename: String,
    xhtml_filename: String,
}

impl Default for Cover {
    fn default() -> Self {
        Cover {
            filename: "".to_string(),
            css_filename: None,
            image_filename: "".to_string(),
            xhtml_filename: "".to_string(),
        }
    }
}

impl Cover {
    #[allow(dead_code)]
    pub fn new<S: Into<String>>(filename: S) -> Cover {
        Cover {
            filename: filename.into(),
            css_filename: None,
            image_filename: "".to_string(),
            xhtml_filename: "".to_string(),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Section {
    filename: String,
    title: String,
    xhtml: XHtmlRoot,
    childs: Vec<Section>,
}

impl Default for Section {
    fn default() -> Section {
        Section {
            filename: String::new(),
            title: String::new(),
            xhtml: XHtmlRoot::default(),
            childs: Vec::new(),
        }
    }
}
impl Section {
    #[allow(dead_code)]
    pub fn new<S: Into<String>>(filename: S) -> Section {
        Section {
            filename: filename.into(),
            title: String::new(),
            xhtml: XHtmlRoot::default(),
            childs: Vec::new(),
        }
    }
}
