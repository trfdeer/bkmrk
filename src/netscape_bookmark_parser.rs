use crate::bookmark::{Bookmark, TagList};
use crate::utils;
use chrono::Utc;
use itertools::Itertools;
use quick_xml::{events::Event, Reader};
use simple_error::{bail, SimpleError};
use std::collections::{HashMap, VecDeque};
use std::path::Path;
use std::str;

pub fn parse_netscape_bookmark_file(
    file_path: &Path,
    append_folder_tags: bool,
) -> Result<Vec<Bookmark>, SimpleError> {
    let file_contents = match utils::files::read_file(file_path) {
        Ok(res) => res
            .replace("<DT>", "<DT />")
            .replace("<DD>", "<DD />")
            .replace("<p>", "<p />"),
        Err(e) => bail!("Failed to read file {}\n{}", file_path.display(), e),
    };

    let mut reader = Reader::from_str(&file_contents);
    reader.trim_text(true);
    reader.trim_markup_names_in_closing_tags(true);

    let mut buf = Vec::new();

    let mut last_event: (String, String) = Default::default();
    let mut new_bookmark: Bookmark = Bookmark::default();

    let mut tags: VecDeque<String> = VecDeque::default();
    let mut bookmarks: Vec<Bookmark> = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => match e.name() {
                b"TITLE" | b"H1" | b"H3" => {
                    last_event = (str::from_utf8(e.name()).unwrap().to_owned(), "open".into());
                }
                b"A" => {
                    last_event = (str::from_utf8(e.name()).unwrap().to_owned(), "open".into());
                    let attributes = e
                        .attributes()
                        .filter_map(|x| x.ok())
                        .map(|x| {
                            (
                                str::from_utf8(x.key).unwrap().to_owned(),
                                str::from_utf8(&x.value[..]).unwrap().to_owned(),
                            )
                        })
                        .collect::<HashMap<String, String>>();

                    let mut bk_tags: Vec<String> = attributes
                        .get("TAGS")
                        .unwrap_or(&String::default())
                        .to_owned()
                        .split(',')
                        .map(String::from)
                        .filter(|x| !x.is_empty())
                        .collect();

                    if append_folder_tags {
                        let mut folder_tags = tags
                            .iter_mut()
                            .map(|x| x.to_owned())
                            .filter(|x| !x.is_empty())
                            .collect::<Vec<String>>();
                        bk_tags.append(&mut folder_tags);
                    }

                    new_bookmark.link =
                        attributes.get("HREF").unwrap().to_owned().trim().to_owned();
                    new_bookmark.tags =
                        TagList(bk_tags.iter().unique().map(String::from).collect());
                    new_bookmark.added_at = attributes
                        .get("ADD_DATE")
                        .unwrap()
                        .to_owned()
                        .parse()
                        .unwrap();
                    new_bookmark.last_modified = attributes
                        .get("LAST_MODIFIED")
                        .unwrap_or(&format!("{}", Utc::now().timestamp()))
                        .to_owned()
                        .parse()
                        .unwrap();
                }
                _ => (),
            },
            Ok(Event::End(ref e)) => match e.name() {
                b"DL" => {
                    tags.pop_back().unwrap_or_default();
                }
                b"A" => {
                    last_event = (str::from_utf8(e.name()).unwrap().to_owned(), "close".into());
                }
                _ => (),
            },
            Ok(Event::Text(t)) => match (last_event.0.as_str(), last_event.1.as_str()) {
                ("TITLE" | "H1" | "H3", "open") => {
                    let new_tag = t.unescape_and_decode(&reader).unwrap();
                    tags.push_back(new_tag);
                }
                ("A", "open") => {
                    new_bookmark.name =
                        utils::squeeze_whitespaces(t.unescape_and_decode(&reader).unwrap().trim());
                    bookmarks.push(new_bookmark.clone());
                    new_bookmark = Bookmark::default();
                }
                ("A", "close") => {
                    let mut bk = bookmarks.pop().unwrap();
                    bk.description = t.unescape_and_decode(&reader).unwrap_or_default();
                    bookmarks.push(bk);
                }
                _ => (),
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
    }

    buf.clear();

    Ok(bookmarks)
}
