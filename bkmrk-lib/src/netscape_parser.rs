use eyre::{eyre, Result};
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

use crate::bookmark::TagList;
use crate::element::{Element, Tags};
use crate::site_metadata::SiteMetadata;
use crate::{utils, Bookmark};
use chrono::Utc;
use std::collections::{HashMap, HashSet};

#[derive(Parser)]
#[grammar = "netscape.pest"]
struct NetScapeParser;

pub fn parse_str(contents: &str, add_folder_labels: bool) -> Result<Vec<Bookmark>> {
    let elements = get_elements(contents)?;
    let bookmarks = get_bookmarks(&elements, None, add_folder_labels)?;
    Ok(bookmarks)
}

fn get_elements(contents: &str) -> Result<Vec<Element>> {
    let mut elements = Vec::new();

    let res = NetScapeParser::parse(Rule::netscape, contents)?;

    for pair in res {
        match pair.as_rule() {
            Rule::element => {
                elements.push(make_element(pair)?);
            }
            Rule::doctype | Rule::EOI => {}
            _ => unreachable!(),
        }
    }

    Ok(elements)
}

fn get_bookmarks(
    elements: &[Element],
    labels: Option<&[String]>,
    add_folder_labels: bool,
) -> Result<Vec<Bookmark>> {
    let mut bookmarks = Vec::new();
    let mut labels = Vec::from(labels.unwrap_or_default());

    for element in elements {
        match element.name {
            Tags::TITLE | Tags::H1 | Tags::H3 => {
                labels.push(element.inner_text.to_owned().unwrap_or_default())
            }
            Tags::DL => {
                let dl_children = element
                    .children
                    .as_ref()
                    .ok_or(eyre!("Failed to get element children"))?;
                let mut ch = get_bookmarks(dl_children, Some(&labels), add_folder_labels)?;
                bookmarks.append(&mut ch);
            }
            Tags::A => {
                let attributes = element.attributes.to_owned().unwrap();
                let mut attr_labels: Vec<String> = attributes
                    .get("TAGS")
                    .unwrap_or(&String::default())
                    .split(",")
                    .filter(|s| !s.is_empty())
                    .map(String::from)
                    .collect::<Vec<_>>();

                if add_folder_labels {
                    attr_labels.extend_from_slice(&labels);
                }

                let attr_labels = attr_labels
                    .into_iter()
                    .collect::<HashSet<_>>()
                    .into_iter()
                    .collect();

                let bk = Bookmark {
                    link: attributes.get("HREF").unwrap().to_owned(),
                    added_at: match attributes.get("ADD_DATE") {
                        Some(ts_str) => ts_str.parse()?,
                        None => Utc::now().timestamp(),
                    },
                    last_modified: match attributes.get("LAST_MODIFIED") {
                        Some(ts_str) => ts_str.parse()?,
                        None => Utc::now().timestamp(),
                    },
                    tags: TagList(attr_labels),
                    metadata: SiteMetadata {
                        title: utils::squeeze_whitespaces(
                            element.inner_text.to_owned().unwrap_or_default().trim(),
                        ),
                        description: element.text.to_owned(),
                        ..Default::default()
                    },
                    ..Default::default()
                };

                bookmarks.push(bk);
            }
            Tags::NONE => unreachable!(),
            _ => {}
        }
    }

    Ok(bookmarks)
}

fn make_element(pair: Pair<Rule>) -> Result<Element> {
    if pair.as_rule() != Rule::element {
        return Err(eyre!("Passed element not an element"));
    }

    let mut el = Element::default();

    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::ident | Rule::tag_names_no_close => el.name = p.as_str().into(),
            Rule::attributes => el.attributes = get_attributes(p)?,
            Rule::innertext => el.inner_text = Some(p.as_str().into()),
            Rule::children => {
                let mut elements = Vec::new();

                for ch in p.into_inner() {
                    elements.push(make_element(ch)?);
                }

                el.children = Some(elements);
            }
            Rule::text => el.text = Some(p.as_str().into()),
            _ => unreachable!(),
        }
    }

    Ok(el)
}

fn get_attributes(pairs: Pair<Rule>) -> Result<Option<HashMap<String, String>>> {
    let mut attributes = HashMap::new();
    if pairs.as_rule() != Rule::attributes {
        return Err(eyre!("Passed element not attributes"));
    }

    let attr_pairs = pairs.into_inner().collect::<Vec<_>>();
    if attr_pairs.is_empty() {
        return Ok(None);
    }

    for attr_pair in attr_pairs {
        let mut val = attr_pair.into_inner();
        let k = val
            .next()
            .ok_or(eyre!("Couldn't get attribute key"))?
            .as_str();
        let v = val
            .next()
            .ok_or(eyre!("Couldn't get attribute key"))?
            .as_str();

        attributes.insert(k.into(), v.into());
    }

    Ok(Some(attributes))
}
