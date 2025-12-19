use clap::Parser;

#[derive(serde::Serialize)]
struct File {
    file_name: String,
    file_type: String,
    sub_files: Option<Vec<File>>,
}

/// Tree but rust
/// Default output as yaml
#[derive(clap::Parser)]
#[command(version, about, long_about = None, verbatim_doc_comment)]
struct Args {
    /// Output as json
    #[arg(short = 'j', long = "json", action = clap::ArgAction::SetTrue)]
    json: bool,

    #[arg(default_value = ".")]
    root: String,
}

fn tree(root: &std::path::Path) -> File {
    fn index(file: &mut File, parent_path: &std::path::Path) {
        if file.file_type != ".dir" {
            return;
        }

        let entries = match std::fs::read_dir(parent_path) {
            Ok(entries) => entries,
            Err(_) => {
                // prevent permission deny
                return;
            }
        };

        file.sub_files = Some(
            entries
                .filter_map(|entry| {
                    let entry = entry.ok()?;
                    let file_name = entry.file_name();
                    let file_type = entry.file_type().ok()?;

                    let mut file_tree = File {
                        file_name: file_name.clone().into_string().ok()?,
                        file_type: std::path::Path::new(&file_name)
                            .extension()
                            .unwrap_or_else(|| {
                                &std::ffi::OsStr::new(if file_type.is_dir() {
                                    ".dir"
                                } else if file_type.is_file() {
                                    ".file"
                                } else {
                                    "None"
                                })
                            })
                            .to_str()
                            .unwrap_or_default()
                            .to_string(),
                        sub_files: None,
                    };
                    if !file_type.is_symlink() {
                        index(
                            &mut file_tree,
                            &parent_path.join(std::path::Path::new(&file_name)),
                        );
                    } else {
                        file_tree.file_type = ".symlink".to_string();
                    }

                    Some(file_tree)
                })
                .collect(),
        );
    }

    let root_file_name = root.to_str().unwrap().to_string();

    let mut root_file = File {
        file_name: root_file_name.clone(),
        file_type: ".dir".to_string(),
        sub_files: None,
    };

    index(
        &mut root_file,
        std::path::Path::new(&std::path::Path::new(&root_file_name)),
    );

    root_file
}

fn main() {
    let args = Args::parse();

    let file = tree(std::path::Path::new(&args.root));
    if args.json {
        print!("{}", serde_json::to_string_pretty(&file).unwrap());
    } else {
        print!("{}", serde_yaml::to_string(&file).unwrap());
    }
}
