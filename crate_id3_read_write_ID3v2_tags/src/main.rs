//!
//! [reference](https://docs.rs/id3/0.5.0/id3)
//!
//! *ID3*
//! > ID3，一般是位于一个mp3文件的开头或末尾的若干字节内，附加了关于该mp3的歌手，标题，专辑名称，年代，风格等信息，该信息就被称为ID3信息，ID3信息分为两个版本，v1和v2版。
//! > 其中：v1版的ID3在mp3文件的末尾128字节，以TAG三个字符开头，后面跟上歌曲信息。
//! > v2版一般位于mp3的开头，可以存储歌词，该专辑的图片等大容量的信息。
use id3::frame::Content;
use id3::{Frame, Tag, Version};

fn main() {
    // Modifying an existing tag
    let tag = Tag::read_from_path("music.mp3").unwrap();
    // print the artist the hard way
    println!("{}", tag.get("TPE1").unwrap().content().text().unwrap());
    // or print it the easy way
    println!("{}", tag.artist().unwrap());
    tag.write_to_path("music.mp3", Version::Id3v24).unwrap();

    // Creating a new tag
    let mut tag = Tag::new();
    // set the album the hard way
    let frame = Frame::with_content("TALB", Content::Text("album".to_string()));
    tag.add_frame(frame);
    // or set it the easy way
    tag.set_album("album");
    tag.write_to_path("music.mp3", Version::Id3v24).unwrap();
}
