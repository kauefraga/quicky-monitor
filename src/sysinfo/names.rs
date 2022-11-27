use sysinfo::{System, SystemExt};

pub fn get_name(system: &System) -> String {
  system.name()
    .unwrap()
    .to_lowercase()
}

pub fn get_hostname(system: &System) -> String {
  system.host_name()
   .unwrap()
   .to_lowercase()
}
