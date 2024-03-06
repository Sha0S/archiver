use chrono::Local;
use std::fs;
use std::env;
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

    let args: Vec<String> = env::args().collect();
    let extension: String = {
        if let Some(s) = args.get(1) {
            s.to_owned()
        } else {
            ".tar.zst".to_owned()
        }
    };


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
            "{}_{}{}",
            source_path.file_name().unwrap().to_str().unwrap(), //this should not fail. It could only fail if it can't extract a filename from the Path.
            Local::now().format("%y_%m_%d_%H_%M"),
            extension
        );
        let destination_path = Path::new(&destination).join(Path::new(&filename));

        println!("Starting {} -> {}", source_path.display(), destination_path.display());
        let t = Instant::now();

        // Create the archive:
        let status = process::Command::new("tar")
            .args([
                "-ca",
                "-f",
                destination_path.to_str().unwrap(),
                "-C",
                &source,
                "."
            ])
            .status()
            .expect("failed to execute process");

        println!("Status: {}", status);
        println!("Time elapsed: {} sec", t.elapsed().as_secs());
    }

    Ok(())
}
