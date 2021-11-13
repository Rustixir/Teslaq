pub mod disk_queue {
    use std::{fs::{File, OpenOptions}, io::{BufRead, BufReader, Error, Read, Write}};


    // 20MB RAM  => for 50_000 request per second is enough 
    // 1GB  HHD  => for Persistent
    // ???  Buf  => for troughout and minimize latency (more is better)
    // 1 Core   

    pub struct DiskQueue {
        id: i32,
        writer : File,
        reader : BufReader<File>,
        path: String,

        total_disk_size: usize,
        used_disk_size: usize,

    }
    impl DiskQueue {
        pub fn open(mut path: String, total_disk_size: usize) -> Result<DiskQueue, Error> {
            path.push_str("disk_queue_1.log"); 
            
            let writer = OpenOptions::new().append(true).create(true).open(&path)?;
            let reader = BufReader::new(File::open(&path)?);

            Ok(DiskQueue {
                id: 1,
                writer,
                reader,
                path,
                total_disk_size,
                used_disk_size: 0,
            })
        }


        pub fn push(&mut self, event: &mut String) {
            event.push_str(" \n");
            let bytes = event.as_bytes();
            self.find_avail_disk(bytes.len());
            let _ = self.writer.write_all(bytes);
        }

     
        // demand.num must be biger than 0
        pub fn demand(&mut self, num: usize) -> Vec<String> {
           self.reader
                .by_ref()
                .lines()
                .take(num)
                .filter_map(|res| {
                    if let Ok(l) = res {
                        Some(l)
                    } else {
                        None
                    }
                }).collect::<Vec<_>>()
        }
 

        // ======================================
        // Private Method
        
        fn use_disk(&mut self, num: usize) {
            self.used_disk_size += num;
        }

        fn find_avail_disk(&mut self, len: usize) {
            let free_disk = self.total_disk_size - self.used_disk_size;
            if free_disk >= len {
                // write
                self.use_disk(len);
            
            } else {    
                // truncate
                self.truncate_disk();
            }

        }


        fn truncate_disk(&mut self) {   
            let old_id = self.id;
            self.id += 1;
            let new_id = self.id;


            let old_path = format!("{}disk_queue_{}.log", &self.path, old_id);
            let new_path = format!("{}disk_queue_{}.log", &self.path, new_id); 

            let new_file = OpenOptions::new()
                                .append(true)
                                .create(true)
                                .open(&new_path).unwrap();

            let _ = std::fs::remove_file(old_path);
            self.writer = new_file;
            self.used_disk_size = 0;



        }  

    }


    

}