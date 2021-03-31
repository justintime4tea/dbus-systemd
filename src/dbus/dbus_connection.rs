use dbus::{arg, nonblock::SyncConnection};
use dbus_tokio::connection::new_system_sync;
use futures::future::{abortable, AbortHandle};
use std::{ops::Deref, sync::Arc};
use tracing::{debug, info};

#[derive(Clone)]
pub struct DBusConnection {
    connection: Arc<SyncConnection>,
    connection_abort_handle: AbortHandle,
}

impl std::fmt::Debug for DBusConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DbusConnection")
            .field("connection_abort_handle", &self.connection_abort_handle)
            .finish()
    }
}

impl DBusConnection {
    pub fn new() -> Self {
        let (resource, connection) = new_system_sync().unwrap();
        let (abortable_resource, connection_abort_handle) = abortable(resource);

        tokio::spawn(async {
            let err = abortable_resource.await;
            if let Err(e) = err {
                debug!("{:?}", e);
            }
        });

        DBusConnection {
            connection,
            connection_abort_handle,
        }
    }
}

impl Drop for DBusConnection {
    fn drop(&mut self) {
        info!("dropping")
        // QUESTION: When to abort?
        // ? If we abort on drop than the connection can't be used in the future (from pool)
        // ? Abort should happen on "graceful shutdown" but how ?
        // self.connection_abort_handle.abort()
    }
}

impl Deref for DBusConnection {
    type Target = SyncConnection;

    fn deref(&self) -> &Self::Target {
        &self.connection
    }
}

#[async_trait::async_trait]
pub trait DBusPeer {
    async fn ping(&self) -> Result<(), dbus_tree::MethodErr>;
    async fn get_machine_id(&self) -> Result<String, dbus_tree::MethodErr>;
}

#[async_trait::async_trait]
pub trait DBusIntrospectable {
    async fn introspect(&self) -> Result<String, dbus_tree::MethodErr>;
}

#[async_trait::async_trait]
pub trait DBusProperties {
    async fn get(
        &self,
        interface: &str,
        property: &str,
    ) -> Result<arg::Variant<Box<dyn arg::RefArg + 'static>>, dbus_tree::MethodErr>;
    async fn get_all(&self, interface: &str) -> Result<arg::PropMap, dbus_tree::MethodErr>;
    async fn set(
        &self,
        interface: &str,
        property: &str,
        value: arg::Variant<Box<dyn arg::RefArg>>,
    ) -> Result<(), dbus_tree::MethodErr>;
}

#[derive(Debug)]
pub struct DBusPropertiesPropertiesChanged {
    pub interface: String,
    pub changed_properties: arg::PropMap,
    pub invalidated_properties: Vec<String>,
}

impl arg::AppendAll for DBusPropertiesPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend<'_>) {
        arg::RefArg::append(&self.interface, i);
        arg::RefArg::append(&self.changed_properties, i);
        arg::RefArg::append(&self.invalidated_properties, i);
    }
}

impl arg::ReadAll for DBusPropertiesPropertiesChanged {
    fn read(i: &mut arg::Iter<'_>) -> Result<Self, arg::TypeMismatchError> {
        Ok(DBusPropertiesPropertiesChanged {
            interface: i.read()?,
            changed_properties: i.read()?,
            invalidated_properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for DBusPropertiesPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
}
