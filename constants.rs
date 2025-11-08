// ...existing code...
// From Copilot: This module provides shared constants used by the tiny curl-like tool.
// From Copilot: We keep these values in a separate file so other modules (like main.rs) can `use` them.
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    // From Copilot: PROTOCOL_STRING is a global, lazily-initialized HashMap that maps
    // From Copilot: a URL scheme (like "http") to the protocol string used in the request line
    // From Copilot: (for example "HTTP/1.1"). We use lazy_static so we can create a non-const
    // From Copilot: HashMap at program start and then access it as a static reference everywhere.
    // From Copilot: The keys and values use `'static` lifetimes because they are string literals
    // From Copilot: baked into the binary for the program's lifetime.
    pub static ref PROTOCOL_STRING: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        // From Copilot: Insert the common mapping for HTTP; add more protocols here if desired.
        map.insert("http", "HTTP/1.1");
        map
    };
}
// From Copilot: DEFAULT_PORT is a simple constant string used when the user doesn't specify a port.
// From Copilot: We keep it as &str because it's a string literal and borrowing it is cheap.
pub const DEFAULT_PORT: &str = "80";
// ...existing code...