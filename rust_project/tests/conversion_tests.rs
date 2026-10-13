use std::error::Error;
use std::fs;
use std::fs::File;
use std::sync::{Mutex, MutexGuard};
use bson::Document;

static FILE_LOCK: Mutex<()> = Mutex::new(());

    fn lock() -> MutexGuard<'static, ()> {
        FILE_LOCK.lock().unwrap_or_else(|e| e.into_inner())
    }

    #[test]
    fn print_folder_contents_to_terminal() {
        let entries = fs::read_dir("./TestFiles/.").expect("The folder is completly empty");

        println!("\n Folder contents: ");

        let mut fajlok = Vec::new();

        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    println!(" - {}", file_name);
                    fajlok.push(file_name.to_string());
                }
            }
        }
    }

    #[test]
    fn test_tanulo_bson_open_and_read() -> Result<(), Box<dyn Error>> {
        let _lock = lock();
        let file = File::open("./TestFiles/tanulo.bson").map_err(|e| {
            format!(
                "tanulo.bson could not be opened (working directory: {:?}): {e}",
                std::env::current_dir()
            )
        })?;

        let doc = Document::from_reader(file)?;
        assert_eq!(doc.get_str("nev")?, "Kovács Anna");
        assert_eq!(doc.get_i32("kor")?, 16);
        Ok(())
    }
