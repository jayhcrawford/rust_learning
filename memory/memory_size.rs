use num_format::{Locale, ToFormattedString};
use sysinfo::System;

fn memory_size() {
    let mut sys = System::new_all();
    sys.refresh_memory();

    // Values are in KiB
    println!(
        "Available memory: {} i32",
        (sys.available_memory() / 256).to_formatted_string(&Locale::en)
    );
}
