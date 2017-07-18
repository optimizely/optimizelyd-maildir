use std::*;
use std::collections::HashSet;
use std::fs::*;
use std::path::Path;
use std::time::SystemTime;
use std::fs::File;
use std::io::BufReader;
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
        if let Ok(mut f) = File::create(&tmp_path) {
            f.write_all(requestBody.as_bytes());

            f.sync_data();
        }

        fs::rename(Path::new(&tmp_path), Path::new(&new_path)); // Rename from tmp to new
        
        true
    }

    pub fn pop(&self, callback:&Fn(&str) -> bool) -> bool {
         let new_path = self.baseDir.clone().as_str().to_owned() + "/" + NEW;
         if let Ok(mut entries) = fs::read_dir(new_path) {
             if let Some(entry) = entries.next() {
                   let entry = entry.unwrap();
                   let file_name = entry.file_name();
                   let cur_path = self.baseDir.clone().as_str().to_owned() + "/" + CURRENT + "/" + file_name.to_str().unwrap();
                   fs::rename(entry.path(), Path::new(&cur_path)); // Rename from new to cur directory
                
                   if let Ok(file) = File::open(&cur_path) {
                        let mut buf_reader = BufReader::new(file);
                        let mut contents = String::new();
                        buf_reader.read_to_string(&mut contents);
                        if callback(contents.as_str()) {
                            fs::remove_file(&cur_path);
                            return true;
                        }
                   }

             }
         }
         return false;
    }

}
