use anyhow::{anyhow, Result};
use dbus::nonblock::Proxy as DBusProxy;
use deadpool::managed::{Manager as ConnectionManager, PoolConfig, RecycleError, RecycleResult};
use std::time::Duration;

use super::DBusConnection;

pub type DBusConnectionPool = deadpool::managed::Pool<DBusConnection, anyhow::Error>;

#[derive(Clone, Debug)]
pub struct DbusConnectionManager;

impl DbusConnectionManager {
    pub async fn make_dbus_proxy(
        destination: String,
        path: String,
        pool: &'_ DBusConnectionPool,
    ) -> Result<DBusProxy<'_, DBusConnection>> {
        match pool.get().await {
            Ok(connection) => {
                let connection = connection.clone();
                Ok(DBusProxy::new(destination, path, Duration::from_secs(10), connection))
            }
            Err(e) => Err(anyhow!("{:?}", e)),
        }
    }
}

#[async_trait::async_trait]
impl ConnectionManager<DBusConnection, anyhow::Error> for DbusConnectionManager {
    async fn create(&self) -> Result<DBusConnection, anyhow::Error> {
        Ok(DBusConnection::new())
    }

    // This is called when the pool is about to recycle a connection; a return of Ok(()) means its ok to recycle.
    async fn recycle(&self, conn: &mut DBusConnection) -> RecycleResult<anyhow::Error> {
        let proxy = DBusProxy::new("", "/org/freedesktop/DBus", Duration::from_secs(1), conn.clone());

        proxy
            .method_call("org.freedesktop.DBus.Peer", "Ping", ())
            .await
            .map_err(|e| RecycleError::from(anyhow::Error::from(e)))
    }
}
