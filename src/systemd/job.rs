use dbus::arg;

pub type JobDto = (u32, String, String, String, dbus::Path<'static>, dbus::Path<'static>);

pub struct Job {
    job_id: u32,
    name: String,
    job_type: String,
    job_state: String,
    job_path: dbus::Path<'static>,
    path: dbus::Path<'static>,
}

impl From<JobDto> for Job {
    fn from(j: JobDto) -> Self {
        Self {
            job_id: j.0,
            name: j.1.clone(),
            job_type: j.2,
            job_state: j.3.clone(),
            job_path: j.4.clone(),
            path: j.5.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Systemd1ManagerJobNew {
    pub id: u32,
    pub job_path: dbus::Path<'static>,
    pub unit: String,
}

impl arg::AppendAll for Systemd1ManagerJobNew {
    fn append(&self, i: &mut arg::IterAppend<'_>) {
        arg::RefArg::append(&self.id, i);
        arg::RefArg::append(&self.job_path, i);
        arg::RefArg::append(&self.unit, i);
    }
}

impl arg::ReadAll for Systemd1ManagerJobNew {
    fn read(i: &mut arg::Iter<'_>) -> Result<Self, arg::TypeMismatchError> {
        Ok(Systemd1ManagerJobNew {
            id: i.read()?,
            job_path: i.read()?,
            unit: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for Systemd1ManagerJobNew {
    const NAME: &'static str = "JobNew";
    const INTERFACE: &'static str = "org.freedesktop.systemd1.Manager";
}

#[derive(Debug)]
pub struct Systemd1ManagerJobRemoved {
    pub id: u32,
    pub job_path: dbus::Path<'static>,
    pub unit: String,
    pub result: String,
}

impl arg::AppendAll for Systemd1ManagerJobRemoved {
    fn append(&self, i: &mut arg::IterAppend<'_>) {
        arg::RefArg::append(&self.id, i);
        arg::RefArg::append(&self.job_path, i);
        arg::RefArg::append(&self.unit, i);
        arg::RefArg::append(&self.result, i);
    }
}

impl arg::ReadAll for Systemd1ManagerJobRemoved {
    fn read(i: &mut arg::Iter<'_>) -> Result<Self, arg::TypeMismatchError> {
        Ok(Systemd1ManagerJobRemoved {
            id: i.read()?,
            job_path: i.read()?,
            unit: i.read()?,
            result: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for Systemd1ManagerJobRemoved {
    const NAME: &'static str = "JobRemoved";
    const INTERFACE: &'static str = "org.freedesktop.systemd1.Manager";
}
