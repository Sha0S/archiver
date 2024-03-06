/*
.\config: contains source|destination directory pairs

Flow:
- process the config file
- for each of the pairs:
    - compress the source directory to a ".tar.gz" archive.
    - save the archive in the destination directory
    - the filename will be: SOURCE_YY_MM_DD_HH_MM.tar.gz
    - we use ".tar.gz" to preserve hardlinks. (Usefull for the ICT.)

Q:
- How do we want to handle shared folders?
    - Each machine doing it's own archive sounds sub-optimal, the machine accessing it ower the network
      would take ages to finish, and we would have to handle situations where the shared folder is not accesible.
    - We could run it only on the owner of the directory, and use a different process to copy them to the other PCs.
- Do we want to automatically delete older entries to free up space?
- Logging?
*/

use chrono::Local;
use std::fs;
use std::path::Path;
use std::process;
use std::time::Instant;

// Return is a vector of (source_dir, destination_dir)
fn read_config() -> Vec<(String, String)> {
    let mut ret: Vec<(String, String)> = Vec::new();

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
                ));
            }
        }
    }

    ret
}

fn main() -> Result<(), std::io::Error> {
    let config = read_config();

    for (source, destination) in config {
        // Sanitiy check:
        if !Path::new(&source).exists() {
            panic!("Source folder not found! {}", source);
        }

        // Create the destination dir, if it does not exist:
        if !Path::new(&destination).exists() {
            fs::create_dir_all(&destination)?;
        }

        // Create output filename:
        let source_path = Path::new(&source);
        let filename = format!(
            "{}_{}.tar.gz",
            source_path.file_name().unwrap().to_str().unwrap(), //this should not fail. It could only fail if it can't extract a filename from the Path.
            Local::now().format("%y_%m_%d_%H_%M")
        );
        let destination_path = Path::new(&destination).join(Path::new(&filename));

        println!("Starting {:?} -> {:?}", source_path, destination_path);
        let t = Instant::now();

        // Create the archive:
        let status = process::Command::new("tar")
            .args([
                "-cf",
                destination_path.to_str().unwrap(),
                source_path.to_str().unwrap(),
            ])
            .status()
            .expect("failed to execute process");

        println!("Status: {}", status);
        println!("Time elapsed: {} sec", t.elapsed().as_secs());
    }

    Ok(())
}
