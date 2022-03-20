extern crate downloader;
extern crate reqwest;
extern crate select;

use anyhow::Result;
use select::document::Document;
use select::predicate::Attr;
use std::env::{self, current_dir};
use std::fs::File;
use std::io::{self, Write};

pub async fn scrape(url: &str) -> Result<String> {
    let resp = reqwest::get(url).await?;

    assert!(resp.status().is_success());

    let t = resp.text().await?;

    let doc = Document::from_read(t.as_bytes()).unwrap();

    let video = doc.find(Attr("property", "og:video")).collect::<Vec<_>>()[0];

    Ok(String::from(video.attr("content").unwrap()))
}

pub async fn download(url: &str) -> Result<String> {
    let mut p = current_dir()?.join("sussy-baka.mp4");

    println!("{:#?}", p);

    let mut resp = reqwest::get(url).await?;

    let mut dest = File::create(&p)?;

    while let Some(bytes) = resp.chunk().await? {
        dest.write_all(&bytes);
    }

    Ok(String::from(p.as_os_str().to_str().unwrap()))
}

// pub async fn download(url: &str) -> Result<String> {
//     let mut p = current_dir()?.join("file");

//     println!("{:#?}", p);

//     let file = reqwest::get(url).await?;

//     let bytes = file.text().await?;

//     let mut dest = File::create(&p)?;

//     copy(&mut bytes.as_bytes(), &mut dest)?;

//     Ok(String::from(p.as_os_str().to_str().unwrap()))
// }

pub async fn yoink(url: &str) -> Result<String> {
    let target_link = scrape(url).await?;
    let str_path = download(&target_link).await?;

    return Ok(str_path);
}
