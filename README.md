# Usage

```bash
cargo add rust_logger
```

```rs
use rust_logger::{Logger, Severity};

fn main() {
    println!("Hello, world!");
    let logger = Logger::new("type_name");
    logger.info("Howdy");
    logger.warning("Howdy", Severity::Medium);
    logger.error("Howdy", Severity::Critical);
    logger.debug("Howdy");
}
```

If you want to see the debug log level you can set `DEBUG=true` as an
environment variable.
