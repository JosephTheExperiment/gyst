use crate::pub_struct;

pub_struct! {
    struct TextFile {
        name: String,
        data: String
    }
}

pub_struct! {
    struct Directory {
        name: String,
        files: Vec<TextFile>,
        directories: Vec<Directory>
    }
}
