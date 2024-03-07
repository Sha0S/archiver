/*
Usage: archive.exe 

config file:
Contains lines, in the following format: 
{Source directory}|{Destination directory}(|{Mode})

The program archives {source directory} to {destination directory}.
Default mode is [OneArchive], where the whole source dir will be compressed to one archive.
Optional mode is [SubFolders], which is activated if the line ends in "|S".
In this mode each subdirectory will be converted to it's own archive, 
saved inside a subfolder of the {destination directory}, named after the current date.
*/

use chrono::Local;
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
                    ".tgz"
                );
                let destination_path = Path::new(&destination).join(Path::new(&filename));

                println!(
                    "Starting {} -> {}",
                    source_path.display(),
                    destination_path.display()
                );

                println!(
                    "\tRunning: tar -czf \"{}\" -C \"{}\" \".\"",
                    destination_path.to_str().unwrap(),
                    &source
                );

                let t = Instant::now();

                // Create the archive:
                let status = process::Command::new("tar")
                    .args([
                        "-czf",
                        destination_path.to_str().unwrap(),
                        "-C",
                        &source,
                        ".",
                    ])
                    .status()
                    .expect("failed to execute process");

                println!("\tStatus: {}", status);
                println!("Time elapsed: {} sec\n", t.elapsed().as_secs());
            }

            Mode::SubFolders => {
                // Check each subdir of source:
                for entry in fs::read_dir(source)? {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_dir() {
                        // Make subdir in destination dir named the current date
                        let final_destination = Path::new(&destination)
                            .join(Path::new(&format!("{}", Local::now().format("%y_%m_%d"))));

                        // If it does not exist then create it:
                        if !final_destination.exists() {
                            fs::create_dir_all(&final_destination)?;
                        }

                        // Create output filename:
                        let filename = format!(
                            "{}_{}{}",
                            path.file_name().unwrap().to_str().unwrap(), //this should not fail. It could only fail if it can't extract a filename from the Path.
                            Local::now().format("%y_%m_%d_%H_%M"),
                            ".tgz"
                        );
                        let destination_path = Path::new(&final_destination).join(Path::new(&filename));

                        println!(
                            "Starting {} -> {}",
                            path.display(),
                            destination_path.display()
                        );

                        println!(
                            "\tRunning: tar -czf \"{}\" -C \"{}\" \".\"",
                            destination_path.to_str().unwrap(),
                            path.to_str().unwrap()
                        );

                        let t = Instant::now();

                        // Create the archive:
                        let status = process::Command::new("tar")
                            .args([
                                "-czf",
                                destination_path.to_str().unwrap(),
                                "-C",
                                path.to_str().unwrap(),
                                ".",
                            ])
                            .status()
                            .expect("failed to execute process");

                        println!("\tStatus: {}", status);
                        println!("Time elapsed: {} sec\n", t.elapsed().as_secs());
                    }

                    // Should we care about singular files?
                }
            }
        }
    }

    Ok(())
}
