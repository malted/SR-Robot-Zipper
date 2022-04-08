use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use zip::write::FileOptions;

fn main() {
    let paths = fs::read_dir("./").unwrap();

    let mut python_files = Vec::new();
    for path in paths {
        let file_name = path.as_ref().unwrap().path().display().to_string();
        if *&file_name.ends_with(".py") {
            python_files.push(path);
        }
    }

    if python_files.len() == 0 {
        panic!("There are no python files in the current directory.");
    }

    if !python_files.iter().any(|i| i.as_ref().unwrap().path().display().to_string()=="./robot.py") {
        panic!("The robot's code's entry point should be contained in a file called robot.py, which can not be found in the current directory.");
    }

    zip(python_files).unwrap();
}

fn zip(py_files: Vec<Result<fs::DirEntry, std::io::Error>>) -> zip::result::ZipResult<()> {
    let zip_path = Path::new("robot.zip");
    let zip_file = File::create(&zip_path)?;

    let mut zip = zip::ZipWriter::new(zip_file);
    
    let zip_options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755);

    for py in py_files {
        let file_name = py.as_ref().unwrap().path().display().to_string();
        
        let mut py_file = File::open(&file_name)?;
        let mut py_file_data = Vec::new();
        py_file.read_to_end(&mut py_file_data)?;

        zip.start_file(&file_name, zip_options)?;
        zip.write_all(&py_file_data)?;
    }

    zip.finish()?;
    
    Ok(())
}