use dbus::arg;

use super::{Mode, UnitStatusDTO};

#[async_trait::async_trait]
pub trait Systemd1Manager {
    async fn get_unit(&self, name: &str) -> Result<dbus::Path<'static>, dbus_tree::MethodErr>;
    async fn get_unit_by_pid(&self, pid: u32) -> Result<dbus::Path<'static>, dbus_tree::MethodErr>;
    async fn load_unit(&self, name: &str) -> Result<dbus::Path<'static>, dbus_tree::MethodErr>;
    // mode strings: replace, fail, isolate, ignore-dependencies, ignore-requirement
    async fn start_unit(&self, name: &str, mode: &Mode) -> Result<dbus::Path<'static>, dbus_tree::MethodErr>;
    async fn start_unit_replace(
        &self,
        old_unit: &str,
        new_unit: &str,
        mode: &Mode,
    ) -> Result<dbus::Path<'static>, dbus_tree::MethodErr>;
    async fn stop_unit(&self, name: &str, mode: &Mode) -> Result<dbus::Path<'static>, dbus_tree::MethodErr>;
    async fn reload_unit(&self, name: &str, mode: &Mode) -> Result<dbus::Path<'static>, dbus_tree::MethodErr>;
    async fn restart_unit(&self, name: &str, mode: &Mode) -> Result<dbus::Path<'static>, dbus_tree::MethodErr>;
    async fn try_restart_unit(&self, name: &str, mode: &Mode) -> Result<dbus::Path<'static>, dbus_tree::MethodErr>;
    async fn reload_or_restart_unit(
        &self,
        name: &str,
        mode: &Mode,
    ) -> Result<dbus::Path<'static>, dbus_tree::MethodErr>;
    async fn reload_or_try_restart_unit(
        &self,
        name: &str,
        mode: &Mode,
    ) -> Result<dbus::Path<'static>, dbus_tree::MethodErr>;
    async fn kill_unit(&self, name: &str, who: &str, signal: i32) -> Result<(), dbus_tree::MethodErr>;
    async fn reset_failed_unit(&self, name: &str) -> Result<(), dbus_tree::MethodErr>;
    async fn set_unit_properties(
        &self,
        name: &str,
        runtime: bool,
        properties: Vec<(&str, arg::Variant<Box<dyn arg::RefArg>>)>,
    ) -> Result<(), dbus_tree::MethodErr>;
    async fn start_transient_unit(
        &self,
        name: &str,
        mode: &Mode,
        properties: Vec<(&str, arg::Variant<Box<dyn arg::RefArg>>)>,
        aux: Vec<(&str, Vec<(&str, arg::Variant<Box<dyn arg::RefArg>>)>)>,
    ) -> Result<dbus::Path<'static>, dbus_tree::MethodErr>;
    async fn get_job(&self, id: u32) -> Result<dbus::Path<'static>, dbus_tree::MethodErr>;
    async fn cancel_job(&self, id: u32) -> Result<(), dbus_tree::MethodErr>;
    async fn clear_jobs(&self) -> Result<(), dbus_tree::MethodErr>;
    async fn reset_failed(&self) -> Result<(), dbus_tree::MethodErr>;
    async fn list_units(&self) -> Result<Vec<UnitStatusDTO>, dbus_tree::MethodErr>;
    async fn list_units_filtered(&self, names: Vec<&str>) -> Result<Vec<UnitStatusDTO>, dbus_tree::MethodErr>;
    async fn list_jobs(
        &self,
    ) -> Result<Vec<(u32, String, String, String, dbus::Path<'static>, dbus::Path<'static>)>, dbus_tree::MethodErr>;
    async fn subscribe(&self) -> Result<(), dbus_tree::MethodErr>;
    async fn unsubscribe(&self) -> Result<(), dbus_tree::MethodErr>;
    async fn dump(&self) -> Result<String, dbus_tree::MethodErr>;
    async fn create_snapshot(&self, name: &str, cleanup: bool) -> Result<dbus::Path<'static>, dbus_tree::MethodErr>;
    async fn remove_snapshot(&self, name: &str) -> Result<(), dbus_tree::MethodErr>;
    async fn reload(&self) -> Result<(), dbus_tree::MethodErr>;
    async fn re_execute(&self) -> Result<(), dbus_tree::MethodErr>;
    async fn exit(&self) -> Result<(), dbus_tree::MethodErr>;
    async fn reboot(&self) -> Result<(), dbus_tree::MethodErr>;
    async fn power_off(&self) -> Result<(), dbus_tree::MethodErr>;
    async fn halt(&self) -> Result<(), dbus_tree::MethodErr>;
    async fn kexec(&self) -> Result<(), dbus_tree::MethodErr>;
    async fn switch_root(&self, new_root: &str, init: &str) -> Result<(), dbus_tree::MethodErr>;
    async fn set_environment_(&self, names: Vec<&str>) -> Result<(), dbus_tree::MethodErr>;
    async fn unset_environment(&self, names: Vec<&str>) -> Result<(), dbus_tree::MethodErr>;
    async fn unset_and_set_environment(
        &self,
        env_to_unset: Vec<&str>,
        env_to_set: Vec<&str>,
    ) -> Result<(), dbus_tree::MethodErr>;
    async fn list_unit_files(&self) -> Result<Vec<(String, String)>, dbus_tree::MethodErr>;
    async fn get_unit_file_state(&self, file_path: &str) -> Result<String, dbus_tree::MethodErr>;
    async fn enable_unit_files(
        &self,
        files: Vec<&str>,
        runtime: bool,
        force: bool,
    ) -> Result<(bool, Vec<(String, String, String)>), dbus_tree::MethodErr>;
    async fn disable_unit_files(
        &self,
        files: Vec<&str>,
        runtime: bool,
    ) -> Result<Vec<(String, String, String)>, dbus_tree::MethodErr>;
    async fn re_enable_unit_files(
        &self,
        files: Vec<&str>,
        runtime: bool,
        force: bool,
    ) -> Result<(bool, Vec<(String, String, String)>), dbus_tree::MethodErr>;
    async fn link_unit_files(
        &self,
        files: Vec<&str>,
        runtime: bool,
        force: bool,
    ) -> Result<Vec<(String, String, String)>, dbus_tree::MethodErr>;
    async fn preset_unit_files(
        &self,
        files: Vec<&str>,
        runtime: bool,
        force: bool,
    ) -> Result<(bool, Vec<(String, String, String)>), dbus_tree::MethodErr>;
    async fn mask_unit_files(
        &self,
        files: Vec<&str>,
        runtime: bool,
        force: bool,
    ) -> Result<Vec<(String, String, String)>, dbus_tree::MethodErr>;
    async fn unmask_unit_files(
        &self,
        files: Vec<&str>,
        runtime: bool,
    ) -> Result<Vec<(String, String, String)>, dbus_tree::MethodErr>;
    async fn set_default_target(
        &self,
        files: Vec<&str>,
        runtime: bool,
    ) -> Result<Vec<(String, String, String)>, dbus_tree::MethodErr>;
    async fn get_default_target(&self) -> Result<String, dbus_tree::MethodErr>;
    async fn version(&self) -> Result<String, dbus_tree::MethodErr>;
    async fn features(&self) -> Result<String, dbus_tree::MethodErr>;
    async fn virtualization(&self) -> Result<String, dbus_tree::MethodErr>;
    async fn architecture(&self) -> Result<String, dbus_tree::MethodErr>;
    async fn tainted(&self) -> Result<String, dbus_tree::MethodErr>;
    async fn firmware_timestamp(&self) -> Result<u64, dbus_tree::MethodErr>;
    async fn firmware_timestamp_monotonic(&self) -> Result<u64, dbus_tree::MethodErr>;
    async fn loader_timestamp(&self) -> Result<u64, dbus_tree::MethodErr>;
    async fn loader_timestamp_monotonic(&self) -> Result<u64, dbus_tree::MethodErr>;
    async fn kernel_timestamp(&self) -> Result<u64, dbus_tree::MethodErr>;
    async fn kernel_timestamp_monotonic(&self) -> Result<u64, dbus_tree::MethodErr>;
    async fn init_rdtimestamp(&self) -> Result<u64, dbus_tree::MethodErr>;
    async fn init_rdtimestamp_monotonic(&self) -> Result<u64, dbus_tree::MethodErr>;
    async fn userspace_timestamp(&self) -> Result<u64, dbus_tree::MethodErr>;
    async fn userspace_timestamp_monotonic(&self) -> Result<u64, dbus_tree::MethodErr>;
    async fn finish_timestamp(&self) -> Result<u64, dbus_tree::MethodErr>;
    async fn finish_timestamp_monotonic(&self) -> Result<u64, dbus_tree::MethodErr>;
    async fn security_start_timestamp(&self) -> Result<u64, dbus_tree::MethodErr>;
    async fn security_start_timestamp_monotonic(&self) -> Result<u64, dbus_tree::MethodErr>;
    async fn security_finish_timestamp(&self) -> Result<u64, dbus_tree::MethodErr>;
    async fn security_finish_timestamp_monotonic(&self) -> Result<u64, dbus_tree::MethodErr>;
    async fn generators_start_timestamp(&self) -> Result<u64, dbus_tree::MethodErr>;
    async fn generators_start_timestamp_monotonic(&self) -> Result<u64, dbus_tree::MethodErr>;
    async fn generators_finish_timestamp(&self) -> Result<u64, dbus_tree::MethodErr>;
    async fn generators_finish_timestamp_monotonic(&self) -> Result<u64, dbus_tree::MethodErr>;
    async fn units_load_start_timestamp(&self) -> Result<u64, dbus_tree::MethodErr>;
    async fn units_load_start_timestamp_monotonic(&self) -> Result<u64, dbus_tree::MethodErr>;
    async fn units_load_finish_timestamp(&self) -> Result<u64, dbus_tree::MethodErr>;
    async fn units_load_finish_timestamp_monotonic(&self) -> Result<u64, dbus_tree::MethodErr>;
    async fn log_level(&self) -> Result<String, dbus_tree::MethodErr>;
    async fn set_log_level(&self, value: String) -> Result<(), dbus_tree::MethodErr>;
    async fn log_target(&self) -> Result<String, dbus_tree::MethodErr>;
    async fn set_log_target(&self, value: String) -> Result<(), dbus_tree::MethodErr>;
    async fn nnames(&self) -> Result<u32, dbus_tree::MethodErr>;
    async fn nfailed_units(&self) -> Result<u32, dbus_tree::MethodErr>;
    async fn njobs(&self) -> Result<u32, dbus_tree::MethodErr>;
    async fn ninstalled_jobs(&self) -> Result<u32, dbus_tree::MethodErr>;
    async fn nfailed_jobs(&self) -> Result<u32, dbus_tree::MethodErr>;
    async fn progress(&self) -> Result<f64, dbus_tree::MethodErr>;
    async fn environment(&self) -> Result<Vec<String>, dbus_tree::MethodErr>;
    async fn confirm_spawn(&self) -> Result<bool, dbus_tree::MethodErr>;
    async fn show_status(&self) -> Result<bool, dbus_tree::MethodErr>;
    async fn unit_path(&self) -> Result<Vec<String>, dbus_tree::MethodErr>;
    async fn default_standard_output(&self) -> Result<String, dbus_tree::MethodErr>;
    async fn default_standard_error(&self) -> Result<String, dbus_tree::MethodErr>;
    async fn runtime_watchdog_usec(&self) -> Result<u64, dbus_tree::MethodErr>;
    async fn set_runtime_watchdog_usec(&self, value: u64) -> Result<(), dbus_tree::MethodErr>;
    async fn shutdown_watchdog_usec(&self) -> Result<u64, dbus_tree::MethodErr>;
    async fn set_shutdown_watchdog_usec(&self, value: u64) -> Result<(), dbus_tree::MethodErr>;
    async fn control_group(&self) -> Result<String, dbus_tree::MethodErr>;
    async fn system_state(&self) -> Result<String, dbus_tree::MethodErr>;
}

#[derive(Debug)]
pub struct Systemd1ManagerStartupFinished {
    pub arg0: u64,
    pub arg1: u64,
    pub arg2: u64,
    pub arg3: u64,
    pub arg4: u64,
    pub arg5: u64,
}

impl arg::AppendAll for Systemd1ManagerStartupFinished {
    fn append(&self, i: &mut arg::IterAppend<'_>) {
        arg::RefArg::append(&self.arg0, i);
        arg::RefArg::append(&self.arg1, i);
        arg::RefArg::append(&self.arg2, i);
        arg::RefArg::append(&self.arg3, i);
        arg::RefArg::append(&self.arg4, i);
        arg::RefArg::append(&self.arg5, i);
    }
}

impl arg::ReadAll for Systemd1ManagerStartupFinished {
    fn read(i: &mut arg::Iter<'_>) -> Result<Self, arg::TypeMismatchError> {
        Ok(Systemd1ManagerStartupFinished {
            arg0: i.read()?,
            arg1: i.read()?,
            arg2: i.read()?,
            arg3: i.read()?,
            arg4: i.read()?,
            arg5: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for Systemd1ManagerStartupFinished {
    const NAME: &'static str = "StartupFinished";
    const INTERFACE: &'static str = "org.freedesktop.systemd1.Manager";
}

#[derive(Debug)]
pub struct Systemd1ManagerReloading {
    pub arg0: bool,
}

impl arg::AppendAll for Systemd1ManagerReloading {
    fn append(&self, i: &mut arg::IterAppend<'_>) {
        arg::RefArg::append(&self.arg0, i);
    }
}

impl arg::ReadAll for Systemd1ManagerReloading {
    fn read(i: &mut arg::Iter<'_>) -> Result<Self, arg::TypeMismatchError> {
        Ok(Systemd1ManagerReloading { arg0: i.read()? })
    }
}

impl dbus::message::SignalArgs for Systemd1ManagerReloading {
    const NAME: &'static str = "Reloading";
    const INTERFACE: &'static str = "org.freedesktop.systemd1.Manager";
}
