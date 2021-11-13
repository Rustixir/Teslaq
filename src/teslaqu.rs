pub mod teslaqu {
    use std::{collections::HashMap, fs, path::Path};

    mod storage;
    use storage::storage::Storage; 


    pub enum Error {
        ExistVNode,
        NotFound,
        FailedToCreateDirectory,
        FailedToCreateFile
    }



    pub struct TeslaQ {
        vnodes : HashMap<String, Storage>,
        root   : String 
    }
    impl TeslaQ {
        pub fn open() -> TeslaQ {
            if Path::new("disk").is_dir() {
                fs::remove_dir_all("disk").unwrap();
            }
            fs::create_dir("disk").unwrap();
            TeslaQ {
                vnodes : HashMap::<String, Storage>::new(),
                root   : "disk".to_owned()
            }
        }


        pub fn create_vnode(&mut self, vn_name: String, total_disk_size: usize, buffer_capacity: usize) -> Result<(), Error> {
            let path = format!("{}/{}", self.root, vn_name);
            match Path::new(&path).is_dir() {
                true => Err(Error::ExistVNode),
                false => {
                    let res = fs::create_dir(&path);
                    match res {
                        Err(_) => Err(Error::FailedToCreateDirectory),
                        Ok(_) => {
                            // maked directory for this vnode
                            let storage = Storage::open(path, total_disk_size, buffer_capacity);
                            match storage {
                                Err(_) => Err(Error::FailedToCreateFile),
                                Ok(storage) => {
                                    self.vnodes.insert(vn_name, storage);
                                    Ok(())
                                }
                            }   
                        }
                    }
                }
            }
            
        }

        pub fn remove_vnode(&mut self, vn_name: String) -> Result<(), ()>{
            match self.vnodes.remove(&vn_name) {
                Some(_) => Ok(()),
                None => Err(()),
            }
        }

        pub fn produce_event(&mut self, vn_name: String, mut event: String) -> Result<(), Error> {
            match self.vnodes.get_mut(&vn_name) {
                None => Err(Error::NotFound),
                Some(vnode) => {
                    vnode.push(&mut event);
                    Ok(())
                }
            }
        }

        pub fn demand_events(&mut self, vn_name: String, num: usize) -> Result<Vec<String>, ()> {
            match self.vnodes.get_mut(&vn_name) {
                None => Err(()),
                Some(storage) => {
                    Ok(storage.demand(num))
                }
            }
        }


    }
    



}