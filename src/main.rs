
use std::fs;
use std::io;
use std::path::PathBuf;

struct MyFile {
    path: String,
    size: u64,
}

fn dir_size(path: &PathBuf) -> io::Result<u64> {
    let mut subdirs = fs::read_dir(path)?;
    subdirs.try_fold(0, |acc, file| {
        let file = file?;
        let size = match file.metadata()? {
            data if data.is_dir() => dir_size(&file.path())?,
            data => data.len(),
        };
        Ok(acc + size)
    })
}

static THOUSAND: f64 = 1_000.0;
static MILLION: f64 = 1_000_000.0;
static BILLION: f64 = 1_000_000_000.0;

fn convert_bytes(b: u64) -> String {
    let b = b as f64;
    if b < THOUSAND {
        return format!("{:.3} bytes", b);
    } else if b < MILLION {
        return format!("{:.3} KB", b / THOUSAND);
    } else if b < BILLION {
        return format!("{:.3} MB", b / MILLION);
    }
    format!("{:.3} GB", b / BILLION)
}

fn main() -> io::Result<()> {
    let path = std::env::args().nth(1).unwrap_or(String::from("."));
    let read_dir = fs::read_dir(path)?;
    let mut files: Vec<MyFile> = read_dir.map(|subdir| {
        let subdir = subdir.unwrap();
        let path = subdir.path();
        let display = path.display().to_string();
        let data = subdir.metadata().unwrap();
        let size = if data.is_dir() { 
            dir_size(&path).unwrap()
        } else { 
            data.len() 
        };
        MyFile { 
            path: display, 
            size: size, 
        }
    }).collect();

    files.sort_by_key(|file| file.size);

    for file in files {
        let converted_size = convert_bytes(file.size);
        println!("{} - {}", file.path, converted_size);
    }

    Ok(())
}