use std::*;
use std::collections::HashSet;
use std::fs::*;
use std::path::Path;
use std::time::SystemTime;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

static TEMP: &'static str = "tmp"; // file is opened and written to here.
static NEW: &'static str = "new"; // next it is moved onto the queue by being moved to new
static CURRENT: &'static str = "cur"; // the pop takes a file from new and moves it to cur, then it opens the file in cur and excutes the closure. If the move failed the pop should just move to the next file. this allowing for multiple queue consumers.
static DOA: &'static str = "doa"; // dead on arrival, this can be used to move messages that have been popped too many times.
static RES: &'static str = "res"; // using the queue for a request/response area, give the user a place to move responses.
const VALIDDIRS: [&'static str; 5] = ["tmp", "new", "cur", "doa", "res"]; // or ["Apples", "Oranges"]
pub struct MaildirQueue {
    base_dir:String,
}

impl MaildirQueue {
    pub fn new(dir:String) -> MaildirQueue {
        MaildirQueue {
            base_dir : dir
        }
    }

    pub fn init(&self) -> Option<&MaildirQueue> {
        let mut maildir_count = 0;
        let mut set = HashSet::new();
        let base_dir = self.base_dir.clone();
        if let Ok(entries) = fs::read_dir(base_dir) {
            for entry in entries {
                    maildir_count += 1;
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

            if set.len() != VALIDDIRS.len() {
                if maildir_count > set.len() {
                    println!("Directory should not have extra entries in it!");
                    return None
                }
                // create appropriate folders
                for dir_name in VALIDDIRS.iter() {
                    if !set.contains(dir_name) {
                        let new_path = self.base_dir.clone().as_str().to_owned() + "/" + dir_name;
                        DirBuilder::new()
                            .recursive(true)
                                .create(new_path).unwrap();
                    }
                }
            }
        }
        return Some(&self)
    }

    fn moveFile(&self, request_body:&str, from:&str, to:&str, suffix:Option<&str>) -> bool {
        let filename = format!("{:?}", SystemTime::now()) + if let Some(suf) = suffix { suf } else { "" };
        let tmp_path = self.base_dir.clone().as_str().to_owned() + "/" + from + "/" + filename.as_str();
        let new_path = self.base_dir.clone().as_str().to_owned() + "/" + to + "/" + filename.as_str();
        if let Ok(mut f) = File::create(&tmp_path) {
            f.write_all(request_body.as_bytes());

            f.sync_data();
        }

        fs::rename(Path::new(&tmp_path), Path::new(&new_path)); // Rename from tmp to new
        
        true
    }
    pub fn push(&self, request_body:&str) -> bool {
       
        return self.moveFile(request_body, TEMP, NEW, None);
    }

    pub fn doa(&self, request_body:&str) -> bool {
        return self.moveFile(request_body, TEMP, DOA, None);
    }

    pub fn res(&self, request_body:&str) -> bool {
        return self.moveFile(request_body, TEMP, RES, None);
    }

    pub fn pop(&self, callback:&Fn(&str) -> bool) -> bool {
         let new_path = self.base_dir.clone().as_str().to_owned() + "/" + NEW;
         if let Ok(mut entries) = fs::read_dir(new_path) {
             if let Some(entry) = entries.next() {
                   let entry = entry.unwrap();
                   let file_name = entry.file_name();
                   let cur_path = self.base_dir.clone().as_str().to_owned() + "/" + CURRENT + "/" + file_name.to_str().unwrap();
                   fs::rename(entry.path(), Path::new(&cur_path)); // Rename from new to cur directory
                
                   if let Ok(file) = File::open(&cur_path) {
                        let mut buf_reader = BufReader::new(file);
                        let mut contents = String::new();
                        buf_reader.read_to_string(&mut contents);
                        if callback(contents.as_str()) {
                            fs::remove_file(&cur_path);
                            return true;
                        }
                        else {
                            fs::remove_file(&cur_path);
                            let fname = file_name.to_str().unwrap();

                            if fname.ends_with("count3") {
                                self.doa(contents.as_str());
                            }
                            else if fname.ends_with("count2") {
                                self.moveFile(contents.as_str(), TEMP, NEW, Some("count3"));
                            }
                            else if fname.ends_with("count1") {
                                self.moveFile(contents.as_str(), TEMP, NEW, Some("count2"));
                            }
                            else {
                                self.moveFile(contents.as_str(), TEMP, NEW, Some("count1"));
                            }
                        }
                   }

             }
         }
         return false;
    }

}
