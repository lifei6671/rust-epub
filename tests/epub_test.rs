use rand::{thread_rng, Rng};
use rust_epub::epub::{EpubBuilder, EpubVersion};
use std::env;
use std::fs;

#[test]
fn test_add_image() {
    let mut epub = EpubBuilder::new("大语言模型：辩论", EpubVersion::V20);
    let current_dir = env::current_dir()
        .unwrap()
        .join("tests")
        .join("testdata")
        .join("cover.jpg")
        .to_str()
        .unwrap()
        .to_string();

    let image_1 = epub.add_image(&current_dir, None);
    println!("{:?}", image_1);
    assert!(image_1.is_ok());
    assert_eq!(image_1.unwrap(), "../images/cover.jpg");

    let ret = epub.add_image(&current_dir, None);

    assert!(ret.is_ok());
    let image_str_2 = ret.unwrap();
    assert_eq!(image_str_2, "../images/image_2.jpg");

    println!("{:?}", image_str_2);
}

#[test]
fn test_add_video() {
    let mut epub = EpubBuilder::new("大语言模型：辩论", EpubVersion::V20);
    let current_dir = env::current_dir()
        .unwrap()
        .join("tests")
        .join("testdata")
        .join("video.mp4")
        .to_str()
        .unwrap()
        .to_string();

    let video_1 = epub.add_video(&current_dir, None);
    println!("{:?}", video_1);
    assert!(video_1.is_ok());

    let video_2 = epub.add_video(&current_dir, None);
    println!("{:?}", video_2);
    assert!(video_2.is_ok());
}

#[test]
fn test_add_audio() {
    let mut epub = EpubBuilder::new("大语言模型：辩论", EpubVersion::V20);
    let current_dir = env::current_dir()
        .unwrap()
        .join("tests")
        .join("testdata")
        .join("audio.mp4")
        .to_str()
        .unwrap()
        .to_string();

    let audio_1 = epub.add_audio(&current_dir, None);
    println!("{:?}", audio_1);
    assert!(audio_1.is_ok());

    let audio_2 = epub.add_audio(&current_dir, None);
    println!("{:?}", audio_2);
    assert!(audio_2.is_ok());
}

#[test]
fn test_add_section() {
    let mut epub = EpubBuilder::new("大语言模型：辩论", EpubVersion::V20);
    let current_dir = env::current_dir()
        .unwrap()
        .join("tests")
        .join("testdata")
        .join("style.css")
        .to_str()
        .unwrap()
        .to_string();

    let ret = epub.add_section("测试章节", "测试章节内容", None, None);

    println!("{:?}", ret);
    assert!(ret.is_ok());
    let parent_filename = ret.unwrap();
    assert_eq!(parent_filename, "section_1.xhtml");

    let sub_ret = epub.add_sub_section(
        Some(parent_filename),
        "测试章节2",
        "测试章节内容2",
        None,
        Some(current_dir),
    );
    println!("{:?}", sub_ret);
    assert!(sub_ret.is_ok());
}

#[test]
fn test_set_cover() {
    let mut epub = EpubBuilder::new("大语言模型：辩论", EpubVersion::V20);
    let current_dir = env::current_dir()
        .unwrap()
        .join("tests")
        .join("testdata")
        .join("cover.jpg")
        .to_str()
        .unwrap()
        .to_string();

    let ret = epub.set_cover(&current_dir, None);
    println!("{:?}", ret);
    assert!(ret.is_ok());
}

#[test]
fn test_encode_toc_xml() {
    let mut epub = EpubBuilder::new("大语言模型：辩论", EpubVersion::V20);
    // let temp_dir: PathBuf = env::temp_dir();
    let mut rng = thread_rng();
    let random_dir_name: String = (0..8)
        .map(|_| {
            let num_or_char = rng.gen_range(0..36);
            if num_or_char < 10 {
                num_or_char.to_string()
            } else {
                let char_code = num_or_char - 10 + 'a' as usize;
                char::from(char_code as u8).to_string()
            }
        })
        .collect();
    let random_dir_path = env::current_dir()
        .unwrap()
        .join("tests")
        .join(random_dir_name);

    let e = fs::create_dir_all(&random_dir_path);
    assert!(e.is_ok());

    let current_dir = env::current_dir()
        .unwrap()
        .join("tests")
        .join("testdata")
        .join("style.css")
        .to_str()
        .unwrap()
        .to_string();

    let section = epub.add_section("测试章节", "测试章节内容", None, None);

    let parent_filename = section.unwrap();
    assert_eq!(parent_filename, "section_1.xhtml");

    let sub_section = epub.add_sub_section(
        Some(parent_filename),
        "测试章节2",
        "测试章节内容2",
        None,
        Some(current_dir.clone()),
    );
    println!("{:?}", sub_section);

    let sub_parent_filename = sub_section.unwrap();

    let sub_sub_section = epub.add_sub_section(
        Some(sub_parent_filename),
        "测试章节3",
        "测试章节内容3",
        None,
        Some(current_dir.clone()),
    );

    println!("{:?}", sub_sub_section);

    let ret = epub.output(random_dir_path.as_path());
    println!("{:?}", ret);
}
