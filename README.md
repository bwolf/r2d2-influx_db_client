# r2d2-influx_db_client &emsp; [![Latest Version]][crates.io]

[crates.io]: https://crates.io/crates/r2d2-influx_db_client

[influx_db_client][influx_db_client] support library for the [r2d2][r2d2] connection pool. 


## Install
Add to `Cargo.toml`:

    influx_db_client = "0.3.6"
    r2d2 = "0.8"
    r2d2-influx_db_client = "0.1.0"


## Example

```rust
use std::time::Duration;

use r2d2_influx_db_client::{Authentication, InfluxDbConnectionManager};

fn main() {
    let con_mgr = InfluxDbConnectionManager::new("localhost", 8086, "tutorial");
    let pool = r2d2::Pool::builder()
        .connection_timeout(Duration::from_secs(1))
        .test_on_check_out(true)
        .max_size(15)
        .build(connection_manager)
        .expect("Pool");

    // Use pool...
}
```

[r2d2]: https://github.com/sfackler/r2d2
[influx_db_client]: https://github.com/driftluo/InfluxDBClient-rs]