mod versions;
mod cores;
mod names;
mod mem;

pub use self::versions::get_os;
pub use self::versions::get_kernel;

pub use self::cores::get_cpu_count;

pub use self::names::get_name;
pub use self::names::get_hostname;

pub use self::mem::get_ram;
pub use self::mem::watch_ram;
