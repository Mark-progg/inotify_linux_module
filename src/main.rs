extern crate inotify;
use std::env;
use inotify::{
    EventMask,
    WatchMask,
    Inotify,
};
use std::time;
use std::io::prelude::*;
use std::fs::File;
/*
struct inotify_event {
               int      wd;       /* Watch descriptor */
               uint32_t mask;     /* Mask describing event */
               uint32_t cookie;   /* Unique cookie associating related
                                     events (for rename(2)) */
               uint32_t len;      /* Size of name field */
               char     name[];   /* Optional null-terminated name */
           };
*/
fn main() {
    let mut inotify = Inotify::init()
        .expect("Failed to initialize inotify");
    let current_dir = env::current_dir()
        .expect("Failed to determine current directory");
    inotify.add_watch(
            current_dir,
            WatchMask::MODIFY | WatchMask::CREATE,
        )
        .expect("Failed to add inotify watch");
    println!("Watching current directory for activity...");
    let mut buffer = [0u8; 4096];
    loop {
        let events = inotify
            .read_events_blocking(&mut buffer)
            .expect("Failed to read inotify events");
        for event in events {
            if (event.mask.contains(EventMask::MODIFY) ||
            event.mask.contains(EventMask::CREATE)
            ) &&
            !event.mask.contains(EventMask::ISDIR) { 
                    std::thread::sleep(time::Duration::from_millis(20));
                    let mut key_buffer = [0u8; 32];
                    println!("File modified: {:?}", event.name);
                    let mut f = File::open("X").unwrap();
                    f.read(&mut key_buffer).unwrap();
                    println!("The key_buffer: {:?}", &key_buffer);
            }
        }
    }
}
