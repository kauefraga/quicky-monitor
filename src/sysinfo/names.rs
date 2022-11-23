use sysinfo::{System, SystemExt};

pub fn get_name(system: &System) -> String {
  system.name()
    .expect("Get the name of the system")
    .to_lowercase()
}

pub fn get_hostname(system: &System) -> String {
  system.host_name()
   .expect("Get the hostname of the system")
   .to_lowercase()
}
