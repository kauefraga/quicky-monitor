use sysinfo::{System, SystemExt};

pub fn get_os(system: &System) -> String {
  system.os_version()
    .unwrap()
    .to_lowercase()
}

pub fn get_kernel(system: &System) -> String {
  system.kernel_version()
   .unwrap()
   .to_lowercase()
}
