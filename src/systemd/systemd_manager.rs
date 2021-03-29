use dbus::Path as DbusPath;
use strum::{AsRefStr, AsStaticStr, IntoStaticStr};

use crate::dbus::{DBusConnectionPool, DbusConnectionManager};

use super::{JobDTO, Mode, Systemd1Manager, UnitStatusDTO};

#[derive(Clone)]
pub struct SystemdManager {
    connection_pool: DBusConnectionPool,
}

impl std::fmt::Debug for SystemdManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SystemdManager").finish()
    }
}

impl SystemdManager {
    pub fn default() -> Self {
        Self {
            connection_pool: DBusConnectionPool::new(DbusConnectionManager, 16),
        }
    }
}

#[async_trait::async_trait]
impl Systemd1Manager for SystemdManager {
    async fn get_unit(&self, name: &str) -> Result<DbusPath<'static>, dbus::MethodErr> {
        match DbusConnectionManager::make_dbus_proxy(SYSTEMD.service.into(), SYSTEMD.path.into(), &self.connection_pool)
            .await
        {
            Ok(proxy) => match proxy.method_call(SYSTEMD.interface, "GetUnit", (name,)).await {
                Ok((path,)) => {
                    let path: DbusPath<'_> = path;
                    Ok(path)
                }
                Err(e) => Err(dbus::MethodErr::from(e)),
            },
            Err(e) => {
                let message = format!("{:?}", e);
                Err(dbus::MethodErr::failed(&message))
            }
        }
    }

    async fn get_unit_by_pid(&self, pid: u32) -> Result<DbusPath<'static>, dbus::MethodErr> {
        match DbusConnectionManager::make_dbus_proxy(SYSTEMD.service.into(), SYSTEMD.path.into(), &self.connection_pool)
            .await
        {
            Ok(proxy) => match proxy.method_call(SYSTEMD.interface, "GetUnitByPid", (pid,)).await {
                Ok((path,)) => {
                    let path: DbusPath<'_> = path;
                    Ok(path)
                }
                Err(e) => Err(dbus::MethodErr::from(e)),
            },
            Err(e) => {
                let message = format!("{:?}", e);
                Err(dbus::MethodErr::failed(&message))
            }
        }
    }

    async fn load_unit(&self, name: &str) -> Result<DbusPath<'static>, dbus::MethodErr> {
        todo!()
    }

    async fn start_unit(&self, name: &str, mode: &Mode) -> Result<DbusPath<'static>, dbus::MethodErr> {
        match DbusConnectionManager::make_dbus_proxy(SYSTEMD.service.into(), SYSTEMD.path.into(), &self.connection_pool)
            .await
        {
            Ok(proxy) => {
                match proxy
                    .method_call(
                        SYSTEMD.interface,
                        "StartUnit",
                        (name, <&'static str>::from(mode).to_lowercase().as_str()),
                    )
                    .await
                {
                    Ok((path,)) => {
                        let path: DbusPath<'_> = path;
                        Ok(path)
                    }
                    Err(e) => Err(dbus::MethodErr::from(e)),
                }
            }
            Err(e) => {
                let message = format!("{:?}", e);
                Err(dbus::MethodErr::failed(&message))
            }
        }
    }

    async fn start_unit_replace(
        &self,
        old_unit: &str,
        new_unit: &str,
        mode: &Mode,
    ) -> Result<DbusPath<'static>, dbus::MethodErr> {
        match DbusConnectionManager::make_dbus_proxy(SYSTEMD.service.into(), SYSTEMD.path.into(), &self.connection_pool)
            .await
        {
            Ok(proxy) => match proxy
                .method_call(
                    SYSTEMD.interface,
                    "StartUnitReplace",
                    (old_unit, new_unit, <&'static str>::from(mode).to_lowercase().as_str()),
                )
                .await
            {
                Ok((path,)) => {
                    let path: DbusPath<'_> = path;
                    Ok(path)
                }
                Err(e) => Err(dbus::MethodErr::from(e)),
            },
            Err(e) => {
                let message = format!("{:?}", e);
                Err(dbus::MethodErr::failed(&message))
            }
        }
    }

    async fn stop_unit(&self, name: &str, mode: &Mode) -> Result<DbusPath<'static>, dbus::MethodErr> {
        match DbusConnectionManager::make_dbus_proxy(SYSTEMD.service.into(), SYSTEMD.path.into(), &self.connection_pool)
            .await
        {
            Ok(proxy) => match proxy
                .method_call(
                    SYSTEMD.interface,
                    "StopUnit",
                    (name, <&'static str>::from(mode).to_lowercase().as_str()),
                )
                .await
            {
                Ok((path,)) => {
                    let path: DbusPath<'_> = path;
                    Ok(path)
                }
                Err(e) => Err(dbus::MethodErr::from(e)),
            },
            Err(e) => {
                let message = format!("{:?}", e);
                Err(dbus::MethodErr::failed(&message))
            }
        }
    }

    async fn reload_unit(&self, name: &str, mode: &Mode) -> Result<DbusPath<'static>, dbus::MethodErr> {
        match DbusConnectionManager::make_dbus_proxy(SYSTEMD.service.into(), SYSTEMD.path.into(), &self.connection_pool)
            .await
        {
            Ok(proxy) => match proxy
                .method_call(
                    SYSTEMD.interface,
                    "ReloadUnit",
                    (name, <&'static str>::from(mode).to_lowercase().as_str()),
                )
                .await
            {
                Ok((path,)) => {
                    let path: DbusPath<'_> = path;
                    Ok(path)
                }
                Err(e) => Err(dbus::MethodErr::from(e)),
            },
            Err(e) => {
                let message = format!("{:?}", e);
                Err(dbus::MethodErr::failed(&message))
            }
        }
    }

    async fn restart_unit(&self, name: &str, mode: &Mode) -> Result<DbusPath<'static>, dbus::MethodErr> {
        match DbusConnectionManager::make_dbus_proxy(SYSTEMD.service.into(), SYSTEMD.path.into(), &self.connection_pool)
            .await
        {
            Ok(proxy) => match proxy
                .method_call(
                    SYSTEMD.interface,
                    "RestartUnit",
                    (name, <&'static str>::from(mode).to_lowercase().as_str()),
                )
                .await
            {
                Ok((path,)) => {
                    let path: DbusPath<'_> = path;
                    Ok(path)
                }
                Err(e) => Err(dbus::MethodErr::from(e)),
            },
            Err(e) => {
                let message = format!("{:?}", e);
                Err(dbus::MethodErr::failed(&message))
            }
        }
    }

    async fn try_restart_unit(&self, name: &str, mode: &Mode) -> Result<DbusPath<'static>, dbus::MethodErr> {
        match DbusConnectionManager::make_dbus_proxy(SYSTEMD.service.into(), SYSTEMD.path.into(), &self.connection_pool)
            .await
        {
            Ok(proxy) => match proxy
                .method_call(
                    SYSTEMD.interface,
                    "TryRestartUnit",
                    (name, <&'static str>::from(mode).to_lowercase().as_str()),
                )
                .await
            {
                Ok((path,)) => {
                    let path: DbusPath<'_> = path;
                    Ok(path)
                }
                Err(e) => Err(dbus::MethodErr::from(e)),
            },
            Err(e) => {
                let message = format!("{:?}", e);
                Err(dbus::MethodErr::failed(&message))
            }
        }
    }

    async fn reload_or_restart_unit(&self, name: &str, mode: &Mode) -> Result<DbusPath<'static>, dbus::MethodErr> {
        match DbusConnectionManager::make_dbus_proxy(SYSTEMD.service.into(), SYSTEMD.path.into(), &self.connection_pool)
            .await
        {
            Ok(proxy) => match proxy
                .method_call(
                    SYSTEMD.interface,
                    "ReloadOrRestartUnit",
                    (name, <&'static str>::from(mode).to_lowercase().as_str()),
                )
                .await
            {
                Ok((path,)) => {
                    let path: DbusPath<'_> = path;
                    Ok(path)
                }
                Err(e) => Err(dbus::MethodErr::from(e)),
            },
            Err(e) => {
                let message = format!("{:?}", e);
                Err(dbus::MethodErr::failed(&message))
            }
        }
    }

    async fn reload_or_try_restart_unit(&self, name: &str, mode: &Mode) -> Result<DbusPath<'static>, dbus::MethodErr> {
        match DbusConnectionManager::make_dbus_proxy(SYSTEMD.service.into(), SYSTEMD.path.into(), &self.connection_pool)
            .await
        {
            Ok(proxy) => match proxy
                .method_call(
                    SYSTEMD.interface,
                    "ReloadOrTryRestartUnit",
                    (name, <&'static str>::from(mode).to_lowercase().as_str()),
                )
                .await
            {
                Ok((path,)) => {
                    let path: DbusPath<'_> = path;
                    Ok(path)
                }
                Err(e) => Err(dbus::MethodErr::from(e)),
            },
            Err(e) => {
                let message = format!("{:?}", e);
                Err(dbus::MethodErr::failed(&message))
            }
        }
    }

    async fn kill_unit(&self, name: &str, who: &str, signal: i32) -> Result<(), dbus::MethodErr> {
        match DbusConnectionManager::make_dbus_proxy(SYSTEMD.service.into(), SYSTEMD.path.into(), &self.connection_pool)
            .await
        {
            Ok(proxy) => match proxy
                .method_call(SYSTEMD.interface, "KillUnit", (name, who, signal))
                .await
            {
                Ok(unit) => {
                    let unit: () = unit;
                    Ok(())
                }
                Err(e) => Err(dbus::MethodErr::from(e)),
            },
            Err(e) => {
                let message = format!("{:?}", e);
                Err(dbus::MethodErr::failed(&message))
            }
        }
    }

    async fn reset_failed_unit(&self, name: &str) -> Result<(), dbus::MethodErr> {
        match DbusConnectionManager::make_dbus_proxy(SYSTEMD.service.into(), SYSTEMD.path.into(), &self.connection_pool)
            .await
        {
            Ok(proxy) => match proxy.method_call(SYSTEMD.interface, "ResetFailedUnit", (name,)).await {
                Ok(unit) => {
                    let unit: () = unit;
                    Ok(())
                }
                Err(e) => Err(dbus::MethodErr::from(e)),
            },
            Err(e) => {
                let message = format!("{:?}", e);
                Err(dbus::MethodErr::failed(&message))
            }
        }
    }

    async fn set_unit_properties(
        &self,
        name: &str,
        runtime: bool,
        properties: Vec<(&str, dbus::arg::Variant<Box<dyn dbus::arg::RefArg>>)>,
    ) -> Result<(), dbus::MethodErr> {
        match DbusConnectionManager::make_dbus_proxy(SYSTEMD.service.into(), SYSTEMD.path.into(), &self.connection_pool)
            .await
        {
            Ok(proxy) => match proxy
                .method_call(SYSTEMD.interface, "SetUnitProperties", (name, runtime, properties))
                .await
            {
                Ok(unit) => {
                    let unit: () = unit;
                    Ok(())
                }
                Err(e) => Err(dbus::MethodErr::from(e)),
            },
            Err(e) => {
                let message = format!("{:?}", e);
                Err(dbus::MethodErr::failed(&message))
            }
        }
    }

    async fn start_transient_unit(
        &self,
        name: &str,
        mode: &Mode,
        properties: Vec<(&str, dbus::arg::Variant<Box<dyn dbus::arg::RefArg>>)>,
        aux: Vec<(&str, Vec<(&str, dbus::arg::Variant<Box<dyn dbus::arg::RefArg>>)>)>,
    ) -> Result<DbusPath<'static>, dbus::MethodErr> {
        match DbusConnectionManager::make_dbus_proxy(SYSTEMD.service.into(), SYSTEMD.path.into(), &self.connection_pool)
            .await
        {
            Ok(proxy) => match proxy
                .method_call(
                    SYSTEMD.interface,
                    "StartTransientUnit",
                    (
                        name,
                        <&'static str>::from(mode).to_lowercase().as_str(),
                        properties,
                        aux,
                    ),
                )
                .await
            {
                Ok((path,)) => {
                    let path: DbusPath<'_> = path;
                    Ok(path)
                }
                Err(e) => Err(dbus::MethodErr::from(e)),
            },
            Err(e) => {
                let message = format!("{:?}", e);
                Err(dbus::MethodErr::failed(&message))
            }
        }
    }

    async fn get_job(&self, id: u32) -> Result<DbusPath<'static>, dbus::MethodErr> {
        match DbusConnectionManager::make_dbus_proxy(SYSTEMD.service.into(), SYSTEMD.path.into(), &self.connection_pool)
            .await
        {
            Ok(proxy) => match proxy.method_call(SYSTEMD.interface, "GetJob", (id,)).await {
                Ok((path,)) => {
                    let path: DbusPath<'_> = path;
                    Ok(path)
                }
                Err(e) => Err(dbus::MethodErr::from(e)),
            },
            Err(e) => {
                let message = format!("{:?}", e);
                Err(dbus::MethodErr::failed(&message))
            }
        }
    }

    async fn cancel_job(&self, id: u32) -> Result<(), dbus::MethodErr> {
        match DbusConnectionManager::make_dbus_proxy(SYSTEMD.service.into(), SYSTEMD.path.into(), &self.connection_pool)
            .await
        {
            Ok(proxy) => match proxy.method_call(SYSTEMD.interface, "CancelJob", (id,)).await {
                Ok(unit) => {
                    let unit: () = unit;
                    Ok(())
                }
                Err(e) => Err(dbus::MethodErr::from(e)),
            },
            Err(e) => {
                let message = format!("{:?}", e);
                Err(dbus::MethodErr::failed(&message))
            }
        }
    }

    async fn clear_jobs(&self) -> Result<(), dbus::MethodErr> {
        match DbusConnectionManager::make_dbus_proxy(SYSTEMD.service.into(), SYSTEMD.path.into(), &self.connection_pool)
            .await
        {
            Ok(proxy) => match proxy.method_call(SYSTEMD.interface, "ClearJobs", ()).await {
                Ok(unit) => {
                    let unit: () = unit;
                    Ok(())
                }
                Err(e) => Err(dbus::MethodErr::from(e)),
            },
            Err(e) => {
                let message = format!("{:?}", e);
                Err(dbus::MethodErr::failed(&message))
            }
        }
    }

    async fn reset_failed(&self) -> Result<(), dbus::MethodErr> {
        match DbusConnectionManager::make_dbus_proxy(SYSTEMD.service.into(), SYSTEMD.path.into(), &self.connection_pool)
            .await
        {
            Ok(proxy) => match proxy.method_call(SYSTEMD.interface, "ResetFailed", ()).await {
                Ok(unit) => {
                    let unit: () = unit;
                    Ok(())
                }
                Err(e) => Err(dbus::MethodErr::from(e)),
            },
            Err(e) => {
                let message = format!("{:?}", e);
                Err(dbus::MethodErr::failed(&message))
            }
        }
    }

    async fn list_units(&self) -> Result<Vec<UnitStatusDTO>, dbus::MethodErr> {
        match DbusConnectionManager::make_dbus_proxy(SYSTEMD.service.into(), SYSTEMD.path.into(), &self.connection_pool)
            .await
        {
            Ok(proxy) => match proxy.method_call(SYSTEMD.interface, "ListUnits", ()).await {
                Ok((units,)) => {
                    let units: Vec<UnitStatusDTO> = units;
                    Ok(units)
                }
                Err(e) => Err(dbus::MethodErr::from(e)),
            },
            Err(e) => {
                let message = format!("{:?}", e);
                Err(dbus::MethodErr::failed(&message))
            }
        }
    }

    async fn list_units_filtered(&self, names: Vec<&str>) -> Result<Vec<UnitStatusDTO>, dbus::MethodErr> {
        match DbusConnectionManager::make_dbus_proxy(SYSTEMD.service.into(), SYSTEMD.path.into(), &self.connection_pool)
            .await
        {
            Ok(proxy) => match proxy
                .method_call(SYSTEMD.interface, "ListUnitsFiltered", (names,))
                .await
            {
                Ok((units,)) => {
                    let units: Vec<UnitStatusDTO> = units;
                    Ok(units)
                }
                Err(e) => Err(dbus::MethodErr::from(e)),
            },
            Err(e) => {
                let message = format!("{:?}", e);
                Err(dbus::MethodErr::failed(&message))
            }
        }
    }

    async fn list_jobs(&self) -> Result<Vec<JobDTO>, dbus::MethodErr> {
        match DbusConnectionManager::make_dbus_proxy(SYSTEMD.service.into(), SYSTEMD.path.into(), &self.connection_pool)
            .await
        {
            Ok(proxy) => match proxy.method_call(SYSTEMD.interface, "ListJobs", ()).await {
                Ok((jobs,)) => {
                    let jobs: Vec<JobDTO> = jobs;
                    Ok(jobs)
                }
                Err(e) => Err(dbus::MethodErr::from(e)),
            },
            Err(e) => {
                let message = format!("{:?}", e);
                Err(dbus::MethodErr::failed(&message))
            }
        }
    }

    async fn subscribe(&self) -> Result<(), dbus::MethodErr> {
        match DbusConnectionManager::make_dbus_proxy(SYSTEMD.service.into(), SYSTEMD.path.into(), &self.connection_pool)
            .await
        {
            Ok(proxy) => match proxy.method_call(SYSTEMD.interface, "Subscribe", ()).await {
                Ok(unit) => {
                    let unit: () = unit;
                    Ok(())
                }
                Err(e) => Err(dbus::MethodErr::from(e)),
            },
            Err(e) => {
                let message = format!("{:?}", e);
                Err(dbus::MethodErr::failed(&message))
            }
        }
    }

    async fn unsubscribe(&self) -> Result<(), dbus::MethodErr> {
        match DbusConnectionManager::make_dbus_proxy(SYSTEMD.service.into(), SYSTEMD.path.into(), &self.connection_pool)
            .await
        {
            Ok(proxy) => match proxy.method_call(SYSTEMD.interface, "Unsubscribe", ()).await {
                Ok(unit) => {
                    let unit: () = unit;
                    Ok(())
                }
                Err(e) => Err(dbus::MethodErr::from(e)),
            },
            Err(e) => {
                let message = format!("{:?}", e);
                Err(dbus::MethodErr::failed(&message))
            }
        }
    }

    async fn dump(&self) -> Result<String, dbus::MethodErr> {
        match DbusConnectionManager::make_dbus_proxy(SYSTEMD.service.into(), SYSTEMD.path.into(), &self.connection_pool)
            .await
        {
            Ok(proxy) => match proxy.method_call(SYSTEMD.interface, "Dump", ()).await {
                Ok((some_string,)) => {
                    let some_string: String = some_string;
                    Ok(some_string)
                }
                Err(e) => Err(dbus::MethodErr::from(e)),
            },
            Err(e) => {
                let message = format!("{:?}", e);
                Err(dbus::MethodErr::failed(&message))
            }
        }
    }

    async fn create_snapshot(&self, name: &str, cleanup: bool) -> Result<DbusPath<'static>, dbus::MethodErr> {
        todo!()
    }

    async fn remove_snapshot(&self, name: &str) -> Result<(), dbus::MethodErr> {
        todo!()
    }

    async fn reload(&self) -> Result<(), dbus::MethodErr> {
        match DbusConnectionManager::make_dbus_proxy(SYSTEMD.service.into(), SYSTEMD.path.into(), &self.connection_pool)
            .await
        {
            Ok(proxy) => match proxy.method_call(SYSTEMD.interface, "Reload", ()).await {
                Ok(unit) => {
                    let unit: () = unit;
                    Ok(())
                }
                Err(e) => Err(dbus::MethodErr::from(e)),
            },
            Err(e) => {
                let message = format!("{:?}", e);
                Err(dbus::MethodErr::failed(&message))
            }
        }
    }

    async fn re_execute(&self) -> Result<(), dbus::MethodErr> {
        todo!()
    }

    async fn exit(&self) -> Result<(), dbus::MethodErr> {
        match DbusConnectionManager::make_dbus_proxy(SYSTEMD.service.into(), SYSTEMD.path.into(), &self.connection_pool)
            .await
        {
            Ok(proxy) => match proxy.method_call(SYSTEMD.interface, "Exit", ()).await {
                Ok(unit) => {
                    let unit: () = unit;
                    Ok(())
                }
                Err(e) => Err(dbus::MethodErr::from(e)),
            },
            Err(e) => {
                let message = format!("{:?}", e);
                Err(dbus::MethodErr::failed(&message))
            }
        }
    }

    async fn reboot(&self) -> Result<(), dbus::MethodErr> {
        match DbusConnectionManager::make_dbus_proxy(SYSTEMD.service.into(), SYSTEMD.path.into(), &self.connection_pool)
            .await
        {
            Ok(proxy) => match proxy.method_call(SYSTEMD.interface, "Reboot", ()).await {
                Ok(unit) => {
                    let unit: () = unit;
                    Ok(())
                }
                Err(e) => Err(dbus::MethodErr::from(e)),
            },
            Err(e) => {
                let message = format!("{:?}", e);
                Err(dbus::MethodErr::failed(&message))
            }
        }
    }

    async fn power_off(&self) -> Result<(), dbus::MethodErr> {
        match DbusConnectionManager::make_dbus_proxy(SYSTEMD.service.into(), SYSTEMD.path.into(), &self.connection_pool)
            .await
        {
            Ok(proxy) => match proxy.method_call(SYSTEMD.interface, "PowerOff", ()).await {
                Ok(unit) => {
                    let unit: () = unit;
                    Ok(())
                }
                Err(e) => Err(dbus::MethodErr::from(e)),
            },
            Err(e) => {
                let message = format!("{:?}", e);
                Err(dbus::MethodErr::failed(&message))
            }
        }
    }

    async fn halt(&self) -> Result<(), dbus::MethodErr> {
        match DbusConnectionManager::make_dbus_proxy(SYSTEMD.service.into(), SYSTEMD.path.into(), &self.connection_pool)
            .await
        {
            Ok(proxy) => match proxy.method_call(SYSTEMD.interface, "Halt", ()).await {
                Ok(unit) => {
                    let unit: () = unit;
                    Ok(())
                }
                Err(e) => Err(dbus::MethodErr::from(e)),
            },
            Err(e) => {
                let message = format!("{:?}", e);
                Err(dbus::MethodErr::failed(&message))
            }
        }
    }

    async fn kexec(&self) -> Result<(), dbus::MethodErr> {
        match DbusConnectionManager::make_dbus_proxy(SYSTEMD.service.into(), SYSTEMD.path.into(), &self.connection_pool)
            .await
        {
            Ok(proxy) => match proxy.method_call(SYSTEMD.interface, "Kexec", ()).await {
                Ok(unit) => {
                    let unit: () = unit;
                    Ok(())
                }
                Err(e) => Err(dbus::MethodErr::from(e)),
            },
            Err(e) => {
                let message = format!("{:?}", e);
                Err(dbus::MethodErr::failed(&message))
            }
        }
    }

    async fn switch_root(&self, new_root: &str, init: &str) -> Result<(), dbus::MethodErr> {
        match DbusConnectionManager::make_dbus_proxy(SYSTEMD.service.into(), SYSTEMD.path.into(), &self.connection_pool)
            .await
        {
            Ok(proxy) => match proxy
                .method_call(SYSTEMD.interface, "SwitchRoot", (new_root, init))
                .await
            {
                Ok(unit) => {
                    let unit: () = unit;
                    Ok(())
                }
                Err(e) => Err(dbus::MethodErr::from(e)),
            },
            Err(e) => {
                let message = format!("{:?}", e);
                Err(dbus::MethodErr::failed(&message))
            }
        }
    }

    async fn set_environment_(&self, names: Vec<&str>) -> Result<(), dbus::MethodErr> {
        match DbusConnectionManager::make_dbus_proxy(SYSTEMD.service.into(), SYSTEMD.path.into(), &self.connection_pool)
            .await
        {
            Ok(proxy) => match proxy.method_call(SYSTEMD.interface, "SetEnvironment", (names,)).await {
                Ok(unit) => {
                    let unit: () = unit;
                    Ok(())
                }
                Err(e) => Err(dbus::MethodErr::from(e)),
            },
            Err(e) => {
                let message = format!("{:?}", e);
                Err(dbus::MethodErr::failed(&message))
            }
        }
    }

    async fn unset_environment(&self, names: Vec<&str>) -> Result<(), dbus::MethodErr> {
        match DbusConnectionManager::make_dbus_proxy(SYSTEMD.service.into(), SYSTEMD.path.into(), &self.connection_pool)
            .await
        {
            Ok(proxy) => match proxy.method_call(SYSTEMD.interface, "UnsetEnvironment", (names,)).await {
                Ok(unit) => {
                    let unit: () = unit;
                    Ok(())
                }
                Err(e) => Err(dbus::MethodErr::from(e)),
            },
            Err(e) => {
                let message = format!("{:?}", e);
                Err(dbus::MethodErr::failed(&message))
            }
        }
    }

    async fn unset_and_set_environment(
        &self,
        env_to_unset: Vec<&str>,
        env_to_set: Vec<&str>,
    ) -> Result<(), dbus::MethodErr> {
        todo!()
    }

    async fn list_unit_files(&self) -> Result<Vec<(String, String)>, dbus::MethodErr> {
        todo!()
    }

    async fn get_unit_file_state(&self, file_path: &str) -> Result<String, dbus::MethodErr> {
        todo!()
    }

    async fn enable_unit_files(
        &self,
        files: Vec<&str>,
        runtime: bool,
        force: bool,
    ) -> Result<(bool, Vec<(String, String, String)>), dbus::MethodErr> {
        todo!()
    }

    async fn disable_unit_files(
        &self,
        files: Vec<&str>,
        runtime: bool,
    ) -> Result<Vec<(String, String, String)>, dbus::MethodErr> {
        todo!()
    }

    async fn re_enable_unit_files(
        &self,
        files: Vec<&str>,
        runtime: bool,
        force: bool,
    ) -> Result<(bool, Vec<(String, String, String)>), dbus::MethodErr> {
        todo!()
    }

    async fn link_unit_files(
        &self,
        files: Vec<&str>,
        runtime: bool,
        force: bool,
    ) -> Result<Vec<(String, String, String)>, dbus::MethodErr> {
        todo!()
    }

    async fn preset_unit_files(
        &self,
        files: Vec<&str>,
        runtime: bool,
        force: bool,
    ) -> Result<(bool, Vec<(String, String, String)>), dbus::MethodErr> {
        todo!()
    }

    async fn mask_unit_files(
        &self,
        files: Vec<&str>,
        runtime: bool,
        force: bool,
    ) -> Result<Vec<(String, String, String)>, dbus::MethodErr> {
        todo!()
    }

    async fn unmask_unit_files(
        &self,
        files: Vec<&str>,
        runtime: bool,
    ) -> Result<Vec<(String, String, String)>, dbus::MethodErr> {
        todo!()
    }

    async fn set_default_target(
        &self,
        files: Vec<&str>,
        runtime: bool,
    ) -> Result<Vec<(String, String, String)>, dbus::MethodErr> {
        todo!()
    }

    async fn get_default_target(&self) -> Result<String, dbus::MethodErr> {
        todo!()
    }

    async fn version(&self) -> Result<String, dbus::MethodErr> {
        todo!()
    }

    async fn features(&self) -> Result<String, dbus::MethodErr> {
        todo!()
    }

    async fn virtualization(&self) -> Result<String, dbus::MethodErr> {
        todo!()
    }

    async fn architecture(&self) -> Result<String, dbus::MethodErr> {
        todo!()
    }

    async fn tainted(&self) -> Result<String, dbus::MethodErr> {
        todo!()
    }

    async fn firmware_timestamp(&self) -> Result<u64, dbus::MethodErr> {
        todo!()
    }

    async fn firmware_timestamp_monotonic(&self) -> Result<u64, dbus::MethodErr> {
        todo!()
    }

    async fn loader_timestamp(&self) -> Result<u64, dbus::MethodErr> {
        todo!()
    }

    async fn loader_timestamp_monotonic(&self) -> Result<u64, dbus::MethodErr> {
        todo!()
    }

    async fn kernel_timestamp(&self) -> Result<u64, dbus::MethodErr> {
        todo!()
    }

    async fn kernel_timestamp_monotonic(&self) -> Result<u64, dbus::MethodErr> {
        todo!()
    }

    async fn init_rdtimestamp(&self) -> Result<u64, dbus::MethodErr> {
        todo!()
    }

    async fn init_rdtimestamp_monotonic(&self) -> Result<u64, dbus::MethodErr> {
        todo!()
    }

    async fn userspace_timestamp(&self) -> Result<u64, dbus::MethodErr> {
        todo!()
    }

    async fn userspace_timestamp_monotonic(&self) -> Result<u64, dbus::MethodErr> {
        todo!()
    }

    async fn finish_timestamp(&self) -> Result<u64, dbus::MethodErr> {
        todo!()
    }

    async fn finish_timestamp_monotonic(&self) -> Result<u64, dbus::MethodErr> {
        todo!()
    }

    async fn security_start_timestamp(&self) -> Result<u64, dbus::MethodErr> {
        todo!()
    }

    async fn security_start_timestamp_monotonic(&self) -> Result<u64, dbus::MethodErr> {
        todo!()
    }

    async fn security_finish_timestamp(&self) -> Result<u64, dbus::MethodErr> {
        todo!()
    }

    async fn security_finish_timestamp_monotonic(&self) -> Result<u64, dbus::MethodErr> {
        todo!()
    }

    async fn generators_start_timestamp(&self) -> Result<u64, dbus::MethodErr> {
        todo!()
    }

    async fn generators_start_timestamp_monotonic(&self) -> Result<u64, dbus::MethodErr> {
        todo!()
    }

    async fn generators_finish_timestamp(&self) -> Result<u64, dbus::MethodErr> {
        todo!()
    }

    async fn generators_finish_timestamp_monotonic(&self) -> Result<u64, dbus::MethodErr> {
        todo!()
    }

    async fn units_load_start_timestamp(&self) -> Result<u64, dbus::MethodErr> {
        todo!()
    }

    async fn units_load_start_timestamp_monotonic(&self) -> Result<u64, dbus::MethodErr> {
        todo!()
    }

    async fn units_load_finish_timestamp(&self) -> Result<u64, dbus::MethodErr> {
        todo!()
    }

    async fn units_load_finish_timestamp_monotonic(&self) -> Result<u64, dbus::MethodErr> {
        todo!()
    }

    async fn log_level(&self) -> Result<String, dbus::MethodErr> {
        todo!()
    }

    async fn set_log_level(&self, value: String) -> Result<(), dbus::MethodErr> {
        match DbusConnectionManager::make_dbus_proxy(SYSTEMD.service.into(), SYSTEMD.path.into(), &self.connection_pool)
            .await
        {
            Ok(proxy) => match proxy.method_call(SYSTEMD.interface, "SetLogLevel", (value,)).await {
                Ok(unit) => {
                    let unit: () = unit;
                    Ok(())
                }
                Err(e) => Err(dbus::MethodErr::from(e)),
            },
            Err(e) => {
                let message = format!("{:?}", e);
                Err(dbus::MethodErr::failed(&message))
            }
        }
    }

    async fn log_target(&self) -> Result<String, dbus::MethodErr> {
        todo!()
    }

    async fn set_log_target(&self, value: String) -> Result<(), dbus::MethodErr> {
        match DbusConnectionManager::make_dbus_proxy(SYSTEMD.service.into(), SYSTEMD.path.into(), &self.connection_pool)
            .await
        {
            Ok(proxy) => match proxy.method_call(SYSTEMD.interface, "SetLogTarget", (value,)).await {
                Ok(unit) => {
                    let unit: () = unit;
                    Ok(())
                }
                Err(e) => Err(dbus::MethodErr::from(e)),
            },
            Err(e) => {
                let message = format!("{:?}", e);
                Err(dbus::MethodErr::failed(&message))
            }
        }
    }

    async fn nnames(&self) -> Result<u32, dbus::MethodErr> {
        todo!()
    }

    async fn nfailed_units(&self) -> Result<u32, dbus::MethodErr> {
        todo!()
    }

    async fn njobs(&self) -> Result<u32, dbus::MethodErr> {
        todo!()
    }

    async fn ninstalled_jobs(&self) -> Result<u32, dbus::MethodErr> {
        todo!()
    }

    async fn nfailed_jobs(&self) -> Result<u32, dbus::MethodErr> {
        todo!()
    }

    async fn progress(&self) -> Result<f64, dbus::MethodErr> {
        todo!()
    }

    async fn environment(&self) -> Result<Vec<String>, dbus::MethodErr> {
        todo!()
    }

    async fn confirm_spawn(&self) -> Result<bool, dbus::MethodErr> {
        todo!()
    }

    async fn show_status(&self) -> Result<bool, dbus::MethodErr> {
        todo!()
    }

    async fn unit_path(&self) -> Result<Vec<String>, dbus::MethodErr> {
        todo!()
    }

    async fn default_standard_output(&self) -> Result<String, dbus::MethodErr> {
        todo!()
    }

    async fn default_standard_error(&self) -> Result<String, dbus::MethodErr> {
        todo!()
    }

    async fn runtime_watchdog_usec(&self) -> Result<u64, dbus::MethodErr> {
        todo!()
    }

    async fn set_runtime_watchdog_usec(&self, value: u64) -> Result<(), dbus::MethodErr> {
        match DbusConnectionManager::make_dbus_proxy(SYSTEMD.service.into(), SYSTEMD.path.into(), &self.connection_pool)
            .await
        {
            Ok(proxy) => match proxy
                .method_call(SYSTEMD.interface, "SetRuntimeWatchdogUsec", (value,))
                .await
            {
                Ok(unit) => {
                    let unit: () = unit;
                    Ok(())
                }
                Err(e) => Err(dbus::MethodErr::from(e)),
            },
            Err(e) => {
                let message = format!("{:?}", e);
                Err(dbus::MethodErr::failed(&message))
            }
        }
    }

    async fn shutdown_watchdog_usec(&self) -> Result<u64, dbus::MethodErr> {
        todo!()
    }

    async fn set_shutdown_watchdog_usec(&self, value: u64) -> Result<(), dbus::MethodErr> {
        todo!()
    }

    async fn control_group(&self) -> Result<String, dbus::MethodErr> {
        todo!()
    }

    async fn system_state(&self) -> Result<String, dbus::MethodErr> {
        todo!()
    }
}

#[derive(Debug)]
struct Object {
    service: &'static str,
    path: &'static str,
    interface: &'static str,
}

static SYSTEMD: &Object = &Object {
    service: "org.freedesktop.systemd1",
    path: "/org/freedesktop/systemd1",
    interface: "org.freedesktop.systemd1.Manager",
};

static SYSTEMD_SERVICE: &Object = &Object {
    service: SYSTEMD.service,
    path: "/",
    interface: "org.freedesktop.systemd1.Service",
};

static SYSTEMD_UNIT: &Object = &Object {
    service: SYSTEMD.service,
    path: "/",
    interface: "org.freedesktop.systemd1.Unit",
};
