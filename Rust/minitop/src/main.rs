use std::time::Duration;
use std::{f64, thread};
use sysinfo::{Disks, System};

fn main() {
    // We create the object wich represents our pc
    let mut sys: System = System::new_all();
    let mut disks = Disks::new_with_refreshed_list();

    // Infinity loop to keep refreshing the resources in real time
    loop {
        // Analize the resources now
        sys.refresh_all();
        disks.refresh(true);

        // Clean the screen (terminal)
        print!("\x1B[2J\x1B[1;1H");

        // Get the OS name
        let os_name = System::name().unwrap_or(String::from("Unknown"));

        // Converto to f64 (float) and divide by 1024.0 (also float)
        let ram_total: f64 = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
        let ram_used: f64 = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;

        let cpu_usage = sys.global_cpu_usage();
        let cpu_cores = sys.cpus().len();

        println!("--------- MINITOP ---------");
        println!("SO: {}", os_name);
        println!("RAM: {:.1}/{:.1} GBs", ram_used, ram_total);
        println!("CPU: {:.1}% {} cores", cpu_usage, cpu_cores);
        println!("---------------------------");
        println!("Storage");

        for disk in disks.list() {
            let total: f64 = disk.total_space() as f64 / 1024.0 / 1024.0 / 1024.0;
            let avaiable: f64 = disk.available_space() as f64 / 1024.0 / 1024.0 / 1024.0;
            let used: f64 = total - avaiable;

            println!("{:?} {:.1}/{:.1} GBs", disk.name(), used, total);
        }

        //println!("============================");
        println!("");
        println!("Ctrl + C to exit");

        thread::sleep(Duration::from_secs(1));
    }
}
