use std::collections::HashMap;
use std::time::SystemTime;

#[allow(dead_code)]
const CSS_FOLDER_NAME: &str  = "css";
#[allow(dead_code)]
const FONT_FOLDER_NAME:&str = "fonts";
#[allow(dead_code)]
const IMAGE_FOLDER_NAME:&str = "images";
#[allow(dead_code)]
const VIDEO_FOLDER_NAME:&str = "videos";
#[allow(dead_code)]
const AUDIO_FOLDER_NAME:&str= "audios";

#[allow(dead_code)]
const COVER_CSS_CONTENT:&str = "\
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
const COVER_CSS_FILE:&str = "cover.css";


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
    /// Book creator
    creator : String,
    /// Book title
    title: String,
    /// Book subject
    subject : Option<String>,
    /// Book description
    description: Option<String>,
    /// Book date
    date : Option<SystemTime>,
    /// Book category
    category : Option<String>,
    /// Book Publishers
    publisher:Option<String>,
    /// Book contributor
    contributor:Option<String>,
    /// Book format
    format : Option<String>,
    /// Book identifier
    identifier : Option<String>,
    /// Book source
    source : Option<String>,
    /// Book language
    language : Option<String>,
    /// Book relation
    relation : Option<String>,
    /// Book coverage
    coverage : Option<String>,
    /// Book rights
    rights : Option<String>,

    /// Book other metadata
    metadata :Option<HashMap<String, String>>,
    /// Custom style sheet collection
    stylesheet: Vec<String>,
    /// Custom font collection
    fonts: Vec<String>,
    /// Books Picture Collection
    images: Vec<String>,
    /// Books Video Collection
    videos: Vec<String>,
    /// Books Audio Collection
    audios: Vec<String>,

}

#[derive(Debug)]
#[allow(dead_code)]
struct Cover {
    filename: String,
    temp_filename: String,
    image_filename: String,
    xhtml_filename: String,
}

