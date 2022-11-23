use sysinfo::{System, SystemExt};

pub fn get_os(system: &System) -> String {
  system.os_version()
    .expect("Get the os version")
    .to_lowercase()
}

pub fn get_kernel(system: &System) -> String {
  system.kernel_version()
   .expect("Get the kernel version")
   .to_lowercase()
}
