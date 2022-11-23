use sysinfo::{System, SystemExt};

pub fn get_cpu_count(system: &System) -> usize {
  system.cpus().len()
}
