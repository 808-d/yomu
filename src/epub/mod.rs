use crate::common::common::File;
use std::{default, path::Path};
#[derive(Default, Debug)]
pub struct Epub {
    xml: XmlDeclaration,
    package: Package,
}

#[derive(Default, Debug)]
struct XmlDeclaration {
    version: String,
    encoding: String,
}

#[derive(Default, Debug)]
struct Package {
    version: String,
    unique_identifier: String,
    metadata: Metadata,
    manifest: Manifest,
    spine: Spine,
    guides: Guides,
}

#[derive(Default, Debug)]
struct Metadata {
    language: String,
    title: String,
    creator: Creator,
    contributor: Contributor,
    identifier: Vec<Identifier>,
    dc_date: String,
    metas: Metas,
}

#[derive(Default, Debug)]
struct Creator {
    text: String,
    file_as: String,
    role: String,
}

#[derive(Default, Debug)]
struct Contributor {
    text: String,
    role: String,
}

#[derive(Default, Debug)]
struct Identifier {
    text: String,
    id: Option<String>,
    scheme: String,
}

#[derive(Default, Debug)]
struct Metas {
    meta: String,
}

#[derive(Default, Debug)]
struct Manifest {
    item: Vec<ManifestItem>,
}

#[derive(Default, Debug)]
struct ManifestItem {
    id: String,
    href: String,
    media_type: String,
}

#[derive(Default, Debug)]
struct Spine {
    itemref: Vec<SpineItemRef>,
    toc: String,
}

#[derive(Default, Debug)]
struct SpineItemRef {
    idref: String,
}

#[derive(Default, Debug)]
struct Guides {
    references: Vec<GuideReference>,
}

#[derive(Default, Debug)]
struct GuideReference {
    guide_type: String,
    title: String,
    href: String,
}
mod epub {
    use super::Epub;
    use crate::common::common::File;
    use color_eyre::eyre::Ok;
    use std::cell::RefCell;
    use std::io::Read;
    use std::path::Path;
    use zip::ZipArchive;

    impl File<Epub> for Epub {
        fn unzip(&self, path: &Path) -> Vec<String> {
            let zip_file = std::fs::File::open(path).unwrap();
            let mut archive = ZipArchive::new(&zip_file).unwrap();
            let mut files: Vec<String> = Vec::new();
            for i in 0..archive.len() {
                let mut file = archive.by_index(i).unwrap();
                if file.is_dir() {
                    continue;
                }
                let mut content = String::new();
                // match fil.read_to_string(&mut content) {
                //     std::result::Result::Ok(_) => files.push(content),
                //     std::result::Result::Err(_) => {
                //         continue;
                //     }
                // }
                files.push(file.name().to_string());
            }
            return files;
        }

        fn merge(&self, data: RefCell<Option<Epub>>) {
            todo!()
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_unzip() {
        let mut epub = Epub::default();
        let path = Path::new("tests/fixtures/sample.epub");
        let result = &mut epub.unzip(path);
    }
}
