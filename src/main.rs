use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Deserialize, Debug, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
enum Quality {
    Bad,
    Good,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
enum Language {
    English,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
struct Video {
    #[serde(default = "get_default_empty_string")]
    slug: String,

    #[serde(default = "get_default_empty_string")]
    body: String,

    title: String,
    recording_quality: Quality,
    speakers: Vec<String>,
    date: String,
    length: u16,
    language: Language,
    youtube: String, // URL
}

fn get_default_empty_string() -> String {
    String::new()
}

fn load_videos() -> HashMap<String, Video> {
    let mut videos = HashMap::new();
    let paths = std::fs::read_dir("data/videos").unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if path.extension().unwrap() == "swp" {
            continue;
        }
        if path.file_name().unwrap() == "skeleton.md" {
            continue;
        }
        let (front_matter, body) = read_md_file_separate_front_matter(&path);
        let mut video: Video = serde_yml::from_str(&front_matter).unwrap();
        video.slug = path.file_stem().unwrap().to_str().unwrap().to_string();
        video.body = markdown2html(&body);

        let path_str = path.as_os_str().to_str().unwrap().to_string();
        videos.insert(path_str, video);
    }

    videos
}

fn read_md_file_separate_front_matter(path: &PathBuf) -> (String, String) {
    let content =
        std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Could not read file {path:?}"));
    let parts = content.split("---").collect::<Vec<_>>();
    assert!(parts.len() == 3, "File {path:?} does not have front matter");
    (parts[1].to_string(), parts[2].to_string())
}

fn markdown2html(content: &str) -> String {
    markdown::to_html_with_options(
        content,
        &markdown::Options {
            compile: markdown::CompileOptions {
                allow_dangerous_html: true,
                //allow_dangerous_protocol: true,
                ..markdown::CompileOptions::default()
            },
            ..markdown::Options::gfm()
        },
    )
    .unwrap()
}

fn main() {
    println!("Hello, world!");
    let videos = load_videos();
    println!("{:?}", videos);
}
