#![allow(unused)]
use std::path::{Path, PathBuf};
use util::{read_file, write_file};

pub mod radeon;

/// Base trait that implements kernel parameter get/set capabilities.
pub trait KernelParameter {
    const NAME: &'static str;

    fn get_path(&self) -> &Path;

    fn get(&self) -> Option<String> {
        let path = self.get_path();
        if path.exists() {
            match read_file(path) {
                Ok(value) => return Some(value),
                Err(why) => {
                    eprintln!("{}: failed to get value: {}", path.display(), why)
                }
            }
        } else {
            eprintln!("{} does not exist", path.display());
        }

        None
    }

    fn set(&self, value: &[u8]) {
        let path = self.get_path();
        if path.exists() {
            if let Err(why) = write_file(path, value) {
                eprintln!("{}: failed to set value: {}", path.display(), why)
            }
        } else {
            eprintln!("{} does not exist", path.display());
        }
    }
}

// Macros to help with constructing kernel parameter structures.

macro_rules! static_parameters {
    ($($struct:tt { $name:tt : $path:expr }),+) => (
        $(
            pub struct $struct;

            impl $struct { pub fn new() -> $struct { $struct } }

            impl KernelParameter for $struct {
                const NAME: &'static str = stringify!($name);

                fn get_path<'a>(&'a self) -> &'a Path {
                    Path::new(stringify!($path))
                }
            }
        )+
    );
}

macro_rules! dynamic_parameters {
    ($($struct:tt { $name:tt : $format:expr }),+) => (
        $(
            pub struct $struct {
                path: PathBuf
            }

            impl $struct {
                pub fn new(unique: &str) -> $struct {
                    $struct {
                        path: PathBuf::from(format!($format, unique))
                    }
                }
            }

            impl KernelParameter for $struct {
                const NAME: &'static str = stringify!($name);

                fn get_path<'a>(&'a self) -> &'a Path { &self.path }
            }
        )+
    );
}

// Kernel parameters which implement the base trait.

static_parameters! {
    DirtyExpire { dirty_expire: "/proc/sys/vm/dirty_expire_centisecs" },
    DirtyWriteback { dirty_writeback: "/proc/sys/vm/dirty_writeback_centisecs" },
    NmiWatchdog { nmi_watchdog : "/proc/sys/kernel/nmi_watchdog" },
    PcieAspm { pcie_aspm: "/sys/module/pcie_aspm/parameters/policy" }
}

dynamic_parameters! {
    DiskIoSched { disk_io_scheduler: "/sys/block/{}/queue/scheduler" },
    PhcControls { phc_controls: "/sys/devices/system/cpu/cpu{}/cpufreq/phc_controls" },
    RadeonDpmState { radeon_dpm_state: "{}/device/power_dpm_state" },
    RadeonDpmForcePerformance {
        radeon_dpm_force_performance_level: "{}/device/power_dpm_force_performance_level"
    },
    RadeonPowerMethod { radeon_power_method: "{}/power_method" },
    RadeonPowerProfile { radeon_power_profile: "{}/power_profile" },
    SndPowerSave { snd_hda_intel_power_save: "/sys/module/{}/parameters/power_save" },
    SndPowerSaveController {
        snd_hda_intel_power_save_controller: "/sys/module/{}/parameters/power_save_controller"
    }
}
