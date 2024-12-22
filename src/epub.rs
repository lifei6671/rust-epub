use crate::mime::first;
use crate::Error;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

#[allow(dead_code)]
const CSS_FOLDER_NAME: &str = "css";
#[allow(dead_code)]
const FONT_FOLDER_NAME: &str = "fonts";
#[allow(dead_code)]
const IMAGE_FOLDER_NAME: &str = "images";
#[allow(dead_code)]
const VIDEO_FOLDER_NAME: &str = "videos";
#[allow(dead_code)]
const AUDIO_FOLDER_NAME: &str = "audios";

#[allow(dead_code)]
const COVER_CSS_CONTENT: &str = "\
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
const COVER_FILE_NAME: &str = "cover.xhtml";
#[allow(dead_code)]
const COVER_CSS_FILE: &str = "cover.css";

/// epub规范版本
#[derive(Debug)]
#[allow(dead_code)]
pub enum EpubVersion {
    V20,
    V30,
}

/// An epub file structure instance
#[derive(Debug)]
#[allow(dead_code)]
pub struct EpubBuilder {
    /// Book title
    title: Mutex<String>,
    /// Book creator
    creator: Option<String>,
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
    cover: Option<Arc<Cover>>,

    /// Book other metadata
    metadata: Option<HashMap<String, String>>,

    /// Custom style sheet collection
    stylesheet: HashMap<String, String>,
    /// Custom font collection
    fonts: HashMap<String, String>,
    /// Books Picture Collection
    images: HashMap<String, String>,
    /// Books Video Collection
    videos: HashMap<String, String>,
    /// Books Audio Collection
    audios: HashMap<String, String>,
}

impl Default for EpubBuilder {
    fn default() -> Self {
        EpubBuilder::new(String::from(""))
    }
}

impl EpubBuilder {
    #[allow(dead_code)]
    pub fn new<S: Into<String>>(title: S) -> EpubBuilder {
        EpubBuilder {
            title: Mutex::from(title.into()),
            creator: None,
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
            stylesheet: HashMap::new(),
            fonts: HashMap::new(),
            images: HashMap::new(),
            videos: HashMap::new(),
            audios: HashMap::new(),
        }
    }

    /// Add a image file to the epub
    pub fn add_image<S1: Into<String>>(
        &mut self,
        source: S1,
        internal_filename: Option<String>,
    ) -> Result<String, Error> {
        let  images = &mut self.images;
        super::add_media(
            source.into(),
            internal_filename.map(|s| s.to_string()),
            String::from("image"),
            String::from(IMAGE_FOLDER_NAME),
            images,
        )
    }

    pub fn set_cover<S1: Into<String>, S2: Into<String>>(
        &mut self,
        internal_image_path: S1,
        internal_css_path: Option<S2>,
    ) -> &mut EpubBuilder {
        self
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Cover {
    filename: String,
    temp_filename: String,
    image_filename: String,
    xhtml_filename: String,
}

impl Cover {
    #[allow(dead_code)]
    pub fn new<S: Into<String>>(filename: S) -> Cover {
        Cover {
            filename: "".to_string(),
            temp_filename: "".to_string(),
            image_filename: "".to_string(),
            xhtml_filename: "".to_string(),
        }
    }
}

