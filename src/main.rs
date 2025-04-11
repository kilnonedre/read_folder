use utils::{disk::get_sys_disk_msg, folder::get_msg_by_folder_name};

mod utils;

fn main() {
    get_msg_by_folder_name();
    get_sys_disk_msg();
    println!("Hello, world!");
}
