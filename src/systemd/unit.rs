use dbus::arg;

pub type UnitStatusDTO = (
    String,
    String,
    String,
    String,
    String,
    String,
    dbus::Path<'static>,
    u32,
    String,
    dbus::Path<'static>,
);

#[derive(Clone, Debug)]
pub struct SystemdUnitStatus {
    pub name: String,
    pub description: String,
    pub loaded: String,
    pub active: String,
    pub status: String,
    pub hwid: String,
    pub object_path: dbus::Path<'static>,
    pub jod_id: u32,
    pub job_type: String,
    pub job_path: dbus::Path<'static>,
}

impl From<UnitStatusDTO> for SystemdUnitStatus {
    fn from(u: UnitStatusDTO) -> Self {
        Self {
            name: u.0.clone(),
            description: u.1.clone(),
            loaded: u.2.clone(),
            active: u.3.clone(),
            status: u.4.clone(),
            hwid: u.5.clone(),
            object_path: u.6.clone(),
            jod_id: u.7,
            job_type: u.8.clone(),
            job_path: u.9.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Systemd1ManagerUnitNew {
    pub arg0: String,
    pub arg1: dbus::Path<'static>,
}

impl arg::AppendAll for Systemd1ManagerUnitNew {
    fn append(&self, i: &mut arg::IterAppend<'_>) {
        arg::RefArg::append(&self.arg0, i);
        arg::RefArg::append(&self.arg1, i);
    }
}

impl arg::ReadAll for Systemd1ManagerUnitNew {
    fn read(i: &mut arg::Iter<'_>) -> Result<Self, arg::TypeMismatchError> {
        Ok(Systemd1ManagerUnitNew {
            arg0: i.read()?,
            arg1: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for Systemd1ManagerUnitNew {
    const NAME: &'static str = "UnitNew";
    const INTERFACE: &'static str = "org.freedesktop.systemd1.Manager";
}

#[derive(Debug)]
pub struct Systemd1ManagerUnitRemoved {
    pub arg0: String,
    pub arg1: dbus::Path<'static>,
}

impl arg::AppendAll for Systemd1ManagerUnitRemoved {
    fn append(&self, i: &mut arg::IterAppend<'_>) {
        arg::RefArg::append(&self.arg0, i);
        arg::RefArg::append(&self.arg1, i);
    }
}

impl arg::ReadAll for Systemd1ManagerUnitRemoved {
    fn read(i: &mut arg::Iter<'_>) -> Result<Self, arg::TypeMismatchError> {
        Ok(Systemd1ManagerUnitRemoved {
            arg0: i.read()?,
            arg1: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for Systemd1ManagerUnitRemoved {
    const NAME: &'static str = "UnitRemoved";
    const INTERFACE: &'static str = "org.freedesktop.systemd1.Manager";
}

#[derive(Clone, Debug)]
pub struct Systemd1ManagerUnitFilesChanged;

impl arg::AppendAll for Systemd1ManagerUnitFilesChanged {
    fn append(&self, _: &mut arg::IterAppend<'_>) {}
}

impl arg::ReadAll for Systemd1ManagerUnitFilesChanged {
    fn read(_: &mut arg::Iter<'_>) -> Result<Self, arg::TypeMismatchError> {
        Ok(Systemd1ManagerUnitFilesChanged)
    }
}

impl dbus::message::SignalArgs for Systemd1ManagerUnitFilesChanged {
    const NAME: &'static str = "UnitFilesChanged";
    const INTERFACE: &'static str = "org.freedesktop.systemd1.Manager";
}
