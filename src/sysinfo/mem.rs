use std::{thread, time};
use sysinfo::{System, SystemExt};

pub fn get_ram(system: &mut System) -> u64 {
  system.refresh_memory();

  // total_memory
  // used_memory
  // total_swap
  // used_swap

  system.total_memory()
}

pub fn watch_ram(system: &mut System) {
  loop {
    system.refresh_memory();

    println!("{} bytes", system.used_memory());

    // Sleeping for 500 ms to let time for the system to run for long
    // enough to have useful information.
    thread::sleep(time::Duration::from_millis(500));
  }
}
