use num_format::{Locale, ToFormattedString};
use sysinfo::System;

fn main() {
    memory_size();
}

fn memory_size() {
    let mut sys = System::new_all();
    sys.refresh_memory();

    println!(
        "Your system can allocate {} i32 elements",
        (sys.available_memory() / 4).to_formatted_string(&Locale::en)
    );

    let system_memory = sys.total_memory() as f64;
    const GIG: f64 = 1_073_741_824.0;

    let total_system_mem_gigs = system_memory / GIG;

    println!("You have {:.2} Gigs of RAM", total_system_mem_gigs);
}
