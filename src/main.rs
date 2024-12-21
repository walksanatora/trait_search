use std::{fs::ReadDir, path::PathBuf};

use clap::Parser;
use ego_tree::NodeRef;
use scraper::{Html, Node, Selector};

#[derive(Parser)]
struct Args {
    r#trait: String,

    #[arg(short,long)]
    doc_folder: Option<PathBuf>
}

fn node_to_text(node: NodeRef<'_,Node>) -> String {
    match node.value() {
        Node::Document => todo!(),
        Node::Fragment => todo!(),
        Node::Doctype(_doctype) => todo!(),
        Node::Comment(_comment) => todo!(),
        Node::Text(text) => text.to_string(),
        Node::Element(_element) => {
            scraper::element_ref::ElementRef::wrap(node).unwrap().text().collect()
        },
        Node::ProcessingInstruction(_processing_instruction) => todo!(),
    }
}

fn work(implementors: &mut Vec<String>, dir: PathBuf, selector: &Selector) {
    if let Ok(read) = std::fs::read_dir(&dir) {
        for file in read {
            if let Ok(entry) = file {
                let path = entry.path();
                if entry.metadata().unwrap().is_dir() {
                    work(implementors, path, selector)
                } else {
                    let ext = path.extension().and_then(|x| x.to_str());
                    let valid = ext == Some("htm") || ext == Some("html");
                    if !valid {
                        continue;
                    }
                    let con = std::fs::read_to_string(entry.path());
                    if con.is_err() {
                        panic!("entry {:?} had a error {:?}", entry.path(), con.unwrap_err())
                        
                    }
                    let contents = con.unwrap();
                    let document = Html::parse_document(&contents);
                    let paths: Vec<_> = document
                        .select(selector)
                        //.filter_map(|el| el.value().attr("href"))
                        .collect();   
                    if !paths.is_empty() {
                        let header = Selector::parse(r#"div.main-heading > h1"#).unwrap();
                        let strct = document.select(&header).next().unwrap().children()
                            .filter_map(|node| {
                                if let Some(element) = node.value().as_element() {
                                    // Ignore <button> tags
                                    if element.name() == "button" {
                                        return None;
                                    }
                                }
                                Some(node_to_text(node))
                            }).collect::<String>();
                        let strct2 = strct.trim();
                        implementors.push(strct2.to_string());
                    }
                }
            } else {
                println!("error in path: {:?}, {:?}", dir, file.unwrap_err())
            }
        }
    }
}

fn main() {
    let args = Args::parse();

    let mut implementors = vec![];

    let sel = format!(r#"a.trait[title="trait {}"]"#, args.r#trait);
    // Create a selector for <a> tags with class="trait" and title="trait serde::ser::Serialize"
    let selector = Selector::parse(
        sel.as_str()
    ).unwrap();

    work(
        &mut implementors,
        args.doc_folder.unwrap_or(PathBuf::from(".")),
        &selector
    );
    println!("{:?}", implementors);
}
