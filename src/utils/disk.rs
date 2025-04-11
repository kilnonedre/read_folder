use std::{fs, path::Path};

use sysinfo::{Disks, System};

pub fn get_sys_disk_msg() {
    let mut sys = System::new_all(); // 获取所有系统信息
    let disks = Disks::new_with_refreshed_list();

    for disk in &disks {
        println!("磁盘名称: {:?}", disk.name());
        println!("挂载点: {:?}", disk.mount_point());
        println!("总空间: {} GB", disk.total_space() / 1_073_741_824); // 转 GB
        println!("可用空间: {} GB", disk.available_space() / 1_073_741_824);
        println!("文件系统类型: {:?}", disk.file_system());
        println!("--------------------------------");
        let mount_path = disk.mount_point();
        list_files(mount_path);
    }
}

fn list_files(path: &Path) {
    println!("列出路径 {:?} 下的内容:", path);

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                println!("📁 文件夹: {:?}", path);
            } else if path.is_file() {
                println!("📄 文件: {:?}", path);
            }
        }
    } else {
        println!("❌ 无法读取 {:?}", path);
    }

    println!("------------------------------");
}
