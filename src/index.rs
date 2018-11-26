// Error handling
use std::error::Error;

// Walk through dirs
use walkdir::WalkDir;

use std::sync::Arc;
use std::sync::Mutex;

// Filesystem watch
use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

// HashMap
use std::collections::HashMap;

pub type Index = HashMap<String, HashMap<String, String>>;

pub fn generate_index(path: String) -> Result<Index, Box<Error>> {
    let mut index = HashMap::new();
    for entry in WalkDir::new(&path).follow_links(true) {
        let entry = entry?;
        let real_path = entry.path();
        let file_path = real_path.strip_prefix(&path)?;
        if real_path.metadata().is_ok() {
            let mut file_info = HashMap::new();
            file_info.insert(
                "type".to_string(),
                if real_path.metadata()?.file_type().is_file() {
                    "File".to_string()
                } else {
                    "Directory".to_string()
                }
            );
            index.insert(
                file_path.to_str().unwrap().to_string(),
                file_info
            );
        } else {
            error!("Invalid path: {}", real_path.display());
        }
    }
    Ok(index)
}

pub fn start_watcher(path: &String, index: Arc<Mutex<Index>>) -> notify::Result<()> {
    let (tx, rx) = channel();
    let mut watcher = try!(watcher(tx, Duration::from_secs(10)));
    try!(watcher.watch(path.clone(), RecursiveMode::Recursive));
    info!("Launch prerequisites achieved.");
    loop {
        match rx.recv() {
            Ok(event) => {
                debug!("Watcher event: {:?}", event);
                let new_index = generate_index(path.clone()).unwrap();
                debug!("New index generated: {:?}", &new_index);
                let mut real_index = index.lock().unwrap();
                *real_index = new_index;
                debug!("Index replaced.");
            },
            Err(e) => println!("Watcher error: {:?}", e)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn list_files() {
        generate_index("./demo".to_string()).unwrap();
    }
}