use chrono::Local;
use std::env;
use std::fs;
use std::path::Path;
use std::process;
use std::time::Instant;

/*
Mode::OneArchive - compress the whole source directory to one archive (default)
Mode::SubFolders - compress each sub-folder into separate archive
*/
enum Mode {
    OneArchive,
    SubFolders,
}

// Return is a vector of (source_dir, destination_dir(, mode))
fn read_config() -> Vec<(String, String, Mode)> {
    let mut ret: Vec<(String, String, Mode)> = Vec::new();

    let p = Path::new(".\\config");
    if let Ok(fileb) = fs::read_to_string(p) {
        for line in fileb.lines() {
            if !line.is_empty() {
                let mut parts = line.split('|');

                ret.push((
                    parts
                        .next()
                        .expect("config file is formated incorrectly")
                        .to_owned(),
                    parts
                        .next()
                        .expect("config file is formated incorrectly")
                        .to_owned(),
                    match parts.next() {
                        Some(x) => {
                            if x == "S" {
                                Mode::SubFolders
                            } else {
                                // Could check for wrong arg here.
                                Mode::OneArchive
                            }
                        }
                        None => Mode::OneArchive,
                    },
                ));
            }
        }
    }

    ret
}

fn main() -> Result<(), std::io::Error> {
    let config = read_config();

    // default extension is .tar.zst
    // passing a arg to the program will overwrite this
    let args: Vec<String> = env::args().collect();
    let extension: String = {
        if let Some(s) = args.get(1) {
            s.to_owned()
        } else {
            ".tar.zst".to_owned()
        }
    };

    for (source, destination, mode) in config {
        // Sanitiy check:
        if !Path::new(&source).exists() {
            panic!("Source folder not found! {}", source);
        }

        // Create the destination dir, if it does not exist:
        if !Path::new(&destination).exists() {
            fs::create_dir_all(&destination)?;
        }

        let source_path = Path::new(&source);

        match mode {
            Mode::OneArchive => {
                // Create output filename:
                let filename = format!(
                    "{}_{}{}",
                    source_path.file_name().unwrap().to_str().unwrap(), //this should not fail. It could only fail if it can't extract a filename from the Path.
                    Local::now().format("%y_%m_%d_%H_%M"),
                    extension
                );
                let destination_path = Path::new(&destination).join(Path::new(&filename));

                println!(
                    "Starting {} -> {}",
                    source_path.display(),
                    destination_path.display()
                );
                let t = Instant::now();

                // Create the archive:
                let status = process::Command::new("tar")
                    .args([
                        "-ca",
                        "-f",
                        destination_path.to_str().unwrap(),
                        "-C",
                        &source,
                        ".",
                    ])
                    .status()
                    .expect("failed to execute process");

                println!("Status: {}", status);
                println!("Time elapsed: {} sec", t.elapsed().as_secs());
            }

            Mode::SubFolders => {
                for entry in fs::read_dir(source)? {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_dir() {
                        // Create output filename:
                        let filename = format!(
                            "{}_{}{}",
                            path.file_name().unwrap().to_str().unwrap(), //this should not fail. It could only fail if it can't extract a filename from the Path.
                            Local::now().format("%y_%m_%d_%H_%M"),
                            extension
                        );
                        let destination_path = Path::new(&destination).join(Path::new(&filename));

                        println!(
                            "Starting {} -> {}",
                            path.display(),
                            destination_path.display()
                        );
                        let t = Instant::now();

                        // Create the archive:
                        let status = process::Command::new("tar")
                            .args([
                                "-ca",
                                "-f",
                                destination_path.to_str().unwrap(),
                                "-C",
                                path.to_str().unwrap(),
                                ".",
                            ])
                            .status()
                            .expect("failed to execute process");

                        println!("Status: {}", status);
                        println!("Time elapsed: {} sec", t.elapsed().as_secs());
                    }

                    // Should we care about singular files?
                }
            }
        }
    }

    Ok(())
}
