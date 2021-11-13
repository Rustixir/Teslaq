pub mod storage {
    use std::collections::VecDeque;

    mod disk_queue;
    use disk_queue::disk_queue::DiskQueue;
    

    pub struct Storage {
        diskq  : DiskQueue,
        bufq   : VecDeque<String>,
        used   : usize,
        bufcap : usize
    }
    impl Storage {
        pub fn open(path: String, total_disk_size: usize, buffer_capacity: usize) -> Result<Storage, ()> {
            let diskq = DiskQueue::open(path, total_disk_size).unwrap();
            let bm = Storage {
                diskq  : diskq, 
                bufq   : VecDeque::with_capacity(buffer_capacity),
                used    : 0,
                bufcap : buffer_capacity
            };
            Ok(bm)
        }

        pub fn push(&mut self, event: &mut String) {
            self.diskq.push(event);
        }


        pub fn demand(&mut self, num: usize) -> Vec<String> {
            self.fill_buffer();
            let mut v = vec![];
            for _ in 0..num {
                let res = self.bufq.pop_front();
                match res {
                    Some(event) => v.push(event),
                    None => break,
                }
            };
            self.used -= v.len();
            v
        }

        // ===================================================
        // # Private Method

        fn fill_buffer(&mut self) {
            match self.calc_buff() {
                Some(demand) => {
                    let events = self.diskq.demand(demand);
                    self.used += events.len();
                    events
                        .into_iter()
                        .for_each(|event| self.bufq.push_back(event));
                }
                None => ()
            }
        }

        fn calc_buff(&self) -> Option<usize> {
            let free = self.bufcap - self.used;
            if free > self.used {
                Some(free)
            
            } else {
                None
            }
        }
        
    }
}