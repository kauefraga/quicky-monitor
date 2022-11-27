use termion::{color, style};
use sysinfo::{System, SystemExt};
use quicky_monitor::sysinfo::{
  get_name, get_hostname,
  get_cpu_count,
  watch_ram,
  get_kernel, get_os
};

fn main() {
  let mut sys = System::new_all();

  println!("{}You are welcome, {}@{}",
    color::Fg(color::Green),
    get_hostname(&sys),
    get_name(&sys)
  );

  println!("Kernel version: {}", get_kernel(&sys));
  println!("OS version: {}", get_os(&sys));

  // Number of CPUs:
  println!("Num CPUs: {}", get_cpu_count(&sys));

  println!("=> system:{}", style::Reset);

  watch_ram(&mut sys);
}
