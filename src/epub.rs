use serde::Deserialize;
/*
 * table contet
 * */

#[derive(Deserialize, Debug)]
struct Toc {
    #[serde(rename = "head")]
    head: Head,
    #[serde(rename = "docTitle")]
    doc_title: DocTitle,
    #[serde(rename = "navMap")]
    nav_map: NavMap,
}

#[derive(Deserialize, Debug)]
struct Head {
    #[serde(rename = "meta")]
    meta: Vec<Meta>,
}

#[derive(Deserialize, Debug)]
struct Meta {
    #[serde(rename = "@content")]
    content: String,
    #[serde(rename = "@name")]
    name: String,
}

#[derive(Deserialize, Debug)]
struct DocTitle {
    #[serde(rename = "text")]
    text: String,
}

#[derive(Deserialize, Debug)]
pub struct NavMap {
    #[serde(rename = "navPoint")]
    nav_points: Vec<NavPoint>,
}

#[derive(Deserialize, Debug)]
struct NavPoint {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@playOrder")]
    play_order: u32,
    #[serde(rename = "navLabel")]
    nav_label: NavLabel,
    #[serde(rename = "content")]
    content: Content,
}

#[derive(Deserialize, Debug)]
struct NavLabel {
    #[serde(rename = "text")]
    text: String,
}

#[derive(Deserialize, Debug)]
struct Content {
    #[serde(rename = "@src")]
    src: String,
}

/* content.opf */
#[derive(Deserialize, Debug)]
struct Package {
    manifest: Manifest,
    spine: Spine,
}

#[derive(Deserialize, Debug)]
struct Manifest {
    #[serde(rename = "item")]
    items: Vec<ManifestItem>,
}

#[derive(Deserialize, Debug)]
struct ManifestItem {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@href")]
    href: String,
    #[serde(rename = "@media-type")]
    media_type: String,
}

#[derive(Deserialize, Debug)]
struct Spine {
    #[serde(rename = "itemref")]
    items: Vec<ItemRef>,
}

#[derive(Deserialize, Debug)]
struct ItemRef {
    #[serde(rename = "@idref")]
    idref: String,
}
pub mod epub {
    use crate::epub::Manifest;
    use crate::epub::ManifestItem;
    use crate::epub::NavMap;
    use crate::epub::Package;
    use crate::epub::Toc;
    use scraper::Html;
    use scraper::Selector;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::io::Read;
    use std::path::Path;
    use std::rc::Rc;
    use zip::ZipArchive;

    pub fn load(path: &Path) -> () {
        let files_map = import_data(path);
        let toc_ncx = files_map.get("toc.ncx").expect("toc.ncx not found");
        let content_opf = files_map.get("content.opf").expect("content.opf not found");
        let nav_map = define_structure(toc_ncx).expect("nav map not found!");
        // some time toc.ncx does not sync with content.opf
        let chapters = define_chapters(content_opf).expect("chapters not found!");
        let extracted_content = merge(nav_map, chapters, &files_map);

        print!("{:#?}", extracted_content);
    }

    fn define_chapters(content_opf: &str) -> Result<Vec<ManifestItem>, quick_xml::DeError> {
        let package: Package = quick_xml::de::from_str(content_opf)?;
        Ok(package.manifest.items)
    }

    /*
     * Import the epub content into object
     */
    fn import_data(path: &Path) -> Rc<HashMap<String, String>> {
        let zip_file = std::fs::File::open(path).unwrap();
        let mut archive = ZipArchive::new(zip_file).unwrap();
        let mut files_map = HashMap::<String, String>::new();
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            // check if its directory, or its a css file
            if file.is_dir() {
                continue;
            }
            let mut content = String::with_capacity(file.size() as usize);
            let _ = file.read_to_string(&mut content);
            files_map.insert(file.name().to_string(), content);
        }
        Rc::new(files_map)
    }

    fn define_structure(toc: &str) -> Result<NavMap, quick_xml::DeError> {
        let table_of_content: Toc = quick_xml::de::from_str(toc)?;
        Ok(table_of_content.nav_map)
    }

    /*
       merge content with chapter
       */
    fn merge(
        table_of_content: NavMap,
        chapters: Vec<ManifestItem>,
        files_map: &HashMap<String, String>,
    ) -> Rc<RefCell<HashMap<String, String>>> {
        let mut result = HashMap::<String, String>::new();
        let total = files_map.values().map(|v| v.len()).sum();
        let content = String::with_capacity(total);
        let is_equal = chapters.len() == table_of_content.nav_points.len();
        /* if table_of_content length equals to chapters then key and value are both from chapters
           if not then take key from table of content and value from chapters
           */
        if !is_equal {
            for chapter in chapters {
                let chapter_file = &chapter.href;
                let file_content = files_map
                    .get(chapter_file)
                    .expect("Error when finding content");
                let cleaned = remove_tags(file_content);
                result.insert(chapter_file.to_string(), cleaned);
            }
        } else {
            for nav_point in table_of_content.nav_points {
                let chapter = &nav_point.content.src;
                let file_content = files_map.get(chapter).expect("Error when finding content");
                let cleaned = remove_tags(file_content);
                result.insert(nav_point.nav_label.text, cleaned);
            }
        }
        Rc::new(RefCell::new(result))
    }

    /*
       remove html tags
       */
    fn remove_tags(raw_content: &str) -> String {
        println!("{}", raw_content);
        let mut lines = String::new();
        let document = Html::parse_document(&raw_content);
        let body_selector = Selector::parse("body").unwrap();
        let p_selector = Selector::parse("p").unwrap();

        if let Some(body) = document.select(&body_selector).next() {
            for el in body.select(&p_selector) {
                let text = el.text().collect::<String>().trim().to_string();
                if !text.is_empty() {
                    lines.push_str(&text);
                }
            }
        }
        lines
    }
}
