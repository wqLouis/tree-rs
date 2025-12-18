use clap::Parser;

#[derive(serde::Serialize)]
struct File {
    file_name: String,
    sub_files: Option<Vec<File>>,
}

/// Tree but rust
#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Output as json
    #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue)]
    json: bool,

    #[arg(default_value = ".")]
    root: String,
}

fn tree(root: &std::path::Path) -> File {
    fn index(file: &mut File, parent_path: &std::path::Path) {
        let mut entries = match std::fs::read_dir(parent_path) {
            Ok(entries) => entries,
            Err(_) => {
                file.sub_files = None;
                return;
            }
        };

        let mut file_list: Vec<File> = Vec::new();

        loop {
            let entry = match entries.next() {
                Some(entry) => match entry {
                    Ok(entry) => entry,
                    Err(_) => break,
                },
                None => break,
            };

            let mut sub_file = File {
                file_name: entry.file_name().to_str().unwrap_or_default().to_string(),
                sub_files: None,
            };

            let sub_file_name = sub_file.file_name.clone();

            index(&mut sub_file, parent_path.join(sub_file_name).as_path());

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
    );

    root_file
}

/// format tree string with File struct
fn format_tree(root: &File) {}

fn main() {
    let args = Args::parse();

    let file = tree(std::path::Path::new(&args.root));
    if args.json {
        print!("{}", serde_json::to_string_pretty(&file).unwrap());
    }
}
