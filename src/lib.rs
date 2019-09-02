use influx_db_client::Client;
use std::fmt;

/// Authentication properties for connecting to InfluxDB.
#[derive(Debug)]
pub struct Authentication {
    username: String,
    password: String,
}

impl Authentication {
    pub fn new<T>(username: T, password: T) -> Self
    where
        T: ToString,
    {
        Authentication {
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

/// Encapsulates the InfluxDB connection properties.
pub struct InfluxDbConnectionManager {
    host: String,
    port: u16,
    db_name: String,
    authentication: Option<Authentication>,
}

impl InfluxDbConnectionManager {

    /// Create a connection manager with just hostname and port.
    pub fn new<T>(host: T, port: u16, db_name: T) -> Self
    where
        T: ToString,
    {
        Self {
            host: host.to_string(),
            port,
            db_name: db_name.to_string(),
            authentication: None,
        }
    }

    /// Create a connection manager with username and password authentication.
    pub fn new_with_authentication<T>(host: T, port: u16, db_name: T, auth: Authentication) -> Self
    where
        T: ToString,
    {
        Self {
            host: host.to_string(),
            port,
            db_name: db_name.to_string(),
            authentication: Some(auth),
        }
    }

    pub fn connect_new(&self) -> Client {
        let addr = format!("http://{}:{}", self.host, self.port);
        let client = Client::new(addr, self.db_name.clone());

        if let Some(auth) = &self.authentication {
            client.set_authentication(&auth.username, &auth.password)
        } else {
            client
        }
    }
}

#[derive(Debug)]
pub struct InfluxDbConnectionManagerError;

impl std::error::Error for InfluxDbConnectionManagerError {
    fn description(&self) -> &str {
        "InfluxDb connection manager error"
    }
}

impl fmt::Display for InfluxDbConnectionManagerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot connect or access InfluxDb")
    }
}

impl r2d2::ManageConnection for InfluxDbConnectionManager {
    type Connection = influx_db_client::Client;
    type Error = InfluxDbConnectionManagerError;

    fn connect(&self) -> Result<Self::Connection, Self::Error> {
        Ok(self.connect_new())
    }

    fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        if conn.ping() {
            Ok(())
        } else {
            Err(InfluxDbConnectionManagerError)
        }
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        self.is_valid(conn).is_err()
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::sync::Once;
    use std::time::Duration;

    use super::{Authentication, InfluxDbConnectionManager};

    static INIT: Once = Once::new();

    fn setup() {
        INIT.call_once(|| {
            if env::var("RUST_LOG").is_err() {
                env::set_var("RUST_LOG", "debug");
            }
            env_logger::init();
        })
    }

    fn make_pool(
        connection_manager: InfluxDbConnectionManager,
    ) -> r2d2::Pool<InfluxDbConnectionManager> {
        return r2d2::Pool::builder()
            .connection_timeout(Duration::from_secs(1))
            .test_on_check_out(true)
            .max_size(15)
            .build(connection_manager)
            .expect("Pool");
    }

    #[test]
    fn pool_without_authentication() {
        setup();
        let con_mgr = InfluxDbConnectionManager::new("localhost", 8086, "tutorial");
        let _pool = make_pool(con_mgr);
    }

    #[test]
    fn pool_with_authentication() {
        setup();
        let auth = Authentication::new("username", "password");
        let con_mgr =
            InfluxDbConnectionManager::new_with_authentication("localhost", 8086, "tutorial", auth);
        let _pool = make_pool(con_mgr);
    }
}
