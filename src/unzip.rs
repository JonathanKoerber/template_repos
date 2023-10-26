use miniz_oxide::inflate::decompress_to_vec_zlib;
use std::fs;
use std::ffi::OsStr;
use std::path::Path;
use zip::result::ZipError;

pub fn unzip(dir_string: &str) {
    if let Ok(entries) = std::fs::read_dir(dir_string) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(extension) = path.extension().and_then(OsStr::to_str) {
                    if extension == "zip" {
                        // Process the zip file
                        println!("Found a zip file: {:?}", path);
                        if let Err(err) = extract_zip(&path) {
                            println!("Failed to extract zip file: {:?}", err);
                        }
                    }
                }
            }
        }
    } else {
        println!("Failed to read directory");
    }
}

fn extract_zip(zip_path: &Path) -> Result<(), ZipError> {
    let file = std::fs::File::open(zip_path)?;
    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = Path::new(".").join(file.name());

        if let Some(parent) = outpath.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        let mut outfile = fs::File::create(&outpath)?;
        std::io::copy(&mut file, &mut outfile)?;
    }

    Ok(())
}

