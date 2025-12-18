struct File {
    file_name: String,
    sub_files: Option<Vec<File>>,
}

async fn indexing(root: &std::path::Path) -> File {
    async fn index(file: &mut File, parent_path: &std::path::Path) {
        use std::path::Path;

        if !(Path::try_exists(&parent_path).unwrap_or(false)) {
            return;
        }

        let mut entries = match std::fs::read_dir(parent_path) {
            Ok(entries) => entries,
            Err(_) => {
                file.sub_files = None;
                return;
            }
        };

        let mut file_list: Vec<File> = Vec::new();

        while let entry = entries.next().unwrap().unwrap() {
            let mut sub_file = File {
                file_name: entry.file_name().to_str().unwrap_or_default().to_string(),
                sub_files: None,
            };
            let sub_file_name = sub_file.file_name.clone();

            Box::pin(index(
                &mut sub_file,
                parent_path.join(sub_file_name).as_path(),
            ))
            .await;

            file_list.push(sub_file);
        }
        file.sub_files = Some(file_list);
    }

    let mut root_file = File {
        file_name: root.to_str().unwrap().to_string(),
        sub_files: None,
    };
    let root_file_name = root_file.file_name.clone();

    index(
        &mut root_file,
        std::path::Path::new(&std::path::Path::new(&root_file_name)),
    )
    .await;

    root_file
}

fn main() {
    let _ = indexing(std::path::Path::new("/"));
}
