use std::*;
use std::collections::HashSet;
use std::fs::*;
use std::path::Path;
use std::time::SystemTime;
use std::fs::File;
use std::io::prelude::*;

static CURRENT: &'static str = "cur";
static TEMP: &'static str = "tmp";
static NEW: &'static str = "new";
const VALIDDIRS: [&'static str; 3] = ["tmp", "new", "cur"]; // or ["Apples", "Oranges"]
pub struct MaildirQueue {
    baseDir:String,
}

impl MaildirQueue {
    pub fn new(dir:String) -> MaildirQueue {
        MaildirQueue {
            baseDir : dir
        }
    }

    pub fn init(&self) -> Option<&MaildirQueue> {
        let mut maildirCount = 0;
        let mut set = HashSet::new();
        let baseDir = self.baseDir.clone();
        if let Ok(entries) = fs::read_dir(baseDir) {
            for entry in entries {
                    maildirCount += 1;
                    if let Ok(entry) = entry {       
                        if let Ok(file_type) = entry.file_type() {
                           let file_name = entry.file_name();
                           println!("{:?}", file_name);
                           if file_type.is_dir() == false {
                               continue;
                           }
                           if let Some(e) = VALIDDIRS.iter().filter(|&&x| Some(x) == file_name.to_str()).next() {
                               set.insert(e.clone());
                            }
                        }
                    }
            }

            if set.len() != 3 {
                if maildirCount > set.len() {
                    println!("Directory should not have extra entries in it!");
                    return None
                }
                // create appropriate folders
                for dirName in VALIDDIRS.iter() {
                    if !set.contains(dirName) {
                        let new_path = self.baseDir.clone().as_str().to_owned() + "/" + dirName;
                        DirBuilder::new()
                            .recursive(true)
                                .create(new_path).unwrap();
                    }
                }
            }
        }
        return Some(&self)
    }

    pub fn push(&self, requestBody:&str) -> bool {
        let filename = format!("{:?}", SystemTime::now());
        let tmp_path = self.baseDir.clone().as_str().to_owned() + "/" + TEMP + "/" + filename.as_str();
        let new_path = self.baseDir.clone().as_str().to_owned() + "/" + NEW + "/" + filename.as_str();
        let mut f = File::create(&tmp_path).expect("Couldn’t open file");
        f.write_all(requestBody.as_bytes()).expect("Couldn’t write body ");

        f.sync_data().expect("Couldn’t sync");

        fs::rename(Path::new(&tmp_path), Path::new(&new_path)); // Rename a.txt to b.txt
        
        true
    }

    pub fn pop(&self) -> &str {
        "test"
    }
}
